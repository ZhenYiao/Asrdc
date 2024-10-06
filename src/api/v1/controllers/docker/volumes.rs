use std::collections::HashMap;
use actix_web::{web, Responder};
use serde::Deserialize;
use serde_json::json;
use crate::api::write::AppResponse;
use crate::docker::clients::DOCKER;
use crate::docker::service::images::ImagesController;
use crate::docker::service::volumes::VolumesController;

#[derive(Deserialize)]
pub struct DockerVolumesCreate {
    pub nodename: String,
    pub name: String,
    pub label: HashMap<String,String>
}
#[derive(Deserialize)]
pub struct DockerVolumesDelete {
    pub nodename: String,
    pub name: String,
    pub force: bool
}
#[derive(Deserialize)]
pub struct DockerVolumesList {
    pub nodename: Vec<String>,
}


pub async fn create(dto: web::Json<DockerVolumesCreate>) -> impl Responder{
    let name = dto.name.clone();
    let label = dto.label.clone();
    let nodename = dto.nodename.clone();
    let client = DOCKER.get().unwrap();
    let client = match client.get_nodename(&nodename){
        None => {
            return AppResponse::err("Nodename ID not found")
        }
        Some(ok) => ok,
    };
    let client = VolumesController::from(client);
    let labels = label.iter().map(|(x,y)|{
        (x.as_str(),y.as_str())
    })
    .collect::<HashMap<&str, &str>>();
    match client.create(&name,Some(labels)).await {
        Ok(_) => {
            AppResponse::ok("Success")
        }
        Err(e) => {
            AppResponse::err(&format!("Error creating volume {}: {}",&nodename,e))
        }
    }
}

pub async fn delete(dto: web::Json<DockerVolumesDelete>) -> impl Responder {
    let name = dto.name.clone();
    let force = dto.force.clone();
    let nodename = dto.nodename.clone();
    let client = DOCKER.get().unwrap();
    let client = match client.get_nodename(&nodename){
        None => {
            return AppResponse::err("Nodename ID not found")
        }
        Some(ok) => ok,
    };
    let client = VolumesController::from(client);
    match client.remove(&name,force).await {
        Ok(_) => {
            AppResponse::ok("Success")
        }
        Err(e) => {
            AppResponse::err(&format!("Error delete volume {}: {}",&nodename,e))
        }
    }
}

pub async fn list(dto: web::Json<DockerVolumesList>) -> impl Responder{
    let nodename = dto.nodename.clone();
    let mut list = Vec::new();
    let client = DOCKER.get().unwrap();

    let mut active = 0;
    let mut unactive = 0;
    for idx in nodename{
        if let Some(client) =  client.get_nodename(&idx){
            list.push(VolumesController::from(client));
            active += 1;
        }else {
            unactive += 1;
        }
    }
    let mut result = Vec::new();
    let mut warnings = Vec::new();
    for idx in list{
        if let Ok(ls) = idx.list().await{
            result.extend(ls.volumes.unwrap_or(Vec::new()));
            warnings.extend(ls.warnings.unwrap_or(Vec::new()));
        }
    }
    let data_set = result.len();
    AppResponse{
        code: 200,
        data: json!({
            "volumes": result,
            "warnings": warnings,
        }),
        msg: json!({
            "success": active,
            "filed": unactive,
            "dataset": data_set,
        }).to_string(),
    }
}