//! Python handler invocation from Rust

use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
};
use pyo3::prelude::*;
use serde_json::{json, Value};
use std::sync::Arc;

/// Python handler wrapper that can be called from Axum
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(handler: Py<PyAny>, is_async: bool) -> Self {
        Self {
            handler: Arc::new(handler),
            is_async,
        }
    }

    /// Call the Python handler
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(&self, _req: Request<Body>) -> Result<Response<Body>, (StatusCode, String)> {
        let handler = self.handler.clone();
        let is_async = self.is_async;

        // Run Python code in blocking task
        let result = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<Value> {
                let handler_obj = handler.bind(py);

                // Call the handler function
                let py_result = if is_async {
                    // For async handlers, we need to use pyo3-async-runtimes
                    // For now, just call it and warn
                    tracing::warn!("Async Python handlers not yet fully supported, calling synchronously");
                    handler_obj.call0()?
                } else {
                    handler_obj.call0()?
                };

                // Convert Python result to JSON
                python_to_json(py, &py_result)
            })
        })
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to spawn blocking task: {}", e),
            )
        })?;

        match result {
            Ok(json_value) => {
                let json_bytes = serde_json::to_vec(&json_value).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize response: {}", e),
                    )
                })?;

                Response::builder()
                    .status(StatusCode::OK)
                    .header("content-type", "application/json")
                    .body(Body::from(json_bytes))
                    .map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to build response: {}", e),
                        )
                    })
            }
            Err(e) => Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Python handler error: {}", e),
            )),
        }
    }
}

/// Convert Python object to JSON Value
fn python_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    // Use json.dumps to convert to JSON string, then parse
    let json_module = py.import("json")?;
    let json_str: String = json_module
        .call_method1("dumps", (obj,))?
        .extract()?;

    serde_json::from_str(&json_str).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to parse JSON: {}", e))
    })
}
