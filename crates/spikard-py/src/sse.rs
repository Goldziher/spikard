//! Python SSE producer bindings

use pyo3::prelude::*;
use pyo3_async_runtimes::tokio::into_future;
use serde_json::Value;
use spikard_http::{SseEvent, SseEventProducer};
use tracing::{debug, error};

/// Python implementation of SseEventProducer
pub struct PythonSseEventProducer {
    /// Python producer instance
    producer: Py<PyAny>,
}

impl PythonSseEventProducer {
    /// Create a new Python SSE event producer
    pub fn new(producer: Py<PyAny>) -> Self {
        Self { producer }
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

impl SseEventProducer for PythonSseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        // Acquire GIL and prepare the async call
        let future = Python::attach(|py| {
            // Call Python producer's next_event method
            let result = match self.producer.bind(py).call_method0("next_event") {
                Ok(coro) => coro,
                Err(e) => {
                    error!("Failed to call next_event: {}", e);
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

                    // Check if result is None (end of stream)
                    if result_bound.is_none() {
                        return None;
                    }

                    // Extract SseEvent from Python object
                    // Expected structure: {data: dict, event_type: str | None, id: str | None, retry: int | None}
                    let data = match result_bound.getattr("data") {
                        Ok(d) => d,
                        Err(e) => {
                            error!("Failed to get data attribute: {}", e);
                            return None;
                        }
                    };

                    let data_json = match Self::python_to_json(py, &data) {
                        Ok(json) => json,
                        Err(e) => {
                            error!("Failed to convert data to JSON: {}", e);
                            return None;
                        }
                    };

                    let event_type: Option<String> = result_bound
                        .getattr("event_type")
                        .ok()
                        .and_then(|v| if v.is_none() { None } else { v.extract().ok() });

                    let id: Option<String> = result_bound
                        .getattr("id")
                        .ok()
                        .and_then(|v| if v.is_none() { None } else { v.extract().ok() });

                    let retry: Option<u64> = result_bound
                        .getattr("retry")
                        .ok()
                        .and_then(|v| if v.is_none() { None } else { v.extract().ok() });

                    // Create Rust SseEvent
                    let mut event = if let Some(et) = event_type {
                        SseEvent::with_type(et, data_json)
                    } else {
                        SseEvent::new(data_json)
                    };

                    if let Some(id_str) = id {
                        event = event.with_id(id_str);
                    }

                    if let Some(retry_ms) = retry {
                        event = event.with_retry(retry_ms);
                    }

                    Some(event)
                })
            }
            Err(e) => {
                error!("Error in next_event: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python SSE producer: on_connect");
        let future_opt = Python::attach(|py| {
            if let Ok(coro) = self.producer.bind(py).call_method0("on_connect")
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
        debug!("Python SSE producer: on_disconnect");
        let future_opt = Python::attach(|py| {
            if let Ok(coro) = self.producer.bind(py).call_method0("on_disconnect")
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

/// Create SseState from Python producer factory
pub fn create_sse_state(factory: &Bound<'_, PyAny>) -> PyResult<spikard_http::SseState<PythonSseEventProducer>> {
    // Call the factory to get a producer instance
    let producer_instance = factory.call0()?;

    // Create Python SSE producer
    let py_producer = PythonSseEventProducer::new(producer_instance.unbind());

    // Create and return SSE state
    Ok(spikard_http::SseState::new(py_producer))
}
