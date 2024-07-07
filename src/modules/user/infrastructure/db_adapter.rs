use async_trait::async_trait;

use crate::error::ApiError;
use crate::modules::user::ports::Repository;
use crate::modules::user::User;
use crate::utils::database::PostgresRepository;

#[async_trait]
impl Repository for PostgresRepository {
    async fn create(&self, username: &str, password: &str) -> Result<User, ApiError> {
        let query = "
            INSERT INTO users (username, password)
            VALUES ($1, $2)
            RETURNING *;
        ";

        let result = sqlx::query_as::<_, User>(query)
            .bind(username)
            .bind(password)
            .fetch_one(&*self.pg_pool)
            .await?;
        Ok(result)
    }

    async fn find_by_username(&self, username: &str) -> Result<User, ApiError> {
        let query = "SELECT * FROM users WHERE username = $1";

        let result = sqlx::query_as::<_, User>(query)
            .bind(username)
            .fetch_one(&*self.pg_pool)
            .await?;

        Ok(result)
    }
}
