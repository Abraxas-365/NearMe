use actix_web::{web, HttpResponse};
use serde::Deserialize;
use std::sync::Arc;

use crate::{
    error::ApiError,
    modules::category::{Category, Service},
};

// Handler for creating a new category
#[derive(Deserialize)]
pub struct CreateCategoryRequest {
    name: String,
    store_id: i32,
}

pub async fn create_category(
    service: web::Data<Arc<Service>>,
    new_category: web::Json<CreateCategoryRequest>,
) -> Result<HttpResponse, ApiError> {
    let category = Category::new(&new_category.name, new_category.store_id);
    let created_category = service.create(category).await?;
    Ok(HttpResponse::Created().json(created_category))
}

// Handler for finding categories by store_id
pub async fn find_categories_by_store(
    service: web::Data<Arc<Service>>,
    store_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    let categories = service.find_categories_form_store(*store_id).await?;
    Ok(HttpResponse::Ok().json(categories))
}

// Handler for deleting a category by id
pub async fn delete_category(
    service: web::Data<Arc<Service>>,
    category_id: web::Path<i32>,
) -> Result<HttpResponse, ApiError> {
    service.delete(*category_id).await?;
    Ok(HttpResponse::NoContent().finish())
}

// Handler for updating a category by id
#[derive(Deserialize)]
pub struct UpdateCategoryRequest {
    name: String,
}

pub async fn update_category(
    service: web::Data<Arc<Service>>,
    category_id: web::Path<i32>,
    update_data: web::Json<UpdateCategoryRequest>,
) -> Result<HttpResponse, ApiError> {
    let updated_category = service.update(*category_id, &update_data.name).await?;
    Ok(HttpResponse::Ok().json(updated_category))
}
