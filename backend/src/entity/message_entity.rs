use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub uuid: String,
    pub conversation_id: String,
    pub sender_id: String,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>,
}
