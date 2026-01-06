use axum::{
    Json,
    body::{Body, Bytes},
    extract::Query,
    http::HeaderMap,
};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};

pub const RESPONSE_TEXT: &str = "what's the time";

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

pub async fn post(
    headers: HeaderMap,
    query: Query<WisSpeechToTextRequestParameters>,
    body: Body,
) -> Json<WisSpeechToTextResponse> {
    let mut stream = body.into_data_stream();

    let mut data: Vec<Bytes> = Vec::new();

    while let Some(chunk) = stream.next().await {
        if let Ok(b) = chunk {
            data.push(b);
        }
    }

    tracing::debug!(
        "Headers: {headers:#?}, Query: {query:#?}, Length: {}",
        data.len()
    );

    let response = WisSpeechToTextResponse {
        audio_duration: 1,
        infer_speedup: Some(1.0),
        infer_time: Some(1.0),
        language: String::from("en"),
        text: String::from(RESPONSE_TEXT),
    };

    Json(response)
}
