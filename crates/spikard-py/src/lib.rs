//! Python bindings for spikard
//!
//! This crate provides Python bindings using PyO3

mod test_client;

use pyo3::prelude::*;
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
pub fn extract_routes_from_app(
    py: Python<'_>,
    app: &Bound<'_, PyAny>,
) -> PyResult<Vec<RouteWithHandler>> {
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
        let json_str: String = py
            .import("json")?
            .call_method1("dumps", (request_schema,))?
            .extract()?;
        Some(serde_json::from_str(&json_str).map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse request schema: {}",
                e
            ))
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
            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                "Failed to parse response schema: {}",
                e
            ))
        })?)
    };

    Ok(RouteMetadata {
        method,
        path,
        handler_name,
        request_schema: request_schema_value,
        response_schema: response_schema_value,
        is_async,
    })
}

/// Process using spikard (legacy function)
#[pyfunction]
fn process() -> PyResult<()> {
    spikard::process().map_err(|e| {
        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Spikard error: {}", e))
    })
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
    // Extract routes from the Python app
    let routes_with_handlers = extract_routes_from_app(py, app)?;

    // Convert to Route + Py<PyAny> pairs
    let routes: Vec<_> = routes_with_handlers
        .into_iter()
        .map(|r| {
            let route = spikard_http::Route {
                method: r.metadata.method.parse().unwrap_or(spikard_http::Method::Get),
                path: r.metadata.path,
                handler_name: r.metadata.handler_name,
                request_validator: r.metadata.request_schema.and_then(|schema| {
                    spikard_http::SchemaValidator::new(schema).ok()
                }),
                response_validator: r.metadata.response_schema.and_then(|schema| {
                    spikard_http::SchemaValidator::new(schema).ok()
                }),
                is_async: r.metadata.is_async,
            };
            (route, r.handler)
        })
        .collect();

    // Create server config (not used for test client, but needed for API)
    let config = spikard_http::ServerConfig::default();

    // Build Axum router with Python handlers
    let axum_router = Server::with_python_handlers(config, routes)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to build router: {}", e)))?;

    // Create test client from the router
    test_client::TestClient::from_router(axum_router)
}

/// Python module for spikard
#[pymodule]
fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<test_client::TestClient>()?;
    m.add_class::<test_client::TestResponse>()?;
    m.add_function(wrap_pyfunction!(create_test_client, m)?)?;
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
