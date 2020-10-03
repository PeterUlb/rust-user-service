use crate::schema::sessions;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionStatus {
    Active = 1,
    Blacklisted = 2,
}

#[derive(Queryable, Serialize, Deserialize, Clone, AsChangeset, Debug)]
pub struct Session {
    pub id: Uuid,
    pub user_id: i64,
    pub platform: String,
    pub sub_platform: String,
    pub refreshed_at: chrono::DateTime<Utc>,
    pub expires_at: chrono::DateTime<Utc>,
    pub status: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name = "sessions"]
pub struct NewSession {
    pub user_id: i64,
    pub platform: String,
    pub sub_platform: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub status: i32,
}
