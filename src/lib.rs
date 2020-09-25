#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use crate::repository::user_repository::UserRepositoryImpl;
use crate::service::user_service::UserServiceImpl;
use actix_web::{middleware, web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

mod api;
mod configuration;
mod db;
mod error;
mod model;
mod repository;
mod schema;
mod service;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let start = std::time::Instant::now();

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

    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone())); // Pool clone will perform a clone on the inner arc
    let user_service = Arc::new(UserServiceImpl::new(Arc::clone(&user_repo)));

    let port = config.app.port;

    // from avoids double Arc, since we already have an Arc and will use that
    // Http server constructs an application instance for each thread, thus application data must be constructed multiple times.
    // If we want to share data between different threads, a shared object should be used, e.g. Arc.
    // Internally, web::Data uses Arc. Thus, in order to avoid creating two Arcs, we should create our Data before registering it using App::app_data().
    let user_service_app_data = web::Data::from(user_service);
    let config_shared_app_data = web::Data::new(config);
    info!("Initial setup took {} ms", start.elapsed().as_millis());
    HttpServer::new(move || {
        App::new()
            .app_data(user_service_app_data.clone())
            .app_data(config_shared_app_data.clone())
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
