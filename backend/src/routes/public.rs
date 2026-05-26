use axum::{Router, extract::State, routing::get};

use crate::app::AppState;

pub async fn hello(State(state): State<AppState>) -> String {
    format!("HALLO")
}

pub fn public_route() -> Router<AppState> {
    Router::new().route("/", get(hello))
}
