use crate::db::PgPooledConnection;
use crate::model::sessions::{NewSession, Session};
use crate::schema::sessions;
use chrono::Utc;
use diesel::prelude::*;
use diesel::{QueryResult, RunQueryDsl};

pub trait SessionRepository {
    fn get_session_by_id(&self, id: uuid::Uuid) -> QueryResult<Option<Session>>;
    fn get_sessions_by_user_id(&self, user_id: i64) -> QueryResult<Vec<Session>>;
    fn create_session(&self, session: &NewSession) -> QueryResult<usize>;
    fn delete_expired_active_sessions(&self, user_id: i64) -> QueryResult<usize>;
    fn update_refreshed_timestamps(
        &self,
        id: uuid::Uuid,
        refreshed_at: chrono::DateTime<Utc>,
        expires_at: chrono::DateTime<Utc>,
    ) -> QueryResult<usize>;
}

impl SessionRepository for PgPooledConnection {
    fn get_session_by_id(&self, id: uuid::Uuid) -> QueryResult<Option<Session>> {
        sessions::table
            .filter(sessions::id.eq(id))
            .first::<Session>(self)
            .optional()
    }

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

    fn update_refreshed_timestamps(
        &self,
        id: uuid::Uuid,
        refreshed_at: chrono::DateTime<Utc>,
        expires_at: chrono::DateTime<Utc>,
    ) -> QueryResult<usize> {
        diesel::update(sessions::table.filter(sessions::id.eq(id)))
            .set((
                sessions::refreshed_at.eq(refreshed_at),
                sessions::expires_at.eq(expires_at),
            ))
            .execute(self)
    }
}
