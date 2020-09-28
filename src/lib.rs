#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
use crate::repository::user_repository::UserRepositoryImpl;
use crate::service::user_service::UserService;
use crate::service::user_service::UserServiceImpl;
use actix_web::{web, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use std::sync::Arc;

mod api;
mod auth;
mod configuration;
mod db;
mod error;
mod middleware;
mod model;
mod repository;
mod schema;
mod service;

#[actix_web::main]
pub async fn run() -> std::io::Result<()> {
    let start = std::time::Instant::now();

    let config = match configuration::Configuration::new() {
        Ok(config) => Arc::new(config),
        Err(e) => {
            env_logger::init(); // Use default settings, since we have no config
            error!("{}", e);
            return Err(std::io::Error::from(std::io::ErrorKind::NotFound));
        }
    };
    println!("{:?}", config);

    // TODO: move
    env_logger::builder()
        .parse_filters(&config.logging.filters)
        .init();

    let manager = ConnectionManager::<PgConnection>::new(&config.database.url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create database pool");
    // test if db conn works
    pool.get().unwrap();

    let argon2_config = Arc::new(argon2::Config::default());

    let user_repo = Arc::new(UserRepositoryImpl::new(pool.clone())); // Pool clone will perform a clone on the inner arc

    // Until https://github.com/actix/actix-web/issues/1710 is solved, we need to an Arc<Box<dyn>>, even though Arc<dyn> should be good enough
    let user_service: Arc<Box<dyn UserService>> = Arc::new(Box::new(UserServiceImpl::new(
        Arc::clone(&user_repo),
        Arc::clone(&argon2_config),
    )));

    let port = config.app.port;

    // actix creates multiple copies of the application state and the handlers. It creates one copy for each thread.
    // from avoids double Arc, since we already have an Arc and will use that
    let user_service_app_data = web::Data::from(user_service);

    info!("Initial setup took {} ms", start.elapsed().as_millis());
    HttpServer::new(move || {
        let mut exempt_path = std::collections::HashMap::new();
        exempt_path.insert(
            String::from("/api/v1/users"),
            vec![actix_web::http::Method::POST],
        );
        exempt_path.insert(
            String::from("/api/v1/users/"),
            vec![actix_web::http::Method::POST],
        );

        let exempt_path = std::rc::Rc::new(exempt_path);
        App::new()
            .app_data(user_service_app_data.clone())
            .wrap(middleware::jwt::JwtAuth::new(
                config.jwt.clone(),
                exempt_path.clone(),
            ))
            .wrap(actix_web::middleware::Logger::default())
            .service(web::scope("/api/v1").configure(api::users::init_routes))
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
