use crate::error::ApiError;
use crate::error::MissingField;
use crate::model::User;
use crate::service::user_service::UserService;
use actix_web::get;
use actix_web::web;
use actix_web::web::Json;
use actix_web::HttpRequest;

pub mod users;

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
    Err(ApiError::MissingFields(vec![MissingField {
        field_name: String::from("abc"),
        internal_code: 33,
    }]))
}
