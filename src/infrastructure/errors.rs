use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use validator::ValidationErrors;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum AppError {
    #[error("Employee not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(#[from] ValidationErrors),

    #[error("Internal server error: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Validation(errors) => {
                let mut msgs = Vec::new();
                for (field, _err) in errors.errors() {
                    msgs.push(format!("{}: validation failed", field));
                }
                (StatusCode::BAD_REQUEST, msgs.join(", "))
            }
            AppError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}
