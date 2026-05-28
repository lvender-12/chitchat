use axum::{Json, extract::Request};
use http::StatusCode;

use crate::{common::response::ApiResponse, errors::error::AppResult};

pub async fn not_found_middleware(req: Request) -> AppResult<(StatusCode, Json<ApiResponse>)> {
    Ok((
        StatusCode::NOT_FOUND,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: format!("{} Not Found", req.uri().path()),
        }),
    ))
}
