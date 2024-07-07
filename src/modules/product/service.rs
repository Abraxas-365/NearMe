use std::sync::Arc;

use crate::error::ApiError;

use super::{ports::Repository, Price, Product, ProductImage};

pub struct Service {
    product_repository: Arc<dyn Repository>,
}

impl Service {
    pub fn new(product_repository: Arc<dyn Repository>) -> Self {
        Self { product_repository }
    }
}

impl Service {
    pub async fn create(&self, product: Product) -> Result<Product, ApiError> {
        let product = self.product_repository.create(product).await?;
        Ok(product)
    }

    pub async fn delete(&self, id: i32) -> Result<(), ApiError> {
        self.product_repository.delete(id).await?;
        Ok(())
    }

    pub async fn delete_by_category(&self, category_id: i32) -> Result<(), ApiError> {
        self.product_repository
            .delete_by_category(category_id)
            .await?;
        Ok(())
    }

    pub async fn update(&self, product: Product) -> Result<Product, ApiError> {
        let product = self.product_repository.update(product).await?;
        Ok(product)
    }

    pub async fn find_by_category(&self, category_id: i32) -> Result<Vec<Product>, ApiError> {
        let products = self
            .product_repository
            .find_by_category(category_id)
            .await?;
        Ok(products)
    }

    pub async fn find_by_store(&self, store_id: i32) -> Result<Vec<Product>, ApiError> {
        let products = self.product_repository.find_by_store(store_id).await?;
        Ok(products)
    }
}

impl Service {
    pub async fn find_images_by_product(
        &self,
        product_id: i32,
    ) -> Result<Vec<ProductImage>, ApiError> {
        let images = self
            .product_repository
            .find_images_by_product(product_id)
            .await?;
        Ok(images)
    }

    pub async fn add_images(&self, image: &[ProductImage]) -> Result<(), ApiError> {
        self.product_repository.add_images(image).await?;
        Ok(())
    }

    pub async fn delete_image(&self, id: i32) -> Result<(), ApiError> {
        self.product_repository.delete_image(id).await?;
        Ok(())
    }
}

impl Service {
    pub async fn add_price(&self, price: Price) -> Result<(), ApiError> {
        self.product_repository.add_price(price).await?;
        Ok(())
    }

    pub async fn delete_price(&self, id: i32) -> Result<(), ApiError> {
        self.product_repository.delete_price(id).await?;
        Ok(())
    }

    pub async fn delete_price_by_product(&self, product_id: i32) -> Result<(), ApiError> {
        self.product_repository
            .delete_price_by_product(product_id)
            .await?;
        Ok(())
    }

    pub async fn find_price_by_product(&self, product_id: i32) -> Result<Vec<Price>, ApiError> {
        let prices = self
            .product_repository
            .find_price_by_product(product_id)
            .await?;
        Ok(prices)
    }
}
