use std::num::NonZeroUsize;
use std::sync::{LazyLock, Mutex};
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_lab::middleware::map_response;
use lru::LruCache;
use tokio::sync::OnceCell;
use tracing::info;
use crate::api::middleware::serve::serve;

pub mod v1;
pub mod write;
mod middleware;

pub static CACHE: LazyLock<Mutex<LruCache<String, Vec<String>>>> = LazyLock::new(||Mutex::new(LruCache::<String,Vec<String>>::new(NonZeroUsize::new(512).unwrap())));

pub async fn api() -> anyhow::Result<()>{
    info!("Api System Start");
    HttpServer::new(move || {
         App::new()
             .wrap(actix_web::middleware::Logger::default())
             .wrap(actix_web::middleware::Compress::default())
             .service(hello)
             .service(
                 web::scope("/api")
                     .configure(v1::configure)
             )
             .wrap(map_response(serve))

    })
        .bind(("127.0.0.1", 32981))?
        .bind_auto_h2c(("127.0.0.1", 32982))?
        .max_connections(usize::MAX)
        .workers(8)
        .run().await?;
    Ok(())
}


#[get("/")]
async fn hello() -> impl Responder {
    "Hello Actix!!!!"
}