use std::sync::{Arc, Mutex};

use anyhow::{Result, anyhow};
use sherpa_rs::tts::{TtsAudio, VitsTts, VitsTtsConfig};

#[derive(Clone)]
pub struct TtsEngine {
    tts: Arc<Mutex<VitsTts>>,
}

impl Default for TtsEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl TtsEngine {
    #[must_use]
    pub fn new() -> Self {
        let config = VitsTtsConfig {
            model: "./models/tts/vits-piper-en_US-amy-medium-fp16/en_US-amy-medium.onnx"
                .to_string(),
            tokens: "./models/tts/vits-piper-en_US-amy-medium-fp16/tokens.txt".to_string(),
            data_dir: "./models/tts/vits-piper-en_US-amy-medium-fp16/espeak-ng-data".to_string(),
            length_scale: 1.0,
            silence_scale: 1.1,
            ..Default::default()
        };

        let tts = Arc::new(Mutex::new(VitsTts::new(config)));
        Self { tts }
    }

    /// # Errors
    /// - when the mutex is poisoned
    pub fn synthesize(&self, text: &str, sid: i32, speed: f32) -> Result<TtsAudio> {
        let mut tts = self
            .tts
            .lock()
            .map_err(|e| anyhow!("TTS mutex poisoned: {e:#?}"))?;

        tts.create(text, sid, speed).map_err(|e| anyhow!("{e:#?}"))
    }
}
