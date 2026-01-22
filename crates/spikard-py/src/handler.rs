//! Python `handler` implementation for spikard_http::Handler trait

use crate::conversion::{json_to_python, python_to_json};
use crate::handler_request::PyHandlerRequest;
use crate::response::StreamingResponse;
use crate::response_interpreter::PyResponseInterpreter;
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
use spikard_bindings_shared::ResponseInterpreter;
use spikard_core::errors::StructuredError;
use spikard_http::{Handler, HandlerResponse, HandlerResult, ParameterValidator, RequestData};
use spikard_http::{ProblemDetails, SchemaValidator};
use std::collections::{HashMap, HashSet};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

/// Global Python async context for pyo3_async_runtimes.
pub static PYTHON_TASK_LOCALS: OnceCell<TaskLocals> = OnceCell::new();

static CONVERT_PARAMS: PyOnceLock<pyo3::Py<pyo3::PyAny>> = PyOnceLock::new();
static MSGSPEC_JSON_ENCODE: PyOnceLock<Option<pyo3::Py<pyo3::PyAny>>> = PyOnceLock::new();
static NEEDS_CONVERSION: PyOnceLock<pyo3::Py<pyo3::PyAny>> = PyOnceLock::new();
static HANDLER_METADATA: PyOnceLock<pyo3::Py<pyo3::PyAny>> = PyOnceLock::new();

fn create_msgspec_decoder<'py>(
    py: Python<'py>,
    handler: Bound<'py, PyAny>,
    body_param_name: &str,
) -> PyResult<Option<pyo3::Py<pyo3::PyAny>>> {
    let func = HANDLER_METADATA.get_or_try_init(py, || {
        let converter_module = py.import("spikard._internal.converters")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(converter_module.getattr("_handler_metadata")?.unbind())
    })?;

    let metadata = func.bind(py).call1((handler,))?;
    let tuple = metadata.cast_into::<pyo3::types::PyTuple>()?;
    let type_hints_obj = tuple.get_item(0)?;
    if type_hints_obj.is_none() {
        return Ok(None);
    }

    let type_hints = type_hints_obj.cast_into::<pyo3::types::PyDict>()?;
    let target_type = match type_hints.get_item(body_param_name)? {
        Some(t) => t,
        None => return Ok(None),
    };

    let converters = py.import("spikard._internal.converters")?;
    let supports_decoder = converters.getattr("supports_msgspec_decoder")?;
    if !supports_decoder.call1((target_type.clone(),))?.extract::<bool>()? {
        return Ok(None);
    }
    let dec_hook = converters.getattr("_default_dec_hook")?;

    let msgspec = py.import("msgspec")?;
    let json_mod = msgspec.getattr("json")?;
    let decoder_type = json_mod.getattr("Decoder")?;

    let kwargs = PyDict::new(py);
    kwargs.set_item("type", target_type)?;
    kwargs.set_item("dec_hook", dec_hook)?;

    let decoder = decoder_type.call((), Some(&kwargs))?;
    Ok(Some(decoder.unbind()))
}

fn is_json_content_type(headers: &HashMap<String, String>) -> bool {
    let ct = headers
        .get("content-type")
        .or_else(|| headers.get("Content-Type"))
        .map(|s| s.as_str())
        .unwrap_or("");

    let ct = ct.to_ascii_lowercase();
    ct.contains("application/json") || ct.contains("+json")
}

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

fn needs_param_conversion<'py>(py: Python<'py>, handler: Bound<'py, PyAny>) -> PyResult<bool> {
    let func = NEEDS_CONVERSION.get_or_try_init(py, || {
        let converter_module = py.import("spikard._internal.converters")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(converter_module.getattr("needs_conversion")?.unbind())
    })?;

    func.bind(py).call1((handler,))?.extract::<bool>()
}

fn handler_param_names<'py>(
    py: Python<'py>,
    handler: Bound<'py, PyAny>,
) -> PyResult<(Option<HashSet<String>>, Option<String>)> {
    let func = HANDLER_METADATA.get_or_try_init(py, || {
        let converter_module = py.import("spikard._internal.converters")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(converter_module.getattr("_handler_metadata")?.unbind())
    })?;

    let metadata = func.bind(py).call1((handler,))?;
    let tuple = metadata.cast_into::<pyo3::types::PyTuple>()?;
    let params_obj = tuple.get_item(1)?;
    let first_param_obj = tuple.get_item(2)?;

    let first_param_name = if first_param_obj.is_none() {
        None
    } else {
        Some(first_param_obj.extract::<String>()?)
    };

    if params_obj.is_none() {
        return Ok((None, first_param_name));
    }

    let params_set = params_obj.cast_into::<pyo3::types::PySet>()?;
    let mut out = HashSet::with_capacity(params_set.len());
    for item in params_set.iter() {
        out.insert(item.extract::<String>()?);
    }
    Ok((Some(out), first_param_name))
}

fn ensure_empty_in_handler_globals<'py>(py: Python<'py>, handler: Bound<'py, PyAny>) -> PyResult<()> {
    let globals = handler.getattr("__globals__")?.cast_into::<PyDict>()?;
    if globals.get_item("Empty")?.is_some() {
        return Ok(());
    }

    let builtins = py.import("builtins")?;
    let empty = match py
        .import("spikard._internal.types")
        .and_then(|types_mod| types_mod.getattr("Empty"))
    {
        Ok(empty) => empty,
        Err(_) => builtins.getattr("object")?.call0()?,
    };

    let _ = globals.set_item("Empty", &empty);
    let _ = builtins.setattr("Empty", empty);

    Ok(())
}

fn request_param_is_request_type<'py>(
    py: Python<'py>,
    handler: Bound<'py, PyAny>,
    param_name: &str,
) -> PyResult<Option<bool>> {
    let func = HANDLER_METADATA.get_or_try_init(py, || {
        let converter_module = py.import("spikard._internal.converters")?;
        Ok::<pyo3::Py<pyo3::PyAny>, PyErr>(converter_module.getattr("_handler_metadata")?.unbind())
    })?;

    let metadata = func.bind(py).call1((handler,))?;
    let tuple = metadata.cast_into::<pyo3::types::PyTuple>()?;
    let type_hints_obj = tuple.get_item(0)?;
    if type_hints_obj.is_none() {
        return Ok(None);
    }

    let type_hints = type_hints_obj.cast_into::<pyo3::types::PyDict>()?;
    let Some(param_type) = type_hints.get_item(param_name)? else {
        return Ok(None);
    };

    let request_type = py.import("spikard.request")?.getattr("Request")?;
    if param_type.is(&request_type) {
        return Ok(Some(true));
    }

    let typing = py.import("typing")?;
    let origin = typing.getattr("get_origin")?.call1((param_type.clone(),))?;
    if !origin.is_none() {
        let args = typing.getattr("get_args")?.call1((param_type,))?;
        let args = args.cast_into::<pyo3::types::PyTuple>()?;
        for arg in args.iter() {
            if arg.is(&request_type) {
                return Ok(Some(true));
            }
        }
    }

    Ok(Some(false))
}

fn msgspec_json_encode(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Vec<u8>> {
    let encode = MSGSPEC_JSON_ENCODE.get_or_try_init(py, || {
        let msgspec = py.import("msgspec")?;
        let json_mod = match msgspec.getattr("json") {
            Ok(json_mod) => json_mod,
            Err(_) => return Ok::<Option<pyo3::Py<pyo3::PyAny>>, PyErr>(None),
        };

        let encode = match json_mod.getattr("encode") {
            Ok(encode) => encode,
            Err(_) => return Ok::<Option<pyo3::Py<pyo3::PyAny>>, PyErr>(None),
        };

        Ok::<Option<pyo3::Py<pyo3::PyAny>>, PyErr>(Some(encode.unbind()))
    })?;

    if let Some(encode) = encode {
        let encoded = encode.bind(py).call1((obj,))?;
        let py_bytes = encoded.cast_into::<pyo3::types::PyBytes>()?;
        Ok(py_bytes.as_bytes().to_vec())
    } else {
        // Fallback: `msgspec.json` isn't available (e.g. in test stubs), so convert to serde_json
        // and serialize in Rust.
        let json_value = python_to_json(py, obj)?;
        serde_json::to_vec(&json_value)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to encode JSON: {e}")))
    }
}

/// Initialize Python async context once using pyo3_async_runtimes to avoid per-request event loop
/// setup and to ensure async handlers run without blocking the GIL.
pub fn init_python_event_loop() -> PyResult<()> {
    Python::attach(|py| {
        if PYTHON_TASK_LOCALS.get().is_some() {
            return Ok(());
        }

        let asyncio = py.import("asyncio")?;

        // Prefer uvloop when available for faster asyncio primitives (notably call_soon_threadsafe),
        // while remaining fully compatible with standard asyncio when uvloop isn't installed.
        if let Ok(uvloop) = py.import("uvloop")
            && let Ok(policy_type) = uvloop.getattr("EventLoopPolicy")
            && let Ok(policy) = policy_type.call0()
        {
            let _ = asyncio.call_method1("set_event_loop_policy", (policy,));
        }

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

/// `Response` result from Python `handler`
pub enum ResponseResult {
    /// Custom `Response` object with status code and headers
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

/// Python `handler` wrapper that implements spikard_http::Handler
#[derive(Clone)]
pub struct PythonHandler {
    handler: Arc<Py<PyAny>>,
    is_async: bool,
    response_validator: Option<Arc<SchemaValidator>>,
    requires_headers: bool,
    requires_cookies: bool,
    body_param_name: String,
    needs_param_conversion: bool,
    handler_params: Option<Arc<HashSet<String>>>,
    request_only_handler: bool,
    body_only_handler: bool,
    msgspec_body_decoder: Option<Arc<Py<PyAny>>>,
}

impl PythonHandler {
    /// Create a new Python `handler` wrapper
    pub fn new(
        handler: Py<PyAny>,
        is_async: bool,
        response_validator: Option<Arc<SchemaValidator>>,
        parameter_validator: Option<ParameterValidator>,
        body_param_name: Option<String>,
    ) -> Self {
        let body_param_name = body_param_name.unwrap_or_else(|| "body".to_string());
        let requires_headers = parameter_validator.as_ref().is_some_and(|v| v.requires_headers());
        let requires_cookies = parameter_validator.as_ref().is_some_and(|v| v.requires_cookies());

        let validator_has_params = parameter_validator.as_ref().is_some_and(|v| v.has_params());
        let (
            needs_param_conversion,
            handler_params,
            request_only_handler,
            body_only_handler,
            msgspec_body_decoder,
            body_param_name,
        ) = Python::attach(|py| {
            let handler_obj = handler.bind(py);
            let _ = ensure_empty_in_handler_globals(py, handler_obj.clone());
            let needs = needs_param_conversion(py, handler_obj.clone()).unwrap_or(true);
            let (params, first_param_name) = handler_param_names(py, handler_obj.clone()).unwrap_or((None, None));
            let request_param_is_request = if matches!(first_param_name.as_deref(), Some("request" | "req")) {
                request_param_is_request_type(py, handler_obj.clone(), first_param_name.as_deref().unwrap())
                    .ok()
                    .flatten()
            } else {
                None
            };
            let request_only = matches!(first_param_name.as_deref(), Some("request" | "req"))
                && params.as_ref().is_some_and(|set| set.len() == 1)
                && matches!(request_param_is_request, Some(true) | None);

            let mut effective_body_param = body_param_name.clone();
            if effective_body_param == "body"
                && let Some(first_param) = first_param_name.as_deref()
                && !request_only
                && !validator_has_params
                && !matches!(first_param, "headers" | "cookies" | "query_params" | "path_params")
                && params.as_ref().is_some_and(|set| set.contains(first_param))
            {
                effective_body_param = first_param.to_string();
            }

            let body_only = matches!(first_param_name.as_deref(), Some(name) if name == effective_body_param)
                && params
                    .as_ref()
                    .is_some_and(|set| set.len() == 1 && set.contains(&effective_body_param));

            let decoder = if body_only && needs {
                create_msgspec_decoder(py, handler_obj.clone(), &effective_body_param)
                    .ok()
                    .flatten()
                    .map(Arc::new)
            } else {
                None
            };

            (
                needs,
                params.map(Arc::new),
                request_only,
                body_only,
                decoder,
                effective_body_param,
            )
        });

        Self {
            handler: Arc::new(handler),
            is_async,
            response_validator,
            requires_headers,
            requires_cookies,
            body_param_name,
            needs_param_conversion,
            handler_params,
            request_only_handler,
            body_only_handler,
            msgspec_body_decoder,
        }
    }

    /// Call the Python `handler`
    ///
    /// This runs the Python code in a blocking task to avoid blocking the Tokio runtime
    pub async fn call(&self, _req: Request<Body>, request_data: RequestData) -> HandlerResult {
        // PERFORMANCE: Clone validated_params directly to avoid intermediate allocation
        let validated_params_for_task = request_data.validated_params.clone();

        let handler = self.handler.clone();
        let is_async = self.is_async;
        let response_validator = self.response_validator.clone();
        let prefer_msgspec_json = true;
        let body_param_name = self.body_param_name.clone();
        let needs_param_conversion = self.needs_param_conversion;
        let handler_params = self.handler_params.clone();
        let request_only_handler = self.request_only_handler;
        let body_only_handler = self.body_only_handler;
        let msgspec_body_decoder = self.msgspec_body_decoder.clone();

        let result = if is_async {
            let coroutine_future = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

                let coroutine = if request_only_handler {
                    let req_obj = Py::new(py, PyHandlerRequest::new(request_data.clone()))?;
                    handler_obj.call1((req_obj,))?
                } else if body_only_handler
                    && request_data.body.is_null()
                    && is_json_content_type(&request_data.headers)
                    && let Some(decoder) = msgspec_body_decoder.as_ref()
                {
                    let raw = request_data.raw_body.as_deref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Missing raw body bytes for JSON request")
                    })?;
                    let decoded = decoder
                        .bind(py)
                        .call_method1("decode", (pyo3::types::PyBytes::new(py, raw),))?;
                    handler_obj.call1((decoded,))?
                } else if body_only_handler && is_json_content_type(&request_data.headers) && !needs_param_conversion {
                    let raw = request_data.raw_body.as_deref().ok_or_else(|| {
                        pyo3::exceptions::PyRuntimeError::new_err("Missing raw body bytes for JSON request")
                    })?;
                    // PERFORMANCE: Take ownership of Arc first, then try_unwrap to avoid double-clone
                    let body_value = if !request_data.body.is_null() {
                        Arc::try_unwrap(request_data.body).unwrap_or_else(|arc| (*arc).clone())
                    } else {
                        serde_json::from_slice::<Value>(raw)
                            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON body: {e}")))?
                    };
                    let py_body = json_to_python(py, &body_value)?;
                    handler_obj.call1((py_body,))?
                } else {
                    let kwargs = if let Some(ref validated) = validated_params_for_task {
                        validated_params_to_py_kwargs(
                            py,
                            validated,
                            &request_data,
                            handler_obj.clone(),
                            &body_param_name,
                            needs_param_conversion,
                            handler_params.as_deref(),
                        )?
                    } else {
                        request_data_to_py_kwargs(
                            py,
                            &request_data,
                            handler_obj.clone(),
                            &body_param_name,
                            needs_param_conversion,
                            handler_params.as_deref(),
                        )?
                    };

                    if kwargs.is_empty() {
                        handler_obj.call0()?
                    } else {
                        let empty_args = PyTuple::empty(py);
                        handler_obj.call(empty_args, Some(&kwargs))?
                    }
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
            .map_err(|e: PyErr| structured_error("python_call_error", format!("Python error calling handler: {e}")))?;

            let coroutine_result = coroutine_future
                .await
                .map_err(|e: PyErr| structured_error("python_async_error", format!("Python async error: {e}")))?;

            Python::attach(|py| python_to_response_result(py, coroutine_result.bind(py), prefer_msgspec_json))
                .map_err(|e: PyErr| structured_error("python_response_error", format!("Python error: {e}")))?
        } else {
            let handler_params = handler_params.clone();
            let request_data_for_sync = request_data.clone();
            let msgspec_body_decoder = msgspec_body_decoder.clone();
            tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<ResponseResult> {
                    let handler_obj = handler.bind(py);

                    let py_result = if request_only_handler {
                        let req_obj = Py::new(py, PyHandlerRequest::new(request_data_for_sync))?;
                        handler_obj.call1((req_obj,))?
                    } else if body_only_handler
                        && request_data_for_sync.body.is_null()
                        && is_json_content_type(&request_data_for_sync.headers)
                        && let Some(decoder) = msgspec_body_decoder.as_ref()
                    {
                        let raw = request_data_for_sync.raw_body.as_deref().ok_or_else(|| {
                            pyo3::exceptions::PyRuntimeError::new_err("Missing raw body bytes for JSON request")
                        })?;
                        let decoded = decoder
                            .bind(py)
                            .call_method1("decode", (pyo3::types::PyBytes::new(py, raw),))?;
                        handler_obj.call1((decoded,))?
                    } else if body_only_handler
                        && is_json_content_type(&request_data_for_sync.headers)
                        && !needs_param_conversion
                    {
                        let raw = request_data_for_sync.raw_body.as_deref().ok_or_else(|| {
                            pyo3::exceptions::PyRuntimeError::new_err("Missing raw body bytes for JSON request")
                        })?;
                        // PERFORMANCE: Take ownership of Arc first, then try_unwrap to avoid double-clone
                        let body_value = if !request_data_for_sync.body.is_null() {
                            Arc::try_unwrap(request_data_for_sync.body).unwrap_or_else(|arc| (*arc).clone())
                        } else {
                            serde_json::from_slice::<Value>(raw).map_err(|e| {
                                pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON body: {e}"))
                            })?
                        };
                        let py_body = json_to_python(py, &body_value)?;
                        handler_obj.call1((py_body,))?
                    } else {
                        let kwargs = if let Some(ref validated) = validated_params_for_task {
                            validated_params_to_py_kwargs(
                                py,
                                validated,
                                &request_data_for_sync,
                                handler_obj.clone(),
                                &body_param_name,
                                needs_param_conversion,
                                handler_params.as_deref(),
                            )?
                        } else {
                            request_data_to_py_kwargs(
                                py,
                                &request_data_for_sync,
                                handler_obj.clone(),
                                &body_param_name,
                                needs_param_conversion,
                                handler_params.as_deref(),
                            )?
                        };

                        if kwargs.is_empty() {
                            handler_obj.call0()?
                        } else {
                            let empty_args = PyTuple::empty(py);
                            handler_obj.call(empty_args, Some(&kwargs))?
                        }
                    };
                    python_to_response_result(py, &py_result, prefer_msgspec_json)
                })
            })
            .await
            .map_err(|e| structured_error("spawn_blocking_error", format!("Spawn blocking error: {e}")))?
            .map_err(|e: PyErr| structured_error("python_error", format!("Python error: {e}")))?
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
            if content_type.starts_with("application/json")
                && let Some(validator) = &response_validator
            {
                let json_value = serde_json::from_slice::<Value>(&raw)
                    .map_err(|e| structured_error("response_parse_error", format!("Failed to parse response: {e}")))?;
                if let Err(errors) = validator.validate(&json_value) {
                    let problem = ProblemDetails::from_validation_error(&errors);
                    return Err(structured_error_response(problem));
                }
            }
            raw
        } else if content_type.starts_with("text/") || content_type.starts_with("application/json") {
            if let Value::String(s) = &json_value {
                if !content_type.starts_with("application/json") {
                    s.as_bytes().to_vec()
                } else {
                    serde_json::to_vec(&json_value).map_err(|e| {
                        structured_error("response_serialize_error", format!("Failed to serialize response: {e}"))
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
                    structured_error("response_serialize_error", format!("Failed to serialize response: {e}"))
                })?
            }
        } else {
            serde_json::to_vec(&json_value).map_err(|e| {
                structured_error("response_serialize_error", format!("Failed to serialize response: {e}"))
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
            .map_err(|e| structured_error("response_build_error", format!("Failed to build response: {e}")))
    }
}

/// Implement the spikard_http::Handler trait for PythonHandler
impl Handler for PythonHandler {
    fn prefers_raw_json_body(&self) -> bool {
        true
    }

    fn prefers_parameter_extraction(&self) -> bool {
        false
    }

    fn wants_headers(&self) -> bool {
        if self.requires_headers {
            return true;
        }

        match self.handler_params.as_ref() {
            None => true,
            // Even when a handler doesn't request `headers`, we still need `Content-Type`
            // to correctly interpret `raw_body` (e.g. to set the `_raw_json` marker for msgspec).
            Some(allowed) => allowed.contains("headers") || allowed.contains(&self.body_param_name),
        }
    }

    fn wants_cookies(&self) -> bool {
        if self.requires_cookies {
            return true;
        }

        match self.handler_params.as_ref() {
            None => true,
            Some(allowed) => allowed.contains("cookies"),
        }
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
/// the `handler` signature (`convert_params`).
fn validated_params_to_py_kwargs<'py>(
    py: Python<'py>,
    validated_params: &Value,
    request_data: &RequestData,
    handler: Bound<'py, PyAny>,
    body_param_name: &str,
    needs_param_conversion: bool,
    handler_params: Option<&HashSet<String>>,
) -> PyResult<Bound<'py, PyDict>> {
    let params_dict = json_to_python(py, validated_params)?;
    let params_dict: Bound<'_, PyDict> = params_dict.extract()?;

    if let Some(raw_bytes) = &request_data.raw_body {
        if needs_param_conversion {
            params_dict.set_item("_raw_body", pyo3::types::PyBytes::new(py, raw_bytes))?;
        }

        if is_json_content_type(&request_data.headers) {
            if !needs_param_conversion {
                // PERFORMANCE: Clone Arc reference for try_unwrap; if sole owner, no clone occurs
                let body_value = if !request_data.body.is_null() {
                    Arc::try_unwrap(Arc::clone(&request_data.body)).unwrap_or_else(|arc| (*arc).clone())
                } else {
                    serde_json::from_slice::<Value>(raw_bytes)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON body: {e}")))?
                };
                let py_body = json_to_python(py, &body_value)?;
                if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                    params_dict.set_item(body_param_name, py_body)?;
                }
            } else {
                if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                    params_dict.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
                }
                params_dict.set_item("_raw_json", true)?;
            }
        } else if !request_data.body.is_null() {
            let py_body = json_to_python(py, &request_data.body)?;
            if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                params_dict.set_item(body_param_name, py_body)?;
            }
        } else if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
            params_dict.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
        }
    } else if !request_data.body.is_null() {
        let py_body = json_to_python(py, &request_data.body)?;
        if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
            params_dict.set_item(body_param_name, py_body)?;
        }
    }

    #[cfg(feature = "di")]
    inject_di_dependencies(py, &params_dict, request_data, handler_params)?;

    let result = if needs_param_conversion {
        convert_params(py, params_dict, handler)?
    } else if let Some(allowed) = handler_params {
        let filtered = PyDict::new(py);
        for name in allowed {
            if let Some(value) = params_dict.get_item(name)? {
                filtered.set_item(name, value)?;
            }
        }
        filtered
    } else {
        params_dict
    };

    strip_internal_keys(&result)?;
    Ok(result)
}

/// Convert Python object to Response`Result`
///
/// Uses ResponseInterpreter to detect streaming, custom, or plain JSON responses.
/// This consolidates response interpretation logic shared across all bindings.
fn python_to_response_result(
    py: Python<'_>,
    obj: &Bound<'_, PyAny>,
    prefer_msgspec_json: bool,
) -> PyResult<ResponseResult> {
    let interpreter = PyResponseInterpreter;
    let obj_py = obj.clone().unbind();

    match interpreter.interpret(&obj_py)? {
        spikard_bindings_shared::InterpretedResponse::Streaming { .. } => {
            // Convert the StreamSource trait object back to HandlerResponse
            // We need to handle the streaming case specially since StreamingResponse
            // already does the conversion
            if obj.is_instance_of::<StreamingResponse>() {
                let streaming: Py<StreamingResponse> = obj.extract()?;
                let handler_response = streaming.borrow(py).to_handler_response(py)?;
                Ok(ResponseResult::Stream(handler_response))
            } else {
                // For generic Python iterators, we'd need additional handling
                // For now, we don't support raw Python iterators without StreamingResponse
                // This maintains backward compatibility
                Err(pyo3::exceptions::PyTypeError::new_err(
                    "Streaming responses must be StreamingResponse instances or generators wrapped in StreamingResponse",
                ))
            }
        }
        spikard_bindings_shared::InterpretedResponse::Custom {
            status, headers, body, ..
        } => {
            let content = body.unwrap_or(Value::Null);
            Ok(ResponseResult::Custom {
                content,
                status_code: status,
                headers,
            })
        }
        spikard_bindings_shared::InterpretedResponse::Plain { body } => {
            if prefer_msgspec_json {
                let bytes = msgspec_json_encode(py, obj)?;
                let mut headers = HashMap::new();
                headers.insert("content-type".to_string(), "application/json".to_string());
                Ok(ResponseResult::Raw {
                    body: bytes,
                    status_code: 200,
                    headers,
                })
            } else {
                Ok(ResponseResult::Json(body))
            }
        }
    }
}

/// Inject DI dependencies into kwargs dict
///
/// Extracts resolved dependencies from request_data and adds them to the kwargs
/// dict so they can be passed to the Python `handler`.
#[cfg(feature = "di")]
fn inject_di_dependencies<'py>(
    py: Python<'py>,
    kwargs: &Bound<'py, PyDict>,
    request_data: &RequestData,
    handler_params: Option<&HashSet<String>>,
) -> PyResult<()> {
    if let Some(ref dependencies) = request_data.dependencies {
        let keys = dependencies.keys();

        for key in keys {
            if handler_params.is_some_and(|set| !set.contains(&key)) {
                continue;
            }
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
    needs_param_conversion: bool,
    handler_params: Option<&HashSet<String>>,
) -> PyResult<Bound<'py, PyDict>> {
    let kwargs = PyDict::new(py);

    if handler_params.is_none() || handler_params.is_some_and(|set| set.contains("path_params")) {
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
    }

    if handler_params.is_none() || handler_params.is_some_and(|set| set.contains("query_params")) {
        if let Value::Object(query_map) = &*request_data.query_params {
            let query_params = PyDict::new(py);
            for (key, value) in query_map {
                let py_value = json_to_python(py, value)?;
                query_params.set_item(key.as_str(), py_value)?;
            }
            kwargs.set_item("query_params", query_params)?;
        } else {
            kwargs.set_item("query_params", PyDict::new(py))?;
        }
    }

    if handler_params.is_none() || handler_params.is_some_and(|set| set.contains("headers")) {
        let headers_dict = PyDict::new(py);
        for (k, v) in request_data.headers.iter() {
            headers_dict.set_item(k, v)?;
        }
        kwargs.set_item("headers", headers_dict)?;
    }

    if handler_params.is_none() || handler_params.is_some_and(|set| set.contains("cookies")) {
        let cookies_dict = PyDict::new(py);
        for (k, v) in request_data.cookies.iter() {
            cookies_dict.set_item(k, v)?;
        }
        kwargs.set_item("cookies", cookies_dict)?;
    }

    if let Some(raw_bytes) = &request_data.raw_body {
        // Only expose these internal fields when conversion logic is enabled.
        if needs_param_conversion {
            kwargs.set_item("_raw_body", pyo3::types::PyBytes::new(py, raw_bytes))?;
        }

        if is_json_content_type(&request_data.headers) {
            if !needs_param_conversion {
                // PERFORMANCE: Clone Arc reference for try_unwrap; if sole owner, no clone occurs
                let body_value = if !request_data.body.is_null() {
                    Arc::try_unwrap(Arc::clone(&request_data.body)).unwrap_or_else(|arc| (*arc).clone())
                } else {
                    serde_json::from_slice::<Value>(raw_bytes)
                        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON body: {e}")))?
                };
                let py_body = json_to_python(py, &body_value)?;
                if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                    kwargs.set_item(body_param_name, py_body)?;
                }
            } else {
                // Keep the fast path for JSON: pass raw bytes through so Python can decode via msgspec.
                if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                    kwargs.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
                }
                kwargs.set_item("_raw_json", true)?;
            }
        } else if !request_data.body.is_null() {
            // For non-JSON payloads (multipart/urlencoded), the Rust middleware already parsed
            // the body into JSON-like builtins; prefer that over raw bytes.
            let py_body = json_to_python(py, &request_data.body)?;
            if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
                kwargs.set_item(body_param_name, py_body)?;
            }
        } else if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
            kwargs.set_item(body_param_name, pyo3::types::PyBytes::new(py, raw_bytes))?;
        }
    } else if handler_params.is_none() || handler_params.is_some_and(|set| set.contains(body_param_name)) {
        let py_body = json_to_python(py, &request_data.body)?;
        kwargs.set_item(body_param_name, py_body)?;
    }

    #[cfg(feature = "di")]
    inject_di_dependencies(py, &kwargs, request_data, handler_params)?;

    let result = if needs_param_conversion || handler_params.is_none() {
        convert_params(py, kwargs, handler)?
    } else {
        kwargs
    };

    strip_internal_keys(&result)?;
    Ok(result)
}

fn strip_internal_keys(kwargs: &Bound<'_, PyDict>) -> PyResult<()> {
    for key in ["_raw_body", "_raw_json"] {
        if kwargs.contains(key)? {
            kwargs.del_item(key)?;
        }
    }
    Ok(())
}

// (intentionally no trailing items)
