// Definitions
use crate::db::PgPooledConnection;
use crate::model::users::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

pub trait UserRepository {
    fn get_user_by_username(&self, username: &str) -> QueryResult<Option<User>>;
    fn create_user(&self, new_user: &NewUser) -> QueryResult<usize>;
}

impl UserRepository for PgPooledConnection {
    fn get_user_by_username(&self, username: &str) -> QueryResult<Option<User>> {
        users::table
            .filter(users::username.eq(username))
            .first::<User>(self)
            .optional()
    }

    fn create_user(&self, new_user: &NewUser) -> QueryResult<usize> {
        diesel::insert_into(users::table)
            .values(new_user)
            .execute(self)
    }
}
