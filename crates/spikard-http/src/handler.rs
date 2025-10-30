//! Python handler invocation from Rust

use crate::debug_log_module;
use axum::{
    body::Body,
    extract::Request,
    http::{Response, StatusCode},
};
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{Value, json};
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
    pub query_params: Value,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub body: Option<Value>,
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

/// Python handler wrapper that can be called from Axum
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    request_validator: Option<crate::SchemaValidator>,
    response_validator: Option<crate::SchemaValidator>,
    parameter_validator: Option<crate::ParameterValidator>,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        request_validator: Option<crate::SchemaValidator>,
        response_validator: Option<crate::SchemaValidator>,
        parameter_validator: Option<crate::ParameterValidator>,
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
    pub async fn call(
        &self,
        _req: Request<Body>,
        request_data: RequestData,
    ) -> Result<Response<Body>, (StatusCode, String)> {
        // DEBUG: Write to file since test client captures stderr
        let _ = std::fs::write(
            "/tmp/spikard_debug.log",
            format!(
                "[UNCONDITIONAL DEBUG] PythonHandler::call() entered\n[UNCONDITIONAL DEBUG] parameter_validator present: {}\n",
                self.parameter_validator.is_some()
            ),
        );
        eprintln!("[UNCONDITIONAL DEBUG] PythonHandler::call() entered");
        eprintln!(
            "[UNCONDITIONAL DEBUG] parameter_validator present: {}",
            self.parameter_validator.is_some()
        );
        // Validate request body in Rust if validator is present
        if let Some(validator) = &self.request_validator
            && let Some(body) = &request_data.body
            && let Err(errors) = validator.validate(body)
        {
            let error_msg = if is_debug_mode() {
                // In DEBUG mode, include full validation errors and request data
                json!({
                    "error": "Request validation failed",
                    "validation_errors": format!("{:?}", errors),
                    "request_body": body,
                    "path_params": request_data.path_params,
                    "query_params": request_data.query_params,
                })
                .to_string()
            } else {
                "Request validation failed".to_string()
            };
            return Err((StatusCode::BAD_REQUEST, error_msg));
        }

        // Validate and extract parameters in Rust if validator is present
        // This returns a validated JSON object with properly typed values
        let validated_params = if let Some(validator) = &self.parameter_validator {
            // Pass query params as Value directly (fast-query-parsers already did type conversion)
            match validator.validate_and_extract(
                &request_data.query_params,
                &request_data.path_params,
                &request_data.headers,
                &request_data.cookies,
            ) {
                Ok(params) => Some(params),
                Err(errors) => {
                    // Return FastAPI-compatible error format with {"detail": [...]}
                    let mut debug_msg = format!(
                        "[UNCONDITIONAL DEBUG] Parameter validation failed with {} errors\n",
                        errors.errors.len()
                    );
                    for (i, err) in errors.errors.iter().enumerate() {
                        debug_msg.push_str(&format!(
                            "[UNCONDITIONAL DEBUG]   Error {}: type={}, loc={:?}, msg={}, input={}, ctx={:?}\n",
                            i, err.error_type, err.loc, err.msg, err.input, err.ctx
                        ));
                    }
                    eprintln!("{}", debug_msg);
                    debug_log_module!(
                        "handler",
                        "Parameter validation failed with {} errors",
                        errors.errors.len()
                    );
                    let error_body = json!({
                        "detail": errors.errors
                    });
                    let error_json = serde_json::to_string_pretty(&error_body)
                        .unwrap_or_else(|e| format!("Failed to serialize: {}", e));
                    debug_msg.push_str(&format!("[UNCONDITIONAL DEBUG] error_body JSON: {}\n", error_json));
                    let _ = std::fs::write("/tmp/spikard_validation_error.log", debug_msg);
                    eprintln!("[UNCONDITIONAL DEBUG] error_body JSON: {}", error_json);
                    debug_log_module!("handler", "Returning 422 with error body: {}", error_body.to_string());
                    return Err((StatusCode::UNPROCESSABLE_ENTITY, error_body.to_string()));
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
            // For async handlers, we need to await the coroutine
            // This must be done inside the blocking task
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<ResponseResult> {
                    let handler_obj = handler.bind(py);

                    // Convert to Python kwargs - use validated params if available
                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data)?
                    };

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
                            "Handler marked as async but did not return a coroutine",
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

                    // Convert Python result to ResponseResult
                    python_to_response_result(py, &py_result)
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
                Python::attach(|py| -> PyResult<ResponseResult> {
                    let handler_obj = handler.bind(py);

                    // Convert to Python kwargs - use validated params if available
                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data)?
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
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to spawn blocking task: {}", e),
                )
            })?
        };

        match result {
            Ok(response_data) => {
                // Check if this is a ResponseData (custom Response object) or just JSON
                let (json_value, status_code, headers) = match response_data {
                    ResponseResult::Custom {
                        content,
                        status_code,
                        headers,
                    } => (content, status_code, headers),
                    ResponseResult::Json(json_value) => (json_value, 200, HashMap::new()),
                };

                // Validate response in Rust if validator is present
                if let Some(validator) = &response_validator
                    && let Err(errors) = validator.validate(&json_value)
                {
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
                        })
                        .to_string()
                    } else {
                        "Internal server error".to_string()
                    };
                    return Err((StatusCode::INTERNAL_SERVER_ERROR, error_msg));
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
            Err(e) => {
                eprintln!("[UNCONDITIONAL DEBUG] Handler caught error, checking if Pydantic ValidationError");
                // Check if this is a Pydantic ValidationError by trying to extract its .json() method
                let pydantic_errors = Python::attach(|py| -> Option<String> {
                    let err_value = e.value(py);
                    let type_name = err_value.get_type().name().ok()?;
                    eprintln!("[UNCONDITIONAL DEBUG] Python exception type: {}", type_name);

                    debug_log_module!("handler", "Caught Python exception: type={}", type_name);

                    // Check if this is a ValidationError from pydantic_core
                    if type_name == "ValidationError" {
                        debug_log_module!("handler", "This is a Pydantic ValidationError!");
                        // Try to call the .json() method
                        if let Ok(json_method) = err_value.getattr("json")
                            && let Ok(json_str) = json_method.call0()
                        {
                            let json_string = json_str.extract::<String>().ok()?;
                            debug_log_module!("handler", "Extracted Pydantic .json(): {}", json_string);
                            return Some(json_string);
                        }
                        debug_log_module!("handler", "Failed to extract .json() from ValidationError");
                    }
                    None
                });

                // If we got Pydantic validation errors, format them FastAPI-style
                if let Some(pydantic_json) = pydantic_errors {
                    debug_log_module!("handler", "Processing Pydantic JSON");
                    // Parse the Pydantic JSON and wrap it in {"detail": [...]}
                    if let Ok(errors_array) = serde_json::from_str::<serde_json::Value>(&pydantic_json) {
                        let error_body = json!({
                            "detail": errors_array
                        });
                        debug_log_module!(
                            "handler",
                            "Returning FastAPI-style Pydantic error with {} errors",
                            errors_array.as_array().map(|a| a.len()).unwrap_or(0)
                        );
                        return Err((StatusCode::UNPROCESSABLE_ENTITY, error_body.to_string()));
                    }
                }

                // Fallback: Check if this is a validation error (missing required parameter or type error)
                let error_str = format!("{}", e);
                let is_validation_error =
                    error_str.contains("missing") && error_str.contains("required") || error_str.contains("argument");

                let status_code = if is_validation_error {
                    StatusCode::UNPROCESSABLE_ENTITY // 422
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR // 500
                };

                let error_msg = if is_debug_mode() {
                    // In DEBUG mode, include Python traceback
                    let traceback = Python::attach(|py| get_python_traceback(py, &e));

                    json!({
                        "error": if is_validation_error { "Validation error" } else { "Python handler error" },
                        "exception": error_str,
                        "traceback": traceback,
                        "request_data": {
                            "path_params": request_data_for_error.path_params,
                            "query_params": request_data_for_error.query_params,
                            "body": request_data_for_error.body,
                        }
                    })
                    .to_string()
                } else if is_validation_error {
                    "Validation error".to_string()
                } else {
                    "Internal server error".to_string()
                };
                Err((status_code, error_msg))
            }
        }
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
    let params_dict = json_to_python(py, validated_params)?;

    // Import our converter module
    let converter_module = py.import("spikard._internal.converters")?;
    let convert_params_func = converter_module.getattr("convert_params")?;

    // Call convert_params(params_dict, handler_func)
    // This will use msgspec to convert types based on handler's signature
    let converted = convert_params_func.call1((params_dict, handler))?;

    // Extract the converted dict
    let kwargs = converted.cast_into::<PyDict>()?;

    // Add request body if present (convert to Python dict/list)
    if let Some(body) = &request_data.body {
        let py_body = json_to_python(py, body)?;
        kwargs.set_item("body", py_body)?;
    }

    Ok(kwargs)
}

/// Convert request data (path params, query params, body) to Python keyword arguments
/// This is the fallback when no parameter validator is present
fn request_data_to_py_kwargs<'py>(py: Python<'py>, request_data: &RequestData) -> PyResult<Bound<'py, PyDict>> {
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
    let json_str = serde_json::to_string(value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize JSON: {}", e)))?;
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
