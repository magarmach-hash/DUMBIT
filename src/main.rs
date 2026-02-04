use clap::{Parser, Subcommand};
use colored::*;
use serde::{Deserialize, Serialize};
use surrealdb::{Surreal, engine::local::Db};
use comfy_table::Table;
use chrono::{Local, NaiveDate};

// -------------------- DATA MODELS --------------------

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Task {
    title: String,
    domain: String,
    difficulty: String,
    est_hours: f32,
    deadline: String,
    status: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UserSettings {
    daily_limit: f32,
    weekly_limit: f32,
    reliability: f32,
}

// -------------------- CLI --------------------

#[derive(Parser)]
#[command(
    name = "dumbit",
    version = "2.0-beta",
    author = "DUMBIT Team",
    about = r#"
  _____   _    _  __  __  ____   _____  _______ 
 |  __ \ | |  | ||  \/  ||  _ \ |_   _||__   __|
 | |  | || |  | || \  / || |_) |  | |     | |   
 | |  | || |  | || |\/| ||  _ <   | |     | |   
 | |__| || |__| || |  | || |_) | _| |_    | |   
 |_____/  \____/ |_|  |_||____/ |_____|   |_|   

        D E C I S I O N   U T I L I T Y
"#
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add {
        title: String,
        #[arg(short, long)]
        hours: f32,
        #[arg(short, long)]
        deadline: String,
        #[arg(short, long, default_value = "Work")]
        domain: String,
        #[arg(short = 'i', long, default_value = "MEDIUM")]
        difficulty: String,
    },
    Today,
    Week,
    Rejected,
    Complete { title: String },
    Delete { title: String },
    SetProfile {
        #[arg(long)]
        daily: f32,
        #[arg(long)]
        weekly: f32,
    },
}

// -------------------- DECISION ENGINE --------------------

async fn evaluate_task(
    db: &Surreal<Db>,
    hours: f32,
    deadline: String,
) -> Result<(bool, String), surrealdb::Error> {
    let today = Local::now().date_naive();
    let target_date = match NaiveDate::parse_from_str(&deadline, "%Y-%m-%d") {
        Ok(d) => d,
        Err(_) => return Ok((false, "Invalid date format".into())),
    };

    if target_date < today {
        return Ok((false, "Deadline is in the past".into()));
    }

    let settings: UserSettings = db
        .select(("settings", "global"))
        .await?
        .unwrap_or(UserSettings {
            daily_limit: 8.0,
            weekly_limit: 40.0,
            reliability: 0.8,
        });

    let effective_limit = if settings.reliability < 0.6 {
        settings.daily_limit * 0.8
    } else {
        settings.daily_limit
    };

    let mut res = db
        .query(
            "SELECT math::sum(est_hours) AS total
             FROM task
             WHERE deadline = $d AND status = 'ACCEPTED'",
        )
        .bind(("d", deadline))
        .await?;

    let current_load: f32 = res
        .take::<Option<f32>>((0, "total"))?
        .unwrap_or(0.0);

    if current_load + hours > effective_limit {
        return Ok((
            false,
            format!(
                "Capacity full (limit {:.1}h, used {:.1}h)",
                effective_limit, current_load
            ),
        ));
    }

    Ok((true, "Approved".into()))
}

// -------------------- MAIN --------------------

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "DUMBIT initialized".cyan().bold());

    let db = Surreal::new::<Db>(()).await?;
    db.use_ns("prod").use_db("tasks").await?;

    let cli = Cli::parse();

    match cli.command {
        Commands::Add {
            title,
            hours,
            domain,
            deadline,
            difficulty,
        } => {
            let (passed, reason) =
                evaluate_task(&db, hours, deadline.clone()).await?;

            let status = if passed { "ACCEPTED" } else { "REJECTED" };

            db.create("task")
                .content(Task {
                    title: title.clone(),
                    domain,
                    difficulty,
                    est_hours: hours,
                    deadline,
                    status: status.into(),
                })
                .await?;

            if passed {
                println!("✔ {}", title.green());
            } else {
                println!("✘ {}", title.red());
                println!("Reason: {}", reason);
            }
        }

        Commands::Today => {
            let today = Local::now().format("%Y-%m-%d").to_string();
            print_table(
                &db,
                "TODAY",
                "SELECT * FROM task WHERE deadline = $d AND status = 'ACCEPTED'",
                Some(today),
            )
            .await;
        }

        Commands::Week => {
            print_table(
                &db,
                "WEEK",
                "SELECT * FROM task WHERE status = 'ACCEPTED' ORDER BY deadline",
                None,
            )
            .await;
        }

        Commands::Rejected => {
            print_table(
                &db,
                "REJECTED",
                "SELECT * FROM task WHERE status = 'REJECTED'",
                None,
            )
            .await;
        }

        Commands::Complete { title } => {
            db.query("UPDATE task SET status = 'COMPLETED' WHERE title = $t")
                .bind(("t", title))
                .await?;
            println!("✅ Completed");
        }

        Commands::Delete { title } => {
            db.query("DELETE task WHERE title = $t")
                .bind(("t", title))
                .await?;
            println!("🗑 Deleted");
        }

        Commands::SetProfile { daily, weekly } => {
            db.upsert(("settings", "global"))
                .content(UserSettings {
                    daily_limit: daily,
                    weekly_limit: weekly,
                    reliability: 0.8,
                })
                .await?;
            println!("Profile updated");
        }
    }

    Ok(())
}

// -------------------- TABLE --------------------

async fn print_table(
    db: &Surreal<Db>,
    title: &str,
    query: &str,
    date: Option<String>,
) {
    let mut res = if let Some(d) = date {
        db.query(query).bind(("d", d)).await.unwrap()
    } else {
        db.query(query).await.unwrap()
    };

    let tasks: Vec<Task> = res.take(0).unwrap_or_default();

    if tasks.is_empty() {
        println!("{}: No tasks", title);
        return;
    }

    let mut table = Table::new();
    table.set_header(vec!["Date", "Task", "Hours", "Status"]);

    for t in tasks {
        table.add_row(vec![
            t.deadline,
            t.title,
            format!("{:.1}", t.est_hours),
            t.status,
        ]);
    }

    println!("\n{}\n{}", title.cyan().bold(), table);
}
