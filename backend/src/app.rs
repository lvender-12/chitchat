use std::sync::Arc;

use axum::Router;
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;

use crate::{model::config_model::ConfigModel, routes::public::public_route};

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
    pub config: ConfigModel,
}

pub fn create_app(state: AppState) -> Router {
    Router::new().merge(public_route()).with_state(state)
}
