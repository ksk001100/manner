use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use crate::{Pool, Response};
use actix_web::{delete, get, post, web, HttpResponse};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct InputUser {
    pub name: String,
    pub email: String,
}

#[get("/api/users")]
pub async fn index(db: web::Data<Pool>) -> Response {
    Ok(web::block(move || get_users(db))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[get("/api/users/{id}")]
pub async fn show(db: web::Data<Pool>, user_id: web::Path<u64>) -> Response {
    Ok(web::block(move || get_user(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[post("/api/users")]
pub async fn new(db: web::Data<Pool>, item: web::Json<InputUser>) -> Response {
    Ok(web::block(move || add_user(db, item))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

#[delete("/api/users/{id}")]
pub async fn destroy(db: web::Data<Pool>, user_id: web::Path<u64>) -> Response {
    Ok(web::block(move || delete_user(db, user_id.into_inner()))
        .await
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError())?)
}

fn get_users(pool: web::Data<Pool>) -> Result<Vec<User>, diesel::result::Error> {
    let conn = pool.get().unwrap();
    Ok(users.load::<User>(&conn)?)
}

fn get_user(pool: web::Data<Pool>, user_id: u64) -> Result<User, diesel::result::Error> {
    let conn = pool.get().unwrap();
    users.find(user_id).get_result::<User>(&conn)
}

fn add_user(
    pool: web::Data<Pool>,
    item: web::Json<InputUser>,
) -> Result<User, diesel::result::Error> {
    use diesel::insert_into;

    let conn = pool.get().unwrap();
    let new_user = NewUser {
        name: &item.name,
        email: &item.email,
        created_at: chrono::Local::now().naive_local(),
        updated_at: chrono::Local::now().naive_local(),
    };

    insert_into(users).values(&new_user).execute(&conn)?;

    Ok(users.order_by(id.desc()).first(&conn).unwrap())
}

fn delete_user(pool: web::Data<Pool>, user_id: u64) -> Result<usize, diesel::result::Error> {
    use diesel::delete;

    let conn = pool.get().unwrap();
    let count = delete(users.find(user_id)).execute(&conn)?;
    Ok(count)
}
