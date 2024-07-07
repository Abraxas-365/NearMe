use std::sync::Arc;

use crate::error::ApiError;

use super::{ports::Repository, Category};

#[derive(Clone)]
pub struct Service {
    category_repository: Arc<dyn Repository>,
}
impl Service {
    pub fn new(category_repository: Arc<dyn Repository>) -> Self {
        Self {
            category_repository,
        }
    }

    pub async fn create(&self, category: Category) -> Result<Category, ApiError> {
        let category = self.category_repository.create(category).await?;
        Ok(category)
    }

    pub async fn find_categories_form_store(
        &self,
        store_id: i32,
    ) -> Result<Vec<Category>, ApiError> {
        let categories = self
            .category_repository
            .find_categories_form_store(store_id)
            .await?;
        Ok(categories)
    }

    pub async fn delete(&self, id: i32) -> Result<(), crate::error::ApiError> {
        self.category_repository.delete(id).await?;
        Ok(())
    }

    pub async fn update(&self, id: i32, name: &str) -> Result<Category, ApiError> {
        let category = self.category_repository.update(id, name).await?;
        Ok(category)
    }
}
