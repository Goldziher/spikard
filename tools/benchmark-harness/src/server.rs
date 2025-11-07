//! Server management - start, stop, health check

use crate::error::{Error, Result};
use std::env;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// Find the workspace root by looking for Cargo.toml
fn find_workspace_root() -> Result<PathBuf> {
    // Start from the current executable's directory
    let exe_path =
        env::current_exe().map_err(|e| Error::ServerStartFailed(format!("Failed to get executable path: {}", e)))?;

    let mut current = exe_path
        .parent()
        .ok_or_else(|| Error::ServerStartFailed("Failed to get executable parent directory".to_string()))?;

    // Walk up the directory tree looking for workspace Cargo.toml
    loop {
        let cargo_toml = current.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if this is the workspace root by looking for [workspace] section
            if let Ok(contents) = std::fs::read_to_string(&cargo_toml)
                && contents.contains("[workspace]")
            {
                return Ok(current.to_path_buf());
            }
        }

        // Move up one directory
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
            // Try SIGTERM first
            unsafe {
                libc::kill(self.process.id() as i32, libc::SIGTERM);
            }

            // Wait up to 5 seconds for graceful shutdown
            for _ in 0..50 {
                match self.process.try_wait() {
                    Ok(Some(_)) => return Ok(()),
                    Ok(None) => std::thread::sleep(Duration::from_millis(100)),
                    Err(e) => return Err(e.into()),
                }
            }

            // Force kill if still running
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
        // Best effort kill
        let _ = self.process.kill();
    }
}

/// Server configuration
pub struct ServerConfig {
    pub framework: String,
    pub port: u16,
    pub app_dir: PathBuf,
}

/// Start a server and wait for it to be ready
pub async fn start_server(config: ServerConfig) -> Result<ServerHandle> {
    let port = config.port;
    let base_url = format!("http://localhost:{}", port);

    // Determine command based on framework
    let mut cmd = match config.framework.as_str() {
        // Rust server is a standalone binary, not using the CLI
        "spikard-rust" => {
            let server_binary = config.app_dir.join("target/release/server");
            let mut cmd = Command::new(server_binary);
            cmd.arg(port.to_string());
            cmd
        }
        // Other Spikard frameworks use the unified CLI
        "spikard-python" | "spikard-node" | "spikard-ruby" => {
            // Find workspace root and construct absolute path to CLI
            let workspace_root = find_workspace_root()?;
            let cli_path = workspace_root.join("target/release/spikard");

            // Determine server file based on framework
            let server_file = match config.framework.as_str() {
                "spikard-python" => "server.py",
                "spikard-node" => "server.js",
                "spikard-ruby" => "server.rb",
                _ => unreachable!(),
            };

            let server_path = config.app_dir.join(server_file);

            let mut cmd = Command::new(cli_path);
            cmd.arg("run").arg(&server_path).arg("--port").arg(port.to_string());
            cmd
        }
        "fastapi" => {
            let mut cmd = Command::new("python");
            cmd.arg("server.py").arg(port.to_string());
            cmd
        }
        "fastify" => {
            let mut cmd = Command::new("node");
            cmd.arg("server.js").arg(port.to_string());
            cmd
        }
        _ => {
            return Err(Error::FrameworkNotFound(config.framework.clone()));
        }
    };

    // Set working directory and spawn process
    // Note: Spikard frameworks use absolute paths, so they don't need current_dir
    // but it doesn't hurt to set it
    if !config.framework.starts_with("spikard-") {
        cmd.current_dir(&config.app_dir);
    }
    // Discard output to avoid blocking when buffers fill up
    cmd.stdout(Stdio::null()).stderr(Stdio::null());

    let process = cmd
        .spawn()
        .map_err(|e| Error::ServerStartFailed(format!("Failed to spawn process: {}", e)))?;

    let mut handle = ServerHandle {
        process,
        port,
        base_url: base_url.clone(),
    };

    // Wait for server to be ready
    let max_attempts = 30; // 30 seconds
    for attempt in 1..=max_attempts {
        sleep(Duration::from_secs(1)).await;

        // Check if process is still running
        match handle.process.try_wait() {
            Ok(Some(status)) => {
                return Err(Error::ServerStartFailed(format!(
                    "Process exited with status: {}",
                    status
                )));
            }
            Ok(None) => {
                // Still running, check health
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

    // Try /health first, then fallback to /
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
