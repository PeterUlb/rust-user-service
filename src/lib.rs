#[macro_use]
extern crate diesel;
use crate::repository::{UserRepository, UserRepositoryImpl};
use crate::service::{UserService, UserServiceImpl};
use actix_web::{middleware, web, App, HttpServer};
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

    // TODO: move
    env_logger::Builder::new()
        .parse_filters(&config.logging.filters)
        .default_format()
        .init();

    let manager = ConnectionManager::<PgConnection>::new(&config.database.url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");
    // test if db conn works
    pool.get().unwrap();

    let user_repo: Arc<dyn UserRepository> = Arc::new(UserRepositoryImpl::new(pool.clone())); // Pool clone will perform a clone on the inner arc
    let user_service: Arc<dyn UserService> = Arc::new(UserServiceImpl::new(user_repo));

    let port = config.app.port;

    let user_service = web::Data::new(user_service);
    let config_shared = web::Data::new(config);

    // Http server constructs an application instance for each thread, thus application data must be constructed multiple times.
    // If we want to share data between different threads, a shared object should be used, e.g. Arc.
    // Internally, web::Data uses Arc. Thus, in order to avoid creating two Arcs, we should create our Data before registering it using App::app_data().
    HttpServer::new(move || {
        App::new()
            .app_data(user_service.clone())
            .app_data(config_shared.clone())
            .wrap(middleware::Logger::default())
            .service(api::hello)
            .service(api::echo)
            .service(web::scope("/user").service(api::user::echo))
            .route("/hey", web::get().to(api::manual_hello))
    })
    .bind(format!("127.0.0.1:{}", port))?
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
