use crate::{
    app::AppState, auth::dto::RegisterDto, entity::user_entity::User, errors::error::AppResult,
};

pub async fn find_by_email(state: &AppState, email: &str) -> AppResult<Option<User>> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(&state.db)
        .await?;
    Ok(user)
}

pub async fn count_users(state: &AppState) -> AppResult<i64> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await?;
    Ok(count)
}

pub async fn register_repository(
    state: &AppState,
    user: RegisterDto,
    uuid: String,
) -> AppResult<()> {
    sqlx::query("INSERT INTO users (uuid, name, email, password) VALUES ($1, $2, $3, $4)")
        .bind(uuid)
        .bind(user.name)
        .bind(user.email)
        .bind(user.password)
        .execute(&state.db)
        .await?;
    Ok(())
}
