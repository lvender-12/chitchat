use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
use validator::Validate;

use crate::{
    app::AppState,
    auth::{
        dto::{LoginDto, RegisterDto},
        service::{login_service, register_service},
    },
    common::response::ApiResponse,
    errors::error::AppResult,
};

#[axum::debug_handler]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterDto>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;
    register_service(&state, body).await?;
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: "Berhasil Mendaftar".to_string(),
        }),
    ))
}

pub async fn login_handler(
    State(state): State<AppState>,
    Json(body): Json<LoginDto>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;
    let cookies = login_service(&state, body).await?;
    Ok((
        StatusCode::OK,
        cookies,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: "Berhasil Login".to_string(),
        }),
    ))
}
