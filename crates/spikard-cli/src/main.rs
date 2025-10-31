//! Spikard CLI
//!
//! Command-line interface that runs a Rust HTTP server with embedded Python

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use spikard_http::{Route, Server, ServerConfig};
use std::path::PathBuf;

// Access to the internal route extraction function from spikard-py
// The lib name is "_spikard" but we import it with the dependency name
use _spikard as spikard_py;

/// Spikard - High-performance HTTP framework with Rust core
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Run a Python application with the Rust HTTP server
    Run {
        /// Path to Python module (e.g., app.py)
        module: PathBuf,

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
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Run {
            module,
            host,
            port,
            workers,
            reload,
        } => {
            if reload {
                println!("⚠️  Auto-reload not yet implemented");
            }

            if workers > 1 {
                println!("⚠️  Multi-worker mode not yet implemented, using single worker");
            }

            run_server(module, host, port)?;
        }
    }

    Ok(())
}

fn run_server(module_path: PathBuf, host: String, port: u16) -> Result<()> {
    // Initialize Python interpreter
    Python::initialize();

    // Extract routes from Python module
    let routes_with_handlers = Python::attach(|py| -> PyResult<Vec<spikard_py::RouteWithHandler>> {
        // Add current directory's venv to sys.path if it exists
        let current_dir = std::env::current_dir().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to get current directory: {}", e))
        })?;

        // Add the spikard package to sys.path
        // The package is in packages/python relative to current directory
        let spikard_package_dir = current_dir.join("packages").join("python");
        if spikard_package_dir.exists() {
            tracing::debug!("Adding spikard package to sys.path: {}", spikard_package_dir.display());
            let sys = py.import("sys")?;
            let sys_path = sys.getattr("path")?;
            sys_path.call_method1("insert", (0, spikard_package_dir.to_string_lossy().as_ref()))?;
        } else {
            tracing::warn!("Could not find spikard package at {}", spikard_package_dir.display());
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
        spikard_py::extract_routes_from_app(py, &app)
    })?;

    // Build routes with handlers for the Axum router
    let routes: Vec<(Route, Py<PyAny>)> = routes_with_handlers
        .into_iter()
        .map(|rwh| {
            Route::from_metadata(rwh.metadata)
                .map(|route| (route, rwh.handler))
                .map_err(|e| anyhow::anyhow!("Failed to create route: {}", e))
        })
        .collect::<Result<Vec<_>>>()?;

    // Configure server
    let config = ServerConfig {
        host: host.clone(),
        port,
        workers: 1,
    };

    // Initialize logging
    Server::init_logging();

    tracing::info!("Starting Spikard server");
    tracing::info!("Module: {}", module_path.display());
    tracing::info!("Registered {} routes", routes.len());
    tracing::info!("Listening on http://{}:{}", host, port);

    // Build Axum router with Python handlers
    let app = Server::with_python_handlers(config.clone(), routes)
        .map_err(|e| anyhow::anyhow!("Failed to build router: {}", e))?;

    // Run server
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?
        .block_on(async {
            let addr = format!("{}:{}", config.host, config.port);
            let socket_addr: std::net::SocketAddr = addr
                .parse()
                .map_err(|e| anyhow::anyhow!("Invalid socket address: {}", e))?;
            let listener = tokio::net::TcpListener::bind(socket_addr)
                .await
                .context("Failed to bind to address")?;

            tracing::info!("Server listening on {}", socket_addr);

            axum::serve(listener, app).await.context("Server error")
        })?;

    Ok(())
}
