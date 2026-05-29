use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{app::AppState, message::handler::ws_handler, middleware::auth::auth_middleware};

pub fn ws_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/ws/message/{uuid}", get(ws_handler))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
