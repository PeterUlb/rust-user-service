use crate::db::PgPooledConnection;
use crate::model::sessions::Session;
use crate::schema::sessions;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

pub trait SessionRepository {
    fn get_sessions_by_user_id(&self, user_id: i64) -> QueryResult<Vec<Session>>;
}

impl SessionRepository for PgPooledConnection {
    fn get_sessions_by_user_id(&self, user_id: i64) -> QueryResult<Vec<Session>> {
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .load::<Session>(self)
    }
}
