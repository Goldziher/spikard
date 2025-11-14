//! Python SSE producer bindings

use pyo3::prelude::*;
use serde_json::Value;
use spikard_http::{SseEvent, SseEventProducer};
use std::sync::Arc;
use tracing::{debug, error};

/// Python implementation of SseEventProducer
pub struct PythonSseEventProducer {
    /// Python producer instance wrapped in Arc for cheap cloning
    producer: Arc<Py<PyAny>>,
}

impl PythonSseEventProducer {
    /// Create a new Python SSE event producer
    pub fn new(producer: Py<PyAny>) -> Self {
        Self {
            producer: Arc::new(producer),
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

impl SseEventProducer for PythonSseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        debug!("Python SSE producer: next_event called");

        let producer = Arc::clone(&self.producer);

        // Run in blocking task with asyncio.run() like regular handlers
        let result = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<Option<SseEvent>> {
                debug!("Python SSE producer: acquired GIL");

                // Call the producer's next_event method (synchronous)
                let result = producer.bind(py).call_method0("next_event")?;
                debug!("Python SSE producer: called next_event method");

                // Check if result is None (end of stream)
                if result.is_none() {
                    debug!("Python SSE producer: received None, ending stream");
                    return Ok(None);
                }

                // Extract SseEvent from Python object
                let data = result.getattr("data")?;
                let data_json = Self::python_to_json(py, &data)?;

                let event_type: Option<String> = result
                    .getattr("event_type")
                    .ok()
                    .and_then(|v| if v.is_none() { None } else { v.extract().ok() });

                let id: Option<String> = result
                    .getattr("id")
                    .ok()
                    .and_then(|v| if v.is_none() { None } else { v.extract().ok() });

                let retry: Option<u64> = result
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

                Ok(Some(event))
            })
        })
        .await;

        match result {
            Ok(Ok(event)) => event,
            Ok(Err(e)) => {
                error!("Python error in next_event: {}", e);
                None
            }
            Err(e) => {
                error!("Tokio error in next_event: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python SSE producer: on_connect called");

        let producer = Arc::clone(&self.producer);

        let _ = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<()> {
                debug!("Python SSE producer: on_connect acquired GIL");
                let coroutine = producer.bind(py).call_method0("on_connect")?;
                let asyncio = py.import("asyncio")?;
                asyncio.call_method1("run", (coroutine,))?;
                debug!("Python SSE producer: on_connect completed");
                Ok(())
            })
        })
        .await;
    }

    async fn on_disconnect(&self) {
        debug!("Python SSE producer: on_disconnect called");

        let producer = Arc::clone(&self.producer);

        let _ = tokio::task::spawn_blocking(move || {
            Python::attach(|py| -> PyResult<()> {
                let coroutine = producer.bind(py).call_method0("on_disconnect")?;
                let asyncio = py.import("asyncio")?;
                asyncio.call_method1("run", (coroutine,))?;
                debug!("Python SSE producer: on_disconnect completed");
                Ok(())
            })
        })
        .await;
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
