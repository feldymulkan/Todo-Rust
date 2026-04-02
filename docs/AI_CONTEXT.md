# AI Context

Professional encoding guidelines and technical context for AI assistants working on the `Todo-Rust` project.

## 🛠 Coding Standards

- **Asynchronous Code**: All endpoints and database interactions must be `async` using `tokio` and `sqlx`.
- **Compile-Time Validation**: Use `sqlx::query!` and `sqlx::query_as!` instead of string-based queries where possible to catch errors at compile time.
- **Error Propagation**: Use the `Result<T, AppError>` return type for all handler functions. Prefer `?` for concise error propagation.
- **Data Encapsulation**: Use `serde` for request/response serialization. Structs or enums should be derived from `Serialize` and `Deserialize` as needed.
- **Strong Typing**: Use specific struct types (e.g., `NewTask`, `UpdateTask`) for API payloads instead of raw JSON.

## 🔄 Patterns to Follow

### Handler Pattern
All handlers should follow this signature:
```rust
pub async fn handler_name(
    State(app_state): State<AppState>,
    // JSON or Path extractors
) -> Result<Json<T> | StatusCode, AppError> { ... }
```

### Database Updates
When updating data, use the `COALESCE` pattern in SQL to support partial updates easily:
```sql
UPDATE table SET column = COALESCE($1, column) WHERE id = $2
```

## 🚀 Potential Next Steps

1.  **Pagination**: Add `limit` and `offset` to the `list_task` handler.
2.  **Authentication**: Implement JWT middleware for task access control.
3.  **Migration Files**: Move manual SQL schema to standard `migrations/` folder using `sqlx-cli`.
4.  **Unit Tests**: Add tests for handlers using `axum-test` or similar tools.
