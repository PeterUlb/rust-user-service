use actix_web::http::StatusCode;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct ErrorCode(pub u16, pub StatusCode);

impl ErrorCode {
    pub const MISSING_FIELDS: ErrorCode = ErrorCode(4001, StatusCode::BAD_REQUEST);
    pub const JSON_VALIDATION_FAILED: ErrorCode = ErrorCode(4002, StatusCode::BAD_REQUEST);

    pub const ENTITY_ALREADY_EXISTS: ErrorCode = ErrorCode(4900, StatusCode::CONFLICT);

    pub const JWT_VALIDATION_ERROR: ErrorCode = ErrorCode(4003, StatusCode::UNAUTHORIZED);
    pub const MISSING_ACCESS_TOKEN_HEADER: ErrorCode = ErrorCode(4002, StatusCode::UNAUTHORIZED);
    pub const NOT_AUTHORIZED_FOR_ACTION: ErrorCode = ErrorCode(4004, StatusCode::UNAUTHORIZED);

    pub const INTERNAL_SERVER_ERROR: ErrorCode = ErrorCode(5000, StatusCode::INTERNAL_SERVER_ERROR);
}
