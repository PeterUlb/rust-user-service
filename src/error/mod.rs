pub mod codes;
pub mod responses;

use std::fmt;
use std::fmt::Debug;

use crate::auth::AuthorizationError;
use crate::error::codes::ErrorCode;
use crate::error::responses::{DefaultErrorResponse, FieldErrorResponse};
use crate::service::session_service::SessionServiceError;
use crate::service::user_service::UserServiceError;
use actix_web::error::BlockingError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize)]
pub struct Field {
    pub field_name: String,
}

#[derive(Debug)]
pub enum ApiError {
    MissingFields(Vec<Field>),
    JsonValidationFailed(Vec<Field>),
    InternalServerError,
    NoAccessTokenHeader,
    JwtValidationError(jsonwebtoken::errors::Error),
    JwtGenerationError,
    EntityAlreadyExists,
    AuthorizationError,
    PasswordInvalid,
    SessionTokenBlacklisted,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        self.into()
    }
}

impl From<&ApiError> for HttpResponse {
    fn from(error: &ApiError) -> Self {
        match error {
            ApiError::InternalServerError => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::INTERNAL_SERVER_ERROR,
                    String::from("Internal Server Error"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::MissingFields(fields) => {
                let resp = FieldErrorResponse::new(
                    ErrorCode::MISSING_FIELDS,
                    String::from("Fields are missing"),
                    fields.to_vec(),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::JsonValidationFailed(fields) => {
                let resp = FieldErrorResponse::new(
                    ErrorCode::JSON_VALIDATION_FAILED,
                    String::from("Validation failed for fields"),
                    fields.to_vec(),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::NoAccessTokenHeader => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::MISSING_ACCESS_TOKEN_HEADER,
                    String::from("Missing Access Token"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::JwtValidationError(e) => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::JWT_VALIDATION_ERROR,
                    String::from(format!("{}", e)),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::EntityAlreadyExists => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::ENTITY_ALREADY_EXISTS,
                    String::from("Entity already exists"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::AuthorizationError => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::NOT_AUTHORIZED_FOR_ACTION,
                    String::from("Not authorized for action"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::JwtGenerationError => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::JWT_GENERATION_ERROR,
                    String::from("Token failure"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::PasswordInvalid => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::PASSWORD_INVALID,
                    String::from("Invalid Password"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
            ApiError::SessionTokenBlacklisted => {
                let resp = DefaultErrorResponse::new(
                    ErrorCode::SESSION_TOKEN_BLACKLISTED,
                    String::from("Session Token Blacklisted"),
                );
                HttpResponse::build(resp.status_code).json(resp)
            }
        }
    }
}

impl<T> From<BlockingError<T>> for ApiError
where
    T: Into<ApiError> + Debug,
{
    fn from(error: BlockingError<T>) -> Self {
        match error {
            BlockingError::Error(e) => e.into(),
            BlockingError::Canceled => ApiError::InternalServerError,
        }
    }
}

impl From<UserServiceError> for ApiError {
    fn from(error: UserServiceError) -> Self {
        match error {
            UserServiceError::DatabaseEntryAlreadyExists => ApiError::EntityAlreadyExists,
            UserServiceError::GenericDatabaseError(e) => e.into(),
            UserServiceError::HashingError => ApiError::InternalServerError,
        }
    }
}

impl From<SessionServiceError> for ApiError {
    fn from(error: SessionServiceError) -> Self {
        match error {
            SessionServiceError::DatabaseEntryAlreadyExists => ApiError::EntityAlreadyExists,
            SessionServiceError::GenericDatabaseError(e) => e.into(),
            SessionServiceError::AuthorizationError(e) => e.into(),
            SessionServiceError::UserServiceError(e) => e.into(),
            SessionServiceError::JwtGenerationError => ApiError::JwtGenerationError,
        }
    }
}

impl From<AuthorizationError> for ApiError {
    fn from(error: AuthorizationError) -> Self {
        match error {
            AuthorizationError::PasswordInvalid => ApiError::PasswordInvalid,
            AuthorizationError::NoAuthorizationForAction => ApiError::AuthorizationError,
            AuthorizationError::UserDoesNotExist => ApiError::AuthorizationError,
            AuthorizationError::JwtValidationError(e) => ApiError::JwtValidationError(e),
            AuthorizationError::SessionTokenBlacklisted => ApiError::SessionTokenBlacklisted,
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ApiError::InternalServerError,
            _ => ApiError::InternalServerError,
        }
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(error: validator::ValidationErrors) -> Self {
        error!("{}", error);
        let keys = error
            .field_errors()
            .keys()
            .map(|s| Field {
                field_name: String::from(*s),
            })
            .collect::<Vec<Field>>();
        ApiError::JsonValidationFailed(keys)
    }
}
