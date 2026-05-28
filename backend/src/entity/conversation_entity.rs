use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub uuid: Uuid,
    pub user1_id: String,
    pub user2_id: String,
    pub created_at: Option<DateTime<Utc>>,
}
