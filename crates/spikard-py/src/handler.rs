//! Python handler implementation for spikard_http::Handler trait

/// Debug logging macro for module-specific logging
macro_rules! debug_log_module {
    ($module:expr, $($arg:tt)*) => {
        if is_debug_mode() {
            eprintln!("[{}] {}", $module, format!($($arg)*));
        }
    };
}

use crate::response::StreamingResponse;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyTuple};
use serde_json::{Value, json};
use spikard_core::errors::StructuredError;
use spikard_http::{Handler, HandlerResponse, HandlerResult, RequestData};
use spikard_http::{ParameterValidator, ProblemDetails, SchemaValidator};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Global Python event loop for async handlers
/// Managed by a dedicated Python thread
pub static PYTHON_EVENT_LOOP: OnceCell<Py<PyAny>> = OnceCell::new();

/// Initialize Python event loop that runs in a dedicated thread
/// This allows proper async/await support without blocking the Rust event loop
pub fn init_python_event_loop() -> PyResult<()> {
    Python::attach(|py| {
        if PYTHON_EVENT_LOOP.get().is_some() {
            return Ok(());
        }

        // Install uvloop for better performance if available
        if let Ok(uvloop) = py.import("uvloop") {
            uvloop.call_method0("install")?;
            eprintln!("[spikard] uvloop installed for enhanced async performance");
        } else {
            eprintln!("[spikard] uvloop not available, using standard asyncio");
        }

        // Create a new event loop that will run in a dedicated thread
        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;

        // Store the event loop
        PYTHON_EVENT_LOOP
            .set(event_loop.into())
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Event loop already initialized"))?;

        // Start the event loop in a dedicated Python thread
        let threading = py.import("threading")?;

        // Create a Python function that runs the event loop
        let globals = pyo3::types::PyDict::new(py);
        globals.set_item("asyncio", asyncio)?;

        let run_loop_code =
            pyo3::ffi::c_str!("def run_loop(loop):\n    asyncio.set_event_loop(loop)\n    loop.run_forever()\n");
        py.run(run_loop_code, Some(&globals), None)?;
        let run_loop_fn = globals.get_item("run_loop")?.unwrap();

        let loop_ref = PYTHON_EVENT_LOOP.get().unwrap().bind(py);

        // Create thread with kwargs for Python 3.14 compatibility
        let thread_kwargs = pyo3::types::PyDict::new(py);
        thread_kwargs.set_item("target", run_loop_fn)?;
        thread_kwargs.set_item("args", (loop_ref,))?;
        thread_kwargs.set_item("daemon", true)?;

        let thread = threading.call_method("Thread", (), Some(&thread_kwargs))?;
        thread.call_method0("start")?;

        eprintln!("[spikard] Started dedicated Python event loop thread for async handlers");
        eprintln!("[spikard] Async handlers will use asyncio.run_coroutine_threadsafe()");

        Ok(())
    })
}

/// Check if DEBUG mode is enabled
fn is_debug_mode() -> bool {
    std::env::var("DEBUG")
        .map(|v| v == "1" || v.to_lowercase() == "true")
        .unwrap_or(false)
}

fn structured_error_response(problem: ProblemDetails) -> (StatusCode, String) {
    let payload = StructuredError::new(
        "validation_error".to_string(),
        problem.title.clone(),
        serde_json::to_value(&problem).unwrap_or_else(|_| json!({})),
    );
    let body = serde_json::to_string(&payload)
        .unwrap_or_else(|_| r#"{"error":"validation_error","code":"validation_error","details":{}}"#.to_string());
    (problem.status_code(), body)
}

/// Response result from Python handler
pub enum ResponseResult {
    /// Custom Response object with status code and headers
    Custom {
        content: Value,
        status_code: u16,
        headers: HashMap<String, String>,
    },
    /// Plain JSON response (defaults to 200 OK)
    Json(Value),
    /// Streaming response backed by async iterator
    Stream(HandlerResponse),
}

/// Python handler wrapper that implements spikard_http::Handler
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    request_validator: Option<Arc<SchemaValidator>>,
    response_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
    body_param_name: String,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        request_validator: Option<Arc<SchemaValidator>>,
        response_validator: Option<Arc<SchemaValidator>>,
        parameter_validator: Option<ParameterValidator>,
        body_param_name: Option<String>,
    ) -> Self {
        Self {
            handler: Arc::new(handler),
            is_async,
            request_validator,
            response_validator,
            parameter_validator,
            body_param_name: body_param_name.unwrap_or_else(|| "body".to_string()),
        }
    }

    /// Call the Python handler
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(&self, _req: Request<Body>, request_data: RequestData) -> HandlerResult {
        if let Some(validator) = &self.request_validator
            && let Err(errors) = validator.validate(&request_data.body)
        {
            let problem = ProblemDetails::from_validation_error(&errors);
            return Err(structured_error_response(problem));
        }

        let validated_params = if let Some(validator) = &self.parameter_validator {
            match validator.validate_and_extract(
                &request_data.query_params,
                &request_data.raw_query_params,
                &request_data.path_params,
                &request_data.headers,
                &request_data.cookies,
            ) {
                Ok(params) => Some(params),
                Err(errors) => {
                    debug_log_module!(
                        "handler",
                        "Parameter validation failed with {} errors",
                        errors.errors.len()
                    );
                    let problem = ProblemDetails::from_validation_error(&errors);
                    debug_log_module!("handler", "Returning 422 with RFC 9457 error");
                    return Err(structured_error_response(problem));
                }
            }
        } else {
            None
        };

        let handler = self.handler.clone();
        let is_async = self.is_async;
        let response_validator = self.response_validator.clone();
        let request_data_for_error = request_data.clone();
        let validated_params_for_task = validated_params.clone();
        let body_param_name = self.body_param_name.clone();

        let result = if is_async {
            // Get the global event loop
            let event_loop = PYTHON_EVENT_LOOP.get().ok_or_else(|| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    "Python event loop not initialized".to_string(),
                )
            })?;

            // Submit coroutine to the dedicated event loop thread using run_coroutine_threadsafe
            let output = tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let handler_obj = handler.bind(py);
                    let asyncio = py.import("asyncio")?;

                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data, handler_obj.clone(), &body_param_name)?
                    };

                    let coroutine = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    };

                    if !coroutine.hasattr("__await__")? {
                        return Err(pyo3::exceptions::PyTypeError::new_err(
                            "Handler marked as async but did not return a coroutine",
                        ));
                    }

                    // Submit to the running event loop using run_coroutine_threadsafe
                    let loop_obj = event_loop.bind(py);
                    let future = asyncio.call_method1("run_coroutine_threadsafe", (coroutine, loop_obj))?;

                    // Wait for the result by calling .result() on the concurrent.futures.Future
                    let result = future.call_method0("result")?;

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

            // Convert Python result to ResponseResult
            Python::attach(|py| python_to_response_result(py, output.bind(py))).map_err(|e: PyErr| {
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Python error: {}", e),
                )
            })?
        } else {
            // Use spawn_blocking for sync handlers
            // Note: block_in_place was tested but is 6% slower under load due to:
            // 1. Limited worker threads (8-10) vs blocking pool (512)
            // 2. Python GIL serialization means more threads waiting = better utilization
            // 3. High concurrency (50+ connections) starves worker threads with block_in_place
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<ResponseResult> {
                    let handler_obj = handler.bind(py);

                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(py, validated, &request_data, handler_obj.clone())?
                    } else {
                        request_data_to_py_kwargs(py, &request_data, handler_obj.clone(), &body_param_name)?
                    };

                    let py_result = if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
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

        let (json_value, status_code, headers) = match result {
            ResponseResult::Stream(handler_response) => {
                return Ok(handler_response.into_response());
            }
            ResponseResult::Custom {
                content,
                status_code,
                headers,
            } => (content, status_code, headers),
            ResponseResult::Json(json_value) => (json_value, 200, HashMap::new()),
        };

        let content_type = headers
            .get("content-type")
            .or_else(|| headers.get("Content-Type"))
            .map(|s| s.as_str())
            .unwrap_or("application/json");

        let body_bytes = if content_type.starts_with("text/") || content_type.starts_with("application/json") {
            if let Value::String(s) = &json_value {
                if !content_type.starts_with("application/json") {
                    s.as_bytes().to_vec()
                } else {
                    serde_json::to_vec(&json_value).map_err(|e| {
                        (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to serialize response: {}", e),
                        )
                    })?
                }
            } else {
                if content_type.starts_with("application/json") {
                    #[allow(clippy::collapsible_if)]
                    if let Some(validator) = &response_validator {
                        if let Err(errors) = validator.validate(&json_value) {
                            let problem = ProblemDetails::from_validation_error(&errors);
                            return Err(structured_error_response(problem));
                        }
                    }
                }
                serde_json::to_vec(&json_value).map_err(|e| {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        format!("Failed to serialize response: {}", e),
                    )
                })?
            }
        } else {
            serde_json::to_vec(&json_value).map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to serialize response: {}", e),
                )
            })?
        };

        let mut response_builder = Response::builder()
            .status(StatusCode::from_u16(status_code).unwrap_or(StatusCode::OK))
            .header("content-type", content_type);

        for (key, value) in headers {
            if key.to_lowercase() != "content-type" {
                response_builder = response_builder.header(key, value);
            }
        }

        response_builder.body(Body::from(body_bytes)).map_err(|e| {
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
    if obj.is_instance_of::<StreamingResponse>() {
        let streaming: Py<StreamingResponse> = obj.extract()?;
        let handler_response = streaming.borrow(py).to_handler_response(py)?;
        return Ok(ResponseResult::Stream(handler_response));
    }

    if obj.hasattr("status_code")? && obj.hasattr("content")? && obj.hasattr("headers")? {
        let status_code: u16 = obj.getattr("status_code")?.extract()?;

        let content_attr = obj.getattr("content")?;
        let content = if content_attr.is_none() {
            Value::Null
        } else {
            python_to_json(py, &content_attr)?
        };

        let headers_dict = obj.getattr("headers")?;
        let mut headers = HashMap::new();

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
        let json_value = python_to_json(py, obj)?;
        Ok(ResponseResult::Json(json_value))
    }
}

/// Convert Python object to JSON Value (optimized zero-copy conversion)
///
/// This function converts Python objects directly to serde_json::Value using PyO3,
/// avoiding the serialize-to-string → parse-from-string overhead of json.dumps/loads.
///
/// For DTOs (msgspec.Struct, dataclass, Pydantic), it first converts to builtins using
/// msgspec.to_builtins() which is much faster than json.dumps().
///
/// Performance improvement: ~40-60% faster than json.dumps() → serde_json::from_str()
fn python_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    use pyo3::types::{PyDict, PyList};

    // Fast path: already a builtin type, convert directly
    if obj.is_none() {
        return Ok(Value::Null);
    }

    if let Ok(b) = obj.extract::<bool>() {
        return Ok(Value::Bool(b));
    }

    if let Ok(i) = obj.extract::<i64>() {
        return Ok(Value::Number(i.into()));
    }

    if let Ok(f) = obj.extract::<f64>() {
        if let Some(num) = serde_json::Number::from_f64(f) {
            return Ok(Value::Number(num));
        }
        return Err(pyo3::exceptions::PyValueError::new_err("Invalid float value"));
    }

    if let Ok(s) = obj.extract::<String>() {
        return Ok(Value::String(s));
    }

    // Handle dict
    #[allow(deprecated)]
    if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (key, value) in dict.iter() {
            let key_str: String = key.extract()?;
            let json_value = python_to_json(py, &value)?;
            map.insert(key_str, json_value);
        }
        return Ok(Value::Object(map));
    }

    // Handle list
    #[allow(deprecated)]
    if let Ok(list) = obj.downcast::<PyList>() {
        let mut arr = Vec::new();
        for item in list.iter() {
            let json_value = python_to_json(py, &item)?;
            arr.push(json_value);
        }
        return Ok(Value::Array(arr));
    }

    // Not a builtin type - could be DTO (msgspec.Struct, dataclass, Pydantic model)
    // Use msgspec.to_builtins() to convert to dict, then convert to JSON
    let serialization_module = py.import("spikard._internal.serialization")?;
    let to_builtins_func = serialization_module.getattr("to_builtins")?;
    let builtin_obj = to_builtins_func.call1((obj,))?;

    // Now convert the builtin object to JSON (recursive call will hit fast path)
    python_to_json(py, &builtin_obj)
}

/// Inject DI dependencies into kwargs dict
///
/// Extracts resolved dependencies from request_data and adds them to the kwargs
/// dict so they can be passed to the Python handler.
#[cfg(feature = "di")]
fn inject_di_dependencies<'py>(
    py: Python<'py>,
    kwargs: &Bound<'py, PyDict>,
    request_data: &RequestData,
) -> PyResult<()> {
    if let Some(ref dependencies) = request_data.dependencies {
        // Get all dependency keys
        let keys = dependencies.keys();

        for key in keys {
            // Get the value for this key using public API
            if let Some(value) = dependencies.get_arc(&key) {
                // Try to downcast to Py<PyAny> (Python objects stored by PythonValueDependency/PythonFactoryDependency)
                if let Ok(py_obj) = value.downcast::<pyo3::Py<PyAny>>() {
                    // Convert to Python object and add to kwargs
                    let obj_ref = py_obj.bind(py);
                    kwargs.set_item(&key, obj_ref)?;
                }
            }
        }
    }
    Ok(())
}

/// Convert validated parameters to Python keyword arguments using pythonize
/// This uses already-validated parameter values and converts them directly to Python
/// objects using the optimized pythonize crate, then lets Python's convert_params
/// filter and convert based on handler signature.
///
/// OPTIMIZATION: Use pythonize to convert serde_json::Value → Python objects directly,
/// which is faster than manual conversion or JSON round-trip.
fn validated_params_to_py_kwargs<'py>(
    py: Python<'py>,
    validated_params: &Value,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    // Convert validated params to Python dict using pythonize (optimized)
    let params_dict = pythonize::pythonize(py, validated_params)?;
    let params_dict: Bound<'_, PyDict> = params_dict.extract()?;

    // Add raw body bytes if present (for msgspec to parse directly)
    if let Some(raw_bytes) = &request_data.raw_body {
        params_dict.set_item("body", pyo3::types::PyBytes::new(py, raw_bytes))?;
        params_dict.set_item("_raw_json", true)?;
    } else if !request_data.body.is_null() {
        let py_body = pythonize::pythonize(py, &request_data.body)?;
        params_dict.set_item("body", py_body)?;
    }

    // Inject DI dependencies if present
    #[cfg(feature = "di")]
    inject_di_dependencies(py, &params_dict, request_data)?;

    // Call convert_params to filter parameters to match handler signature
    let converter_module = py.import("spikard._internal.converters")?;
    let convert_params_func = converter_module.getattr("convert_params")?;
    let converted = convert_params_func.call1((params_dict, handler))?;

    Ok(converted.cast_into::<PyDict>()?)
}

/// Convert request data (path params, query params, body) to Python keyword arguments
/// This is the fallback when no parameter validator is present
fn request_data_to_py_kwargs<'py>(
    py: Python<'py>,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
    body_param_name: &str,
) -> PyResult<Bound<'py, PyDict>> {
    let kwargs = PyDict::new(py);

    let path_params = PyDict::new(py);
    for (key, value) in request_data.path_params.iter() {
        if let Ok(int_val) = value.parse::<i64>() {
            path_params.set_item(key, int_val)?;
        } else if let Ok(float_val) = value.parse::<f64>() {
            path_params.set_item(key, float_val)?;
        } else if value == "true" || value == "false" {
            let bool_val = value == "true";
            path_params.set_item(key, bool_val)?;
        } else {
            path_params.set_item(key, value)?;
        }
    }
    kwargs.set_item("path_params", path_params)?;

    if let Value::Object(query_map) = &request_data.query_params {
        let query_params = PyDict::new(py);
        for (key, value) in query_map {
            let py_value = json_to_python(py, value)?;
            query_params.set_item(key.as_str(), py_value)?;
        }
        kwargs.set_item("query_params", query_params)?;
    } else {
        kwargs.set_item("query_params", PyDict::new(py))?;
    }

    let headers_dict = PyDict::new(py);
    for (k, v) in request_data.headers.iter() {
        headers_dict.set_item(k, v)?;
    }
    kwargs.set_item("headers", headers_dict)?;

    let cookies_dict = PyDict::new(py);
    for (k, v) in request_data.cookies.iter() {
        cookies_dict.set_item(k, v)?;
    }
    kwargs.set_item("cookies", cookies_dict)?;

    // Optimization: Pass raw JSON bytes to Python for msgspec to parse directly
    // This avoids double-parsing (Rust serde + Python msgspec)
    if let Some(raw_bytes) = &request_data.raw_body {
        // Pass raw bytes as PyBytes for msgspec.json.decode() to handle
        kwargs.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
        kwargs.set_item("_raw_json", true)?; // Flag for converter
    } else {
        let py_body = json_to_python(py, &request_data.body)?;
        kwargs.set_item(body_param_name, py_body)?;
    }

    // Inject DI dependencies if present
    #[cfg(feature = "di")]
    inject_di_dependencies(py, &kwargs, request_data)?;

    // Always call convert_params to filter parameters to match handler signature
    // When needs_conversion=False, it will skip type conversion but still filter params
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
    let traceback_module = match py.import("traceback") {
        Ok(module) => module,
        Err(_) => return format!("{}", err),
    };

    let exc_type = err.get_type(py);
    let exc_value = err.value(py);
    let exc_traceback = err.traceback(py);

    match exc_traceback {
        Some(tb) => match traceback_module.call_method1("format_exception", (exc_type, exc_value, tb)) {
            Ok(lines) => {
                if let Ok(list) = lines.extract::<Vec<String>>() {
                    list.join("")
                } else {
                    format!("{}", err)
                }
            }
            Err(_) => format!("{}", err),
        },
        None => match traceback_module.call_method1("format_exception_only", (exc_type, exc_value)) {
            Ok(lines) => {
                if let Ok(list) = lines.extract::<Vec<String>>() {
                    list.join("")
                } else {
                    format!("{}", err)
                }
            }
            Err(_) => format!("{}", err),
        },
    }
}
