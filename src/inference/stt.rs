use std::{path::PathBuf, sync::Mutex};

use anyhow::Result;
use parakeet_rs::{Parakeet, TimestampMode, Transcriber, TranscriptionResult};

pub struct SttEngine {
    pub parakeet: Mutex<Parakeet>,
}

impl SttEngine {
    /// # Errors
    /// - When HOME environment variable is unset
    /// - When `Parakeet` fails to load the model
    pub fn new(model_dir: PathBuf) -> Result<Self> {
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
