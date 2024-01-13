use anyhow::Result;
use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    // Logging
    tracing_subscriber::fmt::init();

    // Setup router
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Started server at http://{}", listener.local_addr()?);
    Ok(axum::serve(listener, app).await?)
}
