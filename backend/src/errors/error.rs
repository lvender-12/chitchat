use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde_json::json;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    // ── Auth ──────────────────────────────────────────
    #[error("unauthorized")]
    Unauthorized,

    #[error("invalid token")]
    InvalidToken,

    #[error("token expired")]
    TokenExpired,

    #[error("invalid credentials")]
    InvalidCredentials,

    // ── Validation ────────────────────────────────────
    #[error("bad request: {0}")]
    BadRequest(String),

    #[error("validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),

    // ── Resource ──────────────────────────────────────
    #[error("not found: {0}")]
    NotFound(String),

    #[error("conflict: {0}")]
    Conflict(String),

    // ── External ──────────────────────────────────────
    #[error(transparent)]
    Database(#[from] sqlx::Error),

    #[error(transparent)]
    Redis(#[from] redis::RedisError),

    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error(transparent)]
    Argon2(#[from] argon2::password_hash::Error),

    #[error(transparent)]
    LavSeed(#[from] lav_seed::LavError),

    #[error(transparent)]
    SerdeJson(#[from] serde_json::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "unauthorized".to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, "invalid token".to_string()),
            AppError::TokenExpired => (StatusCode::UNAUTHORIZED, "token expired".to_string()),
            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "invalid credentials".to_string())
            }

            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Validation(e) => (StatusCode::BAD_REQUEST, e.to_string()),

            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),

            AppError::Database(e) => {
                eprintln!("db error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::Redis(e) => {
                eprintln!("redis error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::Jwt(e) => {
                eprintln!("jwt error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::Argon2(e) => {
                eprintln!("argon2 error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
            AppError::LavSeed(e) => {
                eprintln!("uuid generation error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }

            AppError::SerdeJson(e) => {
                eprintln!("serde json error: {e}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal server error".to_string(),
                )
            }
        };

        (
            status,
            Json(json!({
                "error": message
            })),
        )
            .into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
