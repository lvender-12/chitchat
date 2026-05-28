use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Friend {
    pub uuid: Uuid,
    pub user_id: String,
    pub friend_id: String,
    pub created_at: Option<DateTime<Utc>>,
}
