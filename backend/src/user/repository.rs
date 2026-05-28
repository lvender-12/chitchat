use crate::{app::AppState, errors::error::AppResult, user::dto::ProfileUser};

pub async fn profile_repository(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    let profile = sqlx::query_as::<_, ProfileUser>(
        "SELECT uuid, name, email, created_at FROM users WHERE uuid = $1",
    )
    .bind(uuid)
    .fetch_one(&state.db)
    .await?;
    Ok(profile)
}
