use actix_web::{HttpResponse, ResponseError};
use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP request failed with status: {0}")]
    HttpError(StatusCode),

    #[error("Failed to parse response: {0}")]
    ParseError(#[from] serde_json::Error),

    #[error("Unexpected error: {0}")]
    UnexpectedError(String),
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ApiError::HttpError(status) => {
                HttpResponse::build(actix_web::http::StatusCode::from_u16(status.as_u16()).unwrap())
                    .body(self.to_string())
            }
            ApiError::ParseError(_) => HttpResponse::InternalServerError().body(self.to_string()),
            ApiError::UnexpectedError(_) => {
                HttpResponse::InternalServerError().body(self.to_string())
            }
        }
    }
}
