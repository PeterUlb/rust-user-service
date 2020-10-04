use crate::auth;
use crate::configuration::Jwt;
use crate::model::sessions::{
    LoginDto, NewSession, Session, SessionStatus, TokenDto, TokenPairDto,
};
use crate::repository::session_repository::SessionRepository;
use crate::repository::user_repository::UserRepository;
use crate::service;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum SessionServiceError {
    DatabaseEntryAlreadyExists,
    GenericDatabaseError(diesel::result::Error),
    AuthorizationError(auth::AuthorizationError),
    UserServiceError(service::user_service::UserServiceError),
    JwtGenerationError,
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

pub fn create_login_token_pair<R>(
    repositories: &R,
    login_dto: &LoginDto,
    token_config: &Jwt,
) -> Result<TokenPairDto, SessionServiceError>
where
    R: UserRepository + SessionRepository,
{
    let user = repositories
        .get_user_by_username(&login_dto.username)
        .map_err(|e| SessionServiceError::GenericDatabaseError(e))?
        .ok_or(SessionServiceError::AuthorizationError(
            auth::AuthorizationError::UserDoesNotExist,
        ))?;

    let result =
        service::user_service::validate_password(&user.password, login_dto.password.as_bytes())
            .map_err(|e| SessionServiceError::UserServiceError(e))?;

    if result == false {
        return Err(SessionServiceError::AuthorizationError(
            auth::AuthorizationError::PasswordInvalid,
        ));
    }

    let session = NewSession {
        id: Uuid::new_v4(),
        user_id: user.id,
        platform: login_dto.platform.clone(),
        sub_platform: login_dto.sub_platform.clone(),
        expires_at: chrono::Utc::now()
            + chrono::Duration::milliseconds(token_config.session_exp_ms),
        status: SessionStatus::Active as i32,
    };
    repositories.create_session(&session)?;
    // Cleanup
    repositories.delete_expired_active_sessions(session.user_id)?;
    let session_token = generate_session_token(
        &session.id,
        session.user_id,
        session.expires_at,
        token_config,
    )
    .map_err(|e| {
        error!("{}", e);
        SessionServiceError::JwtGenerationError
    })?;
    let access_token = generate_access_token(session.user_id, token_config).map_err(|e| {
        error!("{}", e);
        SessionServiceError::JwtGenerationError
    })?;

    Ok(TokenPairDto {
        session_token,
        access_token,
    })
}

fn generate_session_token(
    session_id: &Uuid,
    user_id: i64,
    expires_at: chrono::DateTime<Utc>,
    token_config: &Jwt,
) -> Result<TokenDto, jsonwebtoken::errors::Error> {
    let my_claims = crate::auth::SessionClaims {
        exp: expires_at.timestamp(),
        iat: chrono::Utc::now().timestamp(),
        iss: "user-servic".to_owned(),
        session_id: session_id.clone(),
        user_id: user_id,
    };

    let naive = chrono::NaiveDateTime::from_timestamp(my_claims.exp, 0);
    let exp: chrono::DateTime<Utc> = chrono::DateTime::from_utc(naive, Utc);
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &my_claims,
        &jsonwebtoken::EncodingKey::from_secret(token_config.session_secret.as_ref()),
    )
    .map(|token| TokenDto {
        token: token,
        expiration: exp,
    })
}

fn generate_access_token(
    user_id: i64,
    token_config: &Jwt,
) -> Result<TokenDto, jsonwebtoken::errors::Error> {
    let my_claims = crate::auth::AccessClaims {
        exp: (chrono::Utc::now() + chrono::Duration::milliseconds(token_config.access_exp_ms))
            .timestamp(),
        iat: chrono::Utc::now().timestamp(),
        iss: "user-servic".to_owned(),
        user_id: user_id,
    };

    let naive = chrono::NaiveDateTime::from_timestamp(my_claims.exp, 0);
    let exp: chrono::DateTime<Utc> = chrono::DateTime::from_utc(naive, Utc);
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &my_claims,
        &jsonwebtoken::EncodingKey::from_secret(token_config.access_secret.as_ref()),
    )
    .map(|token| TokenDto {
        token: token,
        expiration: exp,
    })
}
