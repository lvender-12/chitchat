use crate::{
    app::AppState,
    auth::{
        dto::RegisterDto,
        repository::{count_users, find_by_email, register_repository},
    },
    errors::error::{AppError, AppResult},
    utils::{hash::hash_password, uuid::generate_uuid},
};

pub async fn register_service(state: &AppState, body: RegisterDto) -> AppResult<()> {
    let existing_email = find_by_email(&state, &body.email).await?;
    if existing_email.is_some() {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    let count = count_users(&state).await? as u64;

    let uuid = generate_uuid(count)?;

    let hash = hash_password(&body.password)?;

    let body = RegisterDto {
        name: body.name,
        email: body.email,
        password: hash,
    };

    register_repository(&state, body, uuid).await?;

    Ok(())
}
