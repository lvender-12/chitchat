use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Friend {
    pub uuid: String,
    pub user_id: String,
    pub friend_id: String,
    pub created_at: Option<DateTime<Utc>>,
}
