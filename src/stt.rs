use std::{env, sync::Mutex};

use parakeet_rs::{Parakeet, TimestampMode, Transcriber, TranscriptionResult};

pub struct SttEngine {
    pub parakeet: Mutex<Parakeet>,
}

impl Default for SttEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl SttEngine {
    #[must_use]
    /// # Panics
    /// When `Parakeet` fails to load the model
    pub fn new() -> Self {
        let home = env::var("HOME").unwrap();
        let model_dir = format!(
            "{home}/.cache/huggingface/hub/models--onnx-community--parakeet-ctc-0.6b-ONNX/snapshots/7df2cab7aed886b8b7f80d68a8214007e4847601/onnx"
        );

        let parakeet = Parakeet::from_pretrained(model_dir, None).unwrap();
        let parakeet = Mutex::new(parakeet);

        Self { parakeet }
    }

    /// # Errors
    /// When we fail to acquire mutex lock
    /// When transcription fails
    pub fn transcribe(
        &self,
        samples: Vec<f32>,
        sample_rate: u32,
        channels: u16,
        mode: Option<TimestampMode>,
    ) -> anyhow::Result<TranscriptionResult> {
        let mut parakeet = self
            .parakeet
            .lock()
            .map_err(|_| anyhow::anyhow!("SttEngine mutex is poisoned"))?;

        Ok(parakeet.transcribe_samples(samples, sample_rate, channels, mode)?)
    }
}
