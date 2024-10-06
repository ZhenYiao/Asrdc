use actix_web::web;

pub mod images;
pub mod volumes;
pub fn router(cfg:&mut web::ServiceConfig){
    cfg
        .service(
            web::scope("/images")
                .route("/pull", web::post().to(images::pull))
                .route("/list", web::post().to(images::list))
                .route("/remove", web::post().to(images::remove))
        )
        .service(
            web::scope("/volumes")
                .route("/create", web::post().to(volumes::create))
                .route("/list", web::post().to(volumes::list))
                .route("/delete", web::post().to(volumes::delete))
        )
    ;
}