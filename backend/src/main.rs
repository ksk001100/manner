#[macro_use]
extern crate diesel;

pub mod auth;
pub mod errors;
pub mod handlers;
pub mod models;
pub mod schema;

use actix_web::{dev::ServiceRequest, get, App, Error, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;
// use diesel::prelude::*;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type Response = Result<HttpResponse, Error>;

async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    match auth::validate_token(credentials.token()) {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");

    let database_url = env::var("DATABASE_URL").expect("Failed to get database url...");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool...");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(auth)
            .data(pool.clone())
            .service(hello)
            .service(handlers::users::index)
            .service(handlers::users::show)
            .service(handlers::users::new)
            .service(handlers::users::destroy)
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
