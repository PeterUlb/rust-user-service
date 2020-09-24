use crate::configuration::Configuration;
use crate::error::ApiError;
use actix_web::get;
use actix_web::web;
use actix_web::web::Json;

#[get("/")]
pub async fn echo(config: web::Data<Configuration>) -> Result<Json<Configuration>, ApiError> {
    //Err(ApiError::new(ApiErrorType::InternalError))

    Ok(Json((**config).clone()))
}
