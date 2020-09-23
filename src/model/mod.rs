use chrono::Utc;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Queryable, Serialize, Deserialize, Clone, AsChangeset, Debug)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub password_version: i32,
    pub date_of_birth: chrono::NaiveDate,
    pub status: i32,
    pub created_at: chrono::DateTime<Utc>,
    pub updated_at: chrono::DateTime<Utc>,
}
