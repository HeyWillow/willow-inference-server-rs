mod tests {
    use axum::{
        body::Body,
        http::{self, Request, StatusCode},
    };
    use tower::ServiceExt;
    use wis_rs::{
        router::router,
        routes::api::willow::{RESPONSE_TEXT, WisSpeechToTextResponse},
        state::State,
    };

    #[tokio::test]
    async fn test_router_api_willow_post() {
        let state = State::new();
        let router = router(state);
        let response = router
            .oneshot(
                Request::builder()
                    .method(http::Method::POST)
                    .header(http::header::CONTENT_TYPE, "application/json")
                    .uri("/api/willow")
                    .body(Body::empty())
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
        let data: WisSpeechToTextResponse =
            serde_json::from_slice(&body).expect("Failed to deserialize JSON response");

        assert_eq!(content_type, "application/json");
        assert_eq!(data.text, RESPONSE_TEXT);
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
