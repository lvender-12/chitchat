use axum::{
    Extension, Json,
    extract::{Path, State},
    response::IntoResponse,
};
use http::StatusCode;

use crate::{
    app::AppState,
    common::response::ApiResponse,
    errors::error::{AppError, AppResult},
    user::{
        repository::friend_received_repository,
        service::{
            add_friend_service, all_friend_service, friend_accepted_service,
            friend_rejected_service, friend_sent_service, profile_service,
        },
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

pub async fn friend_sent_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<impl IntoResponse> {
    let uuid = claims.sub;
    let user = friend_sent_service(&state, uuid).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!(user),
            message: "Berhasil Mendapatkan Permintaan".to_string(),
        }),
    ))
}

pub async fn friend_received_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<impl IntoResponse> {
    let uuid = claims.sub;
    let user = friend_received_repository(&state, uuid).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!(user),
            message: "Berhasil Mendapatkan Permintaan".to_string(),
        }),
    ))
}

pub async fn friend_actions_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path((uuid, action)): Path<(String, String)>,
) -> AppResult<impl IntoResponse> {
    let to_user = claims.sub;
    let from_user = uuid;
    match action.as_str() {
        "accept" => friend_accepted_service(&state, to_user, from_user).await?,
        "reject" => friend_rejected_service(&state, to_user, from_user).await?,
        _ => return Err(AppError::BadRequest("Invalid action".to_string())),
    }
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: format!("Berhasil {} friend request", action).to_string(),
        }),
    ))
}

pub async fn all_friend_handler(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> AppResult<impl IntoResponse> {
    let uuid = claims.sub;
    let friends = all_friend_service(&state, uuid).await?;
    Ok((
        StatusCode::OK,
        Json(ApiResponse {
            data: serde_json::json!(friends),
            message: format!("Berhasil Mendapatkan Semua Friend").to_string(),
        }),
    ))
}
