lazy_static::lazy_static! {
    pub static ref ADDRESS: String = set_address();
    pub static ref PORT: u16 = set_port();
    pub static ref DATABASE_URL: String = set_database_url();
    pub static ref JWT_SECRET: String = set_jwt_secret();
}


fn set_address() -> String {
    dotenv::dotenv().ok();
    std::env::var("ADDRESS").expect("ADDRESS must be set")
}

fn set_port() -> u16 {
    dotenv::dotenv().ok();
    std::env::var("PORT").expect("PORT must be set").parse::<u16>().expect("PORT must be a number")
}

fn set_database_url() -> String {
    dotenv::dotenv().ok();
    std::env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn set_jwt_secret() -> String {
    dotenv::dotenv().ok();
    std::env::var("JWT_SECRET").expect("JWT_SECRET must be set")
}

