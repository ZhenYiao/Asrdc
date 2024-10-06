use actix_web::{web, Responder};
use lru::LruCache;
use serde::Deserialize;
use serde_json::json;
use tokio::sync::mpsc::unbounded_channel;
use uuid::Uuid;
use crate::api::CACHE;
use crate::api::write::AppResponse;
use crate::docker::clients::DOCKER;
use crate::docker::service::images::ImagesController;
use crate::docker::metadata::DockerMetadata;

#[derive(Deserialize)]
pub struct DockerImagePull {
    pub image: String,
    pub tag: String,
    pub nodename: String,
}
#[derive(Deserialize)]
pub struct DockerImageList {
    pub nodename: Vec<String>,
}
#[derive(Deserialize)]
pub struct DockerImageRemove {
    pub nodename: String,
    pub name: String,
    pub tag: String,
}


pub async fn pull(dto: web::Json<DockerImagePull>) -> impl Responder{
    let image = dto.image.clone();
    let tag = dto.tag.clone();
    let nodename = dto.nodename.clone();
    let client = DOCKER.get().unwrap();
    let client = match client.get_nodename(&nodename){
        None => {
            return AppResponse::err("Nodename ID not found")
        }
        Some(ok) => ok,
    };
    let uid = Uuid::new_v4().to_string();
    let rsuid = uid.clone();
    let (rx,mut tx) = unbounded_channel();
    ImagesController::from(client).pull(image,tag,rx);
    tokio::spawn(async move {
       while let Some(message) = tx.recv().await {
           if let Some(msg) = message {
               let mut lock = CACHE.lock().unwrap();
               if lock.contains(&uid.clone()) {
                   lock.get_mut(&uid.clone()).unwrap().push(serde_json::to_string(&msg).unwrap())
               }else {
                   lock.push(uid.clone(), vec![serde_json::to_string(&msg).unwrap()]);
               }
           }else {
               break;
           }
       }
    });
    AppResponse{
        code: 200,
        data: "".to_string(),
        msg: rsuid,
    }
}
pub async fn list(dto: web::Json<DockerImageList>) -> impl Responder {
    let nodename = dto.nodename.clone();
    let mut list = Vec::new();
    let client = DOCKER.get().unwrap();

    let mut active = 0;
    let mut unactive = 0;
    for idx in nodename{
        if let Some(client) =  client.get_nodename(&idx){
            list.push(ImagesController::from(client));
            active += 1;
        }else {
            unactive += 1;
        }
    }
    let mut result = Vec::new();
    for idx in list{
        if let Ok(ls) = idx.list().await{
            result.extend(ls);
        }
    }
    let data_set = result.len();
    AppResponse{
        code: 200,
        data: result,
        msg: json!({
            "success": active,
            "filed": unactive,
            "dataset": data_set,
        }).to_string(),
    }
}
pub async fn remove(dto: web::Json<DockerImageRemove>) -> impl Responder {
    let name = dto.name.clone();
    let tag = dto.tag.clone();
    let nodename = dto.nodename.clone();
    let client = DOCKER.get().unwrap();
    let client = match client.get_nodename(&nodename){
        None => {
            return AppResponse::err("Nodename ID not found")
        }
        Some(ok) => ok,
    };
    let controllers = ImagesController::from(client);
    match controllers.remove(name,tag).await {
        Ok(_) => {
            AppResponse::ok("Image removed")
        }
        Err(e) => {
            AppResponse::err(format!("Failed to remove image: {}", e).as_str())
        }
    }
}
