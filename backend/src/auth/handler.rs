use axum::extract::State;
use http::StatusCode;
use validator::Validate;

use crate::{app::AppState, auth::dto::RegisterDto, errors::error::AppResult};

pub async fn register_handler(
    State(state): State<AppState>,
    body: RegisterDto,
) -> AppResult<(StatusCode, String)> {
    body.validate()?;

    Ok((StatusCode::CREATED, "Berhasil Mendaftar".to_string()))
}
