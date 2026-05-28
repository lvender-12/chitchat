use axum::{Router, middleware::from_fn_with_state, routing::get};

use crate::{app::AppState, middleware::auth::auth_middleware, user::handler::profile_handler};

pub fn protected_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user/profile", get(profile_handler))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
