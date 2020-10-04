use crate::auth;
use crate::model::sessions::Session;
use crate::repository::session_repository::SessionRepository;

#[derive(Debug, PartialEq)]
pub enum SessionServiceError {
    DatabaseEntryAlreadyExists,
    GenericDatabaseError(diesel::result::Error),
    AuthorizationError(auth::AuthorizationError),
}

impl From<diesel::result::Error> for SessionServiceError {
    fn from(error: diesel::result::Error) -> SessionServiceError {
        match error {
            diesel::result::Error::DatabaseError(db_error, _) => match db_error {
                diesel::result::DatabaseErrorKind::UniqueViolation => {
                    SessionServiceError::DatabaseEntryAlreadyExists
                }
                _ => SessionServiceError::GenericDatabaseError(error),
            },
            _ => SessionServiceError::GenericDatabaseError(error),
        }
    }
}

pub fn get_users_sessions(
    session_repository: &impl SessionRepository, //equal to register_user<R> where R: UserRepository
    user_id: i64,
) -> Result<Vec<Session>, SessionServiceError> {
    session_repository
        .get_sessions_by_user_id(user_id)
        .map_err(|e| e.into())
}
