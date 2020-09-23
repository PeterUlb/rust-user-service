use crate::error::{ApiError, ApiErrorType};
use crate::model::User;
use crate::service::UserService;
use actix_web::web;
use actix_web::web::Json;
use actix_web::{get, HttpResponse, Responder};
use std::sync::Arc;

#[get("/")]
pub async fn hello(
    user_service: web::Data<Arc<dyn UserService>>,
) -> Result<Json<Vec<User>>, ApiError> {
    let users = web::block(move || user_service.get_all_user()).await?;
    Ok(Json(users))
}

#[get("/echo")]
pub async fn echo() -> Result<Json<String>, ApiError> {
    Err(ApiError::new(ApiErrorType::InternalError))
    //Ok(Json(String::from("s: &str")))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
