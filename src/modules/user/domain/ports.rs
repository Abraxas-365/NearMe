use async_trait::async_trait;

use crate::error::ApiError;

use super::model::User;

#[async_trait]
pub trait Repository {
    async fn create(&self, username: &str, password: &str) -> Result<User, ApiError>;
    async fn find_by_username(&self, username: &str) -> Result<User, ApiError>;
}
