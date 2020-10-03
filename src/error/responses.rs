use crate::error::codes::ErrorCode;
use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct DefaultErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
    pub code: u16,
}

impl DefaultErrorResponse {
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            status_code: code.1,
            message: message,
            code: code.0,
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct FieldErrorResponse {
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
    pub message: String,
    pub code: u16,
    pub details: Vec<crate::error::Field>,
}

impl FieldErrorResponse {
    pub fn new(code: ErrorCode, message: String, fields: Vec<crate::error::Field>) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: message,
            code: code.0,
            details: fields,
        }
    }
}
