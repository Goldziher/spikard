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
                Ok(route) => {
                    let has_parameter_validator = route.parameter_validator.is_some();
                    eprintln!(
                        "[UNCONDITIONAL DEBUG] Route after from_metadata: {} {} has_parameter_validator={}",
                        route.method.as_str(),
                        route.path,
                        has_parameter_validator
                    );
                    Some((route, r.handler))
                }
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

/// Python module for spikard
#[pymodule]
fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<response::Response>()?;
    m.add_class::<test_client::TestClient>()?;
    m.add_class::<test_client::TestResponse>()?;
    m.add_function(wrap_pyfunction!(create_test_client, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
