mod tests {
    use std::{fs::File, io::Read};

    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;
    #[cfg(feature = "stt")]
    use wis_rs::{inference::stt::SttEngine, routes::api::willow::WisSpeechToTextResponse};
    use wis_rs::{router::router, state::State};

    #[cfg(feature = "stt")]
    #[ignore = "test requires STT model"]
    #[tokio::test]
    async fn test_router_api_willow_post() {
        const TESTDATA_FILE: &str = "tests/assets/whats_the_time.pcm";
        const TESTDATA_RESPONSE: &str = "what's the time";

        let mut test_data: Vec<u8> = Vec::new();

        File::open(TESTDATA_FILE)
            .unwrap_or_else(|e| panic!("failed to open testdata file '{TESTDATA_FILE}': {e}"))
            .read_to_end(&mut test_data)
            .unwrap_or_else(|e| panic!("failed to read testdata file '{TESTDATA_FILE}': {e}"));

        let stt_model_dir = std::path::PathBuf::from("./models/stt");
        let stt_engine = SttEngine::new(stt_model_dir).expect("failed to create STT engine");
        let state = State::new().with_stt_engine(stt_engine);
        let router: axum::Router = router(state);

        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .header("x-audio-bits", 16)
                    .header("x-audio-channel", 1)
                    .header("x-audio-codec", "PCM")
                    .header("x-audio-sample-rate", 16000)
                    .uri("/api/willow")
                    .body(Body::from(test_data))
                    .unwrap(),
            )
            .await
            .unwrap();

        println!("{response:?}");

        let status = response.status();
        let content_type = response
            .headers()
            .get(http::header::CONTENT_TYPE)
            .expect("Content-Type header missing")
            .clone();
        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();

        println!("body: {body:?}");

        let data: WisSpeechToTextResponse =
            serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

        assert_eq!(content_type, "application/json");
        assert_eq!(data.text, TESTDATA_RESPONSE);
        assert_eq!(status, StatusCode::OK);
    }

    #[tokio::test]
    async fn test_router_health_get() {
        let state = State::new();
        let router = router(state);
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}
