use actix_web::web;
use actix_web;

use crate::api::{todo, user};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(todo::todo::config)
            .configure(user::user::config)
    );
}
