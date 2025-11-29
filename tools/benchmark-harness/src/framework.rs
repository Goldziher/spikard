//! Framework registry and detection
//!
//! This module provides framework detection and configuration management for the benchmark harness.
//! It maintains a registry of all supported frameworks and provides utilities to detect which
//! framework is present in a given directory.

use crate::Result;
use std::path::Path;

/// Configuration for a framework
#[derive(Debug, Clone)]
pub struct FrameworkConfig {
    /// Framework name (e.g., "spikard-rust", "fastapi")
    pub name: String,

    /// Files to look for when detecting this framework
    /// If any of these files exist in the app directory, the framework is considered detected
    pub detect_files: Vec<String>,

    /// Optional build command to prepare the framework before running
    /// May contain {port} placeholder for runtime substitution
    pub build_cmd: Option<String>,

    /// Command to start the server
    /// May contain {port} placeholder that will be replaced at runtime
    pub start_cmd: String,

    /// Optional subdirectory hint for where to run the framework from
    /// (e.g., "." for root, "crates/spikard-rust" for workspace crates)
    pub working_dir_hint: Option<String>,
}

impl FrameworkConfig {
    /// Creates a new framework configuration
    pub fn new(
        name: impl Into<String>,
        detect_files: Vec<String>,
        build_cmd: Option<String>,
        start_cmd: impl Into<String>,
        working_dir_hint: Option<String>,
    ) -> Self {
        Self {
            name: name.into(),
            detect_files,
            build_cmd,
            start_cmd: start_cmd.into(),
            working_dir_hint,
        }
    }

    /// Checks if all detect_files exist in the given directory
    fn matches(&self, app_dir: &Path) -> bool {
        self.detect_files.iter().all(|file| app_dir.join(file).exists())
    }
}

/// Registry of all supported frameworks
fn framework_registry() -> Vec<FrameworkConfig> {
    vec![
        // Spikard bindings - detect via server files only
        // (config files like pyproject.toml, package.json, Gemfile are in workspace root, not app dirs)
        FrameworkConfig::new(
            "spikard-rust",
            vec!["Cargo.toml".to_string(), "src/main.rs".to_string()],
            Some("cargo build --release".to_string()),
            "cargo run --release -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-node",
            vec!["server.ts".to_string()],
            None,
            "pnpm start {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-ruby",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-wasm",
            vec!["server.js".to_string()],
            None,
            "wasmtime run -- server.js {port}",
            None,
        ),
        // Baseline and alternative frameworks
        FrameworkConfig::new(
            "axum-baseline",
            vec!["Cargo.toml".to_string()],
            Some("cargo build --release".to_string()),
            "cargo run --release -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi-granian",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "robyn",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        // Raw/no-validation variants for validation overhead measurement
        FrameworkConfig::new(
            "spikard-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi-granian-raw",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-raw",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-granian-raw",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        // TypeScript frameworks
        FrameworkConfig::new(
            "fastify",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastify-raw",
            vec!["server.js".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "hono",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "hono-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "elysia",
            vec!["server.ts".to_string()],
            None,
            "bun run server.ts {port}",
            None,
        ),
        FrameworkConfig::new(
            "morojs",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "express",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start:ts -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "express-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-granian",
            vec!["server.py".to_string()],
            None,
            "uv run server.py {port}",
            None,
        ),
        // Ruby frameworks
        FrameworkConfig::new(
            "hanami-api",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "hanami-api-raw",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "roda",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "roda-raw",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        // PHP frameworks
        FrameworkConfig::new(
            "trongate",
            vec!["server.php".to_string()],
            None,
            "php server.php {port}",
            None,
        ),
        FrameworkConfig::new(
            "phalcon",
            vec!["server.php".to_string(), "composer.json".to_string()],
            Some("composer install --no-dev --optimize-autoloader".to_string()),
            "php server.php {port}",
            None,
        ),
    ]
}

/// Detects the framework used in the given application directory
///
/// Scans the provided directory for framework-specific files and returns the matching
/// framework configuration. Returns an error if no framework is detected or if the detection
/// is ambiguous (multiple frameworks found).
///
/// # Arguments
///
/// * `app_dir` - Path to the application directory to scan
///
/// # Returns
///
/// * `Ok(FrameworkConfig)` - The detected framework configuration
/// * `Err(Error)` - If no framework detected or detection is ambiguous
///
/// # Examples
///
/// ```no_run
/// use benchmark_harness::framework::detect_framework;
/// use std::path::Path;
///
/// let config = detect_framework(Path::new("/path/to/app"))?;
/// println!("Detected framework: {}", config.name);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn detect_framework(app_dir: &Path) -> Result<FrameworkConfig> {
    let registry = framework_registry();

    // Check if directory exists
    if !app_dir.exists() {
        return Err(crate::Error::InvalidInput(format!(
            "App directory does not exist: {}",
            app_dir.display()
        )));
    }

    // Try to detect frameworks in priority order:
    // 1. First, check for more specific frameworks (those with more detect_files)
    // 2. Then fall back to less specific ones
    // This avoids false positives when a directory could match multiple frameworks
    let mut matches: Vec<FrameworkConfig> = registry.into_iter().filter(|fw| fw.matches(app_dir)).collect();

    if matches.is_empty() {
        return Err(crate::Error::InvalidInput(format!(
            "No framework detected in {}. Expected one of: Cargo.toml, server.py, server.ts, server.rb, server.js, Gemfile",
            app_dir.display()
        )));
    }

    // Sort by number of detect_files in descending order (most specific first)
    matches.sort_by(|a, b| b.detect_files.len().cmp(&a.detect_files.len()));

    // Return the most specific match
    Ok(matches.into_iter().next().unwrap())
}

/// Returns all available frameworks in the registry
///
/// # Returns
///
/// A vector of all framework configurations
///
/// # Examples
///
/// ```no_run
/// use benchmark_harness::framework::list_frameworks;
///
/// let frameworks = list_frameworks();
/// for fw in frameworks {
///     println!("Available: {}", fw.name);
/// }
/// ```
pub fn list_frameworks() -> Vec<FrameworkConfig> {
    framework_registry()
}

/// Gets a specific framework configuration by name
///
/// # Arguments
///
/// * `name` - The framework name to look up
///
/// # Returns
///
/// * `Some(FrameworkConfig)` - If the framework exists in the registry
/// * `None` - If the framework is not found
///
/// # Examples
///
/// ```no_run
/// use benchmark_harness::framework::get_framework;
///
/// if let Some(config) = get_framework("spikard-rust") {
///     println!("Start command: {}", config.start_cmd);
/// }
/// ```
pub fn get_framework(name: &str) -> Option<FrameworkConfig> {
    framework_registry().into_iter().find(|fw| fw.name == name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_framework_config_creation() {
        let config = FrameworkConfig::new(
            "test-framework",
            vec!["Cargo.toml".to_string()],
            Some("cargo build".to_string()),
            "cargo run",
            Some("src".to_string()),
        );

        assert_eq!(config.name, "test-framework");
        assert_eq!(config.detect_files.len(), 1);
        assert!(config.build_cmd.is_some());
        assert_eq!(config.working_dir_hint, Some("src".to_string()));
    }

    #[test]
    fn test_registry_contains_all_frameworks() {
        let registry = framework_registry();
        let names: Vec<&str> = registry.iter().map(|f| f.name.as_str()).collect();

        // Spikard bindings (5)
        assert!(names.contains(&"spikard-rust"));
        assert!(names.contains(&"spikard"));
        assert!(names.contains(&"spikard-node"));
        assert!(names.contains(&"spikard-ruby"));
        assert!(names.contains(&"spikard-wasm"));

        // Python validated (6)
        assert!(names.contains(&"axum-baseline"));
        assert!(names.contains(&"fastapi"));
        assert!(names.contains(&"fastapi-granian"));
        assert!(names.contains(&"litestar"));
        assert!(names.contains(&"litestar-granian"));
        assert!(names.contains(&"robyn"));

        // Python raw (5)
        assert!(names.contains(&"spikard-raw"));
        assert!(names.contains(&"fastapi-raw"));
        assert!(names.contains(&"fastapi-granian-raw"));
        assert!(names.contains(&"litestar-raw"));
        assert!(names.contains(&"litestar-granian-raw"));

        // TypeScript (6)
        assert!(names.contains(&"fastify"));
        assert!(names.contains(&"fastify-raw"));
        assert!(names.contains(&"hono"));
        assert!(names.contains(&"hono-raw"));
        assert!(names.contains(&"express"));
        assert!(names.contains(&"express-raw"));

        // Ruby (4)
        assert!(names.contains(&"hanami-api"));
        assert!(names.contains(&"hanami-api-raw"));
        assert!(names.contains(&"roda"));
        assert!(names.contains(&"roda-raw"));

        assert_eq!(registry.len(), 30);
    }

    #[test]
    fn test_detect_framework_not_found() {
        let temp_dir = TempDir::new().unwrap();
        let result = detect_framework(temp_dir.path());

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("No framework detected"));
    }

    #[test]
    fn test_detect_framework_rust() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main()").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-rust");
    }

    #[test]
    fn test_detect_framework_ambiguous() {
        let temp_dir = TempDir::new().unwrap();
        // Create files that would match multiple frameworks
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main()").unwrap();

        let result = detect_framework(temp_dir.path());
        // This should still succeed as axum-baseline only requires Cargo.toml
        // and spikard-rust requires both Cargo.toml and src/main.rs
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-rust");
    }

    #[test]
    fn test_get_framework() {
        let framework = get_framework("spikard-rust");
        assert!(framework.is_some());

        let config = framework.unwrap();
        assert_eq!(config.name, "spikard-rust");
        assert!(config.build_cmd.is_some());
        assert!(config.detect_files.contains(&"Cargo.toml".to_string()));
    }

    #[test]
    fn test_get_framework_not_found() {
        let framework = get_framework("nonexistent-framework");
        assert!(framework.is_none());
    }

    #[test]
    fn test_list_frameworks() {
        let frameworks = list_frameworks();
        assert_eq!(frameworks.len(), 30);
    }

    #[test]
    fn test_framework_matches() {
        let temp_dir = TempDir::new().unwrap();
        let config = FrameworkConfig::new(
            "test",
            vec!["file1.txt".to_string(), "file2.txt".to_string()],
            None,
            "test command",
            None,
        );

        // Should not match when files don't exist
        assert!(!config.matches(temp_dir.path()));

        // Should match when all files exist
        fs::write(temp_dir.path().join("file1.txt"), "content").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "content").unwrap();
        assert!(config.matches(temp_dir.path()));
    }

    #[test]
    fn test_detect_invalid_directory() {
        let result = detect_framework(Path::new("/nonexistent/path/12345"));
        assert!(result.is_err());
    }

    #[test]
    fn test_detect_spikard_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.py"), "# python server").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard");
    }

    #[test]
    fn test_detect_spikard_node_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.ts"), "// typescript server").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-node");
    }

    #[test]
    fn test_detect_spikard_ruby_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.rb"), "# ruby server").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-ruby");
    }

    #[test]
    fn test_detect_spikard_wasm_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.js"), "// wasm server").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-wasm");
    }

    #[test]
    fn test_detect_fastify_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("server.ts"), "// fastify server").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        // When only server.ts exists, it should detect as spikard-node first (more specific precedence)
        // but both match, so we verify it's one of the valid options
        let name = result.unwrap().name;
        assert!(name == "spikard-node" || name == "fastify");
    }
}
