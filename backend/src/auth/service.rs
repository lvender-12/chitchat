use axum_extra::extract::{
    CookieJar,
    cookie::{Cookie, SameSite},
};
use tracing::{debug, info};

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
    debug!(email = %body.email, "checking existing email");
    if existing_email.is_some() {
        return Err(AppError::Conflict("Email already exists".to_string()));
    }

    let count = count_users(&state).await? as u64;
    debug!(user_count = count, "count users");

    let uuid = generate_uuid(count)?;
    debug!(user_id = %uuid, "generated user id");

    let hash = hash_password(&body.password)?;

    let body = RegisterDto {
        name: body.name,
        email: body.email,
        password: hash,
    };
    debug!(name = %body.name, email = %body.email, "register payload received");

    register_repository(&state, body, &uuid).await?;

    Ok(())
}

pub async fn login_service(state: &AppState, body: LoginDto) -> AppResult<CookieJar> {
    debug!(email = %body.email, "login attempt");

    let user = find_by_email(state, &body.email).await?;

    let user = if let Some(user) = user {
        debug!(email = %body.email, user_id = %user.uuid, "user found");
        user
    } else {
        debug!(email = %body.email, "user not found");
        return Err(AppError::NotFound("User not found".to_string()));
    };

    if !verify_password(&body.password, &user.password)? {
        debug!(email = %body.email, user_id = %user.uuid, "invalid password");
        return Err(AppError::InvalidCredentials);
    }

    debug!(email = %body.email, user_id = %user.uuid, "password verified");

    let token = generate_jwt(
        user.uuid.clone(),
        user.email.clone(),
        &state.config.jwt.secret,
        state.config.jwt.expiry,
    )?;

    debug!(user_id = %user.uuid, "jwt generated");

    let cookie = Cookie::build(("token", token))
        .http_only(true)
        .same_site(SameSite::None)
        .path("/")
        .build();

    info!(email = %body.email, user_id = %user.uuid, "login success");

    Ok(CookieJar::new().add(cookie))
}
