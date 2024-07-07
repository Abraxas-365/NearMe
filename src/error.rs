use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Conflict: {0}")]
    Conflict(String),

    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::ParseError(_) => HttpResponse::InternalServerError().json(self.to_string()),
            ApiError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ApiError::DatabaseError(_) => {
                HttpResponse::InternalServerError().json(self.to_string())
            }
            ApiError::NotFound(_) => HttpResponse::NotFound().json(self.to_string()),
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(self.to_string()),
            ApiError::Forbidden(_) => HttpResponse::Forbidden().json(self.to_string()),
            ApiError::Unauthorized(_) => HttpResponse::Unauthorized().json(self.to_string()),
            ApiError::Conflict(_) => HttpResponse::Conflict().json(self.to_string()),
            ApiError::ServiceUnavailable(_) => {
                HttpResponse::ServiceUnavailable().json(self.to_string())
            }
        }
    }
}
