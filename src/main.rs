use wis_rs::hf::download_model;
use wis_rs::router::serve;
use wis_rs::trace::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    tracing::info!("starting");

    download_model().await?;

    serve().await?;

    Ok(())
}
