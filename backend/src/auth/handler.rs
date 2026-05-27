use axum::{Json, extract::State, response::IntoResponse};
use http::StatusCode;
use validator::Validate;

use crate::{
    app::AppState,
    auth::{dto::RegisterDto, service::register_service},
    errors::error::AppResult,
};

#[axum::debug_handler]
pub async fn register_handler(
    State(state): State<AppState>,
    Json(body): Json<RegisterDto>,
) -> AppResult<impl IntoResponse> {
    body.validate()?;
    register_service(&state, body).await?;
    Ok((StatusCode::CREATED, Json("Berhasil Mendaftar")))
}
