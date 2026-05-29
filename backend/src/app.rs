use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state};
use redis::aio::MultiplexedConnection;
use tokio::sync::{Mutex, broadcast};

use crate::{
    message::dto::ChatMessage,
    middleware::{
        api_keys::api_keys_middleware, method_not_allowed::method_not_allowed,
        not_found::not_found_middleware,
    },
    model::config_model::ConfigModel,
    routes::{protected::protected_route, public::public_route, web_socket::ws_route},
};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub config: ConfigModel,
    pub tx: broadcast::Sender<ChatMessage>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .merge(public_route())
        .merge(protected_route(state.clone()))
        .merge(ws_route(state.clone()))
        .layer(from_fn_with_state(state.clone(), api_keys_middleware))
        .with_state(state)
        .fallback(not_found_middleware)
        .method_not_allowed_fallback(method_not_allowed)
}
