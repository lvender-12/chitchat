use crate::{entity::message_entity::Message, errors::error::AppResult};

#[axum::debug_handler]
pub async fn get_messages_handler() -> AppResult<()> {
    Ok(())
}
