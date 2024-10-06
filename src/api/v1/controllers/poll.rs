use actix_web::{web, Responder};
use crate::api::CACHE;
use crate::api::write::AppResponse;

pub async fn poll(path: web::Path<String>) -> impl Responder {
    let mut lock = CACHE.lock().unwrap();
    let uid = path.to_string();
    let resp = lock.get(&uid).clone();
    match resp{
        Some(v) => {
            AppResponse{
                code: 200,
                data: v.clone(),
                msg: "Log query successful".to_string(),
            }
        },
        None => {
            AppResponse{
                code: 404,
                data: Vec::new(),
                msg: "You have no logs on the server".to_string(),
            }
        }
    }

}