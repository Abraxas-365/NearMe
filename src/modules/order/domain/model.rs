use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(FromRow, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub store_id: i32,
    pub customer_id: i32,
    pub order_date: DateTime<Utc>,
    pub status: String,
    pub total_amount: f64,
    pub order_type: String,
    pub table_number: Option<i32>,
    pub delivery_address: Option<String>,
    pub pending_time: DateTime<Utc>,
    pub accepted_time: Option<DateTime<Utc>>,
    pub ready_for_delivery_time: Option<DateTime<Utc>>,
    pub completed_time: Option<DateTime<Utc>>,
}

impl Order {
    pub fn new(
        store_id: i32,
        customer_id: i32,
        total_amount: f64,
        order_type: String,
        table_number: Option<i32>,
        delivery_address: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // This will be set by the database
            store_id,
            customer_id,
            order_date: now,
            status: "pending".to_string(),
            total_amount,
            order_type,
            table_number,
            delivery_address,
            pending_time: now,
            accepted_time: None,
            ready_for_delivery_time: None,
            completed_time: None,
        }
    }
}

#[derive(FromRow, Debug, Clone)]
pub struct OrderItem {
    pub id: i32,
    pub order_id: i32,
    pub price_id: i32,
    pub quantity: i32,
    pub item_price: f64,
}

impl OrderItem {
    pub fn new(order_id: i32, price_id: i32, quantity: i32, item_price: f64) -> Self {
        Self {
            id: 0, // This will be set by the database
            order_id,
            price_id,
            quantity,
            item_price,
        }
    }
}
