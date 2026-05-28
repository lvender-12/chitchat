use axum::{Router, routing::post};

use crate::{
    app::AppState,
    auth::handler::{login_handler, register_handler},
};

pub fn protected_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(register_handler))
        .route("/auth/login", post(login_handler))
}
