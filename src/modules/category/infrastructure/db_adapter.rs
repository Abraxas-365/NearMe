use crate::error::ApiError;
use crate::modules::category::ports::Repository;
use crate::modules::category::Category;
use crate::utils::database::PostgresRepository;
use async_trait::async_trait;

#[async_trait]
impl Repository for PostgresRepository {
    async fn create(&self, category: Category) -> Result<Category, ApiError> {
        let query = "
            INSERT INTO categories (name, store_id)
            VALUES ($1, $2)
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Category>(query)
            .bind(&category.name)
            .bind(category.store_id)
            .fetch_one(&*self.pg_pool)
            .await;

        result.map_err(ApiError::from)
    }

    async fn find_by_store(&self, store_id: i32) -> Result<Vec<Category>, ApiError> {
        let query = "SELECT * FROM categories WHERE store_id = $1;";
        let categories = sqlx::query_as::<_, Category>(query)
            .bind(store_id)
            .fetch_all(&*self.pg_pool)
            .await;

        categories.map_err(ApiError::from)
    }

    async fn delete(&self, id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM categories WHERE id = $1 RETURNING id;";
        let result = sqlx::query(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(_) = result {
            Ok(())
        } else {
            Err(ApiError::NotFound(format!(
                "Category with id {} not found",
                id
            )))
        }
    }

    async fn update(&self, id: i32, name: &str) -> Result<Category, ApiError> {
        let query = "
            UPDATE categories
            SET name = $1
            WHERE id = $2
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Category>(query)
            .bind(name)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(category) = result {
            Ok(category)
        } else {
            Err(ApiError::NotFound(format!(
                "Category with id {} not found",
                id
            )))
        }
    }
}
