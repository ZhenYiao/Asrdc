use actix_web::web;
use crate::api::v1::controllers::poll::poll;

pub mod docker;
pub mod poll;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
        .service(
            web::scope("/docker")
                .configure(docker::router)
        )
        .route("/poll/{path}",web::get().to(poll))
    ;
}