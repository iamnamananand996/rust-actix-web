use actix_web::{App, HttpServer, middleware::Logger, web};
use aws_sdk_s3::config::Region;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

use crate::utils::app_state::AppState;

mod routes;
mod utils;

#[derive(Debug)]
struct MainError {
    error: String,
}

impl std::fmt::Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl std::error::Error for MainError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.error
    }
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.source()
    }
}

#[actix_web::main]
async fn main() -> Result<(), MainError> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe {
            std::env::set_var("RUST_LOG", "actix_web=info");
        }
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address = utils::constants::ADDRESS.clone();
    let port = *utils::constants::PORT;
    let db: DatabaseConnection = Database::connect(utils::constants::DATABASE_URL.clone())
        .await
        .map_err(|e| MainError {
            error: e.to_string(),
        })?;
    Migrator::up(&db, None).await.map_err(|e| MainError {
        error: e.to_string(),
    })?;

    // Initialize AWS S3 client
    let aws_config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(Region::new(utils::constants::AWS_REGION.clone()))
        .load()
        .await;

    // Create S3 client with proper configuration
    let s3_config = aws_sdk_s3::config::Builder::from(&aws_config)
        .force_path_style(false) // Use virtual-hosted-style requests
        .build();
    let s3_client = aws_sdk_s3::Client::from_conf(s3_config);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState {
                db: db.clone(),
                s3_client: s3_client.clone(),
            }))
            .configure(routes::user_routes::user_routes)
            .configure(routes::auth_routes::auth_routes)
            .configure(routes::post_routes::post_routes)
            .configure(routes::file_routes::file_routes)
            .configure(routes::websocket_routes::websocket_routes)
            .configure(routes::websocket_routes::websocket_api_routes)
    })
    .bind(format!("{address}:{port}"))
    .map_err(|e| MainError {
        error: e.to_string(),
    })?
    .run()
    .await
    .map_err(|e| MainError {
        error: e.to_string(),
    })?;

    Ok(())
}
