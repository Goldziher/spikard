//! Python bindings for spikard
//!
//! This crate provides Python bindings using PyO3

use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use spikard_http::RouteMetadata;

/// Extract routes from a Python Spikard application instance (internal function)
///
/// This function is meant to be called from Rust code that has GIL access.
/// It's not exposed as a Python function.
pub fn extract_routes_from_app(
    py: Python<'_>,
    app: &Bound<'_, PyAny>,
) -> PyResult<Vec<RouteMetadata>> {
    // Call app.get_routes() to get the route list
    let routes_list = app.call_method0("get_routes")?;

    let mut routes = Vec::new();

    // Iterate over routes
    for route_obj in routes_list.downcast::<PyList>()?.iter() {
        let metadata = extract_route_metadata(py, &route_obj)?;
        routes.push(metadata);
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
            .import_bound("json")?
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
            .import_bound("json")?
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

/// Python module for spikard
#[pymodule]
fn _spikard(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(process, m)?)?;
    Ok(())
}
