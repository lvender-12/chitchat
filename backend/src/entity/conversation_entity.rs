use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Conversation {
    pub uuid: String,
    pub user1_id: String,
    pub user2_id: String,
    pub created_at: Option<DateTime<Utc>>,
}
