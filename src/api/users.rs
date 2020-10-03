use crate::configuration::Configuration;
use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::service;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::{get, post, web};

#[get("/users")]
pub async fn echo(config: web::Data<Configuration>) -> Result<Json<Configuration>, ApiError> {
    //Err(ApiError::new(ApiErrorType::InternalError))

    Ok(Json((**config).clone()))
}

#[post("/users")]
pub async fn hello(
    req: HttpRequest,
    pool: web::Data<PgPool>,
    argon2_config: web::Data<argon2::Config<'static>>,
) -> Result<Json<Vec<String>>, ApiError> {
    let x = &*req.extensions();
    info!("{:?}", x);
    let conn = db::get_conn(&pool)?;

    let x = web::block(move || -> Result<i32, ApiError> {
        Ok(service::user_service::register_user(
            &conn,
            "Jochen",
            &argon2_config,
        ))
    })
    .await?;
    info!("{}", x);
    Ok(Json(vec!["a".to_owned()]))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(echo);
    cfg.service(hello);
}
