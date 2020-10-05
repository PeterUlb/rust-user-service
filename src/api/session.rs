use crate::auth::AccessClaims;
use crate::configuration::Configuration;
use crate::configuration::Jwt;
use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::model::sessions::{LoginDto, Session};
use crate::service;
use actix_web::web::Json;
use actix_web::{get, http, post, web, HttpMessage, HttpResponse};
use chrono::Utc;

#[get("/sessions")]
pub async fn get_sessions(
    access_claims: AccessClaims,
    pool: web::Data<PgPool>,
) -> Result<Json<Vec<Session>>, ApiError> {
    let conn = db::get_conn(&pool)?;
    let sessions = web::block(move || {
        service::session_service::get_users_sessions(&conn, access_claims.user_id)
    })
    .await?;

    Ok(Json(sessions))
}

#[post("/sessions")]
pub async fn create_session(
    pool: web::Data<PgPool>,
    config: web::Data<Configuration>,
    login_dto: web::Json<LoginDto>,
) -> Result<HttpResponse, ApiError> {
    let conn = db::get_conn(&pool)?;
    let jwt_config = config.jwt.clone();
    let token_pair = web::block(move || {
        service::session_service::create_login_token_pair(&conn, &login_dto, &jwt_config)
    })
    .await?;

    Ok(HttpResponse::Ok()
        .cookie(build_session_cookie(
            config.jwt.clone(),
            token_pair.session_token.token.clone(),
            &token_pair.session_token.expiration,
        ))
        .json(token_pair.access_token))
}

#[post("/sessions/access")]
pub async fn create_access_token(
    pool: web::Data<PgPool>,
    config: web::Data<Configuration>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ApiError> {
    let session_token = req
        .cookie(&config.jwt.session_cookie_name)
        .ok_or(ApiError::MissingSessionCookie)?
        .value()
        .to_string();

    let conn = db::get_conn(&pool)?;
    let jwt_config = config.jwt.clone();
    let token_pair = web::block(move || {
        service::session_service::create_access_token_and_refresh(
            &conn,
            &session_token,
            &jwt_config,
        )
    })
    .await?;

    Ok(HttpResponse::Ok()
        .cookie(build_session_cookie(
            config.jwt.clone(),
            token_pair.session_token.token.clone(),
            &token_pair.session_token.expiration,
        ))
        .json(token_pair.access_token))
}

fn build_session_cookie(
    jwt_config: Jwt,
    token: String,
    exp_time: &chrono::DateTime<Utc>,
) -> actix_web::cookie::Cookie {
    http::Cookie::build(jwt_config.session_cookie_name, token)
        .domain(jwt_config.domain) //TODO
        .path(jwt_config.path) // TODO
        .secure(jwt_config.session_cookie_secure)
        .http_only(true)
        .expires(time::OffsetDateTime::from_unix_timestamp(
            exp_time.timestamp(),
        ))
        .finish()
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sessions);
    cfg.service(create_session);
    cfg.service(create_access_token);
}
