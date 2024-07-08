use std::sync::Arc;

use uuid::Uuid;

use crate::error::ApiError;

use super::{
    ports::{ImageRepository, Repository},
    Price, Product, ProductDetail, ProductImage, ProductUpdateRequest,
};

pub struct Service {
    product_repository: Arc<dyn Repository>,
    image_repository: Arc<dyn ImageRepository>,
}

impl Service {
    pub fn new(
        product_repository: Arc<dyn Repository>,
        image_repository: Arc<dyn ImageRepository>,
    ) -> Self {
        Self {
            product_repository,
            image_repository,
        }
    }
}

impl Service {
    pub async fn get_product_detail(&self, product_id: i32) -> Result<ProductDetail, ApiError> {
        let product = self.find_by_id(product_id).await?;
        let images = self.find_images_by_product(product_id).await?;
        let prices = self.find_price_by_product(product_id).await?;
        Ok(ProductDetail {
            product,
            images,
            prices,
        })
    }

    pub async fn get_product_details_by_category(
        &self,
        category_id: i32,
    ) -> Result<Vec<ProductDetail>, ApiError> {
        let products = self.find_by_category(category_id).await?;
        let mut details = Vec::new();
        for product in products {
            let images = self.find_images_by_product(product.id).await?;
            let prices = self.find_price_by_product(product.id).await?;
            details.push(ProductDetail {
                product,
                images,
                prices,
            });
        }
        Ok(details)
    }

    pub async fn get_product_details_by_store(
        &self,
        store_id: i32,
    ) -> Result<Vec<ProductDetail>, ApiError> {
        let products = self.find_by_store(store_id).await?;
        let mut details = Vec::new();
        for product in products {
            let images = self.find_images_by_product(product.id).await?;
            let prices = self.find_price_by_product(product.id).await?;
            details.push(ProductDetail {
                product,
                images,
                prices,
            });
        }
        Ok(details)
    }
}

impl Service {
    pub async fn create(&self, product: Product) -> Result<Product, ApiError> {
        let product = self.product_repository.create(product).await?;
        Ok(product)
    }

    pub async fn delete(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        self.product_repository.delete(id, store_id).await?;
        Ok(())
    }

    pub async fn delete_by_category(
        &self,
        category_id: i32,
        store_id: i32,
    ) -> Result<(), ApiError> {
        self.product_repository
            .delete_by_category(category_id, store_id)
            .await?;
        Ok(())
    }

    pub async fn update_product(
        &self,
        product_id: i32,
        update_request: ProductUpdateRequest,
        store_id: i32,
    ) -> Result<Product, ApiError> {
        let mut product = self.product_repository.find_by_id(product_id).await?;

        if product.store_id != store_id {
            return Err(ApiError::Forbidden("Store ID mismatch".into()));
        }

        if let Some(sku) = update_request.sku {
            product.sku = sku;
        }
        if let Some(category_id) = update_request.category_id {
            product.category_id = category_id;
        }
        if let Some(name) = update_request.name {
            product.name = name;
        }
        if let Some(description) = update_request.description {
            product.description = description;
        }
        if let Some(visible) = update_request.visible {
            product.visible = visible;
        }

        let updated_product = self.product_repository.update(product, store_id).await?;
        Ok(updated_product)
    }

    pub async fn find_by_category(&self, category_id: i32) -> Result<Vec<Product>, ApiError> {
        let products = self
            .product_repository
            .find_by_category(category_id)
            .await?;
        Ok(products)
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Product, ApiError> {
        let product = self.product_repository.find_by_id(id).await?;
        Ok(product)
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

    pub async fn add_images(&self, image: &[ProductImage], store_id: i32) -> Result<(), ApiError> {
        self.product_repository.add_images(image, store_id).await?;
        Ok(())
    }

    pub async fn delete_image(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        self.product_repository.delete_image(id, store_id).await?;
        Ok(())
    }

    pub async fn post_presigned_url(
        &self,
        product_id: i32,
        store_id: i32,
    ) -> Result<String, ApiError> {
        let key = format!("{}/{}/{}", store_id, product_id, Uuid::new_v4());
        let url = self.image_repository.post_presigned_url(key).await?;
        Ok(url)
    }
}

impl Service {
    pub async fn add_price(&self, price: Price, store_id: i32) -> Result<(), ApiError> {
        self.product_repository.add_price(price, store_id).await?;
        Ok(())
    }

    pub async fn delete_price(&self, id: i32, store_id: i32) -> Result<(), ApiError> {
        self.product_repository.delete_price(id, store_id).await?;
        Ok(())
    }

    pub async fn delete_price_by_product(
        &self,
        product_id: i32,
        store_id: i32,
    ) -> Result<(), ApiError> {
        self.product_repository
            .delete_price_by_product(product_id, store_id)
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

    pub async fn update_price(&self, price: Price, store_id: i32) -> Result<Price, ApiError> {
        let price = self
            .product_repository
            .update_price(price, store_id)
            .await?;
        Ok(price)
    }
}
