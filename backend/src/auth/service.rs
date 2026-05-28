use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};

use crate::{
    app::AppState,
    auth::{
        dto::{LoginDto, RegisterDto},
        repository::{count_users, find_by_email, register_repository},
    },
    errors::error::{AppError, AppResult},
    utils::{
        hash::{hash_password, verify_password},
        jwt::generate_jwt,
        uuid::generate_uuid,
    },
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

    register_repository(&state, body, &uuid).await?;

    Ok(())
}

pub async fn login_service(state: &AppState, body: LoginDto) -> AppResult<CookieJar> {
    let user = if let Some(user) = find_by_email(&state, &body.email).await? {
        user
    } else {
        return Err(AppError::NotFound("User not found".to_string()));
    };

    if !verify_password(&body.password, &user.password)? {
        return Err(AppError::InvalidCredentials);
    }

    let token = generate_jwt(
        user.uuid,
        user.email,
        &state.config.jwt.secret,
        state.config.jwt.expiry,
    )?;

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .build();

    Ok(CookieJar::new().add(cookie))
}
