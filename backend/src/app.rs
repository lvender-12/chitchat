use std::sync::Arc;

use axum::{Router, middleware::from_fn_with_state};
use http::HeaderValue;
use redis::aio::MultiplexedConnection;
use tokio::sync::{Mutex, broadcast};
use tower_http::cors::CorsLayer;

use crate::{
    message::dto::ChatMessage,
    middleware::{
        api_keys::api_keys_middleware, method_not_allowed::method_not_allowed,
        not_found::not_found_middleware,
    },
    model::config_model::ConfigModel,
    routes::{protected::protected_route, public::public_route, web_socket::ws_route},
};

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub config: Arc<ConfigModel>,
    pub tx: broadcast::Sender<ChatMessage>,
}

pub fn create_app(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(state.config.allowed_origins.parse::<HeaderValue>().unwrap())
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PATCH,
            http::Method::DELETE,
            http::Method::OPTIONS,
        ])
        .allow_headers([
            http::header::CONTENT_TYPE,
            http::header::UPGRADE,
            http::header::CONNECTION,
            http::header::HeaderName::from_static("x-api-secret"),
            http::header::HeaderName::from_static("sec-websocket-key"),
            http::header::HeaderName::from_static("sec-websocket-version"),
        ])
        .allow_credentials(true);

    Router::new()
        .merge(public_route())
        .merge(protected_route(state.clone()))
        .merge(ws_route(state.clone()))
        .layer(from_fn_with_state(state.clone(), api_keys_middleware))
        .layer(cors)
        .with_state(state)
        .fallback(not_found_middleware)
        .method_not_allowed_fallback(method_not_allowed)
}
