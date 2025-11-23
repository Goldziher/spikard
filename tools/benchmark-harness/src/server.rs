//! Server management - start, stop, health check

use crate::error::{Error, Result};
use crate::framework::{detect_framework, get_framework};
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
    /// Framework name - if None, will auto-detect from app_dir
    pub framework: Option<String>,
    pub port: u16,
    pub app_dir: PathBuf,
    /// Variant name (e.g., "sync", "async") - optional
    pub variant: Option<String>,
}

/// Start a server and wait for it to be ready
///
/// # Arguments
///
/// * `config` - Server configuration with optional framework name
///   - If `framework` is Some, uses that framework explicitly
///   - If `framework` is None, auto-detects from `app_dir`
///
/// # Behavior
///
/// 1. Resolves framework configuration via registry or auto-detection
/// 2. Executes build command if present in framework config
/// 3. Substitutes {port} placeholder in start command
/// 4. Changes to working directory hint if specified
/// 5. Spawns server process and waits for health check
///
/// # Errors
///
/// Returns error if:
/// - Framework not found or auto-detection fails
/// - Build command execution fails
/// - Server process fails to spawn
/// - Server fails health check within timeout
pub async fn start_server(config: ServerConfig) -> Result<ServerHandle> {
    let port = config.port;
    let base_url = format!("http://localhost:{}", port);

    // Step 1: Resolve framework configuration
    let framework_config = match &config.framework {
        Some(name) => {
            // Explicit framework name provided - look it up in registry
            get_framework(name).ok_or_else(|| Error::FrameworkNotFound(name.clone()))?
        }
        None => {
            // Auto-detect framework from app directory
            detect_framework(&config.app_dir)?
        }
    };

    // Step 2: Execute build command if present
    if let Some(build_cmd) = &framework_config.build_cmd {
        let build_cmd = build_cmd.replace("{port}", &port.to_string());

        // Parse command into executable and arguments
        let parts: Vec<&str> = build_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let executable = parts[0];
            let args = &parts[1..];

            let mut build = Command::new(executable);
            build.args(args);
            build.current_dir(&config.app_dir);

            // Build output is normally visible since it's a one-time operation
            let status = build.status().map_err(|e| {
                Error::ServerStartFailed(format!("Failed to execute build command '{}': {}", build_cmd, e))
            })?;

            if !status.success() {
                return Err(Error::ServerStartFailed(format!(
                    "Build command failed with status: {}",
                    status
                )));
            }
        }
    }

    // Step 3: Build start command with port substitution
    let start_cmd = framework_config.start_cmd.replace("{port}", &port.to_string());

    // Parse command into executable and arguments
    let parts: Vec<&str> = start_cmd.split_whitespace().collect();
    if parts.is_empty() {
        return Err(Error::ServerStartFailed(
            "Empty start command from framework config".to_string(),
        ));
    }

    let executable = parts[0];
    let args = &parts[1..];

    let mut cmd = Command::new(executable);
    cmd.args(args);

    // Step 4: Set working directory
    let working_dir = if let Some(hint) = &framework_config.working_dir_hint {
        config.app_dir.join(hint)
    } else {
        config.app_dir.clone()
    };

    cmd.current_dir(&working_dir);

    // Suppress stdout but capture stderr for debugging startup failures
    cmd.stdout(Stdio::null()).stderr(Stdio::piped());

    // Step 5: Spawn server process
    let mut process = cmd.spawn().map_err(|e| {
        Error::ServerStartFailed(format!(
            "Failed to spawn process for {} with command '{}': {}",
            framework_config.name, start_cmd, e
        ))
    })?;

    // Capture stderr for debugging
    let stderr = process.stderr.take();

    let mut handle = ServerHandle {
        process,
        port,
        base_url: base_url.clone(),
    };

    // Step 6: Wait for health check with timeout
    let max_attempts = 30;
    for attempt in 1..=max_attempts {
        sleep(Duration::from_secs(1)).await;

        match handle.process.try_wait() {
            Ok(Some(status)) => {
                // Process exited - capture stderr output
                let stderr_output = if let Some(mut stderr) = stderr {
                    use std::io::Read;
                    let mut buf = String::new();
                    stderr.read_to_string(&mut buf).ok();
                    if buf.is_empty() {
                        String::new()
                    } else {
                        format!("\n\nServer stderr:\n{}", buf)
                    }
                } else {
                    String::new()
                };

                return Err(Error::ServerStartFailed(format!(
                    "Process exited with status: {}{}",
                    status,
                    stderr_output
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_server_config_with_explicit_framework() {
        let config = ServerConfig {
            framework: Some("spikard-rust".to_string()),
            port: 8080,
            app_dir: PathBuf::from("."),
            variant: None,
        };

        assert_eq!(config.framework, Some("spikard-rust".to_string()));
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_server_config_with_auto_detect() {
        let config = ServerConfig {
            framework: None,
            port: 8080,
            app_dir: PathBuf::from("."),
            variant: None,
        };

        assert_eq!(config.framework, None);
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_port_substitution_in_command() {
        // Test that {port} placeholder is correctly substituted
        let framework = get_framework("spikard-rust").expect("spikard-rust should exist");
        let port = 9000u16;
        let start_cmd = framework.start_cmd.replace("{port}", &port.to_string());

        assert!(start_cmd.contains("9000"));
        assert!(!start_cmd.contains("{port}"));
    }

    #[test]
    fn test_build_command_substitution() {
        let framework = get_framework("spikard-rust").expect("spikard-rust should exist");
        assert!(framework.build_cmd.is_some());

        let build_cmd = framework.build_cmd.unwrap();
        let port = 8000u16;
        let substituted = build_cmd.replace("{port}", &port.to_string());

        // Build command may or may not have {port}, just ensure no errors
        assert!(!substituted.is_empty());
    }

    #[test]
    fn test_working_directory_resolution() {
        let app_dir = PathBuf::from("/app");

        // Test without working_dir_hint
        let framework = get_framework("spikard").expect("spikard should exist");
        let working_dir = if let Some(hint) = &framework.working_dir_hint {
            app_dir.join(hint)
        } else {
            app_dir.clone()
        };
        assert_eq!(working_dir, PathBuf::from("/app"));

        // Test with working_dir_hint (if applicable)
        // Most frameworks don't have hints, but if one did:
        let framework = get_framework("spikard-rust").expect("spikard-rust should exist");
        if let Some(hint) = &framework.working_dir_hint {
            let working_dir = app_dir.join(hint);
            assert!(!working_dir.to_string_lossy().ends_with("/"));
        }
    }

    #[test]
    fn test_is_port_available() {
        // Find an unused port first
        let port = find_available_port(10000).expect("Should find available port");
        assert!(is_port_available(port));
    }

    #[test]
    fn test_find_available_port() {
        let port = find_available_port(20000);
        assert!(port.is_some());

        let port = port.unwrap();
        assert!(port >= 20000);
        assert!(port < 20100);
    }

    #[test]
    fn test_framework_config_access() {
        // Verify that all major frameworks are accessible
        let frameworks = vec![
            "spikard-rust",
            "spikard",
            "spikard-node",
            "spikard-ruby",
            "spikard-wasm",
            "fastapi",
            "robyn",
        ];

        for name in frameworks {
            let fw = get_framework(name);
            assert!(fw.is_some(), "Framework {} should be in registry", name);

            let config = fw.unwrap();
            assert_eq!(config.name, name);
            assert!(!config.start_cmd.is_empty());
            assert!(config.start_cmd.contains("{port}"));
        }
    }

    #[test]
    fn test_auto_detect_rust_framework() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main()").unwrap();

        // Manually test detection logic (start_server would use this internally)
        let detected = detect_framework(temp_dir.path());
        assert!(detected.is_ok());
        assert_eq!(detected.unwrap().name, "spikard-rust");
    }

    #[test]
    fn test_auto_detect_python_framework() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.py"), "# Python server").unwrap();
        fs::write(temp_dir.path().join("pyproject.toml"), "[build-system]").unwrap();

        let detected = detect_framework(temp_dir.path());
        assert!(detected.is_ok());
        assert_eq!(detected.unwrap().name, "spikard");
    }

    #[test]
    fn test_server_config_variant_field() {
        let config = ServerConfig {
            framework: Some("spikard".to_string()),
            port: 8080,
            app_dir: PathBuf::from("."),
            variant: Some("async".to_string()),
        };

        assert_eq!(config.variant, Some("async".to_string()));
    }
}
