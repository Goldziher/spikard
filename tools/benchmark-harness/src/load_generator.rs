//! Load generator integration (oha, bombardier)

use crate::error::{Error, Result};
use crate::fixture::Fixture;
use crate::types::{OhaOutput, ThroughputMetrics};
use std::process::Command;

/// Load generator type
#[derive(Debug, Clone, Copy)]
pub enum LoadGeneratorType {
    Oha,
    Bombardier,
}

/// Load test configuration
pub struct LoadTestConfig {
    pub base_url: String,
    pub duration_secs: u64,
    pub concurrency: usize,
    pub fixture: Option<Fixture>,
}

/// Run a load test using oha
pub async fn run_load_test(
    config: LoadTestConfig,
    generator: LoadGeneratorType,
) -> Result<(OhaOutput, ThroughputMetrics)> {
    match generator {
        LoadGeneratorType::Oha => run_oha(config).await,
        LoadGeneratorType::Bombardier => run_bombardier(config).await,
    }
}

/// Run oha load generator
async fn run_oha(config: LoadTestConfig) -> Result<(OhaOutput, ThroughputMetrics)> {
    // Check if oha is installed
    which::which("oha").map_err(|_| Error::LoadGeneratorNotFound("oha".to_string()))?;

    let url = if let Some(fixture) = &config.fixture {
        // Build URL with query params
        let mut url = format!("{}{}", config.base_url, fixture.request.path);
        if !fixture.request.query_params.is_empty() {
            url.push('?');
            let query: Vec<String> = fixture
                .request
                .query_params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            url.push_str(&query.join("&"));
        }
        url
    } else {
        config.base_url.clone()
    };

    let mut cmd = Command::new("oha");
    cmd.arg("--output-format")
        .arg("json")
        .arg("-z")
        .arg(format!("{}s", config.duration_secs))
        .arg("-c")
        .arg(config.concurrency.to_string());

    // Add method if specified
    if let Some(fixture) = &config.fixture {
        cmd.arg("-m").arg(&fixture.request.method);

        // Add headers
        for (key, value) in &fixture.request.headers {
            cmd.arg("-H").arg(format!("{}: {}", key, value));
        }

        // Add body if present
        if let Some(body) = &fixture.request.body {
            let body_json = serde_json::to_string(body)?;
            cmd.arg("-d").arg(body_json);
            cmd.arg("-H").arg("Content-Type: application/json");
        }
    }

    cmd.arg(&url);

    // Run oha
    let output = cmd
        .output()
        .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to run oha: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::LoadGeneratorFailed(format!("oha exited with error: {}", stderr)));
    }

    // Parse JSON output
    let stdout = String::from_utf8_lossy(&output.stdout);
    let oha_output: OhaOutput = serde_json::from_str(&stdout)
        .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to parse oha output: {}", e)))?;

    // Calculate throughput metrics
    // Note: oha's "total" field is duration in seconds, not request count
    let total_duration = oha_output.summary.total.unwrap_or(0.0);
    let total_requests = (oha_output.summary.requests_per_sec * total_duration) as u64;
    let failed = total_requests - (total_requests as f64 * oha_output.summary.success_rate) as u64;

    let throughput = ThroughputMetrics {
        total_requests,
        requests_per_sec: oha_output.summary.requests_per_sec,
        bytes_per_sec: oha_output.summary.size_per_sec as f64,
        failed_requests: failed,
        success_rate: oha_output.summary.success_rate,
    };

    Ok((oha_output, throughput))
}

/// Run bombardier load generator (fallback)
async fn run_bombardier(_config: LoadTestConfig) -> Result<(OhaOutput, ThroughputMetrics)> {
    // Check if bombardier is installed
    which::which("bombardier").map_err(|_| Error::LoadGeneratorNotFound("bombardier".to_string()))?;

    // For now, return error - we can implement bombardier parsing later
    Err(Error::LoadGeneratorFailed(
        "Bombardier support not yet implemented".to_string(),
    ))
}

/// Find the best available load generator
pub fn find_load_generator() -> Option<LoadGeneratorType> {
    if which::which("oha").is_ok() {
        Some(LoadGeneratorType::Oha)
    } else if which::which("bombardier").is_ok() {
        Some(LoadGeneratorType::Bombardier)
    } else {
        None
    }
}

/// Check if load generator is installed
pub fn check_load_generator(generator: LoadGeneratorType) -> bool {
    let binary = match generator {
        LoadGeneratorType::Oha => "oha",
        LoadGeneratorType::Bombardier => "bombardier",
    };

    which::which(binary).is_ok()
}
