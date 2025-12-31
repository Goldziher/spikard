use serde_json::json;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;

/// Test server instance that manages the subprocess lifecycle
struct TestServer {
    child: Arc<Mutex<Child>>,
    base_url: String,
}

impl TestServer {
    /// Spawn the test app binary and wait for it to be ready
    async fn start() -> Self {
        // Build the binary first to ensure it's up to date
        let build_status = Command::new("cargo")
            .args(["build", "--bin", "spikard-test-app-rust"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .status()
            .expect("Failed to build test binary");

        assert!(
            build_status.success(),
            "Failed to build test app binary"
        );

        // Spawn the binary with stdout/stderr captured
        let mut child = Command::new("cargo")
            .args(["run", "--bin", "spikard-test-app-rust"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start test app binary");

        // Read the server address from stdout
        let stdout = child.stdout.take().expect("Failed to capture stdout");
        let mut reader = BufReader::new(stdout);

        // Find the line that starts with "Server starting on"
        let mut base_url = String::new();
        for _ in 0..10 {
            let mut line = String::new();
            if reader.read_line(&mut line).is_err() {
                break;
            }

            if let Some(addr) = line.trim().strip_prefix("Server starting on ") {
                base_url = format!("http://{}", addr);
                break;
            }
        }

        if base_url.is_empty() {
            panic!("Failed to find server address in output");
        }

        // Wait for server to be ready with retry logic
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", base_url);

        for attempt in 0..20 {
            tokio::time::sleep(Duration::from_millis(100)).await;

            if let Ok(response) = client.get(&health_url).send().await {
                if response.status().is_success() {
                    // Server is ready
                    return Self {
                        child: Arc::new(Mutex::new(child)),
                        base_url,
                    };
                }
            }

            if attempt == 19 {
                panic!("Server failed to become ready after 2 seconds");
            }
        }

        unreachable!()
    }

    /// Get the base URL for making requests
    fn url(&self) -> &str {
        &self.base_url
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        // Kill the server process on cleanup
        if let Ok(mut child) = self.child.lock() {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

#[tokio::test]
async fn test_uses_correct_package_version() {
    let cargo_toml = include_str!("../Cargo.toml");
    assert!(
        cargo_toml.contains(r#"spikard = "0.7.3""#),
        "Expected spikard version 0.7.0 in Cargo.toml"
    );
}

#[tokio::test]
async fn test_responds_to_health_check() {
    let server = TestServer::start().await;
    let url = format!("{}/health", server.url());

    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch health endpoint");

    assert_eq!(response.status(), 200);

    let data: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(data, json!({ "status": "ok" }));
}

#[tokio::test]
async fn test_handles_query_parameters() {
    let server = TestServer::start().await;
    let url = format!("{}/query?name=Alice&age=30", server.url());

    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch query endpoint");

    assert_eq!(response.status(), 200);

    let data: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(data, json!({ "name": "Alice", "age": 30 }));
}

#[tokio::test]
async fn test_echoes_json_requests() {
    let server = TestServer::start().await;
    let url = format!("{}/echo", server.url());

    let payload = json!({ "message": "Hello from Rust!" });
    let client = reqwest::Client::new();

    let response = client
        .post(&url)
        .json(&payload)
        .send()
        .await
        .expect("Failed to post to echo endpoint");

    assert_eq!(response.status(), 200);

    let data: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(data["received"], payload);
    assert_eq!(data["method"], "POST");
}

#[tokio::test]
async fn test_extracts_path_parameters() {
    let server = TestServer::start().await;
    let url = format!("{}/users/42", server.url());

    let response = reqwest::get(&url)
        .await
        .expect("Failed to fetch user endpoint");

    assert_eq!(response.status(), 200);

    let data: serde_json::Value = response.json().await.expect("Failed to parse JSON");
    assert_eq!(data["userId"], "42");
    assert_eq!(data["type"], "String");
}
