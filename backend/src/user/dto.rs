use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::entity::friend_request_entity::FriendStatus;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ProfileUser {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FriendRequestSent {
    pub uuid: Uuid,
    pub to_user_id: String,
    pub name: String,
    pub status: FriendStatus,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct FriendRequestReceived {
    pub uuid: Uuid,
    pub from_user_id: String,
    pub name: String,
    pub status: FriendStatus,
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FriendList {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub conversation_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCache {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct FriendRaw {
    pub friend_id: String,
    pub conversation_id: String,
}
