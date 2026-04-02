# Todo-Rust

A simple and robust Todo list API built with Rust, Axum, and SQLx.

## 🚀 Features

- **Full CRUD**: Create, Read, Update, and Delete tasks.
- **Partial Updates**: Update only the title or completion status using `COALESCE`.
- **Custom Error Handling**: Clean API responses for database errors and 404s.
- **Environment Configuration**: Easy setup using `.env` files.

## 🛠 Tech Stack

- **Framework**: [Axum](https://github.com/tokio-rs/axum)
- **Database**: [PostgreSQL](https://www.postgresql.org/)
- **ORM/Driver**: [SQLx](https://github.com/launchbadge/sqlx)
- **Serialization**: [Serde](https://serde.rs/)
- **Runtime**: [Tokio](https://tokio.rs/)

## 📋 Prerequisites

- **Rust**: Install it via [rustup](https://rustup.rs/).
- **PostgreSQL**: Running instance.
- **SQLx CLI** (Optional but recommended for migrations): `cargo install sqlx-cli`

## ⚙️ Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/feldymulkan/Todo-Rust.git
    cd Todo-Rust
    ```

2.  **Configure Environment**:
    Create a `.env` file in the root directory:
    ```env
    DATABASE_URL=postgres://username:password@localhost:5432/todo_db
    ```

3.  **Database Migration**:
    If you don't have migrations, create the table manually:
    ```sql
    CREATE TABLE tasks (
        id SERIAL PRIMARY KEY,
        title TEXT NOT NULL,
        completed BOOLEAN NOT NULL DEFAULT FALSE
    );
    ```

4.  **Run the application**:
    ```bash
    cargo run
    ```
    The server will start at `http://0.0.0.0:8080`.

## 📂 Project Structure

- `src/main.rs`: Entry point and server configuration.
- `src/handlers/`: Contains the logic for processing API requests.
- `src/routes/`: Route definitions and nesting.
- `src/model/`: Data structures for tasks.
- `src/config/`: Configuration for database and other services.
- `src/error.rs`: Custom error types and response handlers.

## 📄 Documentation

- [API.md](docs/API.md) - Detailed API endpoints.
- [AI_DOCS.md](docs/AI_DOCS.md) - High-level system overview.
- [AI_CONTEXT.md](docs/AI_CONTEXT.md) - Coding patterns and context.
