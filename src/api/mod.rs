use crate::error::ApiError;
use crate::model::User;
use crate::service::user_service::UserService;
use actix_web::web;
use actix_web::web::Json;
use actix_web::HttpRequest;
use actix_web::{get, HttpResponse, Responder};

pub mod user;

#[get("/")]
pub async fn hello(
    req: HttpRequest,
    user_service: web::Data<Box<dyn UserService>>,
) -> Result<Json<Vec<User>>, ApiError> {
    let x = &*req.extensions();
    println!("{:?}", x);
    //let i: i32 = *user_service;
    let users = web::block(move || user_service.get_all_user()).await?;
    Ok(Json(users))
}

#[get("/echo")]
pub async fn echo() -> Result<Json<String>, ApiError> {
    //Err(ApiError::new(ApiErrorType::InternalError))
    Ok(Json(String::from("s: &str")))
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
