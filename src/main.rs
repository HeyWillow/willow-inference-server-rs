use wis_rs::router::serve;
use wis_rs::trace::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    tracing::info!("starting");

    serve().await?;

    Ok(())
}
