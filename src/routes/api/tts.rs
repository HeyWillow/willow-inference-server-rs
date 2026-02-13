#[cfg(feature = "tts")]
use std::sync::Arc;

use axum::{
    extract::{Query, State as AxumState},
    http::{StatusCode, header},
    response::IntoResponse,
};
use serde::Deserialize;
use tokio::task::spawn_blocking;

use crate::{state::State, util::audio::encode_wav};

#[derive(Debug, Deserialize)]
pub struct WisTtsRequestParameters {
    text: String,
}

pub async fn get(
    AxumState(state): AxumState<Arc<State>>,
    Query(parameters): Query<WisTtsRequestParameters>,
) -> impl IntoResponse {
    tracing::debug!("TTS request parameters: {parameters:?}");

    if parameters.text.is_empty() {
        return (
            StatusCode::BAD_REQUEST,
            String::from("text parameter missing"),
        )
            .into_response();
    }

    let Some(tts_engine) = state.tts_engine.clone() else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to get TTS engine"),
        )
            .into_response();
    };

    let tts_audio = spawn_blocking(move || tts_engine.synthesize(&parameters.text, 0, 1.0)).await;

    let Ok(Ok(tts_audio)) = tts_audio else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to get audio from TTS engine"),
        )
            .into_response();
    };

    let Ok(audio_bytes) = encode_wav(&tts_audio.samples, tts_audio.sample_rate) else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            String::from("failed to encode audio in WAV"),
        )
            .into_response();
    };

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
