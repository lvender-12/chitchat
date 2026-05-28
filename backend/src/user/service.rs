use crate::{
    app::AppState,
    errors::error::{AppError, AppResult},
    user::{
        dto::ProfileUser,
        repository::{add_friend_repository, find_by_uuid, profile_repository},
    },
};

pub async fn profile_service(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    let profile = profile_repository(state, uuid).await?;
    Ok(profile)
}

pub async fn add_friend_service(
    state: &AppState,
    uuid: String,
    friend_uuid: String,
) -> AppResult<()> {
    if uuid == friend_uuid {
        return Err(AppError::BadRequest(
            "cannot add self as friend".to_string(),
        ));
    }

    if find_by_uuid(&state, &friend_uuid).await?.is_none() {
        return Err(AppError::NotFound("friend not found".to_string()));
    }

    add_friend_repository(&state, uuid, friend_uuid).await?;
    Ok(())
}
