# AI Documentation

This document provides a high-level overview of the `Todo-Rust` project architecture for AI agents.

## 🏗 System Architecture

The project follows a modular Layered Architecture common in Rust web development:

1.  **Transport/Routing Layer (`src/routes/`)**: Defines HTTP endpoints and maps them to handler functions using Axum.
2.  **Handler Layer (`src/handlers/`)**: Orchestrates request processing, database interaction, and response formatting.
3.  **Data Layer (`src/model/`)**: Defines the core data structures and their serialization rules.
4.  **Configuration Layer (`src/config/`)**: Manages external dependencies like database connections and environment variables.

## 🔑 Core Components

### `AppState`
Located in `src/state.rs`, this struct holds the shared state (Database Pool) and is injected into handlers via Axum's `State` extractor.

### `AppError`
Located in `src/error.rs`, a custom error enum using `thiserror`. It implements `IntoResponse` to automatically convert internal errors into standard JSON API responses.

## 🛠 Design Decisions

- **Axum**: Chosen for its type-safety and tight integration with the `tokio` runtime.
- **SQLx**: Used for asynchronous database interaction and compile-time query validation (via `query!` macros).
- **Dotenvy**: Local environment management via `.env` files.
- **PgPool**: Connection pooling is used to manage database resources efficiently.
