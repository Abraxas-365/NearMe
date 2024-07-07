use actix_web::web;

use super::handlers::{
    create_category, delete_category, find_categories_by_store, update_category,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/categories")
            .route("", web::post().to(create_category))
            .route("/store/{store_id}", web::get().to(find_categories_by_store))
            .route("/{id}", web::delete().to(delete_category))
            .route("/{id}", web::put().to(update_category)),
    );
}
