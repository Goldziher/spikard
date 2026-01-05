//! Python lifecycle hooks implementation
//!
//! This module provides the bridge between Python async functions and Rust's lifecycle hook system.
//! Async Python functions are executed using asyncio.run() in blocking tasks, matching the
//! pattern used in `handler`.rs for consistency.

use axum::{
    body::Body,
    http::{Request, Response},
};
use pyo3::prelude::*;
use spikard_http::lifecycle::adapter::error;
use spikard_http::lifecycle::{HookResult, LifecycleHook};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::request::PyRequest;
use crate::response::Response as PyResponse;

/// Python lifecycle hook wrapper
///
/// Wraps a Python async function and makes it callable from Rust's lifecycle system.
/// Handles conversion between Rust HTTP types and Python `Request`/`Response` objects.
pub struct PythonHook {
    name: String,
    /// Python async function: async def hook(request) -> `Request` | `Response`
    func: Py<PyAny>,
}

impl PythonHook {
    /// Create a new Python hook
    pub fn new(name: String, func: Py<PyAny>) -> Self {
        Self { name, func }
    }
}

impl LifecycleHook<Request<Body>, Response<Body>> for PythonHook {
    fn name(&self) -> &str {
        &self.name
    }

    fn execute_request<'a>(
        &self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = Python::attach(|py| self.func.clone_ref(py));
        let name = self.name.clone();

        Box::pin(async move {
            let result = Python::attach(|py| -> PyResult<Py<PyAny>> {
                let py_req = Py::new(py, PyRequest::from_request(req, py)?)?;
                let result = func.call1(py, (py_req.as_ref(),))?;
                Ok(result)
            })
            .map_err(|e| error::python_error(&name, e))?;

            let processed = Python::attach(|py| -> PyResult<_> {
                let resolved = resolve_hook_result(py, result.clone_ref(py), &name)?;
                execute_python_hook_request(py, resolved, &name)
            })
            .map_err(|e| error::python_error(&name, e))?;

            Ok(processed)
        })
    }

    fn execute_response<'a>(
        &self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        let func = Python::attach(|py| self.func.clone_ref(py));
        let name = self.name.clone();

        Box::pin(async move {
            let (parts, body) = resp.into_parts();
            let body_bytes = spikard_http::lifecycle::adapter::serial::extract_body(body).await?;

            let result = Python::attach(|py| -> PyResult<Py<PyAny>> {
                let py_resp = Py::new(py, PyResponse::from_response_parts(parts, body_bytes.clone(), py)?)?;
                let result = func.call1(py, (py_resp.as_ref(),))?;
                Ok(result)
            })
            .map_err(|e| error::python_error(&name, e))?;

            let processed = Python::attach(|py| -> PyResult<_> {
                let resolved = resolve_hook_result(py, result.clone_ref(py), &name)?;
                execute_python_hook_response(py, resolved, &name)
            })
            .map_err(|e| error::python_error(&name, e))?;

            Ok(processed)
        })
    }
}

/// Resolve a hook return value, awaiting it if necessary.
fn resolve_hook_result(py: Python<'_>, result: Py<PyAny>, name: &str) -> PyResult<Py<PyAny>> {
    let bound_result = result.bind(py);
    let inspect = py.import("inspect")?;
    let is_awaitable: bool = inspect
        .call_method1("isawaitable", (bound_result.clone(),))?
        .extract()?;

    if is_awaitable {
        let asyncio = py.import("asyncio")?;
        let awaited = asyncio
            .call_method1("run", (bound_result.clone(),))
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Hook {name} await failed: {e}")))?;
        Ok(awaited.unbind())
    } else {
        Ok(bound_result.clone().unbind())
    }
}

/// Execute a Python hook with a request, handling both sync and async results
fn execute_python_hook_request(
    py: Python<'_>,
    result: Py<PyAny>,
    name: &str,
) -> PyResult<HookResult<Request<Body>, Response<Body>>> {
    let bound_result = result.clone_ref(py).into_bound(py);
    handle_python_hook_result(py, bound_result, name)
}

/// Execute a Python hook with a response, handling both sync and async results
fn execute_python_hook_response(
    py: Python<'_>,
    result: Py<PyAny>,
    name: &str,
) -> PyResult<HookResult<Response<Body>, Response<Body>>> {
    let bound_result = result.clone_ref(py).into_bound(py);
    validate_and_convert_response(py, bound_result, name)
}

/// Handle the result of a Python hook that was called with a request
fn handle_python_hook_result(
    py: Python<'_>,
    result: Bound<'_, PyAny>,
    name: &str,
) -> PyResult<HookResult<Request<Body>, Response<Body>>> {
    if result.is_instance_of::<PyResponse>() {
        let py_response: PyResponse = result.extract()?;
        let response = py_response.to_response(py)?;
        return Ok(HookResult::ShortCircuit(response));
    }

    if result.is_instance_of::<PyRequest>() {
        let py_request: PyRequest = result.extract()?;
        let request = py_request.to_request(py)?;
        return Ok(HookResult::Continue(request));
    }

    let type_name = result
        .get_type()
        .name()
        .map(|n| n.to_string())
        .unwrap_or_else(|_| "unknown".to_string());
    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
        "Hook {name} must return Request or Response, got {type_name}"
    )))
}

/// Validate and convert a response hook result
fn validate_and_convert_response(
    py: Python<'_>,
    result: Bound<'_, PyAny>,
    name: &str,
) -> PyResult<HookResult<Response<Body>, Response<Body>>> {
    if !result.is_instance_of::<PyResponse>() {
        let type_name = result
            .get_type()
            .name()
            .map(|n| n.to_string())
            .unwrap_or_else(|_| "unknown".to_string());
        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
            "Hook {name} must return Response, got {type_name}"
        )));
    }

    let py_response: PyResponse = result.extract()?;
    let response = py_response.to_response(py)?;
    Ok(HookResult::Continue(response))
}

/// Build LifecycleHooks from Python configuration
///
/// Extracts hook functions from Python dict and wraps them in PythonHook instances.
pub fn build_lifecycle_hooks(_py: Python, config: &Bound<'_, PyAny>) -> PyResult<spikard_http::LifecycleHooks> {
    type PyHookVec = Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>;

    let mut hooks = spikard_http::LifecycleHooks::new();

    let extract_hooks = |hook_list: &Bound<'_, PyAny>, hook_type: &str| -> PyResult<PyHookVec> {
        let mut result = Vec::new();

        if hook_list.is_none() {
            return Ok(result);
        }

        let list = hook_list
            .cast_exact::<pyo3::types::PyList>()
            .map_err(pyo3::PyErr::from)?;
        for (i, item) in list.iter().enumerate() {
            let name = format!("{hook_type}_hook_{i}");
            let func = item.clone().unbind();
            result.push(Arc::new(PythonHook::new(name, func)) as Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>);
        }

        Ok(result)
    };

    if let Ok(on_request) = config.get_item("on_request") {
        for hook in extract_hooks(&on_request, "on_request")? {
            hooks.add_on_request(hook);
        }
    }

    if let Ok(pre_validation) = config.get_item("pre_validation") {
        for hook in extract_hooks(&pre_validation, "pre_validation")? {
            hooks.add_pre_validation(hook);
        }
    }

    if let Ok(pre_handler) = config.get_item("pre_handler") {
        for hook in extract_hooks(&pre_handler, "pre_handler")? {
            hooks.add_pre_handler(hook);
        }
    }

    if let Ok(on_response) = config.get_item("on_response") {
        for hook in extract_hooks(&on_response, "on_response")? {
            hooks.add_on_response(hook);
        }
    }

    if let Ok(on_error) = config.get_item("on_error") {
        for hook in extract_hooks(&on_error, "on_error")? {
            hooks.add_on_error(hook);
        }
    }

    Ok(hooks)
}
