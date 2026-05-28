use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Debug, Serialize, FromRow)]
pub struct ProfileUser {
    pub uuid: String,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct AddFriendPayload {
    pub uuid: String,
}
