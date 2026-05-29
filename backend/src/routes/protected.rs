use axum::{
    Router,
    middleware::from_fn_with_state,
    routing::{get, patch, post},
};

use crate::{
    app::AppState,
    middleware::auth::auth_middleware,
    user::handler::{
        add_friend_handler, all_friend_handler, friend_actions_handler, friend_received_handler,
        friend_sent_handler, profile_handler,
    },
};

pub fn protected_route(state: AppState) -> Router<AppState> {
    Router::new()
        .route("/user/profile", get(profile_handler))
        .route("/user/friend/add/{uuid}", post(add_friend_handler))
        .route("/user/friend/pending/sent", get(friend_sent_handler))
        .route(
            "/user/friend/{uuid}/{action}",
            patch(friend_actions_handler),
        )
        .route(
            "/user/friend/pending/received",
            get(friend_received_handler),
        )
        .route("/user/friend/list", get(all_friend_handler))
        .route_layer(from_fn_with_state(state, auth_middleware))
}
