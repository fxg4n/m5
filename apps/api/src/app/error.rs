use axum::{
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    Database(sqlx::Error),
    Configuration(String),
    Server(Box<dyn std::error::Error>),
    NotFound(String),
    Validation(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::Database(e) => write!(f, "Database error: {}", e),
            AppError::Configuration(e) => write!(f, "Configuration error: {}", e),
            AppError::Server(e) => write!(f, "Server error: {}", e),
            AppError::NotFound(e) => write!(f, "Not found: {}", e),
            AppError::Validation(e) => write!(f, "Validation error: {}", e),
        }
    }
}

impl std::error::Error for AppError {}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Database error: {}", e),
            ),
            AppError::Configuration(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Configuration error: {}", e),
            ),
            AppError::Server(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Server error: {}", e),
            ),
            AppError::NotFound(e) => (
                StatusCode::NOT_FOUND,
                e,
            ),
            AppError::Validation(e) => (
                StatusCode::BAD_REQUEST,
                e,
            ),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}