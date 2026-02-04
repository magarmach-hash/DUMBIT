# DUMBIT

**Decision Utility for Managed Balance, Intent & Time**

A high-discipline CLI tool that acts as your cognitive gatekeeper, preventing over-commitment by intelligently rejecting tasks that exceed your physiological and psychological limits.

```
  _____   _    _  __  __  ____   _____  _______ 
 |  __ \ | |  | ||  \/  ||  _ \ |_   _||__   __|
 | |  | || |  | || \  / || |_) |  | |     | |   
 | |  | || |  | || |\/| ||  _ <   | |     | |   
 | |__| || |__| || |  | || |_) | _| |_    | |   
 |_____/  \____/ |_|  |_||____/ |_____|   |_|   
       D E C I S I O N   U T I L I T Y
```

## Philosophy

DUMBIT enforces discipline through intelligent task evaluation. Instead of optimistically accepting every commitment, it uses a **Decision Engine** to analyze your capacity and reject tasks that would overload you.

## Features

- **Cognitive Gatekeeper**: Automatically rejects tasks exceeding daily capacity
- **Reliability System**: Tracks and rewards task completion with a dynamic trust score
- **Strictness Tax**: Applies stricter limits if reliability drops below 60%
- **Smart Scheduling**: Validates dates and prevents past-dated tasks
- **Local Persistence**: Uses SurrealDB with RocksDB for embedded storage
- **Beautiful CLI**: Colored output and formatted tables for easy reading

## Requirements

- Rust 1.70+ (recommended: latest stable)
- Cargo (comes with Rust)

## Installation

### Method 1: Install Globally (Recommended)

```bash
# Clone the repository
git clone https://github.com/yourusername/dumbit.git
cd dumbit

# Install globally - adds 'dumbit' command to your system
cargo install --path .

# Now use from anywhere!
dumbit set-profile --daily 8 --weekly 40
dumbit add "Write documentation" --hours 2 --deadline 2026-02-05
```

### Method 2: Development Mode

```bash
# Run without installing (inside project directory)
cargo run -- add "Write documentation" --hours 2 --deadline 2026-02-05

# Or build and use the binary directly
cargo build --release
./target/release/dumbit add "Code review" --hours 1.5 --deadline 2026-02-05
```

### After Making Code Changes

```bash
# Reinstall to update the global command
cargo install --path . --force
```

## Usage

### Setup Your Profile

First, configure your daily and weekly limits:

```bash
dumbit set-profile --daily 8.0 --weekly 40.0
```

### Adding Tasks

```bash
# Basic task
dumbit add "Fix authentication bug" --hours 3 --deadline 2026-02-10

# With domain and difficulty
dumbit add "Design system refactor" \
  --hours 5 \
  --deadline 2026-02-15 \
  --domain "Engineering" \
  --difficulty "HIGH"
```

The Decision Engine will:
- ✅ **Accept** if within capacity
- ❌ **Reject** if it would overload you

### View Your Schedule

```bash
# Today's accepted tasks
dumbit today

# Week overview
dumbit week

# See what was rejected
dumbit rejected
```

### Complete Tasks

```bash
dumbit complete "Fix authentication bug"
```

Completing tasks boosts your **reliability score** by 5% (capped at 100%).

### Delete Tasks

```bash
dumbit delete "Old task"
```

## How It Works

### Decision Engine Logic

1. **Date Validation**: Ensures deadline is valid and not in the past
2. **Capacity Check**: Verifies task doesn't exceed total daily limit
3. **Load Calculation**: Sums up already-accepted tasks for that day
4. **Reliability Tax**: If reliability < 60%, reduces effective limit by 20%
5. **Final Decision**: Accepts or rejects based on remaining capacity

### Reliability System

- Starts at 80%
- +5% for each completed task (max 100%)
- If reliability drops below 60%, the system becomes **stricter**

### Example Scenario

```
Your daily limit: 8 hours
Current reliability: 55% (below threshold!)
Effective limit: 6.4 hours (8 × 0.8)

Already accepted for Feb 5:
- Task A: 3 hours
- Task B: 2 hours
Total: 5 hours

New task request: 2 hours
Result: ❌ REJECTED (5 + 2 = 7 > 6.4)
```

## Database Schema

### `settings:global`

| Field         | Type  | Description                      |
|---------------|-------|----------------------------------|
| daily_limit   | float | Max hours per day (default: 8.0) |
| weekly_limit  | float | Max hours per week (40.0)        |
| reliability   | float | User trust score (0.0 to 1.0)    |

### `task`

| Field      | Type   | Description                          |
|------------|--------|--------------------------------------|
| title      | string | Task identifier                      |
| domain     | string | Category (Work, Personal, etc.)      |
| difficulty | string | LOW, MEDIUM, HIGH                    |
| est_hours  | float  | Estimated time                       |
| deadline   | string | ISO date (YYYY-MM-DD)                |
| status     | string | ACCEPTED, REJECTED, or COMPLETED     |

## Development

### Project Structure

```
dumbit/
├── src/
│   └── main.rs          # Core logic
├── Cargo.toml           # Dependencies
└── README.md            # Documentation
```

### Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run tests (if you add them)
cargo test
```

### Code Quality

```bash
# Format code
cargo fmt

# Lint with Clippy
cargo clippy

# Check without building
cargo check
```


## Commands Reference

| Command        | Description                              | Example                                           |
|----------------|------------------------------------------|---------------------------------------------------|
| `add`          | Propose a new task                       | `dumbit add "Task" --hours 2 --deadline 2026-02-10` |
| `today`        | View today's schedule                    | `dumbit today`                                    |
| `week`         | View weekly outlook                      | `dumbit week`                                     |
| `complete`     | Mark task done, boost reliability        | `dumbit complete "Task name"`                     |
| `delete`       | Remove task from history                 | `dumbit delete "Task name"`                       |
| `rejected`     | View tasks blocked by gatekeeper         | `dumbit rejected`                                 |
| `set-profile`  | Update capacity limits                   | `dumbit set-profile --daily 8 --weekly 40`        |

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run `cargo fmt` and `cargo clippy`
5. Submit a pull request

## License

MIT License - See LICENSE file for details

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [SurrealDB](https://surrealdb.com/) - Multi-model database
- [Clap](https://docs.rs/clap/) - Command-line parser
- [Colored](https://docs.rs/colored/) - Terminal colors
- [Comfy-table](https://docs.rs/comfy-table/) - ASCII tables

