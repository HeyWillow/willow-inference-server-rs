use axum::{Router, routing::get};
use tokio::net::TcpListener;

pub fn router() -> axum::Router {
    Router::new().route("/health", get(crate::routes::health::check))
}

/// # Errors
/// if `TcpListener` cannot be bound
/// if `axum::serve` returns an error
pub async fn serve() -> anyhow::Result<()> {
    let router = router();

    let address = "[::]:19001";

    let listener = TcpListener::bind(address).await?;

    axum::serve(listener, router).await?;
    Ok(())
}
