use mimalloc::MiMalloc;

#[cfg(all(feature = "hf", feature = "stt"))]
use wis_rs::hf::download_model;
#[cfg(feature = "stt")]
use wis_rs::inference::stt::SttEngine;
#[cfg(feature = "tts")]
use wis_rs::inference::tts::TtsEngine;
use wis_rs::router::serve;
use wis_rs::state::State;
use wis_rs::trace::init_tracing;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing()?;
    tracing::info!("starting");

    #[cfg(all(feature = "hf", feature = "stt"))]
    let stt_model_dir = download_model().await?;
    #[cfg(all(not(feature = "hf"), feature = "stt"))]
    let stt_model_dir = std::path::PathBuf::from("./models/stt");

    #[cfg(any(feature = "stt", feature = "tts"))]
    ort::init().commit();

    #[cfg(feature = "stt")]
    let stt_engine = SttEngine::new(stt_model_dir)?;
    #[cfg(feature = "tts")]
    let tts_engine = TtsEngine::new();

    #[allow(unused_mut)]
    let mut state = State::new();

    #[cfg(feature = "stt")]
    {
        state = state.with_stt_engine(stt_engine);
    }

    #[cfg(feature = "tts")]
    {
        state = state.with_tts_engine(tts_engine);
    }

    serve(state).await?;

    Ok(())
}
