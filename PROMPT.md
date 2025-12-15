# Project Request: Interactive CLI for Axum Project Generation ("Rust-Hipster")

## 1. Role & Objective
Act as a Senior Rust Engineer. Your task is to architect and implement an interactive CLI tool written in Rust.
**Goal:** Create a "JHipster-like" generator for the Rust ecosystem. The CLI will interview the user and scaffold a full-stack web application using **Axum** (backend) based on the user's choices.

## 2. Core Architecture
The CLI must operate on a **Template-Based Architecture**.
1.  **Interactive Mode:** Use libraries like `dialoguer` or `inquire` to ask the user configuration questions.
2.  **GitHub Integration:** The CLI should not hold all template code hardcoded strings. Instead, it must be capable of fetching/cloning specific template blueprints from a dedicated GitHub repository (or handle a modular local registry that acts like one) based on the selected options.
3.  **Template Independence:** The CLI logic must be decoupled from the template content. Changing a template's implementation should not require recompiling the CLI.

## 3. Feature Matrix (User Choices)
The generated Axum project must support the combinations of the following features. The CLI must ask for:

### A. Database
- [ ] PostgreSQL
- [ ] MySQL
- [ ] MongoDB
- [ ] SQLite

### B. Infrastructure & Caching
- [ ] Redis (Cache/Session)
- [ ] Kafka (Messaging)
- [ ] None

### C. Frontend (SPA)
The generator must create a `client/` folder with a working "Hello World" + Auth setup for:
- [ ] React
- [ ] Vue
- [ ] Svelte
- [ ] Angular

### D. Authentication
- **Strategy:**
    - [ ] Basic (Username/Password or Email/Password)
    - [ ] OAuth2
- **Providers (if OAuth2 selected):**
    - [ ] Discord
    - [ ] Google
    - [ ] Apple
    - [ ] GitHub

### E. DevOps
- [ ] Docker & Docker Compose (Must generate a `docker-compose.yml` tailored to the selected DB/Cache).

## 4. Technical Constraints & Modularity
- **Language:** Rust (2024 edition or later).
- **Web Framework:** Axum.
- **ORM:** SQLx (for SQL DBs) / mongo-rust-driver (for Mongo).
- **Modularity:**
    - The CLI itself must be structured as a Cargo Workspace.
    - **Crucial:** Create a separate crate (library) for each major template logic or strategy if possible, to keep the `main.rs` clean.
    - Ensure clean separation between the *User Interface* (CLI prompts) and the *Generator Logic* (File I/O, Git operations).

## 5. Acceptance Criteria (Measurable)
1.  **Compilation:** The CLI tool must compile with `cargo build` without errors.
2.  **Execution:** Running the CLI allows me to select "Axum + Postgres + Redis + React + GitHub Auth".
3.  **Output:** The tool successfully generates a folder structure containing:
    - A compiling Rust/Axum backend.
    - A `docker-compose.yml` that starts Postgres and Redis.
    - A `client` folder with the React boilerplate.
4.  **Code Quality:** The generated Rust code must pass `cargo clippy`.

## 6. Implementation Plan
Please break this down into steps. Start by setting up the CLI skeleton and the strategy for handling the dynamic template generation.
