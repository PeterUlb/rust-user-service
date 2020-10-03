use crate::db;
use crate::db::PgPool;
use crate::error::ApiError;
use crate::error::MissingField;
use crate::model::User;
use crate::service;
use actix_web::get;
use actix_web::web;
use actix_web::web::Json;
use actix_web::HttpRequest;

pub mod users;

#[get("/")]
pub async fn hello(req: HttpRequest, pool: web::Data<PgPool>) -> Result<Json<Vec<User>>, ApiError> {
    let x = &*req.extensions();
    println!("{:?}", x);
    //let i: i32 = *user_service;
    let conn = db::get_conn(&pool)?;
    let users = web::block(move || service::user_service::get_all_user(&conn)).await?;
    Ok(Json(users))
}

#[get("/echo")]
pub async fn echo() -> Result<Json<String>, ApiError> {
    Err(ApiError::MissingFields(vec![MissingField {
        field_name: String::from("abc"),
        internal_code: 33,
    }]))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(echo);
    cfg.service(hello);
}
