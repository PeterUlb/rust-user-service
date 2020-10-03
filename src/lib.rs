#[macro_use]
extern crate diesel;
#[macro_use]
extern crate log;
extern crate validator;
use actix_web::http::StatusCode;
use actix_web::{web, App, HttpResponse, HttpServer};
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

    let argon2_config = web::Data::new(argon2::Config::default());
    let port = config.app.port;

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
            .data(pool.clone())
            .app_data(argon2_config.clone())
            // FromRequest for Json<T> checks app_data extension map for JsonConfig type, and if peresent uses that
            .app_data(
                web::JsonConfig::default()
                    .limit(4096)
                    .content_type(|_| true)
                    .error_handler(|err, _| {
                        error!("{}", err);
                        HttpResponse::build(StatusCode::BAD_REQUEST).into()
                    }),
            )
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
