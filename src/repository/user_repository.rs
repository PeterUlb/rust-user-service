// Definitions
use crate::db::PgPooledConnection;
use diesel::{QueryResult, RunQueryDsl};

use crate::model::User;

pub trait UserRepository {
    fn get_all(&self) -> QueryResult<Vec<User>>;
    fn get_user_by_id(&self, id: i32) -> i32;
}

impl UserRepository for PgPooledConnection {
    fn get_all(&self) -> QueryResult<Vec<User>> {
        use crate::schema::users::dsl::*;
        return users.load::<User>(self);
    }

    fn get_user_by_id(&self, id: i32) -> i32 {
        return id;
    }
}
