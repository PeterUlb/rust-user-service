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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccessClaims {
    exp: usize, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize, // Optional. Issued at (as UTC timestamp)
    iss: String, // Optional. Issuer
    sub: String, // Optional. Subject (whom token refers to)
}

impl FromRequest for AccessClaims {
    type Error = ApiError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        match req.extensions().get::<AccessClaims>() {
            Some(claims) => ok(claims.clone()),
            None => err(ApiError::InternalServerError), // TODO
        }
    }
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

pub fn get_auth_token(headers: &HeaderMap) -> Option<&str> {
    headers
        .get("Authorization")?
        .to_str()
        .ok()
        .and_then(|s| s.strip_prefix("Bearer "))
}
