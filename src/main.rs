use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::{Database, DatabaseConnection};
use migration::{Migrator, MigratorTrait};

use crate::utils::app_state::AppState;

mod utils;
mod routes;

#[actix_web::main]  
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        unsafe { std::env::set_var("RUST_LOG", "actix_web=info"); }
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address = utils::constants::ADDRESS.clone();
    let port = utils::constants::PORT.clone();
    let db: DatabaseConnection = Database::connect(utils::constants::DATABASE_URL.clone()).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .configure(routes::home_routes::home_routes)
            .configure(routes::auth_routes::auth_routes)
    })
    .bind(format!("{}:{}", address, port))?
    .run()
    .await
}

