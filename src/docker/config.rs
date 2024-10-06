use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Deserialize,Serialize)]
pub struct DockerConfig {
    load_localhost: bool,
    load_remote_http: Vec<String>,
}

impl DockerConfig {
    pub fn new() -> Self {
        Self{
            load_localhost: true,
            load_remote_http: vec![],
        }
    }
    pub fn read() -> DockerConfig {
        if std::fs::read_dir("./config").is_err(){
            std::fs::create_dir("./config").unwrap()
        }
        if let Ok(mut file) = std::fs::File::open("./config/docker.toml"){
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            let serde = toml::from_str::<DockerConfig>(&contents).unwrap_or_else(|_| DockerConfig::new());
            serde
        } else {
            let serde = toml::to_string(&Self::new()).unwrap();
            std::fs::write("./config/docker.toml", serde).unwrap();
            Self::new()
        }
    }
    pub fn commit(&self){
        if std::fs::read_dir("./config").is_err(){
            std::fs::create_dir("./config").unwrap()
        }
        if let Ok(mut file) = std::fs::File::open("./config/docker.toml"){
            file.write(toml::to_string(&self).unwrap().as_bytes()).unwrap();
        }else {
            std::fs::write("./config/docker.toml", toml::to_string(&self).unwrap().as_bytes()).unwrap();
        }
    }
}
