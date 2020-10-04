use crate::auth;
use crate::auth::AccessClaims;
use crate::configuration::Configuration;
use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::model::sessions::{LoginDto, Session, TokenPairDto};
use crate::service;
use actix_web::web::Json;
use actix_web::{get, post, web};

#[get("/sessions/{user_id}")]
pub async fn get_sessions(
    web::Path(user_id): web::Path<i64>,
    access_claims: AccessClaims,
    pool: web::Data<PgPool>,
) -> Result<Json<Vec<Session>>, ApiError> {
    auth::verify_subject(user_id, access_claims.user_id)?;

    let conn = db::get_conn(&pool)?;
    let sessions =
        web::block(move || service::session_service::get_users_sessions(&conn, user_id)).await?;

    Ok(Json(sessions))
}

#[post("/sessions")]
pub async fn create_session(
    pool: web::Data<PgPool>,
    config: web::Data<Configuration>,
    login_dto: web::Json<LoginDto>,
) -> Result<Json<TokenPairDto>, ApiError> {
    let conn = db::get_conn(&pool)?;
    let token_pair = web::block(move || {
        service::session_service::create_login_token_pair(&conn, &login_dto, &config.jwt)
    })
    .await?;

    Ok(Json(token_pair))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sessions);
    cfg.service(create_session);
}
