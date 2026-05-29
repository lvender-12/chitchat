use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub conversation_id: String,
    pub sender_id: String,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct IncomingMessage {
    pub message: String,
}
