use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse {
    pub data: serde_json::Value,
    pub message: String,
}
