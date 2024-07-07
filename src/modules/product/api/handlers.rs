use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::product::{Product, Service},
};

pub async fn get_product_detail(
    service: web::Data<Arc<Service>>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let product_detail = service.get_product_detail(*product_id).await?;
    Ok(HttpResponse::Ok().json(product_detail))
}

// Handler for retrieving product details by category_id
pub async fn get_product_details_by_category(
    service: web::Data<Arc<Service>>,
    category_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let product_details = service
        .get_product_details_by_category(*category_id)
        .await?;
    Ok(HttpResponse::Ok().json(product_details))
}

// Handler for retrieving product details by store_id
pub async fn get_product_details_by_store(
    service: web::Data<Arc<Service>>,
    store_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let product_details = service.get_product_details_by_store(*store_id).await?;
    Ok(HttpResponse::Ok().json(product_details))
}

// Handler for creating a new product
#[derive(Deserialize)]
struct CreateProductRequest {
    category_id: i32,
    name: String,
    description: String,
    store_id: i32,
}

pub async fn create_product(
    service: web::Data<Arc<Service>>,
    new_product: web::Json<CreateProductRequest>,
) -> Result<HttpResponse, ApiError> {
    let product = Product::new(
        new_product.category_id,
        &new_product.name,
        &new_product.description,
        new_product.store_id, //TODO:Get the store id from the session
    );

    let created_product = service.create(product).await?;
    Ok(HttpResponse::Created().json(created_product))
}

// Handler for deleting a product by id
pub async fn delete_product(
    service: web::Data<Arc<Service>>,
    product_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    service.delete(*product_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Add routes to actix-web
