use wis_rs::hf::download_model;
use wis_rs::router::serve;
use wis_rs::state::State;
use wis_rs::trace::init_tracing;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    tracing::info!("starting");

    download_model().await?;
    ort::init().commit()?;

    let state = State::new();

    serve(state).await?;

    Ok(())
}
