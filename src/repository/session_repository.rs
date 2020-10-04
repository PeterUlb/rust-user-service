use crate::db::PgPooledConnection;
use crate::model::sessions::{NewSession, Session};
use crate::schema::sessions;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

pub trait SessionRepository {
    fn get_sessions_by_user_id(&self, user_id: i64) -> QueryResult<Vec<Session>>;
    fn create_session(&self, session: &NewSession) -> QueryResult<usize>;
    fn delete_expired_active_sessions(&self, user_id: i64) -> QueryResult<usize>;
}

impl SessionRepository for PgPooledConnection {
    fn get_sessions_by_user_id(&self, user_id: i64) -> QueryResult<Vec<Session>> {
        sessions::table
            .filter(sessions::user_id.eq(user_id))
            .load::<Session>(self)
    }

    fn create_session(&self, session: &NewSession) -> QueryResult<usize> {
        diesel::insert_into(sessions::table)
            .values(session)
            .execute(self)
    }

    fn delete_expired_active_sessions(&self, user_id: i64) -> QueryResult<usize> {
        diesel::delete(
            sessions::table.filter(
                sessions::user_id
                    .eq(user_id)
                    .and(sessions::expires_at.lt(chrono::Utc::now() - chrono::Duration::hours(1))),
            ),
        )
        .execute(self)
    }
}
