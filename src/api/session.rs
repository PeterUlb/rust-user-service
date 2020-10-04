use crate::auth;
use crate::auth::AccessClaims;
use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::model::sessions::Session;
use crate::service;
use actix_web::web::Json;
use actix_web::{get, web};

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

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sessions);
}
