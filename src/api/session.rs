use crate::auth::AccessClaims;
use crate::error::{ApiError, Field};
use actix_web::web::Json;
use actix_web::{get, web};
//use crate::extractors::

#[get("/sessions/{user_id}")]
pub async fn get_sessions(
    web::Path(user_id): web::Path<String>,
    access_claims: AccessClaims,
) -> Result<Json<i32>, ApiError> {
    info!("{:?}", access_claims);
    let user_id = user_id.parse::<i32>().map_err(|_| {
        ApiError::MissingFields(vec![Field {
            field_name: "/user_id".to_owned(),
        }])
    })?;
    Ok(Json(user_id))
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_sessions);
}
