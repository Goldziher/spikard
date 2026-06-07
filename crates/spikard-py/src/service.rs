#![allow(clippy::too_many_arguments, clippy::unused_async)]

use axum::body::Body;
use axum::http::Request;
use pyo3::prelude::*;
use pyo3::types::{PyList, PyString, PyTuple};
use serde_json;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
/// Generated pyo3 bridge for the `Handler` contract.
///
/// Wraps a Python callable (sync or async) so it can be used
/// as `Arc<dyn Handler>` from Rust async code.
pub struct PyHandlerBridge {
    callable: Py<PyAny>,
    is_async: bool,
}

impl PyHandlerBridge {
    /// Create a bridge from a Python callable.
    pub fn new(py: Python<'_>, callable: &Bound<'_, PyAny>) -> PyResult<Self> {
        let is_async = py
            .import("inspect")?
            .call_method1("iscoroutinefunction", (callable,))?
            .is_truthy()
            .unwrap_or(false);
        Ok(Self {
            callable: callable.clone().unbind(),
            is_async,
        })
    }
}

// SAFETY: Py<PyAny> is Send+Sync when we never alias it without the GIL.
unsafe impl Send for PyHandlerBridge {}
unsafe impl Sync for PyHandlerBridge {}
impl spikard::Handler for PyHandlerBridge {
    fn call(
        &self,
        _request: spikard::Request<spikard::Body>,
        request_data: spikard::RequestData,
    ) -> Pin<Box<dyn Future<Output = spikard::HandlerResult> + Send + '_>> {
        // Acquire Python GIL in a thread-safe context before entering the async block.
        // Py<PyAny> holds a GIL-independent reference that can be used outside the GIL.
        let callable = pyo3::Python::attach(|py| self.callable.clone_ref(py));
        let is_async = self.is_async;

        Box::pin(async move {
            let outcome: Result<spikard::Response, Box<dyn std::error::Error + Send + Sync>> = async move {
                // Serialize the request to a Python-friendly dict via serde_json
                let req_json = serde_json::to_string(&request_data)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

                let raw_result = if is_async {
                    // Async callable: hand off to pyo3_async_runtimes so it drives
                    // the Python event loop without blocking the Tokio executor.
                    let future = pyo3::Python::attach(|py| -> PyResult<_> {
                        let req_obj = py.import("json")?.call_method1("loads", (&req_json,))?;
                        let coro = callable.call1(py, (req_obj,))?;
                        pyo3_async_runtimes::tokio::into_future(coro.into_bound(py))
                    })
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    let py_result = future
                        .await
                        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                    pyo3::Python::attach(|py| {
                        let json_mod = py.import("json")?;
                        let json_str: String = json_mod.call_method1("dumps", (py_result.bind(py),))?.extract()?;
                        Ok::<String, PyErr>(json_str)
                    })
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                } else {
                    // Sync callable: run in a blocking thread so we never hold the GIL
                    // on the async executor.
                    tokio::task::spawn_blocking(move || {
                        pyo3::Python::attach(|py| {
                            let req_obj = py.import("json")?.call_method1("loads", (&req_json,))?;
                            let result = callable.call1(py, (req_obj,))?;
                            let json_mod = py.import("json")?;
                            let json_str: String = json_mod.call_method1("dumps", (result.bind(py),))?.extract()?;
                            Ok::<String, PyErr>(json_str)
                        })
                    })
                    .await
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
                };

                // Deserialize the JSON result back into the wire response DTO.
                let response: spikard::Response = serde_json::from_str(&raw_result)
                    .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
                Ok(response)
            }
            .await;

            spikard::handler_result_from_response(outcome)
        })
    }
}
/// Drive `spikard::App::run` from Python.
///
/// Each entry in `registrations` is a `(method_name, metadata_tuple, callable)` triple
/// produced by the Python service class.
#[pyfunction]
pub fn app_run(_py: Python<'_>, registrations: &Bound<'_, PyList>) -> PyResult<()> {
    let mut owner = spikard::App::new();

    for entry in registrations.iter() {
        let tuple: &Bound<'_, PyTuple> = entry.downcast()?;
        let method_name: String = tuple.get_item(0)?.extract()?;
        let callable = tuple.get_item(2)?;

        match method_name.as_str() {
            "route" => {
                let bridge = PyHandlerBridge::new(_py, &callable)?;
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_item = tuple.get_item(1)?;
                let meta: &Bound<'_, PyTuple> = meta_item.downcast()?;
                let builder_binding: crate::RouteBuilder = meta.get_item(0)?.extract()?;
                let builder: spikard::RouteBuilder = (*builder_binding.inner).clone();
                owner
                    .route(builder, handler)
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unknown registration method: {method_name}"
                )));
            }
        }
    }

    _py.detach(|| pyo3_async_runtimes::tokio::get_runtime().block_on(owner.run()))
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}
/// Drive `spikard::App::into_router` from Python.
///
/// Each entry in `registrations` is a `(method_name, metadata_tuple, callable)` triple
/// produced by the Python service class.
#[pyfunction]
pub fn app_into_router(_py: Python<'_>, registrations: &Bound<'_, PyList>) -> PyResult<()> {
    let mut owner = spikard::App::new();

    for entry in registrations.iter() {
        let tuple: &Bound<'_, PyTuple> = entry.downcast()?;
        let method_name: String = tuple.get_item(0)?.extract()?;
        let callable = tuple.get_item(2)?;

        match method_name.as_str() {
            "route" => {
                let bridge = PyHandlerBridge::new(_py, &callable)?;
                let handler: Arc<dyn spikard::Handler> = Arc::new(bridge);
                let meta_item = tuple.get_item(1)?;
                let meta: &Bound<'_, PyTuple> = meta_item.downcast()?;
                let builder_binding: crate::RouteBuilder = meta.get_item(0)?.extract()?;
                let builder: spikard::RouteBuilder = (*builder_binding.inner).clone();
                owner
                    .route(builder, handler)
                    .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            }
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(format!(
                    "unknown registration method: {method_name}"
                )));
            }
        }
    }

    let _ = owner
        .into_router()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}
