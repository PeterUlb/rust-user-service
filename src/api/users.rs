use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::model::users::RegisterUserDto;
use crate::service;
use crate::validator::Validate;
use actix_web::web::Json;
use actix_web::{post, web};

#[post("/users")]
pub async fn create_user(
    register_dto: web::Json<RegisterUserDto>,
    pool: web::Data<PgPool>,
    argon2_config: web::Data<argon2::Config<'static>>,
) -> Result<Json<String>, ApiError> {
    register_dto.validate()?; //TODO: Extractor for web::JsonValidated

    let conn = db::get_conn(&pool)?;

    web::block(move || service::user_service::register_user(&conn, register_dto.0, &argon2_config))
        .await?;
    Ok(Json(String::from("ok")))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(create_user);
}
