use axum::{
    Extension,
    extract::{
        Path, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};

use crate::{
    app::AppState,
    message::{
        dto::IncomingMessage,
        service::{get_messages_service, save_message_service},
    },
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

    if let Ok(messages) = get_messages_service(&state, &conversation_id).await {
        if let Ok(json) = serde_json::to_string(&messages) {
            let _ = socket.send(Message::Text(json.into())).await;
        }
    }

    loop {
        tokio::select! {
        Some(Ok(msg)) = socket.recv() => {
            if let Message::Text(text) = msg {
                if let Ok(incoming) =
                    serde_json::from_str::<IncomingMessage>(&text)
                {
                    if let Err(e) = save_message_service(&state, sender_id.clone(), conversation_id.clone(), incoming.message.clone()).await {
                        eprintln!("ws_service error: {:?}", e);
                    }
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
