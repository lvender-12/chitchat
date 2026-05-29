use crate::{app::AppState, errors::error::AppResult, user::dto::UserCache};

pub async fn get_friend_cache(state: &AppState, user_id: &str) -> AppResult<Option<UserCache>> {
    let key = format!("friend:{}", user_id);
    let mut conn = state.redis.lock().await;
    let value: Option<String> = redis::cmd("GET").arg(&key).query_async(&mut *conn).await?;

    if let Some(json) = value {
        let user = serde_json::from_str::<UserCache>(&json)?;
        return Ok(Some(user));
    }
    Ok(None)
}

pub async fn set_friend_cache(state: &AppState, user: &UserCache) -> AppResult<()> {
    let key = format!("user:{}", user.uuid);
    let json = serde_json::to_string(user)?;
    let mut conn = state.redis.lock().await;

    redis::cmd("SET")
        .arg(&key)
        .arg(json)
        .query_async::<()>(&mut *conn)
        .await?;

    Ok(())
}
