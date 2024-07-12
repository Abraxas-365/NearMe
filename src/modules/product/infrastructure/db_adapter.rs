use crate::error::ApiError;
use crate::modules::product::ports::Repository;
use crate::modules::product::{Price, Product, ProductImage};
use crate::utils::database::PostgresRepository;
use async_trait::async_trait;
use sqlx::postgres::PgDatabaseError;
use sqlx::{Postgres, Transaction};

#[async_trait]
impl Repository for PostgresRepository {
    async fn create(&self, product: Product) -> Result<Product, ApiError> {
        let mut tx: Transaction<'_, Postgres> =
            self.pg_pool.begin().await.map_err(ApiError::from)?;

        let query = "
            INSERT INTO products (sku, category_id, name, description, store_id, visible, has_multiple_prices)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Product>(query)
            .bind(&product.sku)
            .bind(product.category_id)
            .bind(&product.name)
            .bind(&product.description)
            .bind(product.store_id)
            .bind(product.visible)
            .bind(product.has_multiple_prices)
            .fetch_one(&mut *tx)
            .await;

        let product = match result {
            Ok(product) => product,
            Err(e) => {
                tx.rollback().await.map_err(ApiError::from)?;
                if let Some(db_error) = e.as_database_error() {
                    if let Some(pg_error) = db_error.try_downcast_ref::<PgDatabaseError>() {
                        if pg_error.code() == "23505" {
                            // unique_violation
                            return Err(ApiError::Conflict("SKU already exists".into()));
                        }
                    }
                }
                return Err(ApiError::from(e));
            }
        };

        // Add default price
        let default_price = Price::new(product.id, "Default", 0.00, None, true);
        self.add_price(default_price, product.store_id).await?;

        tx.commit().await.map_err(ApiError::from)?;
        Ok(product)
    }

    async fn delete(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM products WHERE id = $1 AND store_id = $2 RETURNING id;";
        let result = sqlx::query(query)
            .bind(id)
            .bind(store_id)
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

    async fn delete_by_category(&self, category_id: i32, store_id: i32) -> Result<(), ApiError> {
        let query = "DELETE FROM products WHERE category_id = $1 AND store_id = $2;";
        sqlx::query(query)
            .bind(category_id)
            .bind(store_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn update(&self, product: Product, store_id: i32) -> Result<Product, ApiError> {
        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        let query = "
            UPDATE products
            SET sku = $1, category_id = $2, name = $3, description = $4, store_id = $5, visible = $6, has_multiple_prices = $7
            WHERE id = $8 AND store_id = $9
            RETURNING *;
        ";
        let result = sqlx::query_as::<_, Product>(query)
            .bind(&product.sku)
            .bind(product.category_id)
            .bind(&product.name)
            .bind(&product.description)
            .bind(product.store_id)
            .bind(product.visible)
            .bind(product.has_multiple_prices)
            .bind(product.id)
            .bind(store_id)
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

    async fn add_images(&self, images: &[ProductImage], store_id: i32) -> Result<(), ApiError> {
        let mut tx: Transaction<'_, Postgres> =
            self.pg_pool.begin().await.map_err(ApiError::from)?;

        for image in images {
            // Ensure each image belongs to a product from the correct store
            let product = self.find_by_id(image.product_id).await?;
            if product.store_id != store_id {
                return Err(ApiError::Forbidden("Store ID mismatch".into()));
            }

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

    async fn delete_image(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        // First fetch the image to ensure it belongs to the correct store
        let query = "SELECT * FROM product_images WHERE id = $1;";
        let image = sqlx::query_as::<_, ProductImage>(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(image) = image {
            // Check if the product_id belongs to the correct store
            let product = self.find_by_id(image.product_id).await?;
            if product.store_id != store_id {
                return Err(ApiError::Forbidden("Store ID mismatch".into()));
            }

            // Proceed to delete
            let delete_query = "DELETE FROM product_images WHERE id = $1;";
            sqlx::query(delete_query)
                .bind(id)
                .execute(&*self.pg_pool)
                .await
                .map_err(ApiError::from)?;
            Ok(())
        } else {
            Err(ApiError::NotFound(format!(
                "Image with id {} not found",
                id
            )))
        }
    }

    async fn find_images_by_product(&self, product_id: i32) -> Result<Vec<ProductImage>, ApiError> {
        let query = "SELECT * FROM product_images WHERE product_id = $1;";
        let images = sqlx::query_as::<_, ProductImage>(query)
            .bind(product_id)
            .fetch_all(&*self.pg_pool)
            .await;

        images.map_err(ApiError::from)
    }

    async fn delete_images_by_product(
        &self,
        product_id: i32,
        store_id: i32,
    ) -> Result<(), ApiError> {
        // Ensure the product belongs to the correct store
        let product = self.find_by_id(product_id).await?;
        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        let query = "DELETE FROM product_images WHERE product_id = $1;";
        sqlx::query(query)
            .bind(product_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }

    async fn add_price(&self, price: Price, store_id: i32) -> Result<(), ApiError> {
        // Ensure the product belongs to the correct store
        let product = self.find_by_id(price.product_id).await?;
        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        let mut tx: Transaction<'_, Postgres> =
            self.pg_pool.begin().await.map_err(ApiError::from)?;

        if price.is_default {
            // Set all other prices for this product to non-default
            let update_query = "UPDATE prices SET is_default = FALSE WHERE product_id = $1;";
            sqlx::query(update_query)
                .bind(price.product_id)
                .execute(&mut *tx)
                .await
                .map_err(ApiError::from)?;
        }

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
            .fetch_one(&mut *tx)
            .await;

        match result {
            Ok(_) => {
                tx.commit().await.map_err(ApiError::from)?;
                Ok(())
            }
            Err(e) => {
                tx.rollback().await.map_err(ApiError::from)?;
                Err(ApiError::from(e))
            }
        }
    }

    async fn delete_price(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        // First fetch the price to ensure it belongs to a product from the correct store
        let query = "SELECT * FROM prices WHERE id = $1;";
        let price = sqlx::query_as::<_, Price>(query)
            .bind(id)
            .fetch_optional(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;

        if let Some(price) = price {
            // Check if the product_id belongs to the correct store
            let product = self.find_by_id(price.product_id).await?;
            if product.store_id != store_id {
                return Err(ApiError::Forbidden("Store ID mismatch".into()));
            }

            // Proceed to delete
            let delete_query = "DELETE FROM prices WHERE id = $1;";
            sqlx::query(delete_query)
                .bind(id)
                .execute(&*self.pg_pool)
                .await
                .map_err(ApiError::from)?;
            Ok(())
        } else {
            Err(ApiError::NotFound(format!(
                "Price with id {} not found",
                id
            )))
        }
    }

    async fn find_price_by_product(&self, product_id: i32) -> Result<Vec<Price>, ApiError> {
        let query = "SELECT * FROM prices WHERE product_id = $1;";
        let prices = sqlx::query_as::<_, Price>(query)
            .bind(product_id)
            .fetch_all(&*self.pg_pool)
            .await;

        prices.map_err(ApiError::from)
    }

    async fn update_price(&self, price: Price, store_id: i32) -> Result<Price, ApiError> {
        // Ensure the product belongs to the correct store
        let product = self.find_by_id(price.product_id).await?;
        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        let mut tx: Transaction<'_, Postgres> =
            self.pg_pool.begin().await.map_err(ApiError::from)?;

        if price.is_default {
            // Set all other prices for this product to non-default
            let update_query =
                "UPDATE prices SET is_default = FALSE WHERE product_id = $1 AND id != $2;";
            sqlx::query(update_query)
                .bind(price.product_id)
                .bind(price.id)
                .execute(&mut *tx)
                .await
                .map_err(ApiError::from)?;
        }

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
            .fetch_one(&mut *tx)
            .await;

        match result {
            Ok(updated_price) => {
                tx.commit().await.map_err(ApiError::from)?;
                Ok(updated_price)
            }
            Err(e) => {
                tx.rollback().await.map_err(ApiError::from)?;
                Err(ApiError::from(e))
            }
        }
    }

    async fn delete_price_by_product(
        &self,
        product_id: i32,
        store_id: i32,
    ) -> Result<(), ApiError> {
        // Ensure the product belongs to the correct store
        let product = self.find_by_id(product_id).await?;
        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        let query = "DELETE FROM prices WHERE product_id = $1;";
        sqlx::query(query)
            .bind(product_id)
            .execute(&*self.pg_pool)
            .await
            .map_err(ApiError::from)?;
        Ok(())
    }
}
