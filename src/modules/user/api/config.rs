use actix_web::web;

use super::handlers::{login, logout, signup};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .route("/logout", web::get().to(logout))
            .route("/signup", web::post().to(signup))
            .route("/login", web::post().to(login)),
    );
}
