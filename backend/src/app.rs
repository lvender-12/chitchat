use std::sync::Arc;

use axum::Router;
use redis::aio::MultiplexedConnection;
use tokio::sync::Mutex;

use crate::routes::public::public_route;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
    pub redis: Arc<Mutex<MultiplexedConnection>>,
}

pub fn create_app(state: AppState) -> Router {
    Router::new().merge(public_route()).with_state(state)
}
