use async_trait::async_trait;

use crate::error::ApiError;

use super::Category;

#[async_trait]
pub trait Repository {
    async fn create(&self, category: Category) -> Result<Category, ApiError>;
    async fn find_by_store(&self, store_id: i32) -> Result<Vec<Category>, ApiError>;
    async fn delete(&self, id: i32) -> Result<(), ApiError>;
    async fn update(&self, id: i32, name: &str) -> Result<Category, ApiError>;
}
