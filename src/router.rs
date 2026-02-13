use std::sync::Arc;

#[cfg(feature = "stt")]
use axum::routing::post;
use axum::{Router, routing::get};
use tokio::net::TcpListener;

use crate::state::State;

pub fn router(state: State) -> axum::Router {
    #[allow(unused_mut)]
    let mut router = Router::new()
        .route("/about", get(crate::ui::handlers::about))
        .route("/health", get(crate::routes::health::check));

    #[cfg(feature = "stt")]
    {
        router = router.route("/api/willow", post(crate::routes::api::willow::post));
    }

    #[cfg(feature = "tts")]
    {
        router = router.route("/api/tts", get(crate::routes::api::tts::get));
    }

    router.with_state(Arc::new(state))
}

/// # Errors
/// if `TcpListener` cannot be bound
/// if `axum::serve` returns an error
pub async fn serve(state: State) -> anyhow::Result<()> {
    let router = router(state);

    let address = "[::]:19001";

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, router).await?;
    Ok(())
}
