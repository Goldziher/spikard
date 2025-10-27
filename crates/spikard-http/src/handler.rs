//! Python handler invocation from Rust

use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;

/// Check if DEBUG mode is enabled
fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

/// Request data extracted and validated in Rust
#[derive(Debug, Clone)]
pub struct RequestData {
    pub path_params: HashMap<String, String>,
    pub query_params: HashMap<String, String>,
    pub body: Option<Value>,
}

/// Python handler wrapper that can be called from Axum
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    request_validator: Option<crate::SchemaValidator>,
    response_validator: Option<crate::SchemaValidator>,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        request_validator: Option<crate::SchemaValidator>,
        response_validator: Option<crate::SchemaValidator>,
    ) -> Self {
        Self {
            handler: Arc::new(handler),
            is_async,
            request_validator,
            response_validator,
        }
    }

    /// Call the Python handler
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Result<Response<Body>, (StatusCode, String)> {
        // Validate request body in Rust if validator is present
        if let Some(validator) = &self.request_validator {
            if let Some(body) = &request_data.body {
                if let Err(errors) = validator.validate(body) {
                    let error_msg = if is_debug_mode() {
                        // In DEBUG mode, include full validation errors and request data
                        json!({
                            "error": "Request validation failed",
                            "validation_errors": format!("{:?}", errors),
                            "request_body": body,
                            "path_params": request_data.path_params,
                            "query_params": request_data.query_params,
                        }).to_string()
                    } else {
                        format!("Request validation failed")
                    };
                    return Err((StatusCode::BAD_REQUEST, error_msg));
                }
            }
        }

        let handler = self.handler.clone();
        let is_async = self.is_async;
        let response_validator = self.response_validator.clone();
        let request_data_for_error = request_data.clone(); // Clone for error reporting

        let result = if is_async {
            // For async handlers, we need to await the coroutine
            // This must be done inside the blocking task
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<Value> {
                    let handler_obj = handler.bind(py);

                    // Convert all request data to Python kwargs
                    let kwargs = request_data_to_py_kwargs(py, &request_data)?;

                    // Call the handler - this returns a coroutine
                    let coroutine = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        // Call with empty args tuple and kwargs
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    };

                    // Check if it's actually a coroutine
                    if !coroutine.hasattr("__await__")? {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            "Handler marked as async but did not return a coroutine"
                        ));
                    }

                    // Use pyo3_async_runtimes to run the coroutine in the Python event loop
                    // We need to create an event loop if one doesn't exist
                    let asyncio = py.import("asyncio")?;

                    // Try to get the running loop, if it fails, create a new one
                    let loop_result = asyncio.call_method0("get_running_loop");
                    let py_result = if loop_result.is_err() {
                        // No event loop running, create one and run the coroutine
                        let new_loop = asyncio.call_method0("new_event_loop")?;
                        asyncio.call_method1("set_event_loop", (&new_loop,))?;

                        let result = new_loop.call_method1("run_until_complete", (coroutine,))?;

                        // Clean up
                        new_loop.call_method0("close")?;
                        asyncio.call_method1("set_event_loop", (py.None(),))?;

                        result
                    } else {
                        // Event loop already running, this shouldn't happen in our architecture
                        // but handle it gracefully
                        tracing::warn!("Event loop already running, creating task");
                        let create_task = asyncio.call_method1("create_task", (coroutine,))?;

                        // We need to await this somehow - for now just call run_until_complete
                        // on a new loop (this is a fallback)
                        let new_loop = asyncio.call_method0("new_event_loop")?;
                        new_loop.call_method1("run_until_complete", (create_task,))?
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
            })?
        } else {
            // For sync handlers, just call directly in blocking task
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<Value> {
                    let handler_obj = handler.bind(py);

                    // Convert all request data to Python kwargs
                    let kwargs = request_data_to_py_kwargs(py, &request_data)?;

                    let py_result = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        // Call with empty args tuple and kwargs
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    };
                    python_to_json(py, &py_result)
                })
            })
            .await
            .map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to spawn blocking task: {}", e),
                )
            })?
        };

        match result {
            Ok(json_value) => {
                // Validate response in Rust if validator is present
                if let Some(validator) = &response_validator {
                    if let Err(errors) = validator.validate(&json_value) {
                        let error_msg = if is_debug_mode() {
                            json!({
                                "error": "Response validation failed",
                                "validation_errors": format!("{:?}", errors),
                                "response_body": json_value,
                                "request_data": {
                                    "path_params": request_data_for_error.path_params,
                                    "query_params": request_data_for_error.query_params,
                                    "body": request_data_for_error.body,
                                }
                            }).to_string()
                        } else {
                            format!("Internal server error")
                        };
                        return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
                    }
                }

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
            Err(e) => {
                let error_msg = if is_debug_mode() {
                    // In DEBUG mode, include Python traceback
                    let traceback = Python::attach(|py| {
                        get_python_traceback(py, &e)
                    });

                    json!({
                        "error": "Python handler error",
                        "exception": format!("{}", e),
                        "traceback": traceback,
                        "request_data": {
                            "path_params": request_data_for_error.path_params,
                            "query_params": request_data_for_error.query_params,
                            "body": request_data_for_error.body,
                        }
                    }).to_string()
                } else {
                    format!("Internal server error")
                };
                Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg))
            }
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

/// Convert request data (path params, query params, body) to Python keyword arguments
fn request_data_to_py_kwargs<'py>(
    py: Python<'py>,
    request_data: &RequestData,
) -> PyResult<Bound<'py, PyDict>> {
    let kwargs = PyDict::new(py);

    // Add path parameters as individual kwargs
    for (key, value) in &request_data.path_params {
        // Try to parse as int first, fallback to string
        if let Ok(int_val) = value.parse::<i64>() {
            kwargs.set_item(key, int_val)?;
        } else if let Ok(float_val) = value.parse::<f64>() {
            kwargs.set_item(key, float_val)?;
        } else if value == "true" || value == "false" {
            let bool_val = value == "true";
            kwargs.set_item(key, bool_val)?;
        } else {
            kwargs.set_item(key, value)?;
        }
    }

    // Add query parameters as individual kwargs (or as a dict if they conflict with path params)
    for (key, value) in &request_data.query_params {
        // Only add if not already present (path params take precedence)
        if !kwargs.contains(key)? {
            if let Ok(int_val) = value.parse::<i64>() {
                kwargs.set_item(key, int_val)?;
            } else if let Ok(float_val) = value.parse::<f64>() {
                kwargs.set_item(key, float_val)?;
            } else if value == "true" || value == "false" {
                let bool_val = value == "true";
                kwargs.set_item(key, bool_val)?;
            } else {
                kwargs.set_item(key, value)?;
            }
        }
    }

    // Add request body if present (convert to Python dict/list)
    if let Some(body) = &request_data.body {
        // Convert JSON Value to Python object
        let py_body = json_to_python(py, body)?;
        kwargs.set_item("body", py_body)?;
    }

    Ok(kwargs)
}

/// Convert JSON Value to Python object
fn json_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    // Use json.loads to convert JSON string to Python object
    let json_module = py.import("json")?;
    let json_str = serde_json::to_string(value).map_err(|e| {
        pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize JSON: {}", e))
    })?;
    json_module.call_method1("loads", (json_str,))
}

/// Extract Python traceback from exception
fn get_python_traceback(py: Python<'_>, err: &PyErr) -> String {
    // Try to format the full Python traceback
    let traceback_module = match py.import("traceback") {
        Ok(module) => module,
        Err(_) => return format!("{}", err),
    };

    // Get the exception info
    let exc_type = err.get_type(py);
    let exc_value = err.value(py);
    let exc_traceback = err.traceback(py);

    // Format the traceback
    let formatted = match exc_traceback {
        Some(tb) => {
            // Use traceback.format_exception to get full traceback
            match traceback_module.call_method1("format_exception", (exc_type, exc_value, tb)) {
                Ok(lines) => {
                    if let Ok(list) = lines.extract::<Vec<String>>() {
                        list.join("")
                    } else {
                        format!("{}", err)
                    }
                }
                Err(_) => format!("{}", err),
            }
        }
        None => {
            // No traceback available, just format the exception
            match traceback_module.call_method1("format_exception_only", (exc_type, exc_value)) {
                Ok(lines) => {
                    if let Ok(list) = lines.extract::<Vec<String>>() {
                        list.join("")
                    } else {
                        format!("{}", err)
                    }
                }
                Err(_) => format!("{}", err),
            }
        }
    };

    formatted
}
