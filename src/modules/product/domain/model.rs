use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Product {
    pub id: i32,
    pub sku: String,
    pub category_id: i32,
    pub name: String,
    pub description: String,
    pub store_id: i32,
}

impl Product {
    pub fn new(category_id: i32, name: &str, description: &str, store_id: i32) -> Product {
        Self {
            id: 0, //when saved in the db it will get the id from the auto increment
            sku: Product::generate_sku(name),
            category_id,
            name: name.into(),
            description: description.into(),
            store_id,
        }
    }

    pub fn with_custom_sku(mut self, sku: &str) -> Self {
        self.sku = sku.into();
        self
    }

    fn generate_sku(product_name: &str) -> String {
        let short_name = product_name
            .chars()
            .take(3)
            .collect::<String>()
            .to_uppercase();
        let unique_id = Uuid::new_v4().to_string();
        format!("PROD-{}-{}", short_name, unique_id)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProductImage {
    pub id: i32,
    pub product_id: i32,
    pub url: String,
}
impl ProductImage {
    pub fn new(product_id: i32, url: &str) -> Self {
        Self {
            id: 0, //when saved in the db it will get the id from the auto increment
            product_id,
            url: url.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Price {
    pub id: i32,
    pub product_id: i32,
    pub name: String,
    pub price: f64,
    pub discount: Option<f64>,
    pub is_default: bool,
}

impl Price {
    pub fn new(
        product_id: i32,
        name: &str,
        price: f64,
        discount: Option<f64>,
        is_default: bool,
    ) -> Self {
        Self {
            id: 0,
            product_id,
            name: name.into(),
            price,
            discount,
            is_default,
        }
    }
}