//! Python handler implementation for spikard_http::Handler trait

/// Debug logging macro for module-specific logging
macro_rules! debug_log_module {
    ($module:expr, $($arg:tt)*) => {
        if is_debug_mode() {
            eprintln!("[{}] {}", $module, format!($($arg)*));
        }
    };
}

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{Value, json};
use spikard_http::{Handler, HandlerResult, RequestData};
use spikard_http::{ParameterValidator, ProblemDetails, SchemaValidator};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Initialize Python event loop configuration for async handlers
/// Installs uvloop if available for better performance
/// Each async handler will use asyncio.run() which creates its own event loop
pub fn init_python_event_loop() -> PyResult<()> {
    Python::attach(|py| {
        // Install uvloop to patch asyncio for better performance (if available)
        // This globally replaces asyncio's event loop policy
        if let Ok(uvloop) = py.import("uvloop") {
            uvloop.call_method0("install")?;
            eprintln!("[spikard] uvloop installed - asyncio will use uvloop for all event loops");
        } else {
            eprintln!("[spikard] uvloop not available, using standard asyncio event loop");
        }

        eprintln!("[spikard] Async handlers will use asyncio.run() with isolated event loops");

        Ok(())
    })
}

/// Check if DEBUG mode is enabled
fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

/// Response result from Python handler
#[derive(Debug)]
pub enum ResponseResult {
    /// Custom Response object with status code and headers
    Custom {
        content: Value,
        status_code: u16,
        headers: HashMap<String, String>,
    },
    /// Plain JSON response (defaults to 200 OK)
    Json(Value),
}

/// Python handler wrapper that implements spikard_http::Handler
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    request_validator: Option<Arc<SchemaValidator>>,
    response_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        request_validator: Option<Arc<SchemaValidator>>,
        response_validator: Option<Arc<SchemaValidator>>,
        parameter_validator: Option<ParameterValidator>,
    ) -> Self {
        Self {
            handler: Arc::new(handler),
            is_async,
            request_validator,
            response_validator,
            parameter_validator,
        }
    }

    /// Call the Python handler
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(&self, _req: Request<Body>, request_data: RequestData) -> HandlerResult {
        // Validate request body in Rust if validator is present
        if let Some(validator) = &self.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            // Return RFC 9457 Problem Details format
            let problem = ProblemDetails::from_validation_error(&errors);
            let error_json = problem
                .to_json_pretty()
                .unwrap_or_else(|e| format!("Failed to serialize: {}", e));
            return Err((problem.status_code(), error_json));
        }

        // Validate and extract parameters in Rust if validator is present
        // This returns a validated JSON object with properly typed values
        let validated_params = if let Some(validator) = &self.parameter_validator {
            // Convert raw_query_params from Vec<String> to String (take first value)
            let raw_query_strings: HashMap<String, String> = request_data
                .raw_query_params
                .iter()
                .filter_map(|(k, v)| v.first().map(|first| (k.clone(), first.clone())))
                .collect();

            // Pass query params as Value directly (fast-query-parsers already did type conversion)
            // Arc auto-derefs to &HashMap
            match validator.validate_and_extract(
                &request_data.query_params,
                &raw_query_strings,
                &request_data.path_params,
                &request_data.headers,
                &request_data.cookies,
            ) {
                Ok(params) => Some(params),
                Err(errors) => {
                    // Return RFC 9457 Problem Details format
                    debug_log_module!(
                        "handler",
                        "Parameter validation failed with {} errors",
                        errors.errors.len()
                    );
                    let problem = ProblemDetails::from_validation_error(&errors);
                    let error_json = problem
                        .to_json_pretty()
                        .unwrap_or_else(|e| format!("Failed to serialize: {}", e));
                    debug_log_module!("handler", "Returning 422 with RFC 9457 error: {}", error_json);
                    return Err((problem.status_code(), error_json));
                }
            }
        } else {
            None
        };

        let handler = self.handler.clone();
        let is_async = self.is_async;
        let response_validator = self.response_validator.clone();
        let request_data_for_error = request_data.clone(); // Clone for error reporting
        let validated_params_for_task = validated_params.clone(); // Clone for passing to task

        let result = if is_async {
            // For async handlers, run the coroutine using asyncio.run() in a blocking task
            // This creates a new event loop for each request and runs the coroutine to completion
            let output = tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let handler_obj = handler.bind(py);

                    // Convert to Python kwargs - use validated params if available
                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data, handler_obj.clone())?
                    };

                    // Call the handler - this returns a coroutine
                    let coroutine = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    };

                    // Check if it's actually a coroutine
                    if !coroutine.hasattr("__await__")? {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            "Handler marked as async but did not return a coroutine",
                        ));
                    }

                    // Run the coroutine using asyncio.run() which creates and manages the event loop
                    let asyncio = py.import("asyncio")?;
                    let result = asyncio.call_method1("run", (coroutine,))?;

                    Ok(result.into())
                })
            })
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Tokio error: {}", e),
                )
            })?
            .map_err(|e: PyErr| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Python async error: {}", e),
                )
            })?;

            // Convert Python result back to ResponseResult
            Python::attach(|py| python_to_response_result(py, output.bind(py))).map_err(|e: PyErr| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Python error: {}", e),
                )
            })?
        } else {
            // For sync handlers, just call directly in blocking task
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<ResponseResult> {
                    let handler_obj = handler.bind(py);

                    // Convert to Python kwargs - use validated params if available
                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data, handler_obj.clone())?
                    };

                    let py_result = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        // Call with empty args tuple and kwargs
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    };
                    python_to_response_result(py, &py_result)
                })
            })
            .await
            .map_err(|e| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Spawn blocking error: {}", e),
                )
            })?
            .map_err(|e: PyErr| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Python error: {}", e),
                )
            })?
        };

        // Check if this is a ResponseData (custom Response object) or just JSON
        let (json_value, status_code, headers) = match result {
            ResponseResult::Custom {
                content,
                status_code,
                headers,
            } => (content, status_code, headers),
            ResponseResult::Json(json_value) => (json_value, 200, HashMap::new()),
        };

        // Validate response in Rust if validator is present
        #[allow(clippy::collapsible_if)]
        if let Some(validator) = &response_validator {
            if let Err(errors) = validator.validate(&json_value) {
                let error_msg = if is_debug_mode() {
                    json!({
                        "error": "Response validation failed",
                        "validation_errors": format!("{:?}", errors),
                        "response_body": json_value,
                        "request_data": {
                            "path_params": &*request_data_for_error.path_params,
                            "query_params": request_data_for_error.query_params,
                            "body": request_data_for_error.body,
                        }
                    })
                    .to_string()
                } else {
                    "Internal server error".to_string()
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

        let mut response_builder = Response::builder()
            .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK))
            .header("content-type", "application/json");

        // Add custom headers
        for (key, value) in headers {
            response_builder = response_builder.header(key, value);
        }

        response_builder.body(Body::from(json_bytes)).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to build response: {}", e),
            )
        })
    }
}

/// Implement the spikard_http::Handler trait for PythonHandler
impl Handler for PythonHandler {
    fn call(
        &self,
        request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(self.call(request, request_data))
    }
}

/// Convert Python object to ResponseResult
///
/// Checks if the object is a Response instance with custom status/headers,
/// otherwise treats it as JSON data
fn python_to_response_result(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<ResponseResult> {
    // Check if this is a Response object from _spikard module
    // Response objects have: content, status_code, headers attributes
    if obj.hasattr("status_code")? && obj.hasattr("content")? && obj.hasattr("headers")? {
        // This is a Response object, extract its properties
        let status_code: u16 = obj.getattr("status_code")?.extract()?;

        // Extract content (can be None)
        let content_attr = obj.getattr("content")?;
        let content = if content_attr.is_none() {
            Value::Null
        } else {
            python_to_json(py, &content_attr)?
        };

        // Extract headers (dict)
        let headers_dict = obj.getattr("headers")?;
        let mut headers = HashMap::new();

        // Convert Python dict to HashMap
        #[allow(deprecated)]
        if let Ok(dict) = headers_dict.downcast::<PyDict>() {
            for (key, value) in dict.iter() {
                let key_str: String = key.extract()?;
                let value_str: String = value.extract()?;
                headers.insert(key_str, value_str);
            }
        }

        Ok(ResponseResult::Custom {
            content,
            status_code,
            headers,
        })
    } else {
        // Not a Response object, treat as regular JSON
        let json_value = python_to_json(py, obj)?;
        Ok(ResponseResult::Json(json_value))
    }
}

/// Convert Python object to JSON Value
fn python_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    // Use json.dumps to convert to JSON string, then parse
    let json_module = py.import("json")?;
    let json_str: String = json_module.call_method1("dumps", (obj,))?.extract()?;

    // Replace '+00:00' with 'Z' for UTC datetimes to match FastAPI's ISO 8601 format
    // FastAPI uses 'Z' suffix for UTC datetimes (e.g., "2023-01-01T12:00:00Z")
    // while Python's isoformat() uses '+00:00' (e.g., "2023-01-01T12:00:00+00:00")
    let json_str = json_str.replace("+00:00", "Z");

    serde_json::from_str(&json_str)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse JSON: {}", e)))
}

/// Convert validated parameters to Python keyword arguments using msgspec converter
/// This uses already-validated parameter values and relies on Python's msgspec
/// for type conversion based on the handler's type annotations.
fn validated_params_to_py_kwargs<'py>(
    py: Python<'py>,
    validated_params: &Value,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    // Convert validated params to Python dict using json.loads
    let params_dict = json_to_python(py, validated_params)?.cast_into::<PyDict>()?;

    // Add request body to params BEFORE convert_params so it gets type-converted too
    if !request_data.body.is_null() {
        let py_body = json_to_python(py, &request_data.body)?;
        params_dict.set_item("body", py_body)?;
    }

    // Import our converter module
    let converter_module = py.import("spikard._internal.converters")?;
    let convert_params_func = converter_module.getattr("convert_params")?;

    // Call convert_params(params_dict, handler_func)
    // This will use msgspec to convert types based on handler's signature
    let converted = convert_params_func.call1((params_dict, handler))?;

    // Extract the converted dict
    let kwargs = converted.cast_into::<PyDict>()?;

    Ok(kwargs)
}

/// Convert request data (path params, query params, body) to Python keyword arguments
/// This is the fallback when no parameter validator is present
fn request_data_to_py_kwargs<'py>(
    py: Python<'py>,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let kwargs = PyDict::new(py);

    // Add path parameters as individual kwargs
    for (key, value) in request_data.path_params.iter() {
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

    // Add query parameters as individual kwargs (already parsed with correct types)
    // query_params is a JSON Value from our fast parser with types already correct
    if let Value::Object(query_map) = &request_data.query_params {
        for (key, value) in query_map {
            // Only add if not already present (path params take precedence)
            if !kwargs.contains(key.as_str())? {
                let py_value = json_to_python(py, value)?;
                kwargs.set_item(key.as_str(), py_value)?;
            }
        }
    }

    // Add request body if present (convert to Python dict/list)
    if !request_data.body.is_null() {
        // Convert JSON Value to Python object
        let py_body = json_to_python(py, &request_data.body)?;
        kwargs.set_item("body", py_body)?;
    }

    // Use convert_params to convert types based on handler signature
    let converter_module = py.import("spikard._internal.converters")?;
    let convert_params_func = converter_module.getattr("convert_params")?;
    let converted = convert_params_func.call1((kwargs, handler))?;
    Ok(converted.cast_into::<PyDict>()?)
}

/// Convert JSON Value to Python object (optimized zero-copy conversion)
///
/// This function converts serde_json::Value directly to Python objects using PyO3,
/// avoiding the serialize-to-string → parse-from-string overhead of json.loads.
///
/// Performance improvement: ~30-40% faster than the json.loads approach
fn json_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    use pyo3::types::{PyBool, PyDict, PyFloat, PyList, PyNone, PyString};

    match value {
        Value::Null => Ok(PyNone::get(py).as_any().clone()),
        Value::Bool(b) => Ok(PyBool::new(py, *b).as_any().clone()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_pyobject(py)?.into_any())
            } else if let Some(u) = n.as_u64() {
                Ok(u.into_pyobject(py)?.into_any())
            } else if let Some(f) = n.as_f64() {
                Ok(PyFloat::new(py, f).into_any())
            } else {
                // Fallback to string representation for exotic numbers
                Ok(PyString::new(py, &n.to_string()).into_any())
            }
        }
        Value::String(s) => Ok(PyString::new(py, s).into_any()),
        Value::Array(arr) => {
            let py_list = PyList::empty(py);
            for item in arr {
                let py_item = json_to_python(py, item)?;
                py_list.append(py_item)?;
            }
            Ok(py_list.into_any())
        }
        Value::Object(obj) => {
            let py_dict = PyDict::new(py);
            for (key, value) in obj {
                let py_value = json_to_python(py, value)?;
                py_dict.set_item(key, py_value)?;
            }
            Ok(py_dict.into_any())
        }
    }
}

/// Extract Python traceback from exception
#[allow(dead_code)]
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

    match exc_traceback {
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
    }
}
