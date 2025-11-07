//! Error types for benchmark harness

use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Invalid fixture at {path}: {reason}")]
    InvalidFixture { path: PathBuf, reason: String },

    #[error("Server failed to start: {0}")]
    ServerStartFailed(String),

    #[error("Server not healthy after {0} attempts")]
    ServerNotHealthy(u32),

    #[error("Load generator not found: {0}")]
    LoadGeneratorNotFound(String),

    #[error("Load generator failed: {0}")]
    LoadGeneratorFailed(String),

    #[error("Framework not found: {0}")]
    FrameworkNotFound(String),

    #[error("Benchmark failed: {0}")]
    BenchmarkFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
