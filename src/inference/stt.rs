use std::{path::PathBuf, thread};

use anyhow::Result;
use parakeet_rs::{Parakeet, TimestampMode, Transcriber, TranscriptionResult};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

use crate::inference::InferenceResult;

pub struct SttEngine {
    jobs: mpsc::Sender<SttJob>,
}

struct SttJob {
    samples: Vec<f32>,
    sample_rate: u32,
    channels: u16,
    mode: Option<TimestampMode>,
    response: oneshot::Sender<anyhow::Result<(TranscriptionResult, f64)>>,
}

impl SttEngine {
    /// # Errors
    /// - When HOME environment variable is unset
    /// - When `Parakeet` fails to load the model
    pub fn new(model_dir: PathBuf) -> Result<Self> {
        let mut parakeet = Parakeet::from_pretrained(model_dir, None)?;
        let (jobs, mut receiver) = mpsc::channel::<SttJob>(1);

        thread::Builder::new().spawn(move || {
            while let Some(job) = receiver.blocking_recv() {
                let start = Instant::now();
                let output = parakeet
                    .transcribe_samples(job.samples, job.sample_rate, job.channels, job.mode)
                    .map(|output| (output, start.elapsed().as_secs_f64()))
                    .map_err(Into::into);
                let _ = job.response.send(output);
            }
        })?;

        Ok(Self { jobs })
    }

    /// # Errors
    /// When the inference worker has stopped
    /// When transcription fails
    pub async fn transcribe(
        &self,
        samples: Vec<f32>,
        sample_rate: u32,
        channels: u16,
        mode: Option<TimestampMode>,
    ) -> anyhow::Result<InferenceResult<TranscriptionResult>> {
        let duration = (samples.len() as u64 * 1000) / u64::from(sample_rate);
        let (response, receiver) = oneshot::channel();
        self.jobs
            .send(SttJob {
                samples,
                sample_rate,
                channels,
                mode,
                response,
            })
            .await
            .map_err(|_| anyhow::anyhow!("STT worker has stopped"))?;

        let (output, time) = receiver
            .await
            .map_err(|_| anyhow::anyhow!("STT worker has stopped"))??;
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
