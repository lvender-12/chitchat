use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Message {
    pub uuid: Uuid,
    pub conversation_id: Uuid,
    pub sender_id: String,
    pub message: String,
    pub created_at: Option<DateTime<Utc>>,
}
