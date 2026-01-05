//! Python WebSocket `handler` bindings

use crate::conversion::{json_to_python, python_to_json};
use pyo3::prelude::*;
use serde_json::Value;
use spikard_http::WebSocketHandler;
use std::sync::Arc;
use tracing::{debug, error};

/// Python implementation of WebSocketHandler
pub struct PythonWebSocketHandler {
    /// Python `handler` instance wrapped in `Arc` for cheap cloning
    handler: Arc<Py<PyAny>>,
}

impl PythonWebSocketHandler {
    /// Create a new Python WebSocket `handler`
    pub fn new(handler: Py<PyAny>) -> Self {
        Self {
            handler: Arc::new(handler),
        }
    }
}

impl WebSocketHandler for PythonWebSocketHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        debug!("Python WebSocket handler: handle_message");

        let handler = Arc::clone(&self.handler);
        let message = message.clone();

        let result_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let py_message = json_to_python(py, &message)?;
            let result_or_coroutine = handler.bind(py).call_method1("handle_message", (py_message,))?;
            debug!("Python WebSocket handler: called handle_message method");
            Ok(result_or_coroutine.unbind())
        });

        match result_py {
            Ok(result_py) => {
                let resolved = Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let inspect = py.import("inspect")?;
                    let awaitable = result_py.bind(py);
                    let is_awaitable: bool = inspect.call_method1("isawaitable", (awaitable,))?.extract()?;
                    if is_awaitable {
                        let asyncio = py.import("asyncio")?;
                        let loop_obj = asyncio.call_method0("new_event_loop")?;
                        asyncio.call_method1("set_event_loop", (loop_obj.clone(),))?;
                        let awaited = loop_obj.call_method1("run_until_complete", (awaitable,))?;
                        Ok(awaited.unbind())
                    } else {
                        Ok(awaitable.clone().unbind())
                    }
                });

                match resolved {
                    Ok(obj) => {
                        let is_none = Python::attach(|py| obj.bind(py).is_none());
                        if is_none {
                            debug!("Python WebSocket handler: received None response");
                            return None;
                        }
                        Python::attach(|py| python_to_json(py, obj.bind(py)).ok())
                    }
                    Err(e) => {
                        error!("Python error resolving websocket message: {}", e);
                        Python::attach(|py| e.print(py));
                        None
                    }
                }
            }
            Err(e) => {
                error!("Python error in handle_message: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python WebSocket handler: on_connect");

        let handler = Arc::clone(&self.handler);

        let coroutine_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            debug!("Python WebSocket handler: on_connect acquired GIL");
            let coroutine = handler.bind(py).call_method0("on_connect")?;
            Ok(coroutine.unbind())
        });

        if let Ok(coroutine) = coroutine_py {
            let _ = Python::attach(|py| {
                let asyncio = py.import("asyncio")?;
                let loop_obj = asyncio.call_method0("new_event_loop")?;
                asyncio.call_method1("set_event_loop", (loop_obj.clone(),))?;
                let _ = loop_obj.call_method1("run_until_complete", (coroutine.bind(py),))?;
                Ok::<(), PyErr>(())
            });
            debug!("Python WebSocket handler: on_connect completed");
        } else {
            error!("Failed to call on_connect");
        }
    }

    async fn on_disconnect(&self) {
        debug!("Python WebSocket handler: on_disconnect");

        let handler = Arc::clone(&self.handler);

        let coroutine_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let coroutine = handler.bind(py).call_method0("on_disconnect")?;
            Ok(coroutine.unbind())
        });

        if let Ok(coroutine) = coroutine_py {
            let _ = Python::attach(|py| {
                let asyncio = py.import("asyncio")?;
                let loop_obj = asyncio.call_method0("new_event_loop")?;
                asyncio.call_method1("set_event_loop", (loop_obj.clone(),))?;
                let _ = loop_obj.call_method1("run_until_complete", (coroutine.bind(py),))?;
                Ok::<(), PyErr>(())
            });
            debug!("Python WebSocket handler: on_disconnect completed");
        } else {
            error!("Failed to call on_disconnect");
        }
    }
}

/// Create WebSocketState from Python `handler` factory
pub fn create_websocket_state(
    factory: &Bound<'_, PyAny>,
) -> PyResult<spikard_http::WebSocketState<PythonWebSocketHandler>> {
    let handler_instance = factory.call0()?;

    let message_schema = handler_instance.getattr("_message_schema").ok().and_then(|attr| {
        if attr.is_none() {
            None
        } else {
            handler_instance.py().import("json").ok().and_then(|json_module| {
                json_module
                    .call_method1("dumps", (attr,))
                    .ok()
                    .and_then(|json_str: Bound<'_, PyAny>| {
                        let json_string: String = json_str.extract().ok()?;
                        serde_json::from_str(&json_string).ok()
                    })
            })
        }
    });

    let response_schema = handler_instance.getattr("_response_schema").ok().and_then(|attr| {
        if attr.is_none() {
            None
        } else {
            handler_instance.py().import("json").ok().and_then(|json_module| {
                json_module
                    .call_method1("dumps", (attr,))
                    .ok()
                    .and_then(|json_str: Bound<'_, PyAny>| {
                        let json_string: String = json_str.extract().ok()?;
                        serde_json::from_str(&json_string).ok()
                    })
            })
        }
    });

    let py_handler = PythonWebSocketHandler::new(handler_instance.unbind());

    if message_schema.is_some() || response_schema.is_some() {
        #[allow(clippy::redundant_closure)]
        spikard_http::WebSocketState::with_schemas(py_handler, message_schema, response_schema)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))
    } else {
        Ok(spikard_http::WebSocketState::new(py_handler))
    }
}
