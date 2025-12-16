//! Load generator integration (oha, bombardier)

use crate::error::{Error, Result};
use crate::fixture::Fixture;
use crate::types::{OhaOutput, ThroughputMetrics};
use std::process::Command;
use std::time::Duration;

async fn preflight_fixture_request(url: &str, fixture: &Fixture) -> Result<()> {
    let method = reqwest::Method::from_bytes(fixture.request.method.as_bytes()).map_err(|e| {
        Error::InvalidInput(format!(
            "Invalid HTTP method '{}' for fixture {}: {}",
            fixture.request.method, fixture.name, e
        ))
    })?;

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to build preflight HTTP client: {}", e)))?;

    let mut request = client.request(method, url);

    for (key, value) in &fixture.request.headers {
        request = request.header(key, value);
    }

    if let Some(body_raw) = &fixture.request.body_raw {
        request = request.body(body_raw.clone());
    } else if let Some(body) = &fixture.request.body {
        let content_type = fixture.request.headers.iter().find_map(|(key, value)| {
            key.eq_ignore_ascii_case("content-type")
                .then(|| value.to_ascii_lowercase())
        });

        let body_is_string = matches!(body, serde_json::Value::String(_));
        let treat_string_as_raw = body_is_string
            && content_type
                .as_deref()
                .is_some_and(|value| !value.contains("application/json"));

        if treat_string_as_raw {
            if let serde_json::Value::String(text) = body {
                request = request.body(text.clone());
            }
        } else {
            let json_body = serde_json::to_vec(body).map_err(|e| {
                Error::LoadGeneratorFailed(format!(
                    "Failed to serialize JSON body for fixture {}: {}",
                    fixture.name, e
                ))
            })?;
            request = request.body(json_body);
            if content_type.is_none() {
                request = request.header("Content-Type", "application/json");
            }
        }
    }

    let response = request
        .send()
        .await
        .map_err(|e| Error::LoadGeneratorFailed(format!("Preflight request failed for {}: {}", fixture.name, e)))?;

    let expected = fixture.expected_response.status_code;
    let actual = response.status().as_u16();
    if actual == expected {
        return Ok(());
    }

    let body = response.text().await.unwrap_or_default();
    let body_snippet = body.lines().take(20).collect::<Vec<_>>().join("\n").trim().to_string();

    Err(Error::LoadGeneratorFailed(format!(
        "Preflight check failed for fixture {} ({} {}): expected HTTP {}, got HTTP {}{}",
        fixture.name,
        fixture.request.method,
        fixture.request.path,
        expected,
        actual,
        if body_snippet.is_empty() {
            String::new()
        } else {
            format!("\n\nResponse body (first lines):\n{}", body_snippet)
        }
    )))
}

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
    which::which("oha").map_err(|_| Error::LoadGeneratorNotFound("oha".to_string()))?;

    let url = if let Some(fixture) = &config.fixture {
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

    if let Some(fixture) = &config.fixture {
        preflight_fixture_request(&url, fixture).await?;
    }

    let mut cmd = Command::new("oha");
    cmd.arg("--output-format")
        .arg("json")
        .arg("-z")
        .arg(format!("{}s", config.duration_secs))
        .arg("-c")
        .arg(config.concurrency.to_string());

    if let Some(fixture) = &config.fixture {
        cmd.arg("-m").arg(&fixture.request.method);

        let content_type = fixture.request.headers.iter().find_map(|(key, value)| {
            key.eq_ignore_ascii_case("content-type")
                .then(|| value.to_ascii_lowercase())
        });

        for (key, value) in &fixture.request.headers {
            cmd.arg("-H").arg(format!("{}: {}", key, value));
        }

        if let Some(body_raw) = &fixture.request.body_raw {
            cmd.arg("-d").arg(body_raw);
        } else if let Some(body) = &fixture.request.body {
            let body_is_string = matches!(body, serde_json::Value::String(_));
            let treat_string_as_raw = body_is_string
                && content_type
                    .as_deref()
                    .is_some_and(|value| !value.contains("application/json"));

            if treat_string_as_raw {
                if let serde_json::Value::String(text) = body {
                    cmd.arg("-d").arg(text);
                }
            } else {
                let body_json = serde_json::to_string(body)?;
                cmd.arg("-d").arg(body_json);
                if content_type.is_none() {
                    cmd.arg("-H").arg("Content-Type: application/json");
                }
            }
        }
    }

    cmd.arg(&url);

    let output = cmd
        .output()
        .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to run oha: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(Error::LoadGeneratorFailed(format!("oha exited with error: {}", stderr)));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let oha_output: OhaOutput = serde_json::from_str(&stdout)
        .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to parse oha output: {}", e)))?;

    let total_duration = oha_output.summary.total.unwrap_or(0.0);
    let requests_per_sec = oha_output.summary.requests_per_sec.unwrap_or(0.0);
    let success_rate = oha_output.summary.success_rate.unwrap_or(1.0);
    let size_per_sec = oha_output.summary.size_per_sec.unwrap_or(0.0);

    let total_requests = (requests_per_sec * total_duration) as u64;
    let failed = total_requests - (total_requests as f64 * success_rate) as u64;

    let throughput = ThroughputMetrics {
        total_requests,
        requests_per_sec,
        bytes_per_sec: size_per_sec,
        failed_requests: failed,
        success_rate,
    };

    Ok((oha_output, throughput))
}

/// Run bombardier load generator (fallback)
async fn run_bombardier(_config: LoadTestConfig) -> Result<(OhaOutput, ThroughputMetrics)> {
    which::which("bombardier").map_err(|_| Error::LoadGeneratorNotFound("bombardier".to_string()))?;

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
