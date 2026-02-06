# DUMBIT 

**Decision Utility for Managed Balance, Intent & Time**

```
  _____   _    _  __  __  ____   _____  _______ 
 |  __ \ | |  | ||  \/  ||  _ \ |_   _||__   __|
 | |  | || |  | || \  / || |_) |  | |     | |   
 | |  | || |  | || |\/| ||  _ <   | |     | |   
 | |__| || |__| || |  | || |_) | _| |_    | |   
 |_____/  \____/ |_|  |_||____/ |_____|   |_|   
       D E C I S I O N   U T I L I T Y
```

## Vision Document

### Project Name & Overview

**DUMBIT** (Decision Utility for Managed Balance, Intent & Time) is a high-discipline CLI tool that acts as your cognitive gatekeeper. Unlike traditional task managers that optimistically accept every commitment, DUMBIT enforces physiological and psychological limits by intelligently rejecting tasks that would lead to over-commitment and burnout.

Built in Rust for performance and reliability, DUMBIT uses a sophisticated Decision Engine to evaluate your capacity in real-time, ensuring you only commit to what you can actually accomplish.

---

### Problem It Solves

#### The Over-Commitment Crisis

Modern professionals face a critical challenge: **over-commitment**. We say "yes" to too many things, leading to:

- **Burnout** from unrealistic workloads
- **Broken promises** when we can't deliver
- **Chronic stress** from impossible schedules
- **Guilt and anxiety** about incomplete tasks
- **Loss of credibility** when deadlines are missed

Traditional task managers **enable this problem** by accepting every task you add, regardless of whether you have the capacity to complete them.

#### The DUMBIT Solution

DUMBIT solves this by acting as a **Cognitive Gatekeeper**:

1. **Validates capacity** before accepting tasks
2. **Rejects impossible commitments** proactively
3. **Enforces discipline** through a reliability system
4. **Provides instant feedback** on capacity limits
5. **Prevents burnout** by protecting your time

Instead of helping you "do more," DUMBIT helps you **commit realistically** and **deliver consistently**.

---

### Target Users (Personas)

#### Persona 1: Alex - The Overwhelmed Developer

**Demographics:**
- Age: 28
- Role: Full-stack software engineer
- Experience: 5 years
- Work style: Remote, flexible hours

**Pain Points:**
- Accepts too many tickets in sprint planning
- Constantly works overtime to catch up
- Feels guilty about saying "no" to stakeholders
- Struggles with work-life balance

**Goals:**
- Better estimate realistic workload
- Stop over-committing to managers
- Finish work during normal hours
- Build credibility through consistent delivery

**How DUMBIT Helps:**
- Decision Engine rejects tasks that exceed capacity
- Reliability score rewards consistent completion
- Visual feedback on remaining daily hours
- Historical rejection log reveals patterns

---

#### Persona 2: Maya - The Ambitious Student

**Demographics:**
- Age: 21
- Role: Computer Science major
- Context: Junior year, part-time job
- Work style: Balancing classes, assignments, job, social life

**Pain Points:**
- Takes on too many extracurriculars
- Pulls all-nighters regularly
- Misses deadlines despite good intentions
- Feels constant pressure and stress

**Goals:**
- Manage academic and work commitments
- Maintain healthy sleep schedule
- Actually complete what she starts
- Reduce stress and anxiety

**How DUMBIT Helps:**
- Daily hour limits prevent over-scheduling
- Deadline validation ensures realistic planning
- Weekly view shows capacity at a glance
- Rejection feedback teaches capacity awareness

---

#### Persona 3: Jordan - The Freelance Consultant

**Demographics:**
- Age: 35
- Role: Independent business consultant
- Context: Multiple clients, variable workload
- Work style: Project-based, client-driven

**Pain Points:**
- Accepts clients without capacity check
- Double-books time across projects
- Reputation suffers from missed deadlines
- Income uncertainty drives over-commitment

**Goals:**
- Professional reputation management
- Sustainable client load
- Predictable delivery timelines
- Work-life boundaries

**How DUMBIT Helps:**
- Task domains organize by client/project
- Capacity validation before accepting work
- Reliability score tracks professional consistency
- Historical data for better future estimates

---

### Vision Statement

> **"Enable professionals to make realistic commitments and deliver consistently by enforcing physiological and psychological capacity limits through intelligent gatekeeping."**

We envision a world where:
- People commit only to what they can actually accomplish
- Burnout is prevented through proactive capacity management
- Reliability and credibility are built through consistent delivery
- Technology enforces discipline instead of enabling over-commitment
- Mental health is protected by respecting human limits

DUMBIT doesn't help you "do more" — it helps you **do what you commit to, every time**.

---

### Key Features / Goals

#### Core Features (v2.0)

1. **Cognitive Gatekeeper**
   - Automatic capacity validation
   - Real-time load calculation
   - Rejection with clear reasoning
   - Protection against over-commitment

2. **Decision Engine**
   - Daily capacity limits
   - Weekly capacity planning
   - Deadline validation (no past dates)
   - Impossible task detection

3. **Reliability System**
   - Dynamic trust score (0-100%)
   - Rewards for task completion (+5%)
   - Strictness tax for low reliability (<60%)
   - Behavioral incentive system

4. **Task Management**
   - Add tasks with estimated hours
   - View daily/weekly schedules
   - Mark tasks complete
   - Track rejection history
   - Delete tasks

5. **Profile Configuration**
   - Customizable daily limits
   - Customizable weekly limits
   - Persistent settings
   - Default values for new users

6. **Local-First Data**
   - Embedded SurrealDB storage
   - No cloud dependency
   - Complete privacy
   - Instant access

7. **Professional CLI**
   - Colored output
   - Formatted tables
   - Clear error messages
   - Intuitive commands

#### Future Goals (Roadmap)

- **Analytics Dashboard**: Visualize capacity trends
- **Smart Recommendations**: ML-based time estimates
- **Team Edition**: Shared capacity planning
- **Calendar Integration**: Sync with Google/Outlook
- **Mobile Companion**: iOS/Android apps
- **Web Interface**: Browser-based access
- **API**: Integrate with other tools

---

### Success Metrics

#### User Success Metrics

1. **Commitment Accuracy Rate**
   - Target: 90% of accepted tasks completed on time
   - Measurement: Completed tasks / Accepted tasks
   - Timeline: 30-day rolling average

2. **Reliability Score Improvement**
   - Target: Average user reliability >80% within 2 weeks
   - Measurement: Weekly reliability score trends
   - Success: Upward trajectory over time

3. **Over-Commitment Reduction**
   - Target: 50% reduction in rejected tasks over 4 weeks
   - Measurement: Weekly rejection count trends
   - Success: Users learning capacity limits

4. **User Retention**
   - Target: 70% weekly active users (WAU)
   - Measurement: Unique users running commands per week
   - Timeline: 8-week observation period

5. **Burnout Prevention**
   - Target: <10% of users hitting daily limits consistently
   - Measurement: Users at 100% capacity ≥3 days/week
   - Success: Healthy capacity distribution

#### Product Success Metrics

1. **Adoption Rate**
   - Target: 1,000 downloads in first 3 months
   - Measurement: Cargo install counts, GitHub clones
   - Channels: crates.io, GitHub releases

2. **User Engagement**
   - Target: Average 5 commands/day per active user
   - Measurement: Command usage analytics (opt-in)
   - Success: Daily habit formation

3. **Feature Usage**
   - Core features (add, today, complete): >90% adoption
   - Advanced features (domains, difficulty): >40% adoption
   - Measurement: Command frequency analysis

4. **Community Growth**
   - Target: 100 GitHub stars in 6 months
   - Secondary: 20+ community contributions
   - Measurement: GitHub metrics

5. **Reliability System Effectiveness**
   - Target: 80% of users with reliability >70%
   - Measurement: Database snapshots
   - Success: System encourages good behavior

---

### 🔍 Assumptions & Constraints

#### Assumptions

1. **User Behavior**
   - Users are willing to accept rejection of over-commitments
   - Users will provide honest time estimates
   - Users understand their own capacity limits (or will learn)
   - Users value consistency over quantity

2. **Technical**
   - Users have Rust toolchain installed (or will install)
   - Users are comfortable with CLI tools
   - Users have write permissions in working directory
   - Users are on supported platforms (macOS, Linux, Windows)

3. **Usage Patterns**
   - Users check tasks at least once daily
   - Users plan work in advance (not just-in-time)
   - Users work relatively consistent hours
   - Users complete most tasks they accept

4. **Market**
   - Demand exists for discipline-enforcing tools
   - Users are willing to be told "no"
   - CLI tools are acceptable for productivity
   - Local-first approach is valued for privacy

#### Constraints

1. **Technical Constraints**
   - **Platform**: CLI-only (no GUI in v2.0)
   - **Database**: SurrealDB with RocksDB backend (local only)
   - **Language**: Rust (steep learning curve for contributors)
   - **Deployment**: User must build from source or use Cargo
   - **Network**: No cloud sync (future feature)

2. **Resource Constraints**
   - **Team**: Solo developer (limited velocity)
   - **Budget**: Zero budget (open source)
   - **Time**: Side project (non-full-time development)
   - **Infrastructure**: No servers, no hosting costs

3. **Design Constraints**
   - **Simplicity**: Must remain simple and fast
   - **Privacy**: No data collection, no telemetry
   - **Offline**: Must work completely offline
   - **Performance**: Sub-second response times

4. **User Experience Constraints**
   - **Learning Curve**: CLI knowledge required
   - **Installation**: Requires Rust ecosystem familiarity
   - **Configuration**: Manual setup (no installer)
   - **Updates**: Manual cargo install --force

5. **Scope Constraints**
   - **v2.0 Focus**: Core gatekeeper functionality only
   - **No Integrations**: Calendar, email, etc. deferred
   - **No Collaboration**: Single-user only
   - **No Analytics**: Privacy-first approach

6. **Legal/Compliance**
   - **MIT License**: Open source, permissive
   - **No Warranty**: Provided as-is
   - **Privacy**: No data collection = no GDPR concerns
   - **Liability**: Users responsible for their commitments

---

## Architecture Overview

```
┌─────────────────────────────────────────────────┐
│              CLI Interface (Clap)               │
│  (add, today, week, complete, delete, etc.)     │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│           Decision Engine (Rust)                │
│  • Capacity Validation                          │
│  • Date Validation                              │
│  • Reliability Tax System                       │
└────────────────┬────────────────────────────────┘
                 │
                 ▼
┌─────────────────────────────────────────────────┐
│         Data Layer (SurrealDB)                  │
│  • Task Storage                                 │
│  • Settings Storage                             │
│  • RocksDB Backend (Local)                      │
└─────────────────────────────────────────────────┘
```

---

## Quick Start

### Prerequisites

- Rust 1.70+ ([Install Rust](https://rustup.rs/))
- Cargo (comes with Rust)

### Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/dumbit.git
cd dumbit

# Install globally
cargo install --path .

# Verify installation
dumbit --version
```

### First-Time Setup

```bash
# Set your daily and weekly limits
dumbit set-profile --daily 8 --weekly 40

# Add your first task
dumbit add "Complete project setup" --hours 2 --deadline 2026-02-15

# View today's schedule
dumbit today
```

---

## Usage Examples

### Adding Tasks

```bash
# Basic task
dumbit add "Fix authentication bug" --hours 3 --deadline 2026-02-20

# Task with domain and difficulty
dumbit add "Design system refactor" \
  --hours 5 \
  --deadline 2026-02-25 \
  --domain "Engineering" \
  --difficulty "HIGH"

# Will be ACCEPTED if capacity available
✔ ACCEPTED Design system refactor
  Cognitive load within parameters.

# Will be REJECTED if over capacity
✘ REJECTED Fix authentication bug
  REASON: Capacity full. Limit: 8.0h, Current Load: 6.5h
```

### Viewing Tasks

```bash
# Today's accepted tasks
dumbit today

# Weekly outlook
dumbit week

# Rejected tasks (learn from patterns)
dumbit rejected
```

### Completing Tasks

```bash
dumbit complete "Fix authentication bug"

✅ Task 'Fix authentication bug' marked as complete.
UPGRADE: Reliability increased: 80% → 85%
```

### Managing Profile

```bash
# Update capacity limits
dumbit set-profile --daily 6 --weekly 30

🎯 Profile updated: Daily 6h | Weekly 30h
```

---

## Development

### Project Structure

```
dumbit/
├── src/
│   └── main.rs          # Core application logic
├── Cargo.toml           # Dependencies and metadata
├── README.md            # This file
├── LICENSE              # MIT License
└── .gitignore           # Git ignore rules
```

### Building from Source

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Run tests
cargo test

# Format code
cargo fmt

# Lint code
cargo clippy
```

## Quick Start – Local Development

DUMBIT can be run locally using Docker without installing Rust.

### Build the Image
```bash
docker build -t dumbit:latest .
Run the CLI
bash
Copy code
docker run -it --rm \
  -v dumbit-data:/data \
  dumbit:latest
This runs DUMBIT in interactive mode and persists data using a local Docker volume.

Example Usage
bash
Copy code
dumbit set-profile --daily 8 --weekly 40
dumbit add "Initial setup" --hours 2 --deadline 2026-02-15
dumbit today
To reset all stored data:

bash
Copy code
docker volume rm dumbit-data
cpp
Copy code


### Technology Stack

- **Language**: Rust 2021 Edition
- **CLI Framework**: Clap 4.5 (derive API)
- **Database**: SurrealDB 2.0 (embedded RocksDB)
- **Async Runtime**: Tokio 1.40
- **Date/Time**: Chrono 0.4
- **Terminal UI**: Colored 2.1, Comfy-table 7.1

---

## Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

See [USER_STORIES.md](USER_STORIES.md) for planned features.

---

## Acknowledgments

Built with:
- [Rust](https://www.rust-lang.org/) - Systems programming language
- [SurrealDB](https://surrealdb.com/) - Multi-model database
- [Clap](https://docs.rs/clap/) - Command-line parser
- [Tokio](https://tokio.rs/) - Async runtime


