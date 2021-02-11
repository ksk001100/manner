use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::Pool;
use actix_web::{get, web, Error, HttpResponse};
use diesel::{QueryDsl, RunQueryDsl};

#[get("/api/users")]
pub async fn get_users(db: web::Data<Pool>) -> Result<HttpResponse, Error> {
    Ok(web::block(move || get_all_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_all_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    Ok(users.load::<User>(&conn)?)
}
