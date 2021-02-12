#[macro_use]
extern crate diesel;

pub mod handlers;
pub mod models;
pub mod schema;

use actix_web::{get, App, Error, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
// use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
pub type Response = Result<HttpResponse, Error>;

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("Failed to get database url...");
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    let pool: Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create connection pool...");

    HttpServer::new(move || {
        App::new()
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
