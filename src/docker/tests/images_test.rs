use tokio::sync::mpsc::unbounded_channel;
use crate::docker::service::images::ImagesController;
use crate::docker::metadata::DockerMetadata;

#[tokio::test]
async fn docker_test_image_pull() -> anyhow::Result<()> {
    let meta = DockerMetadata::load_localhost()?;
    let image = ImagesController::from(meta);
    let (tx, mut rx) = unbounded_channel();
    image.pull(
        "testcontainers/helloworld".to_string(),
        "latest".to_string(),
        tx.clone(),
    ).await?;
    while let Some(pull) = rx.recv().await {
        if let Some(p) = pull {
            dbg!(p);
        }else {
            break;
        }
    }
    Ok(())
}

#[tokio::test]
async fn docker_test_image_list() -> anyhow::Result<()> {
    let meta = DockerMetadata::load_localhost()?;
    let image = ImagesController::from(meta);
    let resp = image.list().await?;
    assert!(resp.len() > 0);
    Ok(())

}
#[tokio::test]
async fn docker_test_image_delete() -> anyhow::Result<()> {
    let meta = DockerMetadata::load_localhost()?;
    let image = ImagesController::from(meta);
    image.remove("helloworld".to_string(),"latest".to_string()).await?;
    Ok(())
}
