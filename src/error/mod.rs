use std::fmt;
use std::fmt::Debug;

use crate::service::user_service::UserServiceError;
use actix_web::error::BlockingError;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;

pub enum ApiErrorType {
    DatabaseDieselError(diesel::result::Error),
    DbRecordNotFound,
    InternalError,
    JwtDecodeError(jsonwebtoken::errors::Error),
    NoAccessTokenHeader,
}
impl ApiErrorType {
    fn status(&self) -> u16 {
        use ApiErrorType::*;
        match self {
            DatabaseDieselError(_) => 500,
            DbRecordNotFound => 404,
            InternalError => 500,
            JwtDecodeError(_) => 401,
            NoAccessTokenHeader => 401,
        }
    }

    fn message(&self) -> &'static str {
        use ApiErrorType::*;
        match self {
            DatabaseDieselError(_) => self.get_default_text(),
            DbRecordNotFound => self.get_default_text(),
            InternalError => self.get_default_text(),
            JwtDecodeError(_) => self.get_default_text(),
            NoAccessTokenHeader => "provide access token",
        }
    }

    fn code(&self) -> u32 {
        use ApiErrorType::*;
        match self {
            DatabaseDieselError(_) => 1,
            DbRecordNotFound => 2,
            InternalError => 3,
            JwtDecodeError(_) => 4,
            NoAccessTokenHeader => 5,
        }
    }

    fn get_default_text(&self) -> &'static str {
        return match StatusCode::from_u16(self.status()) {
            Ok(status_code) => match status_code.canonical_reason() {
                None => "",
                Some(reason) => reason,
            },
            Err(_) => "",
        };
    }
}

#[derive(Serialize, Debug)]
pub struct ApiError {
    /// The HTTP status code for the error.
    pub status: u16,
    /// A descriptive message regarding the error.
    pub message: &'static str,
    /// An error code to find help for the error
    pub code: u32,
}

impl ApiError {
    pub fn new(error_type: ApiErrorType) -> ApiError {
        error_type.into()
    }
}

impl From<ApiErrorType> for ApiError {
    fn from(error_type: ApiErrorType) -> Self {
        Self {
            status: error_type.status(),
            message: error_type.message(),
            code: error_type.code(),
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
            BlockingError::Canceled => ApiError::new(ApiErrorType::InternalError),
        }
    }
}

impl From<diesel::result::Error> for ApiError {
    fn from(error: diesel::result::Error) -> Self {
        match error {
            diesel::result::Error::NotFound => ApiError::new(ApiErrorType::DbRecordNotFound),
            _ => ApiError::new(ApiErrorType::DatabaseDieselError(error)),
        }
    }
}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let status_code = match StatusCode::from_u16(self.status) {
            Ok(status_code) => status_code,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        HttpResponse::build(status_code).json(self)
    }
}

impl From<UserServiceError> for ApiError {
    fn from(error: UserServiceError) -> Self {
        match error {
            UserServiceError::GenericDatabaseError(e) => e.into(),
        }
    }
}
