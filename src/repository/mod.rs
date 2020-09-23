// Definitions

use diesel::r2d2::ConnectionManager;
use diesel::{r2d2, PgConnection, QueryResult, RunQueryDsl};

use crate::model::User;

pub trait UserRepository: Send + Sync {
    fn get_all(&self) -> QueryResult<Vec<User>>;
    fn get_user_by_id(&self, id: i32) -> i32;
}

// Implementations
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct UserRepositoryImpl {
    pool: DbPool,
}

impl UserRepositoryImpl {
    pub fn new(db_pool: DbPool) -> UserRepositoryImpl {
        println!("New user repo");
        return UserRepositoryImpl { pool: db_pool };
    }
}

impl UserRepository for UserRepositoryImpl {
    fn get_all(&self) -> QueryResult<Vec<User>> {
        let connection = self.pool.get().unwrap();
        use crate::schema::users::dsl::*;
        return users.load::<User>(&connection);
    }

    fn get_user_by_id(&self, id: i32) -> i32 {
        return id;
    }
}
