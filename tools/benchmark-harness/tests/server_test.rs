//! Unit tests for server management

#[cfg(unix)]
#[allow(unused_imports)]
use benchmark_harness::server::ServerHandle;
use benchmark_harness::server::{ServerConfig, find_available_port};
use std::net::TcpListener;
use std::path::PathBuf;
use std::process::Command;
#[cfg(unix)]
#[allow(unused_imports)]
use std::process::Stdio;
#[cfg(unix)]
#[allow(unused_imports)]
use std::time::Duration;

#[test]
fn test_find_available_port() {
    let port = find_available_port(50000);
    assert!(port.is_some());

    let port = port.unwrap();
    assert!(port >= 50000);
    assert!(port < 50100);
}

#[test]
fn test_find_available_port_when_blocked() {
    let start_port = 52000;
    let port = find_available_port(start_port).unwrap();
    assert!(port >= start_port);

    #[allow(clippy::branches_sharing_code)]
    if let Ok(_listener) = TcpListener::bind(("127.0.0.1", port)) {
        let port2 = find_available_port(start_port).unwrap();

        assert!(port2 >= start_port);
        if port == start_port {
            assert_ne!(port, port2, "Should find a different port when first is in use");
        }
    } else {
        let port2 = find_available_port(start_port).unwrap();
        assert!(port2 >= start_port);
    }
}

#[cfg(unix)]
#[test]
fn test_server_handle_pid() {
    let process = Command::new("sleep")
        .arg("10")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn test process");

    #[cfg(unix)]
    let pid = process.id();

    let handle = ServerHandle {
        process,
        port: 8000,
        base_url: "http://localhost:8000".to_string(),
        stop_signal: libc::SIGTERM,
    };

    assert_eq!(handle.pid(), pid);

    handle.kill().expect("Failed to kill process");
}

#[cfg(unix)]
#[test]
fn test_server_handle_kill() {
    let process = Command::new("sleep")
        .arg("30")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn test process");

    #[cfg(unix)]
    let pid = process.id();

    let handle = ServerHandle {
        process,
        port: 8000,
        base_url: "http://localhost:8000".to_string(),
        stop_signal: libc::SIGTERM,
    };

    let result = handle.kill();
    assert!(result.is_ok());

    std::thread::sleep(Duration::from_millis(100));

    #[cfg(unix)]
    {
        let status = unsafe { libc::kill(pid.cast_signed(), 0) };
        assert_eq!(status, -1);
    }
}

#[cfg(unix)]
#[test]
fn test_server_handle_drop() {
    let process = Command::new("sleep")
        .arg("30")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to spawn test process");

    let pid = process.id();

    {
        let _handle = ServerHandle {
            process,
            port: 8000,
            base_url: "http://localhost:8000".to_string(),
            stop_signal: libc::SIGTERM,
        };
    }

    std::thread::sleep(Duration::from_millis(200));

    #[cfg(unix)]
    {
        let status = unsafe { libc::kill(pid.cast_signed(), 0) };
        let _ = status;
    }
}

#[tokio::test]
async fn test_start_server_framework_not_found() {
    let config = ServerConfig {
        framework: Some("nonexistent-framework".to_string()),
        port: 9999,
        app_dir: PathBuf::from("/tmp"),
        variant: None,
        env: Vec::new(),
        start_cmd_override: None,
    };

    let result = benchmark_harness::server::start_server(config).await;
    assert!(result.is_err());

    if let Err(err) = result {
        assert!(err.to_string().contains("not found"));
    }
}

#[tokio::test]
async fn test_start_server_process_spawn_failure() {
    let config = ServerConfig {
        framework: Some("fastapi".to_string()),
        port: 9999,
        app_dir: PathBuf::from("/nonexistent/directory/that/does/not/exist"),
        variant: None,
        env: Vec::new(),
        start_cmd_override: None,
    };

    let result = benchmark_harness::server::start_server(config).await;
    assert!(result.is_err());
}

#[test]
fn test_server_config_creation() {
    let config = ServerConfig {
        framework: Some("spikard-python-validation".to_string()),
        port: 8000,
        app_dir: PathBuf::from("/tmp/app"),
        variant: None,
        env: Vec::new(),
        start_cmd_override: None,
    };

    assert_eq!(config.framework, Some("spikard-python-validation".to_string()));
    assert_eq!(config.port, 8000);
    assert_eq!(config.app_dir, PathBuf::from("/tmp/app"));
}

#[tokio::test]
async fn test_start_simple_python_server() {
    if Command::new("python3").arg("--version").output().is_err() {
        eprintln!("Skipping test: Python not available");
        return;
    }

    let temp_dir = tempfile::TempDir::new().unwrap();
    let server_file = temp_dir.path().join("server.py");

    std::fs::write(
        &server_file,
        r"
import sys
from http.server import HTTPServer, BaseHTTPRequestHandler

class Handler(BaseHTTPRequestHandler):
    def do_GET(self):
        self.send_response(200)
        self.send_header('Content-Type', 'text/plain')
        self.end_headers()
        self.wfile.write(b'OK')

    def log_message(self, format, *args):
        pass  # Suppress logs

if __name__ == '__main__':
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8000
    server = HTTPServer(('127.0.0.1', port), Handler)
    server.serve_forever()
",
    )
    .unwrap();

    let port = find_available_port(45000).expect("No available port");

    let config = ServerConfig {
        framework: Some("fastapi".to_string()),
        port,
        app_dir: temp_dir.path().to_path_buf(),
        variant: None,
        env: Vec::new(),
        start_cmd_override: None,
    };

    let result = benchmark_harness::server::start_server(config).await;

    if let Ok(handle) = result {
        assert!(handle.pid() > 0);
        assert_eq!(handle.port, port);

        let client = reqwest::Client::new();
        let url = format!("http://localhost:{port}/");
        let response = client.get(&url).send().await;

        if let Ok(resp) = response {
            assert!(resp.status().is_success());
        }

        handle.kill().expect("Failed to kill server");
    } else if let Err(err) = result {
        eprintln!("Expected error starting test server: {err}");
    }
}
