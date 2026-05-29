use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::entity::friend_request_entity::FriendStatus;

#[derive(Debug, Serialize, FromRow)]
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
    pub created_at: Option<DateTime<Utc>>,
}
