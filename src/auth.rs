use crate::configuration;
use crate::error::ApiError;
use actix_web::http::header::HeaderMap;
use actix_web::{dev, FromRequest, HttpRequest};
use futures::future::{err, ok, Ready};
use jsonwebtoken::decode;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::Validation;
use serde::Deserialize;
use serde::Serialize;
use std::error;
use std::fmt;

#[derive(Debug)]
pub enum AuthorizationError {
    NoAuthorizationForAction,
    UserDoesNotExist,
    PasswordInvalid,
    JwtValidationError(jsonwebtoken::errors::Error),
    SessionTokenBlacklisted,
}

impl fmt::Display for AuthorizationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl error::Error for AuthorizationError {}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SessionClaims {
    pub exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub session_id: uuid::Uuid,
    pub user_id: i64,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessClaims {
    pub exp: i64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    pub iat: i64, // Optional. Issued at (as UTC timestamp)
    pub iss: String, // Optional. Issuer
    pub user_id: i64,
}

impl FromRequest for AccessClaims {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.extensions().get::<AccessClaims>() {
            Some(claims) => ok(claims.clone()),
            None => {
                error!("Could not extract Claims from JWT (should have been added in middleware)");
                err(ApiError::InternalServerError) // TODO
            }
        }
    }
}

pub fn verify_subject(user_id: i64, sub: i64) -> Result<(), AuthorizationError> {
    if user_id != sub {
        return Err(AuthorizationError::NoAuthorizationForAction);
    }
    Ok(())
}

pub fn decode_access_jwt(
    token: &str,
    jwt_config: &configuration::Jwt,
) -> Result<AccessClaims, ApiError> {
    let decoding_key = DecodingKey::from_secret(&jwt_config.access_secret.as_ref());
    decode::<AccessClaims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| {
            error!("{}", e);
            ApiError::JwtValidationError(e)
        })
}

pub fn decode_session_jwt(
    token: &str,
    jwt_config: &configuration::Jwt,
) -> Result<SessionClaims, AuthorizationError> {
    let decoding_key = DecodingKey::from_secret(&jwt_config.session_secret.as_ref());
    decode::<SessionClaims>(token, &decoding_key, &Validation::default())
        .map(|data| data.claims)
        .map_err(|e| {
            error!("{}", e);
            AuthorizationError::JwtValidationError(e)
        })
}

pub fn get_auth_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")?
        .to_str()
        .ok()
        .and_then(|s| s.strip_prefix("Bearer "))
}
