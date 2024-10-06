use tokio::sync::OnceCell;
use crate::docker::metadata::DockerMetadata;


pub static DOCKER: OnceCell<DockerClients> = OnceCell::const_new();


pub struct DockerClients {
    inner: Vec<DockerMetadata>
}
impl DockerClients {
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }
    pub fn push(&mut self, docker: DockerMetadata) {
        self.inner.push(docker);
    }
    pub fn pop(&mut self) -> Option<DockerMetadata> {
        self.inner.pop()
    }
    pub fn get(&self, index: usize) -> Option<&DockerMetadata> {
        self.inner.get(index)
    }
    pub fn get_nodename(&self, nodename: &str) -> Option<DockerMetadata> {
        for (_,idx) in self.inner.iter().enumerate() {
            if idx.nodename == nodename{
                return Some(idx.clone())
            }
        }
        None
    }
    pub fn remove_nodename(&mut self, nodename: &str) {
        self.inner.retain(|docker| docker.nodename != nodename);
    }
    // pub async fn init(){
    //     DOCKER.get_or_init(||async {
    //
    //     }).await;
    // }
}

