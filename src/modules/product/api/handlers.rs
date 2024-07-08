use actix_session::Session;
use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::product::{Product, ProductImage, ProductUpdateRequest, Service},
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

// Handler for adding images to a product
#[derive(Deserialize)]
pub struct AddImagesRequest {
    pub product_id: i32,
    pub urls: Vec<String>,
}
pub async fn add_images(
    service: web::Data<Arc<Service>>,
    add_images_request: web::Json<AddImagesRequest>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let store_id = session.get::<i32>("user_id").unwrap().unwrap(); // TODO: Handle session error

    let images: Vec<ProductImage> = add_images_request
        .urls
        .iter()
        .map(|url| ProductImage::new(add_images_request.product_id, url))
        .collect();

    service.add_images(&images, store_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Handler for deleting an image
pub async fn delete_image(
    service: web::Data<Arc<Service>>,
    image_id: web::Path<i32>,
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let store_id = session.get::<i32>("user_id").unwrap().unwrap(); // TODO: Handle session error properly
    service.delete_image(*image_id, store_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Handler for generating a presigned URL for image upload
pub async fn get_presigned_url(
    service: web::Data<Arc<Service>>,
    path: web::Path<(i32,)>, // product_id
    session: Session,
) -> Result<HttpResponse, ApiError> {
    let store_id = session.get::<i32>("user_id").unwrap().unwrap(); // TODO: Handle session error properly
    let url = service.post_presigned_url(path.0, store_id).await?;
    Ok(HttpResponse::Ok().json(url))
}
