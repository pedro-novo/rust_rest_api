use actix_web::web;
use actix_web::{web::{
    Data,
    Json,
}, post, get, put, delete, HttpResponse};
use crate::{models::user::User, repository::database::Database};

#[get("")]
pub async fn get_users(db: Data<Database>) -> HttpResponse {
    let users = db.get_users();
    HttpResponse::Ok().json(users)
}

#[get("/{id}")]
pub async fn get_user_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.get_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[post("")]
pub async fn create_user(db: Data<Database>, new_user: Json<User>) -> HttpResponse {
    let user = db.create_user(new_user.into_inner());
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[put("/{id}")]
pub async fn update_user_by_id(db: Data<Database>, id: web::Path<String>, updated_user: Json<User>) -> HttpResponse {
    let user = db.update_user_by_id(&id, updated_user.into_inner());
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[delete("/{id}")]
pub async fn delete_user_by_id(db: Data<Database>, id: web::Path<String>) -> HttpResponse {
    let user = db.delete_user_by_id(&id);
    match user {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().body("User not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .service(get_users)
            .service(get_user_by_id)
            .service(create_user)
            .service(update_user_by_id)
            .service(delete_user_by_id)
    );
}