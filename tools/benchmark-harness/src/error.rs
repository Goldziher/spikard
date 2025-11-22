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

    #[error(
        "Framework not found: {0}\n\nEnsure the framework app directory exists in one of:\n  - benchmarks/{0}\n  - e2e/{{language}}\n  - examples/{0}\n\nFor Spikard bindings, check e2e/python, e2e/node, e2e/ruby"
    )]
    FrameworkNotFound(String),

    #[error("Benchmark failed: {0}")]
    BenchmarkFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error(
        "Workload suite '{0}' not found\n\nAvailable suites:\n  - all (all workloads)\n  - baseline (basic HTTP requests)\n  - json-bodies (JSON request/response)\n  - path-params (path parameters)\n  - query-params (query string parameters)\n  - forms (form submissions)\n  - streaming (WebSocket/SSE)"
    )]
    WorkloadNotFound(String),

    #[error(
        "Framework '{framework}' execution failed: {source}\n\nTroubleshooting:\n  1. Check that dependencies are installed\n  2. Verify the server starts manually\n  3. Check port availability\n  4. Review server logs for errors"
    )]
    FrameworkExecutionFailed {
        framework: String,
        #[source]
        source: Box<Error>,
    },
}
