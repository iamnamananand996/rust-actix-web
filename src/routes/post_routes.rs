use crate::routes::{handlers, middlewares::auth_middlewares};
use actix_web::{middleware::from_fn, web};

pub fn post_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/post")
            .wrap(from_fn(auth_middlewares::auth_middleware))
            .service(handlers::post_handler::get_post)
            .service(handlers::post_handler::posts)
            .service(handlers::post_handler::create_post)
            .service(handlers::post_handler::update_post)
            .service(handlers::post_handler::delete_post)
            .service(handlers::post_handler::get_posts_by_user),
    );
}
