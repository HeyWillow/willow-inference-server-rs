use std::sync::Arc;

use axum::{
    extract::{Query, State as AxumState},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::time::Instant;

use crate::{state::State, util::audio::encode_wav};

#[derive(Debug, Deserialize)]
pub struct WisTextToSpeechRequestParameters {
    voice: Option<String>,
    text: String,
}

/// # Errors
/// - when URL param text is empty
/// - when we fail to load the STT engine from the `Axum` state
pub async fn get(
    AxumState(state): AxumState<Arc<State>>,
    Query(params): Query<WisTextToSpeechRequestParameters>,
) -> impl IntoResponse {
    if params.text.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            String::from("text parameter is required"),
        )
            .into_response();
    }

    tracing::info!("TTS request parameters: {params:?}");

    let voice = params.voice.unwrap_or(String::from("af_sarah"));

    let Some(tts_engine) = state.tts_engine.as_ref() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to load STT engine"),
        )
            .into_response();
    };

    let start_synth = Instant::now();

    let Ok(samples) = tts_engine.synthesize(&params.text, &voice) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to synthesize text"),
        )
            .into_response();
    };

    let synth_time = start_synth.elapsed().as_secs_f64();
    let synth_time_ms = synth_time * 1000.0;

    let start_encode = Instant::now();

    let Ok(audio_bytes) = encode_wav(&samples) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to encode to WAV"),
        )
            .into_response();
    };

    let encode_time = start_encode.elapsed().as_secs_f64();
    let encode_time_ms = encode_time * 1000.0;

    tracing::info!("synth time: {synth_time_ms}ms, encode time: {encode_time_ms}ms");

    (
        StatusCode::OK,
        [
            (header::CONTENT_TYPE, "audio/wav"),
            (
                header::CONTENT_DISPOSITION,
                "inline; filename=\"speech.wav\"",
            ),
        ],
        audio_bytes,
    )
        .into_response()
}
