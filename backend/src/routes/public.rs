use axum::{Router, routing::post};

use crate::{app::AppState, auth::handler::register_handler};

pub fn public_route() -> Router<AppState> {
    Router::new().route("/register", post(register_handler))
}
