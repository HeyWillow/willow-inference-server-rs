pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

pub mod hf;
pub mod router;
pub mod routes;
pub mod state;
#[cfg(feature = "stt")]
pub mod stt;
pub mod trace;
#[cfg(feature = "tts")]
pub mod tts;
pub mod ui;
pub mod util;
