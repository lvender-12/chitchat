use crate::{
    app::AppState,
    entity::message_entity::Message,
    errors::error::AppResult,
    message::{
        dto::ChatMessage,
        repository::{get_messages, save_message},
    },
};

pub async fn save_message_service(
    state: &AppState,
    uuid: String,
    conversation_id: String,
    message: String,
) -> AppResult<()> {
    let chat_msg = ChatMessage {
        conversation_id,
        sender_id: uuid,
        message,
    };
    save_message(&state, &chat_msg).await?;

    let _ = state.tx.send(chat_msg);
    Ok(())
}

pub async fn get_messages_service(
    state: &AppState,
    conversation_id: &String,
) -> AppResult<Vec<Message>> {
    let messages = get_messages(&state, &conversation_id).await?;
    Ok(messages)
}
