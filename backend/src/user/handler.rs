use axum::{Extension, Json, extract::State, response::IntoResponse};
use http::StatusCode;

use crate::{
    app::AppState, common::response::ApiResponse, errors::error::AppResult,
    user::service::profile_service, utils::jwt::Claims,
};

pub async fn profile_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<impl IntoResponse> {
    let uuid = claims.sub;
    let profile = profile_service(&state, uuid).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!(profile),
            message: "Berhasil Mendapatkan Data".to_string(),
        }),
    ))
}
