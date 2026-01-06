use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};
use tokio::net::TcpListener;

use crate::state::State;

pub fn router(state: State) -> axum::Router {
    Router::new()
        .route("/health", get(crate::routes::health::check))
        .route("/api/willow", post(crate::routes::api::willow::post))
        .with_state(Arc::new(state))
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
