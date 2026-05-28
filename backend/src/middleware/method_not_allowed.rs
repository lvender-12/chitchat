use axum::{Json, extract::Request};
use http::StatusCode;

use crate::{common::response::ApiResponse, errors::error::AppResult};

pub async fn method_not_allowed(req: Request) -> AppResult<(StatusCode, Json<ApiResponse>)> {
    Ok((
        StatusCode::METHOD_NOT_ALLOWED,
        Json(ApiResponse {
            data: serde_json::json!({}),
            message: format!(
                "Method {} not allowed on path {}",
                req.method(),
                req.uri().path()
            ),
        }),
    ))
}
