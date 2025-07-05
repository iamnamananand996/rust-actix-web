lazy_static::lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
    pub static ref S3_BUCKET_NAME: String = set_s3_bucket_name();
    pub static ref AWS_REGION: String = set_aws_region();
}

fn set_address() -> String {
    dotenv::dotenv().ok();
    std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string())
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    std::env::var("PORT")
        .unwrap_or("3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a number")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn set_jwt_secret() -> String {
    dotenv::dotenv().ok();
    std::env::var("JWT_SECRET").unwrap_or("secret".to_string())
}

fn set_s3_bucket_name() -> String {
    dotenv::dotenv().ok();
    std::env::var("S3_BUCKET_NAME").expect("S3_BUCKET_NAME must be set")
}

fn set_aws_region() -> String {
    dotenv::dotenv().ok();
    std::env::var("AWS_REGION").unwrap_or("us-east-1".to_string())
}
