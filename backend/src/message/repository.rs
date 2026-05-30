use crate::{
    app::AppState, entity::message_entity::Message as MessageEntity, errors::error::AppResult,
    message::dto::ChatMessage,
};

pub async fn save_message(state: &AppState, chat_msg: &ChatMessage) -> AppResult<()> {
    sqlx::query(
        "INSERT INTO messages (conversation_id, sender_id, message) VALUES ($1::UUID, $2, $3)",
    )
    .bind(&chat_msg.conversation_id)
    .bind(&chat_msg.sender_id)
    .bind(&chat_msg.message)
    .execute(&state.db)
    .await?;
    Ok(())
}

pub async fn get_messages(
    state: &AppState,
    conversation_id: &str,
) -> AppResult<Vec<MessageEntity>> {
    let messages = sqlx::query_as::<_, MessageEntity>(
        "SELECT uuid, conversation_id, sender_id, message, created_at
        FROM messages WHERE conversation_id = $1::UUID ORDER BY created_at ASC",
    )
    .bind(conversation_id)
    .fetch_all(&state.db)
    .await?;
    Ok(messages)
}
