use crate::auth;
use crate::configuration::Jwt;
use crate::model::sessions::{
    LoginDto, NewSession, Session, SessionStatus, TokenDto, TokenPairDto,
};
use crate::model::users::UserStatus;
use crate::repository::session_repository::SessionRepository;
use crate::repository::user_repository::UserRepository;
use crate::service;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug)]
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

impl From<service::user_service::UserServiceError> for SessionServiceError {
    fn from(error: service::user_service::UserServiceError) -> SessionServiceError {
        SessionServiceError::UserServiceError(error)
    }
}

impl From<auth::AuthorizationError> for SessionServiceError {
    fn from(error: auth::AuthorizationError) -> SessionServiceError {
        SessionServiceError::AuthorizationError(error)
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
        service::user_service::validate_password(&user.password, login_dto.password.as_bytes())?;

    if result == false {
        return Err(SessionServiceError::AuthorizationError(
            auth::AuthorizationError::PasswordInvalid,
        ));
    }

    if user.status != UserStatus::Active as i32 {
        return Err(SessionServiceError::AuthorizationError(
            auth::AuthorizationError::PasswordInvalid,
        )); // TODO: Own error
    }

    let session = NewSession {
        id: Uuid::new_v4(),
        user_id: user.id,
        platform: login_dto.platform.clone(),
        sub_platform: login_dto.sub_platform.clone(),
        refreshed_at: chrono::Utc::now(),
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

pub fn create_access_token_and_refresh<R>(
    repositories: &R,
    session_token: &str,
    token_config: &Jwt,
) -> Result<TokenPairDto, SessionServiceError>
where
    R: UserRepository + SessionRepository,
{
    let claims = auth::decode_session_jwt(session_token, token_config)?;
    let session = match repositories.get_session_by_id(claims.session_id)? {
        Some(session) => session,
        None => return Err(auth::AuthorizationError::NoAuthorizationForAction.into()),
    };
    auth::verify_subject(claims.user_id, session.user_id)?;
    if session.status == SessionStatus::Blacklisted as i32 {
        return Err(auth::AuthorizationError::SessionTokenBlacklisted.into());
    }

    let now = chrono::Utc::now();
    let new_exp = now + chrono::Duration::milliseconds(token_config.session_exp_ms);
    repositories.update_refreshed_timestamps(claims.session_id, now, new_exp)?;
    let session_token = generate_session_token(&session.id, session.user_id, new_exp, token_config)
        .map_err(|e| {
            error!("{}", e);
            SessionServiceError::JwtGenerationError
        })?;
    let access_token = generate_access_token(claims.user_id, token_config).map_err(|e| {
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
