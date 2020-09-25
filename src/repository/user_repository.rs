// Definitions

use crate::db;
use crate::db::PgPool;
use diesel::{QueryResult, RunQueryDsl};

use crate::model::User;

pub trait UserRepository: Send + Sync {
    fn get_all(&self) -> QueryResult<Vec<User>>;
    fn get_user_by_id(&self, id: i32) -> i32;
}

pub struct UserRepositoryImpl {
    pool: PgPool,
}

impl UserRepositoryImpl {
    pub fn new(pg_pool: PgPool) -> Box<dyn UserRepository> {
        info!("Created User Repository");
        return Box::new(UserRepositoryImpl { pool: pg_pool });
    }
}

impl UserRepository for UserRepositoryImpl {
    fn get_all(&self) -> QueryResult<Vec<User>> {
        let connection = db::get_conn(&self.pool)?;
        use crate::schema::users::dsl::*;
        return users.load::<User>(&connection);
    }

    fn get_user_by_id(&self, id: i32) -> i32 {
        return id;
    }
}
