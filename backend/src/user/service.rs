use crate::{
    app::AppState,
    errors::error::AppResult,
    user::{dto::ProfileUser, repository::profile_repository},
};

pub async fn profile_service(state: &AppState, uuid: String) -> AppResult<ProfileUser> {
    let profile = profile_repository(state, uuid).await?;
    Ok(profile)
}
