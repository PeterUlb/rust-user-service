#[macro_use]
extern crate diesel;
use crate::repository::UserRepositoryImpl;
use crate::service::{UserService, UserServiceImpl};
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

mod api;
mod configuration;
mod error;
mod model;
mod repository;
mod schema;
mod service;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let config = match configuration::Configuration::new() {
        Ok(config) => config,
        Err(e) => panic!(e),
    };

    println!("{:?}", config);

    let manager = ConnectionManager::<PgConnection>::new(config.database.url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");
    // test if db conn works
    pool.get().unwrap();

    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone())); // Pool clone will perform a clone on the inner arc
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo.clone()));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(user_service.clone()))
            .service(api::hello)
            .service(api::echo)
            .route("/hey", web::get().to(api::manual_hello))
    })
    .bind(format!("127.0.0.1:{}", config.app.port))?
    .run()
    .await
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
