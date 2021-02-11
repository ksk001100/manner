#[macro_use]
extern crate diesel;

pub mod handlers;
pub mod models;
pub mod schema;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenv::dotenv;
use std::env;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[get("/api")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello API!")
}

#[post("/api/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
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
            .service(echo)
            .service(handlers::users::get_users)
            .route("/api/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:8088")?
    .run()
    .await
}
