use mimalloc::MiMalloc;

use wis_rs::hf::download_model;
use wis_rs::router::serve;
use wis_rs::state::State;
use wis_rs::stt::SttEngine;
use wis_rs::trace::init_tracing;
use wis_rs::tts::TtsEngine;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    tracing::info!("starting");

    download_model().await?;
    ort::init().commit();

    let stt_engine = SttEngine::new();
    let tts_engine = TtsEngine::new();

    let state = State::new()
        .with_stt_engine(stt_engine)
        .with_tts_engine(tts_engine);

    serve(state).await?;

    Ok(())
}
