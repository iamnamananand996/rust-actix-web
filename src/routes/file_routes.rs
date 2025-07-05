use crate::routes::{handlers, middlewares::auth_middlewares};
use actix_web::{middleware::from_fn, web};

pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/file")
            .wrap(from_fn(auth_middlewares::auth_middleware))
            .service(handlers::file_handler::upload_file),
    );
}
