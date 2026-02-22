use axum::routing::{post, get, patch, delete};
use axum::{Router};
use crate::handlers::handler_tasks;
use crate::state::AppState;


pub fn task_router() -> Router<AppState> {
    Router::new()
        .route("/tasks", post(handler_tasks::create_task))
        .route("/tasks", get(handler_tasks::list_task))
        .route("/tasks/{id}", patch(handler_tasks::update_task))
        .route("/tasks/{id}", delete(handler_tasks::delete_task))
        .route("/tasks/{id}", get(handler_tasks::detail_task))
}