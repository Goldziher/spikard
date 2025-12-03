//! Python WebSocket handler bindings

use crate::conversion::{json_to_python, python_to_json};
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
}

impl WebSocketHandler for PythonWebSocketHandler {
    async fn handle_message(&self, message: Value) -> Option<Value> {
        debug!("Python WebSocket handler: handle_message");

        let handler = Arc::clone(&self.handler);
        let message = message.clone();

        // First, call the handler and get the result (sync or coroutine)
        let result_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let py_message = json_to_python(py, &message)?;
            let result_or_coroutine = handler.bind(py).call_method1("handle_message", (py_message,))?;
            debug!("Python WebSocket handler: called handle_message method");
            Ok(result_or_coroutine.unbind())
        });

        match result_py {
            Ok(result_py) => {
                // Check if it's a coroutine and handle accordingly
                let is_coroutine = Python::attach(|py| -> PyResult<bool> {
                    let asyncio = py.import("asyncio")?;
                    asyncio.call_method1("iscoroutine", (result_py.bind(py),))?.extract()
                })
                .unwrap_or(false);

                if is_coroutine {
                    debug!("Python WebSocket handler: result is a coroutine, awaiting...");
                    let future_result =
                        Python::attach(|py| pyo3_async_runtimes::tokio::into_future(result_py.bind(py).clone()));

                    match future_result {
                        Ok(future) => match future.await {
                            Ok(result) => {
                                debug!("Python WebSocket handler: coroutine completed");
                                let is_none = Python::attach(|py| result.bind(py).is_none());
                                if is_none {
                                    debug!("Python WebSocket handler: received None response");
                                    return None;
                                }
                                Python::attach(|py| python_to_json(py, result.bind(py)).ok())
                            }
                            Err(e) => {
                                error!("Python error in coroutine: {}", e);
                                None
                            }
                        },
                        Err(e) => {
                            error!("Failed to convert coroutine to future: {}", e);
                            None
                        }
                    }
                } else {
                    // Synchronous result
                    debug!("Python WebSocket handler: synchronous result");
                    let is_none = Python::attach(|py| result_py.bind(py).is_none());
                    if is_none {
                        debug!("Python WebSocket handler: received None response");
                        return None;
                    }
                    Python::attach(|py| python_to_json(py, result_py.bind(py)).ok())
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
            let future_result =
                Python::attach(|py| pyo3_async_runtimes::tokio::into_future(coroutine.bind(py).clone()));

            if let Ok(future) = future_result {
                let _ = future.await;
                debug!("Python WebSocket handler: on_connect completed");
            } else {
                error!("Failed to convert on_connect coroutine to future");
            }
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
            let future_result =
                Python::attach(|py| pyo3_async_runtimes::tokio::into_future(coroutine.bind(py).clone()));

            if let Ok(future) = future_result {
                let _ = future.await;
                debug!("Python WebSocket handler: on_disconnect completed");
            } else {
                error!("Failed to convert on_disconnect coroutine to future");
            }
        } else {
            error!("Failed to call on_disconnect");
        }
    }
}

/// Create WebSocketState from Python handler factory
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
