//! Spikard CLI
//!
//! Command-line interface that runs a Rust HTTP server with embedded Python

use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use pyo3::prelude::*;
use spikard_http::{Route, Router, Server, ServerConfig};
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
    let (router, _handlers) = Python::attach(|py| -> PyResult<(Router, Vec<Py<PyAny>>)> {
        // Add module directory to sys.path
        let sys = py.import("sys")?;
        let sys_path = sys.getattr("path")?;
        let module_dir = module_path
            .parent()
            .context("Module path has no parent directory")
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;
        sys_path.call_method1("insert", (0, module_dir))?;

        // Import the user's module
        let module_name = module_path
            .file_stem()
            .and_then(|s| s.to_str())
            .context("Invalid module name")
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

        let user_module = py.import(module_name)?;

        // Find the Spikard app instance
        let app = user_module.getattr("app")?;

        // Extract routes directly from the app using spikard-py's internal function
        let route_metadata_list = spikard_py::extract_routes_from_app(py, &app)?;

        // Build router from metadata
        let mut router = Router::new();
        let mut handlers = Vec::new();

        for metadata in route_metadata_list {
            tracing::info!(
                "Registering route: {} {}",
                metadata.method,
                metadata.path
            );

            let route = Route::from_metadata(metadata)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))?;

            router.add_route(route);

            // TODO: Store Python handler references
            // handlers.push(handler.to_object(py));
        }

        Ok((router, handlers))
    })?;

    // Configure and start server
    let config = ServerConfig {
        host: host.clone(),
        port,
        workers: 1,
    };

    let server = Server::new(config, router);

    // Initialize logging
    Server::init_logging();

    tracing::info!("Starting Spikard server");
    tracing::info!("Module: {}", module_path.display());
    tracing::info!("Listening on http://{}:{}", host, port);

    // Run server
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .context("Failed to create Tokio runtime")?
        .block_on(async {
            server.serve().await
        })
        .map_err(|e| anyhow::anyhow!("Server error: {}", e))?;

    Ok(())
}
