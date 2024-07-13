use chrono::{DateTime, Utc};
use sqlx::FromRow;

#[derive(sqlx::Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "VARCHAR")]
pub enum OrderType {
    #[sqlx(rename = "delivery")]
    Delivery,
    #[sqlx(rename = "on_site")]
    OnSite,
    #[sqlx(rename = "takeaway")]
    Takeaway,
}

#[derive(sqlx::Type, Debug, PartialEq, Clone)]
#[sqlx(type_name = "VARCHAR")]
pub enum OrderStatus {
    #[sqlx(rename = "pending")]
    Pending,
    #[sqlx(rename = "accepted")]
    Accepted,
    #[sqlx(rename = "ready_for_delivery")]
    ReadyForDelivery,
    #[sqlx(rename = "completed")]
    Completed,
}

#[derive(FromRow, Debug, Clone)]
pub struct Order {
    pub id: i32,
    pub store_id: i32,
    pub customer_id: i32,
    pub order_date: DateTime<Utc>,
    pub status: OrderStatus,
    pub total_amount: f64,
    pub order_type: OrderType,
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
        order_type: OrderType,
        table_number: Option<i32>,
        delivery_address: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: 0, // This will be set by the database
            store_id,
            customer_id,
            order_date: now,
            status: OrderStatus::Pending,
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
