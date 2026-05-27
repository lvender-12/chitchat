use crate::{app::AppState, auth::dto::RegisterDto, errors::error::AppResult};

pub async fn register_service(state: &AppState, body: RegisterDto) -> AppResult<()> {
    Ok(())
}
