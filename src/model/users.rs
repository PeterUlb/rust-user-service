use crate::schema::users;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserStatus {
    NotVerified = 1,
    Active = 2,
    Suspended = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PasswordVersion {
    ARGON2_1 = 1,
}

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

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
    pub password_version: i32,
    pub date_of_birth: chrono::NaiveDate,
    pub status: i32,
}

#[derive(Debug, Validate, Deserialize, Serialize)]
pub struct RegisterUserDto {
    #[validate(length(min = 6))]
    pub username: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 6))]
    pub password: String,
    pub date_of_birth: chrono::NaiveDate,
}

impl RegisterUserDto {
    pub fn into_new_user(self, password_version: PasswordVersion, status: UserStatus) -> NewUser {
        NewUser {
            username: self.username,
            email: self.email,
            password: self.password,
            password_version: password_version as i32,
            date_of_birth: self.date_of_birth,
            status: status as i32,
        }
    }
}
