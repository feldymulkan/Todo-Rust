use axum::{Json, extract::{Path, State}, http::StatusCode};
use crate::state::AppState;
use crate::model::task::{Task, NewTask, UpdateTask};
use crate::error::AppError;

pub async fn create_task(
    State(app_state): State<AppState>,
    Json(payload): Json<NewTask>
) -> Result<Json<Task>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "INSERT INTO tasks (title) VALUES ($1) RETURNING id, title, completed",
        payload.title,
    ).fetch_one(&app_state.db_pool).await?;
    Ok(Json(data_result))
}

pub async fn list_task(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Task>>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks ORDER BY ID",
    )
    .fetch_all(&app_state.db_pool)
    .await?;

    Ok(Json(data_result))

}

pub async fn detail_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>
) -> Result<Json<Task>, AppError> {
    let data_result = sqlx::query_as!(
        Task,
        "SELECT id, title, completed FROM tasks WHERE id = $1",
        task_id
    )
    .fetch_one(&app_state.db_pool)
    .await?;

    Ok(Json(data_result))
}

pub async fn update_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>,
    Json(payload): Json<UpdateTask>
)-> Result<StatusCode, AppError> {
    let updated = sqlx::query!(
        "UPDATE tasks SET title = COALESCE($1, title), completed = COALESCE($2, completed) WHERE id = $3",
        payload.title,
        payload.completed,
        task_id
    )
    .execute(&app_state.db_pool)
    .await?;

    if updated.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task with ID {} not found", task_id)))
    }

    // match updated {
    //     Ok(result) => {
    //         if result.rows_affected() > 0 {
    //             StatusCode::OK
    //         } else {
    //             StatusCode::NOT_FOUND
    //         }
    //     },
    //     Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    // }

}

pub async fn delete_task(
    State(app_state): State<AppState>,
    Path(task_id): Path<i32>
) -> Result<StatusCode, AppError> {
    let deleted = sqlx::query!(
        "DELETE FROM tasks where id = $1",
        task_id
    )
    .execute(&app_state.db_pool)
    .await?;

    if deleted.rows_affected() > 0 {
        Ok(StatusCode::OK)
    } else {
        Err(AppError::NotFound(format!("Task with ID {} not found", task_id)))
    }
}
