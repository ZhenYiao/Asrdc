use std::collections::HashMap;
use bollard::Docker;
use bollard::models::Network;
use bollard::network::{CreateNetworkOptions, ListNetworksOptions};
use uuid::Uuid;
use crate::docker::metadata::DockerMetadata;

pub struct NetworksController {
    inner: DockerMetadata
}

impl From<DockerMetadata> for NetworksController {
    fn from(meta: DockerMetadata) -> Self {
        Self { inner: meta }
    }
}

impl From<Docker> for NetworksController {
    fn from(value: Docker) -> Self {
        let uid = Uuid::new_v4().to_string();
        Self{ inner: DockerMetadata::new(uid,value) }
    }
}
impl NetworksController {
    pub async fn create(&self,name: String, labels: HashMap<String, String>) -> anyhow::Result<()>{
        let client = self.inner.connect.clone();
        let option = CreateNetworkOptions{
            name,
            check_duplicate: true,
            internal: false,
            ingress: false,
            labels,
            ..Default::default()
        };
        client.create_network(option).await?;
        Ok(())

    }
    pub async fn list(&self) -> anyhow::Result<Vec<Network>>{
        let client = self.inner.connect.clone();
        let option = Some(ListNetworksOptions::<String>{
            ..Default::default()
        });
        let resp = client.list_networks(option).await?;
        Ok(resp)
    }
}