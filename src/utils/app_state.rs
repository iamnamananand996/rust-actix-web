use sea_orm::DatabaseConnection;

pub struct AppState {
    pub db: DatabaseConnection,
    pub s3_client: aws_sdk_s3::Client,
}
