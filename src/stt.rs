use std::{env, sync::Mutex};

use anyhow::Result;
use parakeet_rs::{Parakeet, TimestampMode, Transcriber, TranscriptionResult};

pub struct SttEngine {
    pub parakeet: Mutex<Parakeet>,
}

impl SttEngine {
    /// # Errors
    /// - When HOME environment variable is unset
    /// - When `Parakeet` fails to load the model
    pub fn new() -> Result<Self> {
        let home = env::var("HOME")?;
        let model_dir = format!(
            "{home}/.cache/huggingface/hub/models--onnx-community--parakeet-ctc-0.6b-ONNX/snapshots/7df2cab7aed886b8b7f80d68a8214007e4847601/onnx"
        );

        let parakeet = Parakeet::from_pretrained(model_dir, None)?;
        let parakeet = Mutex::new(parakeet);

        Ok(Self { parakeet })
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
