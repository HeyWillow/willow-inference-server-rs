#[allow(clippy::doc_markdown)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[cfg(feature = "hf")]
pub mod hf;
#[cfg(any(feature = "stt", feature = "tts"))]
pub mod inference;
pub mod router;
pub mod routes;
pub mod state;
pub mod trace;
pub mod ui;
pub mod util;
