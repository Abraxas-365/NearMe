use std::time::Duration;

use async_trait::async_trait;
use aws_sdk_s3::presigning::PresigningConfig;

use crate::{
    error::ApiError, modules::product::ports::ImageRepository, utils::bucket::S3Repository,
};

#[async_trait]
impl ImageRepository for S3Repository {
    async fn post_presigned_url(&self, key: String) -> Result<String, ApiError> {
        let config = PresigningConfig::expires_in(Duration::from_secs(300)).unwrap(); // URL valid for 5 minutes
        let presigned_request = self
            .client
            .put_object()
            .bucket(&self.bucket)
            .key(&key)
            .presigned(config)
            .await
            .map_err(|e| ApiError::UnexpectedError(e.to_string()))?;

        Ok(presigned_request.uri().to_string())
    }
}
