//! Load generator integration (oha, bombardier)

use crate::error::{Error, Result};
use crate::fixture::Fixture;
use crate::types::{OhaOutput, ThroughputMetrics};
use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;
use uuid::Uuid;

fn multipart_spec_from_fixture(fixture: &Fixture) -> Option<(usize, usize)> {
    let body = fixture.request.body.as_ref()?;
    let obj = body.as_object()?;
    let files_received = obj.get("files_received")?.as_u64()? as usize;
    let total_bytes = obj.get("total_bytes")?.as_u64()? as usize;
    Some((files_received.max(1), total_bytes))
}

/// Build a multipart form-data body with boundary parameter
fn build_multipart_body(fixture: &Fixture) -> Result<(String, String)> {
    let boundary = Uuid::new_v4().to_string();

    let mut body = String::new();

    for (key, value) in &fixture.request.data {
        body.push_str(&format!("--{}\r\n", boundary));
        body.push_str(&format!("Content-Disposition: form-data; name=\"{}\"\r\n\r\n", key));
        body.push_str(value);
        body.push_str("\r\n");
    }

    for file in &fixture.request.files {
        body.push_str(&format!("--{}\r\n", boundary));
        body.push_str(&format!(
            "Content-Disposition: form-data; name=\"{}\"; filename=\"{}\"\r\n",
            file.field_name, file.filename
        ));

        let content_type = if file.content_type.is_empty() {
            "application/octet-stream".to_string()
        } else {
            file.content_type.clone()
        };
        body.push_str(&format!("Content-Type: {}\r\n\r\n", content_type));
        body.push_str(&file.content);
        body.push_str("\r\n");
    }

    // Some workload definitions represent multipart payloads via a synthetic JSON body
    // (files_received/total_bytes) rather than concrete `files`/`data`. In that case, synthesize
    // file parts so the request includes a boundary and matches the expected size/count.
    if fixture.request.files.is_empty() && fixture.request.data.is_empty() {
        if let Some((file_count, total_bytes)) = multipart_spec_from_fixture(fixture) {
            let per_file = (total_bytes / file_count).max(1);
            let mut remaining = total_bytes;

            for idx in 0..file_count {
                let size = if idx + 1 == file_count {
                    remaining.max(1)
                } else {
                    remaining = remaining.saturating_sub(per_file);
                    per_file
                };

                body.push_str(&format!("--{}\r\n", boundary));
                body.push_str(&format!(
                    "Content-Disposition: form-data; name=\"file\"; filename=\"upload-{}.bin\"\r\n",
                    idx + 1
                ));
                body.push_str("Content-Type: application/octet-stream\r\n\r\n");
                body.push_str(&"x".repeat(size));
                body.push_str("\r\n");
            }
        } else {
            let synthetic_content = fixture.request.body_raw.as_deref().unwrap_or("x").to_string();

            body.push_str(&format!("--{}\r\n", boundary));
            body.push_str("Content-Disposition: form-data; name=\"file\"; filename=\"upload.bin\"\r\n");
            body.push_str("Content-Type: application/octet-stream\r\n\r\n");
            body.push_str(&synthetic_content);
            body.push_str("\r\n");
        }
    }

    body.push_str(&format!("--{}--\r\n", boundary));

    let content_type = format!("multipart/form-data; boundary={}", boundary);

    Ok((body, content_type))
}

fn fixture_declares_multipart(fixture: &Fixture) -> bool {
    fixture.request.headers.iter().any(|(key, value)| {
        key.eq_ignore_ascii_case("content-type") && value.to_ascii_lowercase().contains("multipart/form-data")
    })
}

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

    let is_multipart =
        fixture_declares_multipart(fixture) || !fixture.request.files.is_empty() || !fixture.request.data.is_empty();
    for (key, value) in &fixture.request.headers {
        // Multipart requests require a boundary parameter, so we always override the fixture
        // content-type with a boundary-aware header built from the generated body.
        if is_multipart && key.eq_ignore_ascii_case("content-type") {
            continue;
        }
        request = request.header(key, value);
    }

    if is_multipart {
        let (body, content_type) = build_multipart_body(fixture)?;
        request = request.header("Content-Type", &content_type);
        request = request.body(body);
    } else if let Some(body_raw) = &fixture.request.body_raw {
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

    let mut temp_paths_to_cleanup: Vec<PathBuf> = Vec::new();

    if let Some(fixture) = &config.fixture {
        cmd.arg("-m").arg(&fixture.request.method);

        let is_multipart = fixture_declares_multipart(fixture)
            || !fixture.request.files.is_empty()
            || !fixture.request.data.is_empty();
        let content_type = fixture.request.headers.iter().find_map(|(key, value)| {
            key.eq_ignore_ascii_case("content-type")
                .then(|| value.to_ascii_lowercase())
        });

        for (key, value) in &fixture.request.headers {
            if is_multipart && key.eq_ignore_ascii_case("content-type") {
                continue;
            }
            cmd.arg("-H").arg(format!("{}: {}", key, value));
        }

        if is_multipart {
            // Use oha's curl-compatible multipart mode (`-F`) instead of `-d`, because multipart
            // bodies must start with `--<boundary>` and clap rejects values that look like flags.
            for (key, value) in &fixture.request.data {
                cmd.arg("-F").arg(format!("{}={}", key, value));
            }

            let temp_dir = std::env::temp_dir().join("spikard-bench-multipart");
            std::fs::create_dir_all(&temp_dir)
                .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to create temp dir: {}", e)))?;

            let mut temp_paths: Vec<PathBuf> = Vec::new();
            let mut add_temp_file = |field_name: &str, filename: &str, content: &[u8]| -> Result<()> {
                let file_path = temp_dir.join(format!("{}-{}-{}", field_name, filename, Uuid::new_v4()));
                std::fs::write(&file_path, content)
                    .map_err(|e| Error::LoadGeneratorFailed(format!("Failed to write temp file: {}", e)))?;
                cmd.arg("-F").arg(format!("{}=@{}", field_name, file_path.display()));
                temp_paths.push(file_path);
                Ok(())
            };

            if fixture.request.files.is_empty() {
                if let Some((file_count, total_bytes)) = multipart_spec_from_fixture(fixture) {
                    let per_file = (total_bytes / file_count).max(1);
                    let mut remaining = total_bytes;
                    for idx in 0..file_count {
                        let size = if idx + 1 == file_count {
                            remaining.max(1)
                        } else {
                            remaining = remaining.saturating_sub(per_file);
                            per_file
                        };
                        add_temp_file("file", &format!("upload-{}.bin", idx + 1), &vec![b'x'; size])?;
                    }
                } else {
                    add_temp_file(
                        "file",
                        "upload.bin",
                        fixture.request.body_raw.as_deref().unwrap_or("x").as_bytes(),
                    )?;
                }
            } else {
                for file in &fixture.request.files {
                    let field_name = if file.field_name.is_empty() {
                        "file"
                    } else {
                        &file.field_name
                    };
                    let filename = if file.filename.is_empty() {
                        "upload.bin"
                    } else {
                        &file.filename
                    };
                    add_temp_file(field_name, filename, file.content.as_bytes())?;
                }
            }

            temp_paths_to_cleanup.extend(temp_paths);
        } else if let Some(body_raw) = &fixture.request.body_raw {
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

    // Best-effort cleanup (multipart temp files).
    for path in temp_paths_to_cleanup {
        let _ = std::fs::remove_file(path);
    }

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
