use std::sync::RwLock;

use kokoros::tts::koko::TTSKoko;

pub struct TtsEngine {
    pub kokoros: RwLock<TTSKoko>,
}

impl TtsEngine {
    #[must_use]
    pub async fn new() -> Self {
        let model_path = "kokoro-v1.0.onnx";
        let voices_path = "voices-v1.0.bin";

        let kokoros = TTSKoko::new(model_path, voices_path).await;

        Self {
            kokoros: RwLock::new(kokoros),
        }
    }
    pub fn synthesize(&self, txt: &str, voice: &str) -> anyhow::Result<Vec<f32>> {
        let koko = self
            .kokoros
            .read()
            .map_err(|_| anyhow::anyhow!("TtsEngine mutex is poisoned"))?;

        let result = koko
            .tts_raw_audio(txt, "en", voice, 1.0, None, None, None, None)
            .unwrap();

        Ok(result)
    }
}
