use async_trait::async_trait;

use crate::error::ApiError;

use super::{Price, Product, ProductImage};

#[async_trait]
pub trait Repository {
    async fn create(&self, product: Product) -> Result<Product, ApiError>;
    async fn delete(&self, id: i32) -> Result<(), ApiError>;
    async fn delete_by_category(&self, category_id: i32) -> Result<(), ApiError>;
    async fn update(&self, product: Product) -> Result<Product, ApiError>;
    async fn find_by_category(&self, category_id: i32) -> Result<Vec<Product>, ApiError>;
    async fn find_by_store(&self, store_id: i32) -> Result<Vec<Product>, ApiError>;
    async fn find_by_id(&self, id: i32) -> Result<Product, ApiError>;

    async fn add_images(&self, image: &[ProductImage]) -> Result<(), ApiError>;
    async fn delete_image(&self, id: i32) -> Result<(), ApiError>;
    async fn find_images_by_product(&self, product_id: i32) -> Result<Vec<ProductImage>, ApiError>;
    async fn delete_images_by_product(&self, product_id: i32) -> Result<(), ApiError>;

    async fn add_price(&self, price: Price) -> Result<(), ApiError>;
    async fn delete_price(&self, id: i32) -> Result<(), ApiError>;
    async fn find_price_by_product(&self, product_id: i32) -> Result<Vec<Price>, ApiError>;
    async fn update_price(&self, price: Price) -> Result<Price, ApiError>;
    async fn delete_price_by_product(&self, product_id: i32) -> Result<(), ApiError>;
}
