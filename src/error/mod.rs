pub mod codes;
pub mod responses;

use std::fmt;
use std::fmt::Debug;

use crate::error::codes::ErrorCode;
use crate::error::responses::{DefaultErrorResponse, MissingFieldsErrorResponse};
use crate::service::user_service::UserServiceError;
use actix_web::error::BlockingError;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Serialize)]
pub struct MissingField {
    pub field_name: String,
    pub internal_code: i32,
}

#[derive(Debug)]
pub enum ApiError {
    MissingFields(Vec<MissingField>),
    InternalServerError,
    NoAccessTokenHeader,
    JwtValidationError(jsonwebtoken::errors::Error),
}

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
                let resp = MissingFieldsErrorResponse::new(fields.to_vec());
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
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
            UserServiceError::GenericDatabaseError(e) => e.into(),
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
