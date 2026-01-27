//! Framework registry and detection
//!
//! This module provides framework detection and configuration management for the benchmark harness.
//! It maintains a registry of all supported frameworks and provides utilities to detect which
//! framework is present in a given directory.

use crate::Result;
use std::path::Path;
use std::process::Command;

/// Configuration for a framework
#[derive(Debug, Clone)]
pub struct FrameworkConfig {
    /// Framework name (e.g., "spikard-rust-validation", "fastapi")
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

    /// Optional list of workload categories supported by this framework.
    /// If None, all categories are assumed supported.
    pub supported_categories: Option<Vec<String>>,
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
            supported_categories: None,
        }
    }

    #[must_use]
    pub fn with_supported_categories(mut self, categories: Vec<String>) -> Self {
        self.supported_categories = Some(categories);
        self
    }

    #[must_use]
    pub fn supports_category(&self, category: &str) -> bool {
        self.supported_categories
            .as_ref()
            .is_none_or(|categories| categories.iter().any(|item| item == category))
    }

    /// Checks if all `detect_files` exist in the given directory
    fn matches(&self, app_dir: &Path) -> bool {
        self.detect_files.iter().all(|file| app_dir.join(file).exists())
    }
}

/// Registry of all supported frameworks
fn php_extension_available(extension: &str) -> bool {
    let output = Command::new("php").arg("-m").output();
    let Ok(output) = output else {
        return false;
    };
    if !output.status.success() {
        return false;
    }
    let Ok(stdout) = String::from_utf8(output.stdout) else {
        return false;
    };
    stdout.lines().any(|line| line.trim().eq_ignore_ascii_case(extension))
}

fn framework_registry() -> Vec<FrameworkConfig> {
    let mut frameworks = vec![
        // Generated benchmark apps (from app-generator)
        FrameworkConfig::new(
            "spikard-python",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-rust",
            vec!["Cargo.toml".to_string(), "server.rs".to_string()],
            Some("cargo build --release".to_string()),
            "./target/release/server {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-node",
            vec!["server.ts".to_string()],
            None,
            "pnpm tsx tools/benchmark-harness/apps/spikard-node/server.ts {port}",
            Some("../../../..".to_string()),
        ),
        FrameworkConfig::new(
            "spikard-ruby",
            vec!["server.rb".to_string()],
            None,
            "ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-php",
            vec!["server.php".to_string()],
            None,
            "php -S 0.0.0.0:{port} server.php",
            None,
        ),
        // Pre-existing validation apps
        FrameworkConfig::new(
            "spikard-rust-validation",
            vec!["Cargo.toml".to_string(), "src/main.rs".to_string()],
            Some("cargo build --release".to_string()),
            "./target/release/spikard-rust-bench {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-python-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-node-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-bun-validation",
            vec!["server.ts".to_string()],
            None,
            "bun run server.ts {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-ruby-validation",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-php-validation",
            vec!["server.php".to_string()],
            None,
            "./start.sh {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-wasm-validation",
            vec!["server.ts".to_string()],
            None,
            "deno run --allow-net --allow-read server.ts {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-rust-raw",
            vec!["Cargo.toml".to_string(), "src/main.rs".to_string()],
            Some("cargo build --release".to_string()),
            "./target/release/spikard-rust-bench {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-python-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-node-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-bun-raw",
            vec!["server.ts".to_string()],
            None,
            "bun run server.ts {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-ruby-raw",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-php-raw",
            vec!["server.php".to_string()],
            None,
            "./start.sh {port}",
            None,
        ),
        FrameworkConfig::new(
            "spikard-wasm-raw",
            vec!["server.ts".to_string()],
            None,
            "deno run --allow-net --allow-read server.ts {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi-uvicorn-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "fastapi-uvicorn-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "fastapi-python",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastapi-granian-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "robyn-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
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
            "litestar-uvicorn-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-granian-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "robyn-raw",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "fastify-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string(), "forms".to_string()]),
        FrameworkConfig::new(
            "fastify-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "hono-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "hono-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "elysia-validation",
            vec!["server.ts".to_string()],
            None,
            "bun run server.ts {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "morojs-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "morojs-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "kito-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "kito-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm run start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "express-validation",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string()]),
        FrameworkConfig::new(
            "express-raw",
            vec!["server.ts".to_string()],
            None,
            "pnpm start -- {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-uvicorn-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "litestar-granian-validation",
            vec!["server.py".to_string()],
            None,
            "uv run python server.py {port}",
            None,
        ),
        FrameworkConfig::new(
            "hanami-api-validation",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string(), "forms".to_string()]),
        FrameworkConfig::new(
            "hanami-api-raw",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "roda-validation",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        )
        .with_supported_categories(vec!["json-bodies".to_string(), "forms".to_string()]),
        FrameworkConfig::new(
            "roda-raw",
            vec!["server.rb".to_string()],
            None,
            "bundle exec ruby server.rb {port}",
            None,
        ),
        FrameworkConfig::new(
            "trongate-raw",
            vec!["server.php".to_string()],
            None,
            "php server.php {port}",
            None,
        ),
    ];

    if php_extension_available("phalcon") {
        frameworks.push(FrameworkConfig::new(
            "phalcon-raw",
            vec!["server.php".to_string(), "composer.json".to_string()],
            Some("composer install --no-dev --optimize-autoloader".to_string()),
            "php -S 0.0.0.0:{port} server.php",
            None,
        ));
        frameworks.push(
            FrameworkConfig::new(
                "phalcon-validation",
                vec!["server.php".to_string(), "composer.json".to_string()],
                Some("composer install --no-dev --optimize-autoloader".to_string()),
                "php -S 0.0.0.0:{port} server.php",
                None,
            )
            .with_supported_categories(vec!["json-bodies".to_string()]),
        );
    }

    frameworks
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

    if !app_dir.exists() {
        return Err(crate::Error::InvalidInput(format!(
            "App directory does not exist: {}",
            app_dir.display()
        )));
    }

    let mut matches: Vec<FrameworkConfig> = registry.into_iter().filter(|fw| fw.matches(app_dir)).collect();

    if matches.is_empty() {
        return Err(crate::Error::InvalidInput(format!(
            "No framework detected in {}. Expected one of: Cargo.toml, server.py, server.ts, server.rb, Gemfile",
            app_dir.display()
        )));
    }

    if matches.len() > 1
        && let Some(dir_name) = app_dir.file_name().and_then(|name| name.to_str())
        && let Some(index) = matches.iter().position(|fw| fw.name == dir_name)
    {
        return Ok(matches.swap_remove(index));
    }

    // Prefer validation variants when multiple frameworks match
    if matches.len() > 1
        && let Some(validation_match) = matches.iter().find(|fw| fw.name.contains("-validation"))
    {
        return Ok(validation_match.clone());
    }

    matches.sort_by(|a, b| b.detect_files.len().cmp(&a.detect_files.len()));

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
#[must_use]
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
/// if let Some(config) = get_framework("spikard-rust-validation") {
///     println!("Start command: {}", config.start_cmd);
/// }
/// ```
#[must_use]
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

        assert!(names.contains(&"spikard-rust-validation"));
        assert!(names.contains(&"spikard-python-validation"));
        assert!(names.contains(&"spikard-node-validation"));
        assert!(names.contains(&"spikard-bun-validation"));
        assert!(names.contains(&"spikard-ruby-validation"));
        assert!(names.contains(&"spikard-php-validation"));
        assert!(names.contains(&"spikard-wasm-validation"));
        assert!(names.contains(&"spikard-rust-raw"));
        assert!(names.contains(&"spikard-python-raw"));
        assert!(names.contains(&"spikard-node-raw"));
        assert!(names.contains(&"spikard-bun-raw"));
        assert!(names.contains(&"spikard-ruby-raw"));
        assert!(names.contains(&"spikard-php-raw"));
        assert!(names.contains(&"spikard-wasm-raw"));

        assert!(names.contains(&"fastapi-uvicorn-validation"));
        assert!(names.contains(&"fastapi-uvicorn-raw"));
        assert!(names.contains(&"fastapi-python"));
        assert!(names.contains(&"fastapi-granian-validation"));
        assert!(names.contains(&"litestar-uvicorn-validation"));
        assert!(names.contains(&"litestar-granian-validation"));
        assert!(names.contains(&"robyn-validation"));

        assert!(names.contains(&"fastapi-raw"));
        assert!(names.contains(&"fastapi-granian-raw"));
        assert!(names.contains(&"litestar-uvicorn-raw"));
        assert!(names.contains(&"litestar-granian-raw"));
        assert!(names.contains(&"robyn-raw"));

        assert!(names.contains(&"fastify-validation"));
        assert!(names.contains(&"fastify-raw"));
        assert!(names.contains(&"hono-validation"));
        assert!(names.contains(&"hono-raw"));
        assert!(names.contains(&"elysia-validation"));
        assert!(names.contains(&"morojs-validation"));
        assert!(names.contains(&"morojs-raw"));
        assert!(names.contains(&"express-validation"));
        assert!(names.contains(&"express-raw"));
        assert!(names.contains(&"kito-validation"));
        assert!(names.contains(&"kito-raw"));

        assert!(names.contains(&"hanami-api-validation"));
        assert!(names.contains(&"hanami-api-raw"));
        assert!(names.contains(&"roda-validation"));
        assert!(names.contains(&"roda-raw"));

        assert!(names.contains(&"trongate-raw"));
        if php_extension_available("phalcon") {
            assert!(names.contains(&"phalcon-raw"));
            assert!(names.contains(&"phalcon-validation"));
        } else {
            assert!(!names.contains(&"phalcon-raw"));
            assert!(!names.contains(&"phalcon-validation"));
        }

        let expected_len = if php_extension_available("phalcon") { 49 } else { 47 };
        assert_eq!(registry.len(), expected_len);
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
        assert_eq!(result.unwrap().name, "spikard-rust-validation");
    }

    #[test]
    fn test_detect_framework_ambiguous() {
        let temp_dir = TempDir::new().unwrap();
        fs::write(temp_dir.path().join("Cargo.toml"), "[package]").unwrap();
        fs::create_dir_all(temp_dir.path().join("src")).unwrap();
        fs::write(temp_dir.path().join("src").join("main.rs"), "fn main()").unwrap();

        let result = detect_framework(temp_dir.path());
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-rust-validation");
    }

    #[test]
    fn test_get_framework() {
        let framework = get_framework("spikard-rust-validation");
        assert!(framework.is_some());

        let config = framework.unwrap();
        assert_eq!(config.name, "spikard-rust-validation");
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
        let expected_len = if php_extension_available("phalcon") { 49 } else { 47 };
        assert_eq!(frameworks.len(), expected_len); // Base: 47, +2 with phalcon
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

        assert!(!config.matches(temp_dir.path()));

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
        let app_dir = temp_dir.path().join("spikard-python-validation");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("server.py"), "# python server").unwrap();

        let result = detect_framework(&app_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-python-validation");
    }

    #[test]
    fn test_detect_spikard_node_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        let app_dir = temp_dir.path().join("spikard-node-validation");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("server.ts"), "// typescript server").unwrap();

        let result = detect_framework(&app_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-node-validation");
    }

    #[test]
    fn test_detect_spikard_ruby_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        let app_dir = temp_dir.path().join("spikard-ruby-validation");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("server.rb"), "# ruby server").unwrap();

        let result = detect_framework(&app_dir);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().name, "spikard-ruby-validation");
    }

    #[test]
    fn test_detect_spikard_wasm_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        let app_dir = temp_dir.path().join("spikard-wasm-validation");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("server.ts"), "// wasm server").unwrap();

        let result = detect_framework(&app_dir);
        assert!(result.is_ok());
        let name = result.unwrap().name;
        assert_eq!(name, "spikard-wasm-validation");
    }

    #[test]
    fn test_detect_fastify_with_server_only() {
        let temp_dir = TempDir::new().unwrap();
        let app_dir = temp_dir.path().join("fastify-raw");
        fs::create_dir_all(&app_dir).unwrap();
        fs::write(app_dir.join("server.ts"), "// fastify server").unwrap();

        let result = detect_framework(&app_dir);
        assert!(result.is_ok());
        let name = result.unwrap().name;
        assert_eq!(name, "fastify-raw");
    }
}
