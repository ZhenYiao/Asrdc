use bollard::{Docker, API_DEFAULT_VERSION};

#[derive(Clone)]
pub struct DockerMetadata {
    pub nodename: String,
    pub connect: Docker,
}

impl DockerMetadata {
    pub fn new(nodename: String, connect: Docker) -> Self {
        DockerMetadata { nodename, connect }
    }
    pub fn load_localhost() -> anyhow::Result<DockerMetadata> {
        let nodename = "localhost";
        let client = Docker::connect_with_unix_defaults()?;
        Ok(DockerMetadata::new(nodename.to_string(), client))
    }
    pub fn load_remote_http(nodename:&str,addr: &str) -> anyhow::Result<DockerMetadata> {
        let client = Docker::connect_with_http(addr,120, API_DEFAULT_VERSION)?;
        Ok(DockerMetadata::new(nodename.to_string(), client))
    }
}