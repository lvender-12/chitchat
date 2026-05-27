use std::sync::Arc;

use anyhow::{Context, Result};
use redis::{Client, aio::MultiplexedConnection};
use tokio::sync::Mutex;

use crate::model::config_model::RedisConfig;

pub async fn load_redis(conf: &RedisConfig) -> Result<Arc<Mutex<MultiplexedConnection>>> {
    let url = format!(
        "redis://{}:{}@{}:{}/0",
        conf.username, conf.password, conf.host, conf.port
    );
    let client = Client::open(url).context("failed to create redis client")?;
    let conn = client
        .get_multiplexed_async_connection()
        .await
        .context("failed to connect redis")?;

    Ok(Arc::new(Mutex::new(conn)))
}
