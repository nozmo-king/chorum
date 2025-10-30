use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Invalid proof of work")]
    InvalidProofOfWork,
    
    #[error("Challenge expired")]
    ChallengeExpired,
    
    #[error("Challenge not found")]
    ChallengeNotFound,
    
    #[error("Invalid public key")]
    InvalidPublicKey,
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Not found")]
    NotFound,
    
    #[error("Internal server error")]
    Internal,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::Database(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Database error"),
            AppError::Serialization(_) => (StatusCode::BAD_REQUEST, "Invalid request format"),
            AppError::InvalidProofOfWork => (StatusCode::BAD_REQUEST, "Invalid proof of work"),
            AppError::ChallengeExpired => (StatusCode::BAD_REQUEST, "Challenge expired"),
            AppError::ChallengeNotFound => (StatusCode::NOT_FOUND, "Challenge not found"),
            AppError::InvalidPublicKey => (StatusCode::UNAUTHORIZED, "Invalid public key"),
            AppError::Validation(ref msg) => (StatusCode::BAD_REQUEST, msg.as_str()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found"),
            AppError::Internal => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        let body = Json(json!({
            "error": error_message,
        }));

        (status, body).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;