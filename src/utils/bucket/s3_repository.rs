use aws_config::BehaviorVersion;
use aws_sdk_s3::Client;

pub struct S3Repository {
    pub client: Client,
    pub bucket: String,
}

impl S3Repository {
    pub async fn new(bucket: String) -> Self {
        let config = aws_config::defaults(BehaviorVersion::latest()).load().await;
        let client = Client::new(&config);

        S3Repository { client, bucket }
    }
}
