use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Authentication(String),

    #[error("Authorization error: {0}")]
    Authorization(String),

    #[error("Validation error: {0}")]
    Validation(Vec<ValidationError>),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Database error: {source}")]
    Database {
        #[from]
        source: sqlx::Error,
        context: Option<String>,
    },

    #[error("Rate limit exceeded. Try again in {0} seconds")]
    RateLimit(u64),

    #[error("External service error: {service} - {message}")]
    ExternalService {
        service: String,
        message: String,
        status: Option<u16>,
    },

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Internal server error")]
    Internal(#[from] anyhow::Error),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationError {
    pub field: String,
    pub code: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
    pub status_code: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation_errors: Option<Vec<ValidationError>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub help: Option<String>,
}

impl AppError {
    pub fn validation_error(errors: Vec<ValidationError>) -> Self {
        Self::Validation(errors)
    }

    pub fn database_error(error: sqlx::Error, context: impl Into<String>) -> Self {
        Self::Database {
            source: error,
            context: Some(context.into()),
        }
    }

    pub fn external_service_error(service: impl Into<String>, message: impl Into<String>, status: Option<u16>) -> Self {
        Self::ExternalService {
            service: service.into(),
            message: message.into(),
            status,
        }
    }

    fn error_response(&self) -> ErrorResponse {
        let (status_code, error_type, help) = match self {
            AppError::Authentication(_) => (
                StatusCode::UNAUTHORIZED,
                "authentication_error",
                Some("Please check your credentials and try again"),
            ),
            AppError::Authorization(_) => (
                StatusCode::FORBIDDEN,
                "authorization_error",
                Some("You don't have permission to perform this action"),
            ),
            AppError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                "validation_error",
                Some("Please check the input requirements"),
            ),
            AppError::NotFound(_) => (
                StatusCode::NOT_FOUND,
                "not_found",
                None,
            ),
            AppError::Conflict(_) => (
                StatusCode::CONFLICT,
                "conflict",
                None,
            ),
            AppError::Database { .. } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "database_error",
                Some("If the problem persists, please contact support"),
            ),
            AppError::RateLimit(_) => (
                StatusCode::TOO_MANY_REQUESTS,
                "rate_limit_error",
                Some("Please try again later"),
            ),
            AppError::ExternalService { .. } => (
                StatusCode::BAD_GATEWAY,
                "external_service_error",
                Some("The service is temporarily unavailable"),
            ),
            AppError::InvalidInput(_) => (
                StatusCode::BAD_REQUEST,
                "invalid_input",
                Some("Please check your input and try again"),
            ),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "internal_error",
                Some("If the problem persists, please contact support"),
            ),
        };

        ErrorResponse {
            error: error_type.to_string(),
            message: self.to_string(),
            status_code: status_code.as_u16(),
            validation_errors: if let AppError::Validation(errors) = self {
                Some(errors.clone())
            } else {
                None
            },
            error_code: Some(format!("E{}", status_code.as_u16())),
            help,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status_code = match &self {
            AppError::Authentication(_) => StatusCode::UNAUTHORIZED,
            AppError::Authorization(_) => StatusCode::FORBIDDEN,
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::Conflict(_) => StatusCode::CONFLICT,
            AppError::Database { .. } => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::RateLimit(_) => StatusCode::TOO_MANY_REQUESTS,
            AppError::ExternalService { .. } => StatusCode::BAD_GATEWAY,
            AppError::InvalidInput(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = serde_json::to_string(&self.error_response())
            .unwrap_or_else(|_| {
                r#"{"error":"internal_server_error","message":"Failed to serialize error","status_code":500}"#.to_string()
            });

        (status_code, body).into_response()
    }
}
