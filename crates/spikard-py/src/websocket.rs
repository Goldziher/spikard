//! Python WebSocket handler bindings

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::into_future;
use serde_json::Value;
use spikard_http::WebSocketHandler;
use tracing::{debug, error};

/// Python implementation of WebSocketHandler
pub struct PythonWebSocketHandler {
    /// Python handler instance
    handler: Py<PyAny>,
}

impl PythonWebSocketHandler {
    /// Create a new Python WebSocket handler
    pub fn new(handler: Py<PyAny>) -> Self {
        Self { handler }
    }

    /// Convert JSON Value to Python dict (zero-copy approach)
    fn json_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, pyo3::PyAny>> {
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
                    Ok(PyNone::get(py).as_any().clone())
                }
            }
            Value::String(s) => Ok(PyString::new(py, s).into_any()),
            Value::Array(arr) => {
                let list = PyList::empty(py);
                for item in arr {
                    list.append(Self::json_to_python(py, item)?)?;
                }
                Ok(list.into_any())
            }
            Value::Object(obj) => {
                let dict = PyDict::new(py);
                for (key, val) in obj {
                    dict.set_item(key, Self::json_to_python(py, val)?)?;
                }
                Ok(dict.into_any())
            }
        }
    }

    /// Convert Python object to JSON Value
    fn python_to_json(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
        // Serialize Python object to JSON string, then parse
        let json_module = py.import("json")?;
        let json_str: String = json_module.call_method1("dumps", (obj,))?.extract()?;
        serde_json::from_str(&json_str)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to convert to JSON: {}", e)))
    }
}

impl WebSocketHandler for PythonWebSocketHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        // Acquire GIL and prepare the async call
        let future = Python::attach(|py| {
            // Convert JSON Value to Python dict
            let py_message = match Self::json_to_python(py, &message) {
                Ok(msg) => msg,
                Err(e) => {
                    error!("Failed to convert message to Python: {}", e);
                    return None;
                }
            };

            // Call Python handler's handle_message method
            let result = match self.handler.bind(py).call_method1("handle_message", (py_message,)) {
                Ok(coro) => coro,
                Err(e) => {
                    error!("Failed to call handle_message: {}", e);
                    return None;
                }
            };

            // Convert coroutine to Rust future
            match into_future(result) {
                Ok(fut) => Some(fut),
                Err(e) => {
                    error!("Failed to convert coroutine to future: {}", e);
                    None
                }
            }
        });

        // If we couldn't create the future, return None
        let future = match future {
            Some(f) => f,
            None => return None,
        };

        // Await the future (GIL is released during await)
        match future.await {
            Ok(result) => {
                // Re-acquire GIL to process result
                Python::attach(|py| {
                    let result_bound = result.bind(py);

                    // Check if result is None
                    if result_bound.is_none() {
                        return None;
                    }

                    // Convert Python result to JSON Value
                    match Self::python_to_json(py, result_bound) {
                        Ok(json_val) => Some(json_val),
                        Err(e) => {
                            error!("Failed to convert response to JSON: {}", e);
                            None
                        }
                    }
                })
            }
            Err(e) => {
                error!("Error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python WebSocket handler: on_connect");
        let future_opt = Python::attach(|py| {
            if let Ok(coro) = self.handler.bind(py).call_method0("on_connect")
                && let Ok(future) = into_future(coro)
            {
                return Some(future);
            }
            None
        });

        if let Some(future) = future_opt {
            let _ = future.await;
        }
    }

    async fn on_disconnect(&self) {
        debug!("Python WebSocket handler: on_disconnect");
        let future_opt = Python::attach(|py| {
            if let Ok(coro) = self.handler.bind(py).call_method0("on_disconnect")
                && let Ok(future) = into_future(coro)
            {
                return Some(future);
            }
            None
        });

        if let Some(future) = future_opt {
            let _ = future.await;
        }
    }
}

/// Create WebSocketState from Python handler factory
pub fn create_websocket_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::WebSocketState<PythonWebSocketHandler>> {
    // Call the factory to get a handler instance
    let handler_instance = factory.call0()?;

    // Create Python WebSocket handler
    let py_handler = PythonWebSocketHandler::new(handler_instance.unbind());

    // Create and return WebSocket state
    Ok(spikard_http::WebSocketState::new(py_handler))
}
