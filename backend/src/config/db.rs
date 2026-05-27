use std::time::Duration;

use anyhow::{Context, Result};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

use crate::model::config_model::DbConfig;

pub async fn load_db(conf: &DbConfig) -> Result<Pool<Postgres>> {
    let url = format!(
        "postgresql://{}:{}@{}:{}/{}",
        conf.username, conf.password, conf.host, conf.port, conf.name
    );

    let pool = PgPoolOptions::new()
        .min_connections(10)
        .max_connections(100)
        .acquire_timeout(Duration::from_secs(5))
        .idle_timeout(Duration::from_secs(60 * 30))
        .test_before_acquire(true)
        .connect(&url)
        .await
        .context("failed to connect database")?;

    Ok(pool)
}
