use crate::{
    app::AppState, entity::user_entity::User, errors::error::AppResult, user::dto::ProfileUser,
};

pub async fn profile_repository(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    let profile = sqlx::query_as::<_, ProfileUser>(
        "SELECT uuid, name, email, created_at FROM users WHERE uuid = $1",
    )
    .bind(uuid)
    .fetch_one(&state.db)
    .await?;
    Ok(profile)
}

pub async fn find_by_uuid(state: &AppState, uuid: &String) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE uuid = $1")
        .bind(uuid)
        .fetch_optional(&state.db)
        .await?;
    Ok(user)
}

pub async fn add_friend_repository(
    state: &AppState,
    uuid: String,
    friend_uuid: String,
) -> AppResult<()> {
    sqlx::query("INSERT INTO friend_requests (from_user_id, to_user_id) VALUES ($1, $2)")
        .bind(uuid)
        .bind(friend_uuid)
        .execute(&state.db)
        .await?;
    Ok(())
}
