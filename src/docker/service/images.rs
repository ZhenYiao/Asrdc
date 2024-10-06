use std::task::ready;
use bollard::Docker;
use bollard::image::{BuildImageOptions, CreateImageOptions, ListImagesOptions, RemoveImageOptions, TagImageOptions};
use bollard::models::{CreateImageInfo, ImageSummary};
use uuid::uuid;
use crate::docker::metadata::DockerMetadata;
use futures_core::stream::Stream;
use futures_util::{StreamExt, TryStreamExt};
use tokio::sync::mpsc;

pub struct ImagesController {
    inner: DockerMetadata
}
impl From<DockerMetadata> for ImagesController {
    fn from(meta: DockerMetadata) -> Self {
        Self { inner: meta }
    }
}
impl From<Docker> for ImagesController {
    fn from(value: Docker) -> Self {
        let id = uuid::Uuid::new_v4().to_string();
        Self{ inner:DockerMetadata::new(id,value) }
    }
}

impl ImagesController {
    #[inline]
    pub async fn pull(&self, name: String, tag: String, tx: mpsc::UnboundedSender<Option<CreateImageInfo>>) -> anyhow::Result<()>{
        let client = self.inner.connect.clone();
        let options = Some(CreateImageOptions{
            from_image: format!("{}:{}",name.clone(),tag.clone()),
            ..Default::default()
        });
        let mut next = client.create_image(options, None, None);
        while let Some(next) = next.next().await {
            if let Ok(image) = next {
                tx.send(Some(image))?
            }
        }
        tx.send(None)?;
        Ok(())
    }
    #[inline]
    pub async fn list(&self) -> anyhow::Result<Vec<ImageSummary>>{
        let client = self.inner.connect.clone();
        let options = Some(ListImagesOptions::<String>{
            all: true,
            ..Default::default()
        });
        Ok(client.list_images(options).await?)
    }
    #[inline]
    pub async fn remove(&self, name: String,tag: String) -> anyhow::Result<()>{
        let client = self.inner.connect.clone();
        let delete_options = Some(RemoveImageOptions{
            force: false,
            noprune: false,
        });
        client.remove_image(&format!("{}:{}",name,tag),delete_options, None).await?;
        Ok(())
    }
    #[inline]
    pub async fn tag(&self, name: String, tag: String) -> anyhow::Result<()>{
        let client = self.inner.connect.clone();
        let tag_options = Some(TagImageOptions {
            tag,
            ..Default::default()
        });
        client.tag_image(&name,tag_options).await?;
        Ok(())
    }
    #[inline]
    pub async fn build(&self){
        let client = self.inner.connect.clone();
        let build_options = Some(BuildImageOptions{
            dockerfile: "Dockerfile",
            nocache: true,
            t: "latest",
            rm: true,
            ..Default::default()
        });
    }
}