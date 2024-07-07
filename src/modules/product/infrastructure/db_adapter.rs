use crate::error::ApiError;
use crate::modules::product::ports::Repository;
use crate::modules::product::{Price, Product, ProductImage};
use crate::utils::database::PostgresRepository;
use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;

#[async_trait]
impl Repository for PostgresRepository {
    async fn create(&self, product: Product) -> Result<Product, ApiError> {
        let query = "
            INSERT INTO products (sku, category_id, name, description, store_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Product>(query)
            .bind(&product.sku)
            .bind(product.category_id)
            .bind(&product.name)
            .bind(&product.description)
            .bind(product.store_id)
            .fetch_one(&*self.pg_pool)
            .await;

        result.map_err(ApiError::from)
    }

    async fn delete(&self, id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM products WHERE id = $1 RETURNING id;";
        let result = sqlx::query(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(_) = result {
            Ok(())
        } else {
            Err(ApiError::NotFound(format!(
                "Product with id {} not found",
                id
            )))
        }
    }

    async fn delete_by_category(&self, category_id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM products WHERE category_id = $1;";
        sqlx::query(query)
            .bind(category_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn update(&self, product: Product) -> Result<Product, ApiError> {
        let query = "
            UPDATE products
            SET sku = $1, category_id = $2, name = $3, description = $4, store_id = $5
            WHERE id = $6
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Product>(query)
            .bind(&product.sku)
            .bind(product.category_id)
            .bind(&product.name)
            .bind(&product.description)
            .bind(product.store_id)
            .bind(product.id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(product) = result {
            Ok(product)
        } else {
            Err(ApiError::NotFound(format!(
                "Product with id {} not found",
                product.id
            )))
        }
    }

    async fn find_by_category(&self, category_id: i32) -> Result<Vec<Product>, ApiError> {
        let query = "SELECT * FROM products WHERE category_id = $1;";
        let products = sqlx::query_as::<_, Product>(query)
            .bind(category_id)
            .fetch_all(&*self.pg_pool)
            .await;

        products.map_err(ApiError::from)
    }

    async fn find_by_store(&self, store_id: i32) -> Result<Vec<Product>, ApiError> {
        let query = "SELECT * FROM products WHERE store_id = $1;";
        let products = sqlx::query_as::<_, Product>(query)
            .bind(store_id)
            .fetch_all(&*self.pg_pool)
            .await;

        products.map_err(ApiError::from)
    }

    async fn find_by_id(&self, id: i32) -> Result<Product, ApiError> {
        let query = "SELECT * FROM products WHERE id = $1;";
        let product = sqlx::query_as::<_, Product>(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(product) = product {
            Ok(product)
        } else {
            Err(ApiError::NotFound(format!(
                "Product with id {} not found",
                id
            )))
        }
    }

    async fn add_images(&self, images: &[ProductImage]) -> Result<(), ApiError> {
        let mut tx: Transaction<'_, Postgres> =
            self.pg_pool.begin().await.map_err(ApiError::from)?;

        for image in images {
            let query = "
                INSERT INTO product_images (product_id, url)
                VALUES ($1, $2);
            ";
            sqlx::query(query)
                .bind(image.product_id)
                .bind(&image.url)
                .execute(&mut *tx)
                .await
                .map_err(ApiError::from)?;
        }

        tx.commit().await.map_err(ApiError::from)?;
        Ok(())
    }

    async fn delete_image(&self, id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM product_images WHERE id = $1;";
        sqlx::query(query)
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn find_images_by_product(&self, product_id: i32) -> Result<Vec<ProductImage>, ApiError> {
        let query = "SELECT * FROM product_images WHERE product_id = $1;";
        let images = sqlx::query_as::<_, ProductImage>(query)
            .bind(product_id)
            .fetch_all(&*self.pg_pool)
            .await;

        images.map_err(ApiError::from)
    }

    async fn delete_images_by_product(&self, product_id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM product_images WHERE product_id = $1;";
        sqlx::query(query)
            .bind(product_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn add_price(&self, price: Price) -> Result<(), ApiError> {
        let query = "
            INSERT INTO prices (product_id, name, price, discount, is_default)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Price>(query)
            .bind(price.product_id)
            .bind(&price.name)
            .bind(price.price)
            .bind(price.discount)
            .bind(price.is_default)
            .fetch_one(&*self.pg_pool)
            .await;

        result.map_err(ApiError::from)?;
        Ok(())
    }

    async fn delete_price(&self, id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM prices WHERE id = $1;";
        sqlx::query(query)
            .bind(id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn find_price_by_product(&self, product_id: i32) -> Result<Vec<Price>, ApiError> {
        let query = "SELECT * FROM prices WHERE product_id = $1;";
        let prices = sqlx::query_as::<_, Price>(query)
            .bind(product_id)
            .fetch_all(&*self.pg_pool)
            .await;

        prices.map_err(ApiError::from)
    }

    async fn update_price(&self, price: Price) -> Result<Price, ApiError> {
        let query = "
            UPDATE prices
            SET product_id = $1, name = $2, price = $3, discount = $4, is_default = $5
            WHERE id = $6
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Price>(query)
            .bind(price.product_id)
            .bind(&price.name)
            .bind(price.price)
            .bind(price.discount)
            .bind(price.is_default)
            .bind(price.id)
            .fetch_one(&*self.pg_pool)
            .await;

        result.map_err(ApiError::from)
    }

    async fn delete_price_by_product(&self, product_id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM prices WHERE product_id = $1;";
        sqlx::query(query)
            .bind(product_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }
}
