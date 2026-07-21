#[cfg(feature = "tts")]
use std::thread;

use anyhow::{Result, anyhow};
use sherpa_rs::tts::{TtsAudio, VitsTts, VitsTtsConfig};
use tokio::{
    sync::{mpsc, oneshot},
    time::Instant,
};

use crate::inference::InferenceResult;

#[derive(Clone)]
pub struct TtsEngine {
    jobs: mpsc::Sender<TtsJob>,
}

struct TtsJob {
    text: String,
    sid: i32,
    speed: f32,
    response: oneshot::Sender<Result<(TtsAudio, f64)>>,
}

impl TtsEngine {
    /// # Errors
    /// - when the TTS worker thread cannot be spawned
    pub fn new() -> Result<Self> {
        let config = VitsTtsConfig {
            model: "./models/tts/vits-piper-en_US-amy-medium/en_US-amy-medium.onnx".to_string(),
            tokens: "./models/tts/vits-piper-en_US-amy-medium/tokens.txt".to_string(),
            data_dir: "./models/tts/vits-piper-en_US-amy-medium/espeak-ng-data".to_string(),
            length_scale: 1.0,
            silence_scale: 1.1,
            ..Default::default()
        };

        let mut tts = VitsTts::new(config);
        let (jobs, mut receiver) = mpsc::channel::<TtsJob>(1);

        thread::Builder::new()
            .name(String::from("wis-tts"))
            .stack_size(8 * 1024 * 1024)
            .spawn(move || {
                while let Some(job) = receiver.blocking_recv() {
                    let start = Instant::now();
                    let speech = tts
                        .create(&job.text, job.sid, job.speed)
                        .map(|speech| (speech, start.elapsed().as_secs_f64()))
                        .map_err(|e| anyhow!("{e:#?}"));
                    let _ = job.response.send(speech);
                }
            })?;

        Ok(Self { jobs })
    }

    /// # Errors
    /// - when the inference worker has stopped
    pub async fn synthesize(
        &self,
        text: &str,
        sid: i32,
        speed: f32,
    ) -> Result<InferenceResult<TtsAudio>> {
        let (response, receiver) = oneshot::channel();
        self.jobs
            .send(TtsJob {
                text: text.to_owned(),
                sid,
                speed,
                response,
            })
            .await
            .map_err(|_| anyhow!("TTS worker has stopped"))?;

        let (speech, time) = receiver
            .await
            .map_err(|_| anyhow!("TTS worker has stopped"))??;
        let time_ms = time * 1000.0;
        let duration = (speech.samples.len() as u64 * 1000) / u64::from(speech.sample_rate);
        #[allow(clippy::cast_precision_loss)]
        let speedup = if time > 0.0 {
            duration as f64 / time_ms
        } else {
            0.0
        };

        let result = InferenceResult {
            duration,
            output: speech,
            speedup,
            time,
        };

        Ok(result)
    }
}
