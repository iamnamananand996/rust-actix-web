use actix_web::{middleware::from_fn, web};
use crate::routes::{handlers, middlewares::auth_middlewares};

pub fn home_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/home")
            .wrap(from_fn(auth_middlewares::auth_middleware))
            .service(handlers::home_handler::home)
            .service(handlers::home_handler::users),
    );
}
