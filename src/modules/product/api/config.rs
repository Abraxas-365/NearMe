use actix_web::web;

use super::handlers::{
    create_product, delete_product, get_product_detail, get_product_details_by_category,
    get_product_details_by_store, update_product,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/products")
            .route("/{id}", web::get().to(get_product_detail))
            .route(
                "/category/{id}",
                web::get().to(get_product_details_by_category),
            )
            .route("/store/{id}", web::get().to(get_product_details_by_store))
            .route("", web::post().to(create_product))
            .route("/{id}", web::delete().to(delete_product))
            .route("/{id}", web::patch().to(update_product)),
    );
}
