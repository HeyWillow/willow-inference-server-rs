use std::{path::PathBuf, sync::Mutex};

use anyhow::Result;
use parakeet_rs::{Parakeet, TimestampMode, Transcriber, TranscriptionResult};
use tokio::time::Instant;

use crate::inference::InferenceResult;

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
    ) -> anyhow::Result<InferenceResult<TranscriptionResult>> {
        let mut parakeet = self
            .parakeet
            .lock()
            .map_err(|_| anyhow::anyhow!("SttEngine mutex is poisoned"))?;

        let duration = (samples.len() as u64 * 1000) / u64::from(sample_rate);
        let start = Instant::now();

        let output = parakeet.transcribe_samples(samples, sample_rate, channels, mode)?;

        let time = start.elapsed().as_secs_f64();
        let time_ms = time * 1000.0;
        #[allow(clippy::cast_precision_loss)]
        let speedup = if time_ms > 0.0 {
            (duration as f64) / time_ms
        } else {
            0.0
        };

        let result = InferenceResult {
            duration,
            output,
            speedup,
            time,
        };

        Ok(result)
    }
}
