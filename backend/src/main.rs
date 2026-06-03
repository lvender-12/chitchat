use std::sync::Arc;

use backend::{
    app::{AppState, create_app},
    config::{config::load_config, db::load_db, redis::load_redis},
    message::dto::ChatMessage,
};
use tokio::sync::broadcast;
use tracing::{debug, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let conf = load_config().expect("config error");

    debug!("{:?}", conf);

    let db = load_db(&conf.db)
        .await
        .expect("Gagal konek ke PostgreSQL, pastikan DB menyala dan config benar");

    let redis = load_redis(&conf.redis)
        .await
        .expect("Gagal konek ke Redis, pastikan Redis menyala dan config benar");

    let host = format!("{}:{}", conf.app.host, conf.app.port);

    let (tx, _) = broadcast::channel::<ChatMessage>(100);

    let state = AppState {
        db,
        redis,
        config: Arc::new(conf),
        tx,
    };

    debug!("{:?}", state);

    {
        let mut conn = state.redis.lock().await;
        redis::cmd("FLUSHDB")
            .query_async::<()>(&mut *conn)
            .await
            .expect("Gagal flush Redis");
        info!("Flush DB Redis")
    }

    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .expect("Failed to bind TCP listener");

    info!("Listening on {}", host);
    axum::serve(listener, app).await.expect("Server crashed");
}
