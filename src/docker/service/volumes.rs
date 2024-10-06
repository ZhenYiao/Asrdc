use std::collections::HashMap;
use bollard::Docker;
use bollard::models::VolumeListResponse;
use bollard::volume::{CreateVolumeOptions, ListVolumesOptions, PruneVolumesOptions, RemoveVolumeOptions};
use uuid::Uuid;
use crate::docker::metadata::DockerMetadata;

pub struct VolumesController{
    inner: DockerMetadata
}

impl From<DockerMetadata> for VolumesController {
    fn from(value: DockerMetadata) -> Self {
        Self { inner: value }
    }
}

impl From<Docker> for VolumesController {
    fn from(value: Docker) -> Self {
        let uid = Uuid::new_v4().to_string();
        Self{ inner:DockerMetadata::new(uid,value) }
    }
}

impl VolumesController {
    pub async fn create(&self,name: &str,label: Option<HashMap<&str,&str>>) -> anyhow::Result<()> {
        let client = self.inner.connect.clone();
        let option = CreateVolumeOptions{
            name,
            labels: label.unwrap_or(Default::default()),
            ..Default::default()
        };
        client.create_volume(option).await?;
        Ok(())
    }
    pub async fn delete(&self,name: &str,force: bool) -> anyhow::Result<()> {
        let client = self.inner.connect.clone();
        let option = Some(RemoveVolumeOptions{
            force,
        });
        client.remove_volume(name, option).await?;
        Ok(())
    }
    pub async fn list(&self) -> anyhow::Result<VolumeListResponse>{
        let client = self.inner.connect.clone();
        let option = Some(ListVolumesOptions::<String>{
            ..Default::default()
        });
        let list = client.list_volumes(option).await?;
        Ok(list)
    }
    pub async fn prune(&self,filters: HashMap<String,Vec<String>>) -> anyhow::Result<()> {
        let client = self.inner.connect.clone();
        let options = Some(PruneVolumesOptions{
            filters,
        });
        client.prune_volumes(options).await?;
        Ok(())
    }

    pub async fn remove(&self,name: &str,force: bool) -> anyhow::Result<()> {
        let client = self.inner.connect.clone();
        let option = Some(RemoveVolumeOptions{
            force,
        });
        client.remove_volume(name, option).await?;
        Ok(())
    }
}