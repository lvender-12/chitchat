use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::error::AppResult,
    user::{
        dto::AddFriendPayload,
        service::{add_friend_service, profile_service},
    },
    utils::jwt::Claims,
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

pub async fn add_friend_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(friend_uuid): Path<String>,
) -> AppResult<impl IntoResponse> {
    let uuid = claims.sub;
    add_friend_service(&state, uuid, friend_uuid).await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: "Berhasil Mengirim Permintaan".to_string(),
        }),
    ))
}
