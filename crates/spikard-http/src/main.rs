//! Spikard HTTP Server
//!
//! HTTP API server for spikard

use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Deserialize)]
struct ProcessRequest {
    input: String,
}

#[derive(Serialize)]
struct ProcessResponse {
    success: bool,
    message: String,
}

async fn health() -> &'static str {
    "OK"
}

async fn process(Json(payload): Json<ProcessRequest>) -> Json<ProcessResponse> {
    // TODO: Implement actual processing using spikard library
    match spikard::process() {
        Ok(_) => Json(ProcessResponse {
            success: true,
            message: format!("Processed: {}", payload.input),
        }),
        Err(e) => Json(ProcessResponse {
            success: false,
            message: format!("Error: {}", e),
        }),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "spikard_http=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Router::new()
        .route("/health", get(health))
        .route("/process", post(process))
        .layer(TraceLayer::new_for_http());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    tracing::info!("Listening on {}", listener.local_addr()?);

    axum::serve(listener, app).await?;

    Ok(())
}
