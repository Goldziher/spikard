//! Python handler implementation for spikard_http::Handler trait

use crate::conversion::{json_to_python, python_to_json};
use crate::response::StreamingResponse;
use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
};
use once_cell::sync::OnceCell;
use pyo3::prelude::*;
use pyo3::sync::PyOnceLock;
use pyo3::types::{PyDict, PyTuple};
use pyo3_async_runtimes::TaskLocals;
use serde_json::{Value, json};
use spikard_core::errors::StructuredError;
use spikard_http::{Handler, HandlerResponse, HandlerResult, ParameterValidator, RequestData};
use spikard_http::{ProblemDetails, SchemaValidator};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Global Python async context for pyo3_async_runtimes.
pub static PYTHON_TASK_LOCALS: OnceCell<TaskLocals> = OnceCell::new();

static CONVERT_PARAMS: PyOnceLock<pyo3::Py<pyo3::PyAny>> = PyOnceLock::new();
static MSGSPEC_JSON_ENCODE: PyOnceLock<pyo3::Py<pyo3::PyAny>> = PyOnceLock::new();

fn convert_params<'py>(
    py: Python<'py>,
    params: Bound<'py, PyDict>,
    handler: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let func = CONVERT_PARAMS.get_or_try_init(py, || {
        let converter_module = py.import("spikard._internal.converters")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(converter_module.getattr("convert_params")?.unbind())
    })?;

    let converted = func.bind(py).call1((params, handler))?;
    Ok(converted.cast_into::<PyDict>()?)
}

fn msgspec_json_encode(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    let encode = MSGSPEC_JSON_ENCODE.get_or_try_init(py, || {
        let msgspec = py.import("msgspec")?;
        let json_mod = msgspec.getattr("json")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(json_mod.getattr("encode")?.unbind())
    })?;

    let encoded = encode.bind(py).call1((obj,))?;
    let py_bytes = encoded.cast_into::<pyo3::types::PyBytes>()?;
    Ok(py_bytes.as_bytes().to_vec())
}

/// Initialize Python async context once using pyo3_async_runtimes to avoid per-request event loop
/// setup and to ensure async handlers run without blocking the GIL.
pub fn init_python_event_loop() -> PyResult<()> {
    Python::attach(|py| {
        if PYTHON_TASK_LOCALS.get().is_some() {
            return Ok(());
        }

        let asyncio = py.import("asyncio")?;
        let event_loop = asyncio.call_method0("new_event_loop")?;

        let task_locals = TaskLocals::new(event_loop.clone()).copy_context(py)?;
        PYTHON_TASK_LOCALS
            .set(task_locals)
            .map_err(|_| pyo3::exceptions::PyRuntimeError::new_err("Python async context already initialized"))?;

        let threading = py.import("threading")?;
        let globals = PyDict::new(py);
        globals.set_item("asyncio", asyncio)?;

        let run_loop_code =
            pyo3::ffi::c_str!("def run_loop(loop):\n    asyncio.set_event_loop(loop)\n    loop.run_forever()\n");
        py.run(run_loop_code, Some(&globals), None)?;
        let run_loop_fn = globals
            .get_item("run_loop")?
            .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Failed to load run_loop helper"))?;

        let thread_kwargs = PyDict::new(py);
        thread_kwargs.set_item("target", run_loop_fn)?;
        thread_kwargs.set_item("args", (event_loop,))?;
        thread_kwargs.set_item("daemon", true)?;

        let thread = threading.call_method("Thread", (), Some(&thread_kwargs))?;
        thread.call_method0("start")?;

        Ok(())
    })
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

fn structured_error(code: &str, message: impl Into<String>) -> (StatusCode, String) {
    let payload = StructuredError::simple(code.to_string(), message.into());
    let body = serde_json::to_string(&payload)
        .unwrap_or_else(|_| r#"{"error":"internal_error","code":"internal_error","details":{}}"#.to_string());
    (StatusCode::INTERNAL_SERVER_ERROR, body)
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
    /// Pre-serialized response body (typically JSON bytes)
    Raw {
        body: Vec<u8>,
        status_code: u16,
        headers: HashMap<String, String>,
    },
    /// Streaming response backed by async iterator
    Stream(HandlerResponse),
}

/// Python handler wrapper that implements spikard_http::Handler
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    response_validator: Option<Arc<SchemaValidator>>,
    parameter_validator: Option<ParameterValidator>,
    body_param_name: String,
}

impl PythonHandler {
    /// Create a new Python handler wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        response_validator: Option<Arc<SchemaValidator>>,
        parameter_validator: Option<ParameterValidator>,
        body_param_name: Option<String>,
    ) -> Self {
        Self {
            handler: Arc::new(handler),
            is_async,
            response_validator,
            parameter_validator,
            body_param_name: body_param_name.unwrap_or_else(|| "body".to_string()),
        }
    }

    /// Call the Python handler
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(&self, _req: Request<Body>, request_data: RequestData) -> HandlerResult {
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
                    let problem = ProblemDetails::from_validation_error(&errors);
                    return Err(structured_error_response(problem));
                }
            }
        } else {
            None
        };

        let handler = self.handler.clone();
        let is_async = self.is_async;
        let response_validator = self.response_validator.clone();
        let prefer_msgspec_json = response_validator.is_none();
        let _request_data_for_error = request_data.clone();
        let body_param_name = self.body_param_name.clone();
        let validated_params_for_task = validated_params.clone();

        let result = if is_async {
            let coroutine_future = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

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

                let task_locals = PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Python async context not initialized"))?
                    .clone();

                let awaitable = coroutine.clone();
                pyo3_async_runtimes::into_future_with_locals(&task_locals, awaitable)
            })
            .map_err(|e: PyErr| {
                structured_error("python_call_error", format!("Python error calling handler: {}", e))
            })?;

            let coroutine_result = coroutine_future
                .await
                .map_err(|e: PyErr| structured_error("python_async_error", format!("Python async error: {}", e)))?;

            Python::attach(|py| python_to_response_result(py, coroutine_result.bind(py), prefer_msgspec_json))
                .map_err(|e: PyErr| structured_error("python_response_error", format!("Python error: {}", e)))?
        } else {
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
                    python_to_response_result(py, &py_result, prefer_msgspec_json)
                })
            })
            .await
            .map_err(|e| structured_error("spawn_blocking_error", format!("Spawn blocking error: {}", e)))?
            .map_err(|e: PyErr| structured_error("python_error", format!("Python error: {}", e)))?
        };

        let (json_value, status_code, headers, raw_body_bytes) = match result {
            ResponseResult::Stream(handler_response) => {
                return Ok(handler_response.into_response());
            }
            ResponseResult::Custom {
                content,
                status_code,
                headers,
            } => (content, status_code, headers, None),
            ResponseResult::Json(json_value) => (json_value, 200, HashMap::new(), None),
            ResponseResult::Raw {
                body,
                status_code,
                headers,
            } => (Value::Null, status_code, headers, Some(body)),
        };

        let content_type = headers
            .get("content-type")
            .or_else(|| headers.get("Content-Type"))
            .map(|s| s.as_str())
            .unwrap_or("application/json");

        let body_bytes = if let Some(raw) = raw_body_bytes {
            raw
        } else if content_type.starts_with("text/") || content_type.starts_with("application/json") {
            if let Value::String(s) = &json_value {
                if !content_type.starts_with("application/json") {
                    s.as_bytes().to_vec()
                } else {
                    serde_json::to_vec(&json_value).map_err(|e| {
                        structured_error(
                            "response_serialize_error",
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
                    structured_error(
                        "response_serialize_error",
                        format!("Failed to serialize response: {}", e),
                    )
                })?
            }
        } else {
            serde_json::to_vec(&json_value).map_err(|e| {
                structured_error(
                    "response_serialize_error",
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

        response_builder
            .body(Body::from(body_bytes))
            .map_err(|e| structured_error("response_build_error", format!("Failed to build response: {}", e)))
    }
}

/// Implement the spikard_http::Handler trait for PythonHandler
impl Handler for PythonHandler {
    fn prefers_raw_json_body(&self) -> bool {
        true
    }

    fn prefers_parameter_extraction(&self) -> bool {
        self.parameter_validator.is_some()
    }

    fn call(
        &self,
        request: Request<Body>,
        request_data: RequestData,
    ) -> Pin<Box<dyn Future<Output = HandlerResult> + Send + '_>> {
        Box::pin(self.call(request, request_data))
    }
}

/// Convert validated parameters (from Rust schema validation) to Python keyword arguments.
///
/// This uses the already-validated JSON object produced by `ParameterValidator::validate_and_extract` and
/// (a) adds the request body (prefer raw bytes if available) and (b) lets Python filter/re-map based on
/// the handler signature (`convert_params`).
fn validated_params_to_py_kwargs<'py>(
    py: Python<'py>,
    validated_params: &Value,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
) -> PyResult<Bound<'py, PyDict>> {
    let params_dict = json_to_python(py, validated_params)?;
    let params_dict: Bound<'_, PyDict> = params_dict.extract()?;

    if let Some(raw_bytes) = &request_data.raw_body {
        params_dict.set_item("body", pyo3::types::PyBytes::new(py, raw_bytes))?;
        params_dict.set_item("_raw_json", true)?;
    } else if !request_data.body.is_null() {
        let py_body = json_to_python(py, &request_data.body)?;
        params_dict.set_item("body", py_body)?;
    }

    #[cfg(feature = "di")]
    inject_di_dependencies(py, &params_dict, request_data)?;

    convert_params(py, params_dict, handler)
}

/// Convert Python object to ResponseResult
///
/// Checks if the object is a Response instance with custom status/headers,
/// otherwise treats it as JSON data
fn python_to_response_result(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    prefer_msgspec_json: bool,
) -> PyResult<ResponseResult> {
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
        if prefer_msgspec_json {
            let bytes = msgspec_json_encode(py, obj)?;
            let mut headers = HashMap::new();
            headers.insert("content-type".to_string(), "application/json".to_string());
            return Ok(ResponseResult::Raw {
                body: bytes,
                status_code: 200,
                headers,
            });
        }

        let json_value = python_to_json(py, obj)?;
        Ok(ResponseResult::Json(json_value))
    }
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
        let keys = dependencies.keys();

        for key in keys {
            if let Some(value) = dependencies.get_arc(&key)
                && let Ok(py_obj) = value.downcast::<pyo3::Py<PyAny>>()
            {
                let obj_ref = py_obj.bind(py);
                kwargs.set_item(&key, obj_ref)?;
            }
        }
    }
    Ok(())
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

    if let Some(raw_bytes) = &request_data.raw_body {
        // Always expose raw bytes for handlers which want them (e.g. `body: bytes`).
        kwargs.set_item("_raw_body", pyo3::types::PyBytes::new(py, raw_bytes))?;

        // Keep the fast path: when raw JSON bytes are available, pass them through so
        // Python can decode via msgspec without round-tripping through `serde_json::Value`
        // → Python builtins.
        kwargs.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
        kwargs.set_item("_raw_json", true)?;
    } else {
        let py_body = json_to_python(py, &request_data.body)?;
        kwargs.set_item(body_param_name, py_body)?;
    }

    #[cfg(feature = "di")]
    inject_di_dependencies(py, &kwargs, request_data)?;

    convert_params(py, kwargs, handler)
}

// (intentionally no trailing items)
