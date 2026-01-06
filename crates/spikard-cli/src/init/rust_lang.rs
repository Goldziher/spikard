//! Rust Project Scaffolder
//!
//! Generates a minimal Rust project structure with Spikard integration.
//! Creates both library and binary targets with integration tests.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// Rust project scaffolder
pub struct RustScaffolder;

impl ProjectScaffolder for RustScaffolder {
    #[allow(clippy::vec_init_then_push)]
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let kebab_name = Self::to_kebab_case(project_name);
        let mut files = Vec::new();

        // Create Cargo.toml
        files.push(ScaffoldedFile::new(
            PathBuf::from("Cargo.toml"),
            self.generate_cargo_toml(&kebab_name),
        ));

        // Create Cargo.lock (empty placeholder)
        files.push(ScaffoldedFile::new(PathBuf::from("Cargo.lock"), String::new()));

        // Create src/main.rs
        files.push(ScaffoldedFile::new(
            PathBuf::from("src/main.rs"),
            self.generate_main_rs(),
        ));

        // Create src/lib.rs
        files.push(ScaffoldedFile::new(PathBuf::from("src/lib.rs"), self.generate_lib_rs()));

        // Create tests/integration_test.rs
        files.push(ScaffoldedFile::new(
            PathBuf::from("tests/integration_test.rs"),
            self.generate_integration_test(),
        ));

        // Create .gitignore
        files.push(ScaffoldedFile::new(
            PathBuf::from(".gitignore"),
            self.generate_gitignore(),
        ));

        // Create README.md
        files.push(ScaffoldedFile::new(
            PathBuf::from("README.md"),
            self.generate_readme(project_name, &kebab_name),
        ));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        vec![
            format!("cd {}", project_name),
            "cargo build".to_string(),
            "cargo run".to_string(),
        ]
    }
}

impl RustScaffolder {
    /// Convert a project name to kebab-case (for crate names)
    fn to_kebab_case(name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_uppercase() {
                    format!("-{}", c.to_lowercase())
                } else if c == '_' {
                    "-".to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>()
            .trim_start_matches('-')
            .to_string()
    }

    fn generate_cargo_toml(&self, kebab_name: &str) -> String {
        format!(
            r#"[package]
name = "{kebab_name}"
version = "0.1.0"
edition = "2024"
rust-version = "1.75"
authors = ["Your Name <you@example.com>"]
license = "MIT"
description = "A Spikard-powered HTTP application"
repository = "https://github.com/yourusername/{kebab_name}"

[dependencies]
spikard-http = "0.6"
tokio = {{ version = "1", features = ["full"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"

[dev-dependencies]
"#
        )
    }

    fn generate_main_rs(&self) -> String {
        r#"//! Main HTTP server entry point
//!
//! This is the binary target for running the Spikard HTTP server.

use spikard_http::{Handler, Route, Router, Server, ServerConfig};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Create a simple router
    let mut router = Router::new();

    // Register routes here
    // Example:
    // router.register_route(Route::new(
    //     "/".to_string(),
    //     spikard_http::Method::Get,
    //     Arc::new(my_handler),
    // ));

    // Create server configuration
    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(8000)
        .enable_http_trace(true)
        .build();

    // Create and start the HTTP server
    let server = Server::new(config, router);

    println!("Server starting on http://127.0.0.1:8000");
    println!("Press Ctrl+C to stop");

    server.run().await?;

    Ok(())
}
"#
        .to_string()
    }

    fn generate_lib_rs(&self) -> String {
        r#"//! Spikard HTTP Application Library
//!
//! This library contains the core logic for the HTTP application.
//! The binary in `main.rs` uses this library to run the server.

/// Health check handler
///
/// Returns a simple JSON response indicating the server is healthy.
pub async fn health_handler() -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    Ok(serde_json::json!({
        "status": "healthy",
        "message": "Server is running"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_health_handler() {
        let result = health_handler().await;
        assert!(result.is_ok());

        let value = result.unwrap();
        assert_eq!(value["status"], "healthy");
    }
}
"#
        .to_string()
    }

    fn generate_integration_test(&self) -> String {
        r"//! Integration tests
//!
//! Tests that verify the HTTP server and handlers work correctly.

#[test]
fn test_server_initialization() {
    // Integration test example
    // This would typically test the full HTTP stack
    assert!(true);
}

#[tokio::test]
async fn test_health_endpoint() {
    // You would start a test server here and make real HTTP requests
    // Example using a test client
    assert!(true);
}
"
        .to_string()
    }

    fn generate_gitignore(&self) -> String {
        r"# Rust build artifacts
/target/
Cargo.lock

# IDE
.vscode/
.idea/
*.swp
*.swo
*~
*.rs.bk

# Environment
.env
.env.local

# OS
.DS_Store
Thumbs.db

# Testing
*.profdata
"
        .to_string()
    }

    fn generate_readme(&self, project_name: &str, kebab_name: &str) -> String {
        format!(
            r"# {project_name}

A Rust HTTP server powered by Spikard.

## Requirements

- Rust 1.75+

## Getting Started

### Build

```bash
cargo build --release
```

### Run

```bash
cargo run
```

The server will start on `http://127.0.0.1:8000`.

### Test

```bash
cargo test
```

## Running the Binary

```bash
cargo run --release
```

## Project Structure

```
{kebab_name}/
├── src/
│   ├── main.rs      # Binary entry point
│   └── lib.rs       # Library code
├── tests/
│   └── integration_test.rs
├── Cargo.toml       # Project manifest
└── README.md
```

## Development

### Format Code

```bash
cargo fmt
```

### Lint

```bash
cargo clippy -- -D warnings
```

## Next Steps

1. Update `src/main.rs` to define your HTTP handlers
2. Implement logic in `src/lib.rs`
3. Add tests in `tests/integration_test.rs`
4. Build and run with `cargo run`

## Documentation

- [Spikard Documentation](https://spikard.dev)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_kebab_case() {
        assert_eq!(RustScaffolder::to_kebab_case("MyProject"), "my-project");
        assert_eq!(RustScaffolder::to_kebab_case("my_project"), "my-project");
        assert_eq!(RustScaffolder::to_kebab_case("myProject"), "my-project");
        assert_eq!(RustScaffolder::to_kebab_case("my-project"), "my-project");
    }

    #[test]
    fn test_rust_scaffolder_generates_cargo_toml() {
        let scaffolder = RustScaffolder;
        let content = scaffolder.generate_cargo_toml("my-project");

        assert!(content.contains("name = \"my-project\""));
        assert!(content.contains("edition = \"2024\""));
        assert!(content.contains("spikard-http"));
        assert!(content.contains("tokio"));
    }

    #[test]
    fn test_rust_scaffolder_generates_main_rs() {
        let scaffolder = RustScaffolder;
        let content = scaffolder.generate_main_rs();

        assert!(content.contains("#[tokio::main]"));
        assert!(content.contains("async fn main()"));
        assert!(content.contains("Server::new"));
        assert!(content.contains("127.0.0.1"));
        assert!(content.contains("8000"));
    }

    #[test]
    fn test_rust_scaffolder_generates_lib_rs() {
        let scaffolder = RustScaffolder;
        let content = scaffolder.generate_lib_rs();

        assert!(content.contains("health_handler"));
        assert!(content.contains("async fn health_handler"));
        assert!(content.contains("#[tokio::test]"));
    }

    #[test]
    fn test_rust_scaffolder_next_steps() {
        let scaffolder = RustScaffolder;
        let steps = scaffolder.next_steps("my-project");

        assert_eq!(steps.len(), 3);
        assert!(steps[0].contains("cd my-project"));
        assert_eq!(steps[1], "cargo build");
        assert_eq!(steps[2], "cargo run");
    }
}
