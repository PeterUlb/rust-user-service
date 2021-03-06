// Definitions
use crate::db::PgPooledConnection;
use crate::model::users::{NewUser, User};
use crate::schema::users;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

pub trait UserRepository {
    fn get_user_by_id(&self, id: i64) -> QueryResult<Option<User>>;
    fn get_user_by_username(&self, username: &str) -> QueryResult<Option<User>>;
    fn create_user(&self, new_user: &mut NewUser) -> QueryResult<usize>;
}

impl UserRepository for PgPooledConnection {
    fn get_user_by_id(&self, id: i64) -> QueryResult<Option<User>> {
        users::table
            .filter(users::id.eq(id))
            .first::<User>(self)
            .optional()
    }

    fn get_user_by_username(&self, username: &str) -> QueryResult<Option<User>> {
        let username_upper = username.to_uppercase();
        users::table
            .filter(users::username.eq(&username_upper))
            .first::<User>(self)
            .optional()
    }

    fn create_user(&self, new_user: &mut NewUser) -> QueryResult<usize> {
        new_user.username = new_user.username.to_uppercase();
        new_user.email = new_user.email.to_uppercase();
        diesel::insert_into(users::table)
            .values(&*new_user)
            .execute(self)
    }
}
