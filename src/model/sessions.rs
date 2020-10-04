use crate::schema::sessions;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
    pub refreshed_at: Option<chrono::DateTime<Utc>>,
    pub expires_at: chrono::DateTime<Utc>,
    pub status: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Insertable, Debug)]
#[table_name = "sessions"]
pub struct NewSession {
    pub id: Uuid,
    pub user_id: i64,
    pub platform: String,
    pub sub_platform: String,
    pub expires_at: chrono::DateTime<Utc>,
    pub status: i32,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct LoginDto {
    #[validate(length(min = 6))]
    pub username: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub platform: String,
    pub sub_platform: String,
}

#[derive(Deserialize, Serialize)]
pub struct TokenDto {
    pub token: String,
    pub expiration: chrono::DateTime<Utc>,
}

#[derive(Deserialize, Serialize)]
pub struct TokenPairDto {
    pub session_token: TokenDto,
    pub access_token: TokenDto,
}
