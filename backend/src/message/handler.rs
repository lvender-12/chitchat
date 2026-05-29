use axum::{
    Extension,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};

use crate::{
    app::AppState, entity::message_entity::Message as MessageEntity, message::dto::ChatMessage,
    utils::jwt::Claims,
};

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    Extension(claims): Extension<Claims>,
    State(state): State<AppState>,
    Path(uuid): Path<String>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket_message(socket, claims, state, uuid))
}

pub async fn handle_socket_message(
    mut socket: WebSocket,
    claims: Claims,
    state: AppState,
    conversation_id: String,
) {
    let mut rx = state.tx.subscribe();
    let sender_id = claims.sub;
    loop {
        tokio::select! {
        Some(Ok(msg)) = socket.recv() => {
                        if let Message::Text(text) = msg {
                            if let Ok(chat_msg) = serde_json::from_str::<ChatMessage>(&text) {
                                let _ = state.tx.send(chat_msg);
                            }
                        }
                    }
                    Ok(chat_msg) = rx.recv() => {
                        if chat_msg.conversation_id == conversation_id
                            && chat_msg.sender_id != sender_id {
                            let json = serde_json::to_string(&chat_msg).unwrap();
                            let _ = socket.send(Message::Text(json.into())).await;
                        }
                    }
                }
    }
}
