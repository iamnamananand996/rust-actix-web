use actix_web::{middleware::from_fn, web};
use crate::routes::{handlers, middlewares::auth_middlewares};

pub fn user_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .wrap(from_fn(auth_middlewares::auth_middleware))
            .service(handlers::user_handler::get_user)
            .service(handlers::user_handler::users)
            .service(handlers::user_handler::update),
    );
}
