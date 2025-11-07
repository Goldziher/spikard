//! Spikard CLI
//!
//! Unified command-line interface for running Spikard applications
//! across multiple language bindings (Rust, Python, Node.js, Ruby)

use anyhow::{Context, Result, bail};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Spikard - High-performance HTTP framework with Rust core
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a Spikard application
    Run {
        /// Path to application file (e.g., server.py, server.js, server.rb)
        /// Language is auto-detected from file extension
        file: PathBuf,

        /// Explicitly specify the language/runtime
        #[arg(long, short = 'l')]
        lang: Option<Language>,

        /// Host to bind to
        #[arg(long, default_value = "127.0.0.1")]
        host: String,

        /// Port to bind to
        #[arg(long, default_value = "8000")]
        port: u16,

        /// Number of worker processes
        #[arg(long, default_value = "1")]
        workers: usize,

        /// Enable auto-reload on code changes
        #[arg(long)]
        reload: bool,
    },
    /// Show which language runtimes are compiled into this binary
    Features,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum Language {
    Rust,
    Python,
    Node,
    Ruby,
}

impl Language {
    fn from_extension(ext: &str) -> Option<Self> {
        match ext {
            "rs" => Some(Language::Rust),
            "py" => Some(Language::Python),
            "js" | "ts" | "mjs" | "cjs" => Some(Language::Node),
            "rb" => Some(Language::Ruby),
            _ => None,
        }
    }

    fn is_supported(self) -> bool {
        match self {
            Language::Rust => true,
            #[cfg(feature = "python")]
            Language::Python => true,
            #[cfg(not(feature = "python"))]
            Language::Python => false,
            #[cfg(feature = "node")]
            Language::Node => true,
            #[cfg(not(feature = "node"))]
            Language::Node => false,
            #[cfg(feature = "ruby")]
            Language::Ruby => true,
            #[cfg(not(feature = "ruby"))]
            Language::Ruby => false,
        }
    }

    fn name(self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::Node => "node",
            Language::Ruby => "ruby",
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Features => {
            println!("Spikard CLI - Compiled Language Support\n");
            println!("Rust:   ✓ (always available)");
            #[cfg(feature = "python")]
            println!("Python: ✓");
            #[cfg(not(feature = "python"))]
            println!("Python: ✗ (rebuild with --features python)");
            #[cfg(feature = "node")]
            println!("Node:   ✓");
            #[cfg(not(feature = "node"))]
            println!("Node:   ✗ (rebuild with --features node)");
            #[cfg(feature = "ruby")]
            println!("Ruby:   ✓");
            #[cfg(not(feature = "ruby"))]
            println!("Ruby:   ✗ (rebuild with --features ruby)");
        }
        Commands::Run {
            file,
            lang,
            host,
            port,
            workers,
            reload,
        } => {
            // Detect language from file extension or explicit flag
            let detected_lang = lang.or_else(|| {
                file.extension()
                    .and_then(|ext| ext.to_str())
                    .and_then(Language::from_extension)
            });

            let language = detected_lang.ok_or_else(|| {
                anyhow::anyhow!("Could not determine language from file extension. Use --lang to specify explicitly.")
            })?;

            // Check if language is supported in this build
            if !language.is_supported() {
                bail!(
                    "Language '{}' is not supported in this build.\n\
                     Rebuild with: cargo build --release -p spikard-cli --features {}",
                    language.name(),
                    language.name()
                );
            }

            if reload {
                println!("⚠️  Auto-reload not yet implemented");
            }

            if workers > 1 {
                println!("⚠️  Multi-worker mode not yet implemented, using single worker");
            }

            match language {
                Language::Rust => run_rust_server(file, host, port)?,
                #[cfg(feature = "python")]
                Language::Python => run_python_server(file, host, port)?,
                #[cfg(feature = "node")]
                Language::Node => run_node_server(file, host, port)?,
                #[cfg(feature = "ruby")]
                Language::Ruby => run_ruby_server(file, host, port)?,
                _ => unreachable!("Unsupported language should have been caught earlier"),
            }
        }
    }

    Ok(())
}

// ============================================================================
// Rust server (always available)
// ============================================================================

fn run_rust_server(_file: PathBuf, _host: String, _port: u16) -> Result<()> {
    bail!(
        "Rust server support not yet implemented.\n\
           For now, use the spikard-http crate directly to build a Rust server."
    );
}

// ============================================================================
// Python server (requires 'python' feature)
// ============================================================================

#[cfg(feature = "python")]
fn run_python_server(module_path: PathBuf, host: String, port: u16) -> Result<()> {
    use spikard_http::{Route, Server, ServerConfig};
    use std::sync::Arc;

    // Initialize Python interpreter
    Python::initialize();

    // Initialize Python event loop for async handlers (now in spikard-py)
    _spikard::init_python_event_loop().context("Failed to initialize Python event loop")?;

    // Extract routes from Python module
    let routes_with_handlers = Python::attach(|py| -> PyResult<Vec<_spikard::RouteWithHandler>> {
        // Add current directory's venv to sys.path if it exists
        let current_dir = std::env::current_dir().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get current directory: {}", e))
        })?;

        // Try to find and add virtual environment site-packages
        let venv_paths = [
            current_dir.join(".venv").join("lib"),
            std::env::var("VIRTUAL_ENV")
                .ok()
                .map(PathBuf::from)
                .unwrap_or_default()
                .join("lib"),
        ];

        let sys = py.import("sys")?;
        let sys_path = sys.getattr("path")?;

        // Add site-packages from venv
        for venv_lib in &venv_paths {
            if venv_lib.exists() {
                // Find pythonX.Y directory
                if let Ok(entries) = std::fs::read_dir(venv_lib) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.is_dir() {
                            let site_packages = path.join("site-packages");
                            if site_packages.exists() {
                                tracing::debug!("Adding to sys.path: {}", site_packages.display());
                                sys_path.call_method1("insert", (0, site_packages.to_string_lossy().as_ref()))?;
                            }
                        }
                    }
                }
            }
        }

        // Add the spikard package to sys.path (for development)
        // Try to find workspace root by walking up directories
        let spikard_package_dir = {
            let mut search_dir = current_dir.clone();
            let mut found = None;
            for _ in 0..10 {
                let candidate = search_dir.join("packages").join("python");
                if candidate.exists() {
                    found = Some(candidate);
                    break;
                }
                if let Some(parent) = search_dir.parent() {
                    search_dir = parent.to_path_buf();
                } else {
                    break;
                }
            }
            found.or_else(|| {
                let fallback = current_dir.join("packages").join("python");
                if fallback.exists() { Some(fallback) } else { None }
            })
        };

        if let Some(pkg_dir) = spikard_package_dir {
            tracing::debug!("Adding spikard package to sys.path: {}", pkg_dir.display());
            sys_path.call_method1("insert", (0, pkg_dir.to_string_lossy().as_ref()))?;
        }

        // Also add target/release to sys.path so Python can find _spikard extension module
        let mut search_dir = current_dir.clone();
        for _ in 0..10 {
            let target_release = search_dir.join("target").join("release");
            if target_release.exists() {
                tracing::debug!("Adding target/release to sys.path: {}", target_release.display());
                sys_path.call_method1("insert", (0, target_release.to_string_lossy().as_ref()))?;
                break;
            }
            if let Some(parent) = search_dir.parent() {
                search_dir = parent.to_path_buf();
            } else {
                break;
            }
        }

        // Get absolute path to module
        let abs_path = module_path.canonicalize().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to resolve module path: {}", e))
        })?;

        let abs_path_str = abs_path
            .to_str()
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Module path contains invalid UTF-8"))?;

        let module_name = abs_path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>("Invalid module name"))?;

        tracing::debug!("Loading Python module: {} from {}", module_name, abs_path_str);

        // Use importlib to load module from file path
        let importlib_util = py.import("importlib.util")?;

        // Create module spec from file location
        let spec = importlib_util.call_method1("spec_from_file_location", (module_name, abs_path_str))?;

        // Create module from spec
        let module_from_spec = importlib_util.getattr("module_from_spec")?;
        let user_module = module_from_spec.call1((&spec,))?;

        // Execute the module
        let spec_loader = spec.getattr("loader")?;
        spec_loader.call_method1("exec_module", (&user_module,))?;

        tracing::debug!("Module loaded successfully");

        // Find the Spikard app instance
        let app = user_module.getattr("app")?;

        tracing::debug!("Found app instance, extracting routes");

        // Extract routes with handlers from the app
        _spikard::extract_routes_from_app(py, &app)
    })?;

    // Build routes with handlers for the Axum router
    // Wrap each Python handler in PythonHandler and Arc<dyn Handler>
    let routes: Vec<(Route, Arc<dyn spikard_http::Handler>)> = routes_with_handlers
        .into_iter()
        .map(|rwh| {
            let path = rwh.metadata.path.clone();
            Route::from_metadata(rwh.metadata.clone())
                .and_then(|route| {
                    // Create PythonHandler with validators from route
                    let python_handler = _spikard::PythonHandler::new(
                        rwh.handler,
                        rwh.metadata.is_async,
                        route.request_validator.clone(),
                        route.response_validator.clone(),
                        route.parameter_validator.clone(),
                    );
                    // Wrap in Arc<dyn Handler>
                    let arc_handler: Arc<dyn spikard_http::Handler> = Arc::new(python_handler);
                    Ok((route, arc_handler))
                })
                .map_err(|e| anyhow::anyhow!("Failed to create route for {}: {}", path, e))
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Configure server
    let config = ServerConfig {
        host: host.clone(),
        port,
        workers: 1,
    };

    // Initialize logging
    Server::init_logging();

    tracing::info!("Starting Spikard Python server");
    tracing::info!("Module: {}", module_path.display());
    tracing::info!("Registered {} routes", routes.len());
    tracing::info!("Listening on http://{}:{}", host, port);

    // Build Axum router with Python handlers (using new with_handlers method)
    let app = Server::with_handlers(config.clone(), routes)
        .map_err(|e| anyhow::anyhow!("Failed to build Axum router with handlers: {}", e))?;

    // Run server
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?
        .block_on(async {
            let addr = format!("{}:{}", config.host, config.port);
            let socket_addr: std::net::SocketAddr = addr
                .parse()
                .with_context(|| format!("Invalid socket address: {}", addr))?;
            let listener = tokio::net::TcpListener::bind(socket_addr)
                .await
                .with_context(|| format!("Failed to bind to {}:{}", config.host, config.port))?;

            tracing::info!("Server listening on {}", socket_addr);

            axum::serve(listener, app).await.context("Server unexpectedly stopped")
        })?;

    Ok(())
}

// ============================================================================
// Node.js server (requires 'node' feature)
// ============================================================================

#[cfg(feature = "node")]
fn run_node_server(_file: PathBuf, _host: String, _port: u16) -> Result<()> {
    bail!("Node.js server support not yet implemented");
}

// ============================================================================
// Ruby server (requires 'ruby' feature)
// ============================================================================

#[cfg(feature = "ruby")]
fn run_ruby_server(_file: PathBuf, _host: String, _port: u16) -> Result<()> {
    bail!("Ruby server support not yet implemented");
}
