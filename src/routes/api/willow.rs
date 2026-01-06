use std::sync::Arc;

use axum::{
    Json,
    body::Body,
    extract::{Query, State as AxumState},
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
};
use futures_util::StreamExt;
use parakeet_rs::TimestampMode;
use serde::{Deserialize, Serialize};
use tokio::time::Instant;

use crate::{state::State, util::http::parse_header};

// 30s of 16KHz 16-bit mono audio
const MAX_AUDIO_BYTES: usize = 960_000;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct WisSpeechToTextRequestParameters {
    beam_size: Option<u32>,
    detect_language: Option<bool>,
    force_language: Option<bool>,
    model: Option<String>,
    save_audio: Option<bool>,
    stats: Option<bool>,
    translate: Option<bool>,
    voice_auth: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WisSpeechToTextResponse {
    audio_duration: u64,
    infer_speedup: Option<f64>,
    infer_time: Option<f64>,
    language: String,
    pub text: String,
}

/// # Errors
/// * when any of the following request headers are missing or invalid
/// - x-audio-bits
/// - x-audio-channel
/// - x-audio-codec
/// - x-sample_rate
/// * when transcription fails
pub async fn post(
    AxumState(state): AxumState<Arc<State>>,
    headers: HeaderMap,
    query: Query<WisSpeechToTextRequestParameters>,
    body: Body,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let _bits = parse_header::<u16>(&headers, "x-audio-bits")?;
    let channels: u16 = parse_header::<u16>(&headers, "x-audio-channel")?;
    let _codec = parse_header::<String>(&headers, "x-audio-codec")?;
    let sample_rate = parse_header::<u32>(&headers, "x-audio-sample-rate")?;

    let mut data = Vec::with_capacity(MAX_AUDIO_BYTES);

    let mut stream = body.into_data_stream();

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(bytes) => data.extend_from_slice(&bytes),
            Err(e) => return Err((StatusCode::BAD_REQUEST, format!("{e:?}"))),
        }
    }

    let n_samples = data.len() / 2;
    let mut samples = Vec::with_capacity(n_samples);

    samples.extend(
        data.chunks_exact(2)
            .map(|c| i16::from_le_bytes([c[0], c[1]])),
    );

    let samples: Vec<f32> = samples.iter().map(|&s| f32::from(s)).collect();
    let audio_duration = (samples.len() as u64 * 1000) / u64::from(sample_rate);

    tracing::debug!(
        "Headers: {headers:#?}, Query: {query:#?}, Length: {}",
        data.len()
    );

    let start = Instant::now();
    let transcript =
        state
            .stt_engine
            .transcribe(samples, sample_rate, channels, Some(TimestampMode::Words));

    let Ok(transcript) = transcript else {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("transcription failed"),
        ));
    };

    let infer_time = start.elapsed().as_secs_f64();
    let infer_time_ms = infer_time * 1000.0;
    #[allow(clippy::cast_precision_loss)]
    let infer_speedup = if infer_time_ms > 0.0 {
        (audio_duration as f64) / infer_time_ms
    } else {
        0.0
    };

    tracing::debug!("{transcript:?}");
    tracing::info!(
        "inference took {infer_time}s: {} - speedup: {infer_speedup}x",
        transcript.text
    );

    let response = WisSpeechToTextResponse {
        audio_duration,
        infer_speedup: Some(infer_speedup),
        infer_time: Some(infer_time),
        language: String::from("en"),
        text: transcript.text,
    };

    Ok(Json(response))
}
