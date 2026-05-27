use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Type)]
#[sqlx(type_name = "friend_status", rename_all = "lowercase")]
pub enum FriendStatus {
    Pending,
    Accepted,
    Rejected,
}

impl Default for FriendStatus {
    fn default() -> Self {
        FriendStatus::Pending
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FriendRequest {
    pub uuid: String,
    pub from_user_id: String,
    pub to_user_id: String,
    pub status: FriendStatus,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
