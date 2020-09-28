use crate::configuration::Configuration;
use crate::error::ApiError;
use crate::service::user_service::UserService;
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
    user_service: web::Data<Box<dyn UserService>>,
) -> Result<Json<Vec<String>>, ApiError> {
    let x = &*req.extensions();
    info!("{:?}", x);
    let x = web::block(move || -> Result<i32, ApiError> { Ok(user_service.register_user("Blub")) })
        .await?;
    info!("{}", x);
    Ok(Json(vec!["a".to_owned()]))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(echo);
    cfg.service(hello);
}
