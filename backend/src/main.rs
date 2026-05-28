use backend::{
    app::{AppState, create_app},
    config::{config::load_config, db::load_db, redis::load_redis},
};

#[tokio::main]
async fn main() {
    let conf = load_config().expect("config error");
    let db = load_db(&conf.db)
        .await
        .expect("Gagal konek ke PostgreSQL, pastikan DB menyala dan config benar");
    let redis = load_redis(&conf.redis)
        .await
        .expect("Gagal konek ke Redis, pastikan Redis menyala dan config benar");

    let host = format!("{}:{}", conf.app.host, conf.app.port);

    let state = AppState {
        db,
        redis,
        config: conf,
    };
    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind(&host)
        .await
        .expect("Failed to bind TCP listener");

    println!("listening on {}", host);
    axum::serve(listener, app).await.expect("Server crashed");
}
