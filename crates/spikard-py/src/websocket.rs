//! Python WebSocket handler bindings

use pyo3::prelude::*;
use serde_json::Value;
use spikard_http::WebSocketHandler;
use std::sync::Arc;
use tracing::{debug, error};

/// Python implementation of WebSocketHandler
pub struct PythonWebSocketHandler {
    /// Python handler instance wrapped in Arc for cheap cloning
    handler: Arc<Py<PyAny>>,
}

impl PythonWebSocketHandler {
    /// Create a new Python WebSocket handler
    pub fn new(handler: Py<PyAny>) -> Self {
        Self {
            handler: Arc::new(handler),
        }
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
        debug!("Python WebSocket handler: handle_message");

        let handler = Arc::clone(&self.handler);

        // Run in blocking task with asyncio.run() like SSE
        let result = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<Option<Value>> {
                // Convert JSON Value to Python dict
                let py_message = Self::json_to_python(py, &message)?;

                // Call the handler's handle_message method
                let coroutine = handler.bind(py).call_method1("handle_message", (py_message,))?;
                debug!("Python WebSocket handler: called handle_message method");

                // Run the coroutine using asyncio.run()
                let asyncio = py.import("asyncio")?;
                let result = asyncio.call_method1("run", (coroutine,))?;
                debug!("Python WebSocket handler: asyncio.run() completed");

                // Check if result is None
                if result.is_none() {
                    debug!("Python WebSocket handler: received None response");
                    return Ok(None);
                }

                // Convert Python result to JSON Value
                let json_val = Self::python_to_json(py, &result)?;
                Ok(Some(json_val))
            })
        })
        .await;

        match result {
            Ok(Ok(value)) => value,
            Ok(Err(e)) => {
                error!("Python error in handle_message: {}", e);
                None
            }
            Err(e) => {
                error!("Tokio error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python WebSocket handler: on_connect");

        let handler = Arc::clone(&self.handler);

        let _ = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<()> {
                debug!("Python WebSocket handler: on_connect acquired GIL");
                let coroutine = handler.bind(py).call_method0("on_connect")?;
                let asyncio = py.import("asyncio")?;
                asyncio.call_method1("run", (coroutine,))?;
                debug!("Python WebSocket handler: on_connect completed");
                Ok(())
            })
        })
        .await;
    }

    async fn on_disconnect(&self) {
        debug!("Python WebSocket handler: on_disconnect");

        let handler = Arc::clone(&self.handler);

        let _ = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<()> {
                let coroutine = handler.bind(py).call_method0("on_disconnect")?;
                let asyncio = py.import("asyncio")?;
                asyncio.call_method1("run", (coroutine,))?;
                debug!("Python WebSocket handler: on_disconnect completed");
                Ok(())
            })
        })
        .await;
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
