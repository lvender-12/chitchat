use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, post},
};

use crate::{
    app::AppState, message::handler::get_messages_handler, middleware::auth::auth_middleware,
};

pub fn ws_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/ws/messages", get(get_messages_handler))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
