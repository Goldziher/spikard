//! Python bindings for spikard
//!
//! This crate provides Python bindings using PyO3

pub mod handler;
mod response;
mod test_client;

use pyo3::prelude::*;

// Export handler for use in CLI and server
pub use handler::{PythonHandler, init_python_event_loop};
use pyo3::types::PyList;
use spikard_http::RouteMetadata;
use spikard_http::server::Server;

/// Route with Python handler
pub struct RouteWithHandler {
    pub metadata: RouteMetadata,
    pub handler: Py<PyAny>,
}

/// Extract routes from a Python Spikard application instance (internal function)
///
/// This function is meant to be called from Rust code that has GIL access.
/// It's not exposed as a Python function.
pub fn extract_routes_from_app(py: Python<'_>, app: &Bound<'_, PyAny>) -> PyResult<Vec<RouteWithHandler>> {
    // Call app.get_routes() to get the route list
    let routes_list = app.call_method0("get_routes")?;

    let mut routes = Vec::new();

    // Iterate over routes
    for route_obj in routes_list.cast::<PyList>()?.iter() {
        let metadata = extract_route_metadata(py, &route_obj)?;

        // Get the handler function
        let handler: Py<PyAny> = route_obj.getattr("handler")?.into();

        routes.push(RouteWithHandler { metadata, handler });
    }

    Ok(routes)
}

/// Extract route metadata from a Python Route object
fn extract_route_metadata(py: Python<'_>, route: &Bound<'_, PyAny>) -> PyResult<RouteMetadata> {
    let method: String = route.getattr("method")?.extract()?;
    let path: String = route.getattr("path")?.extract()?;
    let handler_name: String = route.getattr("handler_name")?.extract()?;
    let is_async: bool = route.getattr("is_async")?.extract()?;

    // Extract schemas (can be None)
    let request_schema = route.getattr("request_schema")?;
    let request_schema_value = if request_schema.is_none() {
        None
    } else {
        let json_str: String = py.import("json")?.call_method1("dumps", (request_schema,))?.extract()?;
        Some(serde_json::from_str(&json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse request schema: {}", e))
        })?)
    };

    let response_schema = route.getattr("response_schema")?;
    let response_schema_value = if response_schema.is_none() {
        None
    } else {
        let json_str: String = py
            .import("json")?
            .call_method1("dumps", (response_schema,))?
            .extract()?;
        Some(serde_json::from_str(&json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse response schema: {}", e))
        })?)
    };

    let parameter_schema = route.getattr("parameter_schema")?;
    let parameter_schema_value = if parameter_schema.is_none() {
        None
    } else {
        let json_str: String = py
            .import("json")?
            .call_method1("dumps", (parameter_schema,))?
            .extract()?;
        Some(serde_json::from_str(&json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse parameter schema: {}", e))
        })?)
    };

    let file_params = route.getattr("file_params")?;
    let file_params_value = if file_params.is_none() {
        None
    } else {
        let json_str: String = py.import("json")?.call_method1("dumps", (file_params,))?.extract()?;
        Some(serde_json::from_str(&json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to parse file params: {}", e))
        })?)
    };

    Ok(RouteMetadata {
        method,
        path,
        handler_name,
        request_schema: request_schema_value,
        response_schema: response_schema_value,
        parameter_schema: parameter_schema_value,
        file_params: file_params_value,
        is_async,
        cors: None,
    })
}

/// Process using spikard (legacy function)
#[pyfunction]
fn process() -> PyResult<()> {
    spikard::process().map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Spikard error: {}", e)))
}

/// Create a test client from a Spikard application
///
/// Args:
///     app: A Spikard application instance
///
/// Returns:
///     TestClient: A test client for making requests to the app
#[pyfunction]
fn create_test_client(py: Python<'_>, app: &Bound<'_, PyAny>) -> PyResult<test_client::TestClient> {
    // DEBUG: Log test client creation
    let _ = std::fs::write("/tmp/create_test_client.log", "create_test_client() called\n");
    eprintln!("[UNCONDITIONAL DEBUG] create_test_client() called");

    // Initialize debug logging
    spikard_http::debug::init();

    // Extract routes from the Python app
    let routes_with_handlers = extract_routes_from_app(py, app)?;
    let _ = std::fs::write(
        "/tmp/routes_extracted.log",
        format!("Extracted {} routes\n", routes_with_handlers.len()),
    );

    // Create schema registry for deduplication across all routes
    let schema_registry = spikard_http::SchemaRegistry::new();

    // Convert to Route + Py<PyAny> pairs
    // Use Route::from_metadata() to enable type hint parsing and auto-generation
    let routes: Vec<_> = routes_with_handlers
        .into_iter()
        .filter_map(|r| {
            let has_explicit_parameter_schema = r.metadata.parameter_schema.is_some();
            eprintln!(
                "[UNCONDITIONAL DEBUG] Route: {} {} has_explicit_parameter_schema={}",
                r.metadata.method, r.metadata.path, has_explicit_parameter_schema
            );

            // Use Route::from_metadata() which handles type hint parsing and auto-generation
            // Pass the registry to enable schema deduplication
            match spikard_http::Route::from_metadata(r.metadata, &schema_registry) {
                Ok(route) => Some((route, r.handler)),
                Err(e) => {
                    eprintln!("[UNCONDITIONAL DEBUG] Failed to create route: {}", e);
                    None
                }
            }
        })
        .collect();

    let _ = std::fs::write(
        "/tmp/routes_converted.log",
        format!("Converted {} routes\n", routes.len()),
    );

    // Create server config (not used for test client, but needed for API)
    let config = spikard_http::ServerConfig::default();

    // Build Axum router with Python handlers
    eprintln!(
        "[UNCONDITIONAL DEBUG] Building Axum router with {} routes",
        routes.len()
    );

    // Wrap each Python handler in PythonHandler and Arc<dyn Handler>
    let handler_routes: Vec<(spikard_http::Route, std::sync::Arc<dyn spikard_http::Handler>)> = routes
        .into_iter()
        .map(|(route, py_handler)| {
            let python_handler = PythonHandler::new(
                py_handler,
                route.is_async,
                route.request_validator.clone(),
                route.response_validator.clone(),
                route.parameter_validator.clone(),
            );
            let arc_handler: std::sync::Arc<dyn spikard_http::Handler> = std::sync::Arc::new(python_handler);
            (route, arc_handler)
        })
        .collect();

    let axum_router = Server::with_handlers(config, handler_routes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build router: {}", e)))?;

    let _ = std::fs::write("/tmp/axum_router_built.log", "Axum router built successfully\n");

    // Create test client from the router
    eprintln!("[UNCONDITIONAL DEBUG] Creating TestClient from Axum router");

    let client = test_client::TestClient::from_router(axum_router)?;
    let _ = std::fs::write("/tmp/test_client_created.log", "TestClient created successfully\n");

    Ok(client)
}

/// Run Spikard server from Python
///
/// This function enables Python to run Spikard, rather than having the Rust CLI embed Python.
/// This allows Python to manage its own event loop, enabling natural async/await support.
///
/// Args:
///     app: Spikard application instance
///     host: Host to bind to (default: "127.0.0.1")
///     port: Port to bind to (default: 8000)
///     workers: Number of workers (default: 1)
///
/// Example:
///     ```python
///     from spikard import Spikard
///
///     app = Spikard()
///
///     @app.get("/")
///     async def root():
///         return {"message": "Hello"}
///
///     if __name__ == "__main__":
///         app.run(host="0.0.0.0", port=8000)
///     ```
#[pyfunction]
#[pyo3(signature = (app, host="127.0.0.1".to_string(), port=8000, workers=1))]
fn run_server(py: Python<'_>, app: &Bound<'_, PyAny>, host: String, port: u16, workers: usize) -> PyResult<()> {
    use spikard_http::{Route, Server, ServerConfig};
    use std::sync::Arc;

    if workers > 1 {
        eprintln!("⚠️  Multi-worker mode not yet implemented, using single worker");
    }

    // Install uvloop if available (Python manages event loop)
    init_python_event_loop()?;

    // Extract routes from the Python app
    let routes_with_handlers = extract_routes_from_app(py, app)?;

    // Create schema registry for deduplication across all routes
    let schema_registry = spikard_http::SchemaRegistry::new();

    // Build routes with handlers for the Axum router
    // Wrap each Python handler in PythonHandler and Arc<dyn Handler>
    let routes: Vec<(Route, Arc<dyn spikard_http::Handler>)> = routes_with_handlers
        .into_iter()
        .map(|rwh| {
            let path = rwh.metadata.path.clone();
            Route::from_metadata(rwh.metadata.clone(), &schema_registry)
                .map(|route| {
                    // Create PythonHandler with validators from route
                    let python_handler = PythonHandler::new(
                        rwh.handler,
                        rwh.metadata.is_async,
                        route.request_validator.clone(),
                        route.response_validator.clone(),
                        route.parameter_validator.clone(),
                    );
                    // Wrap in Arc<dyn Handler>
                    let arc_handler: Arc<dyn spikard_http::Handler> = Arc::new(python_handler);
                    (route, arc_handler)
                })
                .map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to create route for {}: {}",
                        path, e
                    ))
                })
        })
        .collect::<Result<Vec<_>, _>>()?;

    // Configure server
    let config = ServerConfig {
        host: host.clone(),
        port,
        ..Default::default()
    };

    // Initialize logging
    Server::init_logging();

    eprintln!("[spikard] Starting Spikard server (Python manages event loop)");
    eprintln!("[spikard] Registered {} routes", routes.len());
    eprintln!("[spikard] Listening on http://{}:{}", host, port);

    // Build Axum router with Python handlers
    let app_router = Server::with_handlers(config.clone(), routes).map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build Axum router: {}", e))
    })?;

    // GIL is released when py goes out of scope at end of function
    // Run server in Tokio runtime
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|e| {
            pyo3::Python::attach(|_py| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to create Tokio runtime: {}", e))
            })
        })?
        .block_on(async {
            let addr = format!("{}:{}", config.host, config.port);
            let socket_addr: std::net::SocketAddr = addr.parse().map_err(|e| {
                pyo3::Python::attach(|_py| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid socket address {}: {}", addr, e))
                })
            })?;

            let listener = tokio::net::TcpListener::bind(socket_addr).await.map_err(|e| {
                pyo3::Python::attach(|_py| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
                        "Failed to bind to {}:{}: {}",
                        config.host, config.port, e
                    ))
                })
            })?;

            eprintln!("[spikard] Server listening on {}", socket_addr);

            axum::serve(listener, app_router).await.map_err(|e| {
                pyo3::Python::attach(|_py| {
                    PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Server error: {}", e))
                })
            })
        })
}

/// Python module for spikard
#[pymodule]
fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<response::Response>()?;
    m.add_class::<test_client::TestClient>()?;
    m.add_class::<test_client::TestResponse>()?;
    m.add_function(wrap_pyfunction!(create_test_client, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    m.add_function(wrap_pyfunction!(run_server, m)?)?;
    Ok(())
}
