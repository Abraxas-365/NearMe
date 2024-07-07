use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub store_id: i32,
}
impl Category {
    pub fn new(name: &str, store_id: i32) -> Self {
        Self {
            id: 0, //when saved in the db it will get the id from the auto increment
            name: name.into(),
            store_id,
        }
    }
}
