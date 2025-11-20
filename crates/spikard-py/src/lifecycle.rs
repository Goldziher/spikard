//! Python lifecycle hooks implementation
//!
//! This module provides the bridge between Python async functions and Rust's lifecycle hook system.
//! Async Python functions are executed using asyncio.run() in blocking tasks, matching the
//! pattern used in handler.rs for consistency.

use axum::{
    body::Body,
    http::{Request, Response},
};
use pyo3::prelude::*;
use spikard_http::lifecycle::{HookResult, LifecycleHook};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::request::PyRequest;
use crate::response::Response as PyResponse;

/// Python lifecycle hook wrapper
///
/// Wraps a Python async function and makes it callable from Rust's lifecycle system.
/// Handles conversion between Rust HTTP types and Python Request/Response objects.
pub struct PythonHook {
    name: String,
    /// Python async function: async def hook(request) -> Request | Response
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
        &'a self,
        req: Request<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Request<Body>, Response<Body>>, String>> + Send + 'a>> {
        // Clone the func for use across threads
        let func = Python::attach(|py| self.func.clone_ref(py));
        let name = self.name.clone();

        Box::pin(async move {
            // Run Python async function in a blocking task
            let result = tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<HookResult<Request<Body>, Response<Body>>> {
                    // Convert Rust request to Python Request
                    let py_req = Py::new(py, PyRequest::from_request(req, py)?)?;

                    // Call the Python function
                    let result = func.call1(py, (py_req.bind(py),))?;

                    // Check if it's a coroutine (async function)
                    if result.bind(py).hasattr("__await__")? {
                        // Run the coroutine using asyncio.run()
                        let asyncio = py.import("asyncio")?;
                        let completed_result = asyncio.call_method1("run", (result,))?;

                        // Check if result is Request or Response
                        if completed_result.is_instance_of::<PyResponse>() {
                            let py_response: PyResponse = completed_result.extract()?;
                            let response = py_response.to_response(py)?;
                            return Ok(HookResult::ShortCircuit(response));
                        }

                        if completed_result.is_instance_of::<PyRequest>() {
                            let py_request: PyRequest = completed_result.extract()?;
                            let request = py_request.to_request(py)?;
                            return Ok(HookResult::Continue(request));
                        }

                        let type_name = completed_result
                            .get_type()
                            .name()
                            .map(|n| n.to_string())
                            .unwrap_or_else(|_| "unknown".to_string());
                        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Hook must return Request or Response, got {}",
                            type_name
                        )));
                    }

                    // Synchronous function - check result directly
                    if result.bind(py).is_instance_of::<PyResponse>() {
                        let py_response: PyResponse = result.extract(py)?;
                        let response = py_response.to_response(py)?;
                        return Ok(HookResult::ShortCircuit(response));
                    }

                    if result.bind(py).is_instance_of::<PyRequest>() {
                        let py_request: PyRequest = result.extract(py)?;
                        let request = py_request.to_request(py)?;
                        return Ok(HookResult::Continue(request));
                    }

                    let type_name = result
                        .bind(py)
                        .get_type()
                        .name()
                        .map(|n| n.to_string())
                        .unwrap_or_else(|_| "unknown".to_string());
                    Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                        "Hook must return Request or Response, got {}",
                        type_name
                    )))
                })
            })
            .await
            .map_err(|e| format!("Hook '{}' task error: {}", name, e))?
            .map_err(|e: PyErr| format!("Hook '{}' Python error: {}", name, e))?;

            Ok(result)
        })
    }

    fn execute_response<'a>(
        &'a self,
        resp: Response<Body>,
    ) -> Pin<Box<dyn Future<Output = Result<HookResult<Response<Body>, Response<Body>>, String>> + Send + 'a>> {
        // Clone the func for use across threads
        let func = Python::attach(|py| self.func.clone_ref(py));
        let name = self.name.clone();

        Box::pin(async move {
            // Buffer the response body BEFORE entering blocking task
            // This is necessary because Body is an async stream
            let (parts, body) = resp.into_parts();
            use axum::body::to_bytes;
            let body_bytes = to_bytes(body, usize::MAX)
                .await
                .map_err(|e| format!("Failed to buffer response body: {}", e))?;

            // Run Python async function in a blocking task
            let result = tokio::task::spawn_blocking(move || {
                Python::attach(|py| -> PyResult<HookResult<Response<Body>, Response<Body>>> {
                    // Convert Rust response to Python Response with buffered body
                    let py_resp = Py::new(py, PyResponse::from_response_parts(parts, body_bytes.clone(), py)?)?;

                    // Call the Python function
                    let result = func.call1(py, (py_resp.bind(py),))?;

                    // Check if it's a coroutine (async function)
                    if result.bind(py).hasattr("__await__")? {
                        // Run the coroutine using asyncio.run()
                        let asyncio = py.import("asyncio")?;
                        let completed_result = asyncio.call_method1("run", (result,))?;

                        // Must return a Response
                        if !completed_result.is_instance_of::<PyResponse>() {
                            let type_name = completed_result
                                .get_type()
                                .name()
                                .map(|n| n.to_string())
                                .unwrap_or_else(|_| "unknown".to_string());
                            return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                                "Hook must return Response, got {}",
                                type_name
                            )));
                        }

                        let py_response: PyResponse = completed_result.extract()?;
                        let response = py_response.to_response(py)?;
                        return Ok(HookResult::Continue(response));
                    }

                    // Synchronous function - check result directly
                    if !result.bind(py).is_instance_of::<PyResponse>() {
                        let type_name = result
                            .bind(py)
                            .get_type()
                            .name()
                            .map(|n| n.to_string())
                            .unwrap_or_else(|_| "unknown".to_string());
                        return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(format!(
                            "Hook must return Response, got {}",
                            type_name
                        )));
                    }

                    let py_response: PyResponse = result.extract(py)?;
                    let response = py_response.to_response(py)?;
                    Ok(HookResult::Continue(response))
                })
            })
            .await
            .map_err(|e| format!("Hook '{}' task error: {}", name, e))?
            .map_err(|e: PyErr| format!("Hook '{}' Python error: {}", name, e))?;

            Ok(result)
        })
    }
}

/// Build LifecycleHooks from Python configuration
///
/// Extracts hook functions from Python dict and wraps them in PythonHook instances.
pub fn build_lifecycle_hooks(_py: Python, config: &Bound<'_, PyAny>) -> PyResult<spikard_http::LifecycleHooks> {
    let mut hooks = spikard_http::LifecycleHooks::new();
    type PyHookVec = Vec<Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>>;

    // Helper to extract hooks from a list
    let extract_hooks = |hook_list: &Bound<'_, PyAny>, hook_type: &str| -> PyResult<PyHookVec> {
        let mut result = Vec::new();

        if hook_list.is_none() {
            return Ok(result);
        }

        let list = hook_list
            .cast_exact::<pyo3::types::PyList>()
            .map_err(pyo3::PyErr::from)?;
        for (i, item) in list.iter().enumerate() {
            let name = format!("{}_hook_{}", hook_type, i);
            let func = item.clone().unbind();
            result.push(Arc::new(PythonHook::new(name, func)) as Arc<dyn LifecycleHook<Request<Body>, Response<Body>>>);
        }

        Ok(result)
    };

    // Extract each hook type from the config dict
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
