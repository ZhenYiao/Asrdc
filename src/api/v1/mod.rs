use actix_web::web;

pub mod controllers;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/v1")
                .configure(controllers::routes)
        );
}