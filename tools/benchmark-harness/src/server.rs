//! Server management - start, stop, health check

use crate::error::{Error, Result};
use crate::framework::{detect_framework, get_framework};
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::sleep;

#[cfg(unix)]
use std::os::unix::process::CommandExt;

/// Server process handle
pub struct ServerHandle {
    pub process: Child,
    pub port: u16,
    pub base_url: String,
    #[cfg(unix)]
    pub stop_signal: i32,
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
            let pid = self.process.id() as i32;

            unsafe {
                let signal = self.stop_signal;
                if libc::kill(-pid, signal) != 0 {
                    let err = std::io::Error::last_os_error();
                    if err.raw_os_error() == Some(libc::ESRCH) {
                        libc::kill(pid, signal);
                    }
                }
            }

            let max_wait_iters = if self.stop_signal == libc::SIGINT { 150 } else { 50 };
            for _ in 0..max_wait_iters {
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
        #[cfg(unix)]
        {
            let pid = self.process.id() as i32;
            unsafe {
                if libc::kill(-pid, libc::SIGKILL) != 0 {
                    let err = std::io::Error::last_os_error();
                    if err.raw_os_error() == Some(libc::ESRCH) {
                        libc::kill(pid, libc::SIGKILL);
                    }
                }
            }
        }
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
    /// Extra environment variables injected into the server process.
    pub env: Vec<(String, String)>,
    /// Optional override for the framework start command.
    ///
    /// The string is split by whitespace, so it must not require shell quoting.
    pub start_cmd_override: Option<String>,
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
    let base_url = format!("http://127.0.0.1:{}", port);

    let framework_config = match &config.framework {
        Some(name) => get_framework(name).ok_or_else(|| Error::FrameworkNotFound(name.clone()))?,
        None => detect_framework(&config.app_dir)?,
    };

    if let Some(build_cmd) = &framework_config.build_cmd {
        let build_cmd = build_cmd.replace("{port}", &port.to_string());

        let parts: Vec<&str> = build_cmd.split_whitespace().collect();
        if !parts.is_empty() {
            let executable = parts[0];
            let args = &parts[1..];

            let mut build = Command::new(executable);
            build.args(args);
            build.current_dir(&config.app_dir);

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

    let start_cmd = config
        .start_cmd_override
        .as_deref()
        .unwrap_or(&framework_config.start_cmd)
        .replace("{port}", &port.to_string());

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
    cmd.env("PORT", port.to_string());
    for (key, value) in &config.env {
        cmd.env(key, value);
    }

    let working_dir = if let Some(hint) = &framework_config.working_dir_hint {
        config.app_dir.join(hint)
    } else {
        config.app_dir.clone()
    };

    cmd.current_dir(&working_dir);

    cmd.stdout(Stdio::null()).stderr(Stdio::piped());

    #[cfg(unix)]
    let stop_signal = if executable.contains("python") || executable == "uv" || executable.ends_with("/python") {
        libc::SIGINT
    } else {
        libc::SIGTERM
    };

    #[cfg(unix)]
    unsafe {
        cmd.pre_exec(|| {
            if libc::setpgid(0, 0) != 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }

    let mut process = cmd.spawn().map_err(|e| {
        Error::ServerStartFailed(format!(
            "Failed to spawn process for {} with command '{}': {}",
            framework_config.name, start_cmd, e
        ))
    })?;

    let stderr_output = Arc::new(Mutex::new(Vec::<u8>::new()));
    let stderr_tail_limit = 64 * 1024;
    if let Some(stderr) = process.stderr.take() {
        let stderr_output = Arc::clone(&stderr_output);
        std::thread::spawn(move || {
            use std::io::Read;
            let mut reader = stderr;
            let mut buf = [0u8; 8192];
            loop {
                match reader.read(&mut buf) {
                    Ok(0) => break,
                    Ok(n) => {
                        if let Ok(mut out) = stderr_output.lock() {
                            out.extend_from_slice(&buf[..n]);
                            if out.len() > stderr_tail_limit {
                                let drain = out.len() - stderr_tail_limit;
                                out.drain(..drain);
                            }
                        }
                    }
                    Err(_) => break,
                }
            }
        });
    }

    let mut handle = ServerHandle {
        process,
        port,
        base_url: base_url.clone(),
        #[cfg(unix)]
        stop_signal,
    };

    let stderr_snapshot = || -> String {
        let Ok(out) = stderr_output.lock() else {
            return String::new();
        };
        let Ok(text) = String::from_utf8(out.clone()) else {
            return String::new();
        };
        let text = text.trim();
        if text.is_empty() {
            String::new()
        } else {
            format!("\n\nServer stderr (tail):\n{}", text)
        }
    };

    let max_attempts = 30;
    for attempt in 1..=max_attempts {
        sleep(Duration::from_secs(1)).await;

        match handle.process.try_wait() {
            Ok(Some(status)) => {
                return Err(Error::ServerStartFailed(format!(
                    "Process exited with status: {}{}",
                    status,
                    stderr_snapshot()
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
            return Err(Error::ServerStartFailed(format!(
                "Server not healthy after {} attempts{}",
                max_attempts,
                stderr_snapshot()
            )));
        }
    }

    Ok(handle)
}

/// Check if server is healthy
async fn health_check(base_url: &str) -> bool {
    let Ok(client) = reqwest::Client::builder().timeout(Duration::from_secs(2)).build() else {
        return false;
    };

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
            env: Vec::new(),
            start_cmd_override: None,
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
            env: Vec::new(),
            start_cmd_override: None,
        };

        assert_eq!(config.framework, None);
        assert_eq!(config.port, 8080);
    }

    #[test]
    fn test_port_substitution_in_command() {
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

        assert!(!substituted.is_empty());
    }

    #[test]
    fn test_working_directory_resolution() {
        let app_dir = PathBuf::from("/app");

        let framework = get_framework("spikard-python").expect("spikard-python should exist");
        let working_dir = if let Some(hint) = &framework.working_dir_hint {
            app_dir.join(hint)
        } else {
            app_dir.clone()
        };
        assert_eq!(working_dir, PathBuf::from("/app"));

        let framework = get_framework("spikard-rust").expect("spikard-rust should exist");
        if let Some(hint) = &framework.working_dir_hint {
            let working_dir = app_dir.join(hint);
            assert!(!working_dir.to_string_lossy().ends_with("/"));
        }
    }

    #[test]
    fn test_is_port_available() {
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
        let frameworks = vec![
            "spikard-rust",
            "spikard-python",
            "spikard-node",
            "spikard-ruby",
            "spikard-wasm",
            "fastapi-uvicorn-dto",
            "robyn-dto",
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

        let detected = detect_framework(temp_dir.path());
        assert!(detected.is_ok());
        assert_eq!(detected.unwrap().name, "spikard-rust");
    }

    #[test]
    fn test_auto_detect_python_framework() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.py"), "# Python server").unwrap();

        let detected = detect_framework(temp_dir.path());
        assert!(detected.is_ok());
        assert_eq!(detected.unwrap().name, "spikard-python");
    }

    #[test]
    fn test_server_config_variant_field() {
        let config = ServerConfig {
            framework: Some("spikard".to_string()),
            port: 8080,
            app_dir: PathBuf::from("."),
            variant: Some("async".to_string()),
            env: Vec::new(),
            start_cmd_override: None,
        };

        assert_eq!(config.variant, Some("async".to_string()));
    }
}
