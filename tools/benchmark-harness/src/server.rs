//! Server management - start, stop, health check

use crate::error::{Error, Result};
use std::env;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// Find the workspace root by looking for Cargo.toml
#[allow(dead_code)]
fn find_workspace_root() -> Result<PathBuf> {
    let exe_path =
        env::current_exe().map_err(|e| Error::ServerStartFailed(format!("Failed to get executable path: {}", e)))?;

    let mut current = exe_path
        .parent()
        .ok_or_else(|| Error::ServerStartFailed("Failed to get executable parent directory".to_string()))?;

    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists()
            && let Ok(contents) = std::fs::read_to_string(&cargo_toml)
            && contents.contains("[workspace]")
        {
            return Ok(current.to_path_buf());
        }

        current = current.parent().ok_or_else(|| {
            Error::ServerStartFailed("Could not find workspace root (no Cargo.toml with [workspace])".to_string())
        })?;
    }
}

/// Server process handle
pub struct ServerHandle {
    pub process: Child,
    pub port: u16,
    pub base_url: String,
}

impl ServerHandle {
    /// Get the process ID
    pub fn pid(&self) -> u32 {
        self.process.id()
    }

    /// Kill the server gracefully
    pub fn kill(mut self) -> Result<()> {
        #[cfg(unix)]
        {
            unsafe {
                libc::kill(self.process.id() as i32, libc::SIGTERM);
            }

            for _ in 0..50 {
                match self.process.try_wait() {
                    Ok(Some(_)) => return Ok(()),
                    Ok(None) => std::thread::sleep(Duration::from_millis(100)),
                    Err(e) => return Err(e.into()),
                }
            }

            self.process.kill()?;
        }

        #[cfg(not(unix))]
        {
            self.process.kill()?;
        }

        Ok(())
    }
}

impl Drop for ServerHandle {
    fn drop(&mut self) {
        let _ = self.process.kill();
    }
}

/// Server configuration
pub struct ServerConfig {
    pub framework: String,
    pub port: u16,
    pub app_dir: PathBuf,
    /// Variant name (e.g., "sync", "async") - optional
    pub variant: Option<String>,
}

/// Start a server and wait for it to be ready
pub async fn start_server(config: ServerConfig) -> Result<ServerHandle> {
    let port = config.port;
    let base_url = format!("http://localhost:{}", port);

    let mut cmd = match config.framework.as_str() {
        "spikard-rust" => {
            let server_binary = config.app_dir.join("target/release/spikard-rust-bench");
            let mut cmd = Command::new(server_binary);
            cmd.arg(port.to_string());
            cmd
        }
        "spikard-python" => {
            let server_file = if let Some(ref variant) = config.variant
                && variant == "async"
            {
                "server_async.py"
            } else {
                "server.py"
            };
            let server_path = config.app_dir.join(server_file);
            let mut cmd = Command::new("uv");
            cmd.arg("run").arg("python").arg(server_path).arg(port.to_string()); // Positional arg, not --port
            // Set PYTHONPATH to packages/python relative to workspace root
            cmd.env("PYTHONPATH", "packages/python");
            cmd
        }
        "spikard-node" => {
            let server_path = config.app_dir.join("server.ts");
            let mut cmd = Command::new("ts-node");
            cmd.arg(server_path).arg("--port").arg(port.to_string());
            cmd
        }
        "spikard-ruby" => {
            let mut cmd = Command::new("ruby");
            cmd.arg("server.rb").arg(port.to_string());
            cmd
        }
        "fastapi" => {
            let mut cmd = Command::new("uv");
            cmd.arg("run").arg("python").arg("server.py").arg(port.to_string());
            cmd
        }
        "fastapi-granian" => {
            let mut cmd = Command::new("uv");
            cmd.arg("run").arg("python").arg("server.py").arg(port.to_string());
            // Set PYTHONPATH to packages/python relative to workspace root
            cmd.env("PYTHONPATH", "packages/python");
            cmd
        }
        "robyn" => {
            let mut cmd = Command::new(".venv/bin/python");
            cmd.arg("server.py").arg(port.to_string());
            cmd
        }
        "fastify" => {
            let mut cmd = Command::new("node");
            cmd.arg("server.js").arg(port.to_string());
            cmd
        }
        "spikard-rust-workloads" => {
            let server_binary = config.app_dir.join("target/release/spikard-rust-bench");
            let mut cmd = Command::new(server_binary);
            cmd.arg(port.to_string());
            cmd
        }
        "axum-baseline" => {
            let server_binary = config.app_dir.join("target/release/spikard-rust-bench");
            let mut cmd = Command::new(server_binary);
            cmd.arg(port.to_string());
            cmd
        }
        "spikard-python-workloads" => {
            let server_path = config.app_dir.join("server.py");
            let mut cmd = Command::new("uv");
            cmd.arg("run").arg("python").arg(server_path).arg(port.to_string());
            cmd.env("PYTHONPATH", "packages/python");
            cmd
        }
        _ => {
            return Err(Error::FrameworkNotFound(config.framework.clone()));
        }
    };

    if !config.framework.starts_with("spikard-")
        || config.framework == "spikard-ruby"
        || config.framework == "spikard-rust-workloads"
        || config.framework == "axum-baseline"
        || config.framework == "robyn"
    {
        cmd.current_dir(&config.app_dir);
    }
    // Temporarily enable output for debugging
    cmd.stdout(Stdio::inherit()).stderr(Stdio::inherit());

    let process = cmd
        .spawn()
        .map_err(|e| Error::ServerStartFailed(format!("Failed to spawn process: {}", e)))?;

    let mut handle = ServerHandle {
        process,
        port,
        base_url: base_url.clone(),
    };

    let max_attempts = 30;
    for attempt in 1..=max_attempts {
        sleep(Duration::from_secs(1)).await;

        match handle.process.try_wait() {
            Ok(Some(status)) => {
                return Err(Error::ServerStartFailed(format!(
                    "Process exited with status: {}",
                    status
                )));
            }
            Ok(None) => {
                if health_check(&base_url).await {
                    return Ok(handle);
                }
            }
            Err(e) => {
                return Err(Error::ServerStartFailed(format!(
                    "Failed to check process status: {}",
                    e
                )));
            }
        }

        if attempt == max_attempts {
            handle.kill()?;
            return Err(Error::ServerNotHealthy(max_attempts));
        }
    }

    Ok(handle)
}

/// Check if server is healthy
async fn health_check(base_url: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    for path in ["/health", "/"] {
        let url = format!("{}{}", base_url, path);
        if matches!(client.get(&url).send().await, Ok(r) if r.status().is_success()) {
            return true;
        }
    }

    false
}

/// Find an available port starting from the given port
pub fn find_available_port(start: u16) -> Option<u16> {
    (start..(start + 100)).find(|&port| is_port_available(port))
}

/// Check if a port is available
fn is_port_available(port: u16) -> bool {
    std::net::TcpListener::bind(("127.0.0.1", port)).is_ok()
}
