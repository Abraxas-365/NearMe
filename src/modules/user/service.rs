use std::sync::Arc;

use bcrypt::{hash, verify};
use reqwest::StatusCode;

use crate::error::ApiError;

use super::{ports::Repository, User};

pub struct Service {
    user_repository: Arc<dyn Repository>,
}

impl Service {
    pub fn new(user_repository: Arc<dyn Repository>) -> Self {
        Self { user_repository }
    }
}

impl Service {
    pub async fn create(&self, username: &str, password: &str) -> Result<User, ApiError> {
        if let Ok(_) = self.user_repository.find_by_username(username).await {
            return Err(ApiError::HttpError(StatusCode::UNAUTHORIZED));
        }
        let password_hash = hash(password, bcrypt::DEFAULT_COST)?;
        self.user_repository.create(username, &password_hash).await
    }

    pub async fn authenticate(&self, username: &str, password: &str) -> Result<User, ApiError> {
        let user = self.user_repository.find_by_username(username).await?;
        match verify(password, &user.password)? {
            true => Ok(user),
            false => Err(ApiError::HttpError(StatusCode::UNAUTHORIZED)),
        }
    }
}
