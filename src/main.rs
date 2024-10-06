use asrdc::api::api;
// use asrdc::log::logger_init;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // logger_init();
    api().await?;
    Ok(())
}