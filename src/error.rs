use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

#[derive(Serialize)]
struct AppErrorResponse {
    status: u16,
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, data) = match &self {
            AppError::DatabaseError(e) => {
                eprintln!("Database error: {}", e);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    AppErrorResponse {
                        status: 99,
                        message: "Database error".to_string(),
                    },
                )
            },
            AppError::NotFound(msg)=> {
                (
                    StatusCode::NOT_FOUND,
                    AppErrorResponse {
                        status: 404,
                        message: format!("Not found: {}", msg),
                    },
                )
            },
        };
        (status, Json(data)).into_response()
    }
}