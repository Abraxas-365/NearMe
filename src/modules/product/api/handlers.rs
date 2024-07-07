use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::product::{Product, ProductUpdateRequest, Service},
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
pub struct CreateProductRequest {
    category_id: i32,
    name: String,
    description: String,
    visible: bool,
    store_id: i32, //TODO dlete this
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
        new_product.visible,
    );

    let created_product = service.create(product).await?;
    Ok(HttpResponse::Created().json(created_product))
}

// Handler for deleting a product by id
pub async fn delete_product(
    service: web::Data<Arc<Service>>,
    product_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let store_id = session.get::<i32>("user_id").unwrap().unwrap(); //TODO
    service.delete(*product_id, store_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

#[derive(Deserialize)]
pub struct UpdateVisibilityRequest {
    visible: bool,
}

pub async fn update_product(
    service: web::Data<Arc<Service>>,
    product_id: web::Path<i32>,
    update_request: web::Json<ProductUpdateRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let store_id = session.get::<i32>("user_id").unwrap().unwrap(); //TODO: Handle session error properly
    let updated_product = service
        .update_product(*product_id, update_request.into_inner(), store_id)
        .await?;
    Ok(HttpResponse::Ok().json(updated_product))
}
