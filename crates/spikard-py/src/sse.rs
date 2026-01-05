//! Python SSE producer bindings

use crate::conversion::python_to_json;
use pyo3::prelude::*;
use spikard_http::{SseEvent, SseEventProducer};
use std::sync::Arc;
use tracing::{debug, error};

/// Python implementation of SseEventProducer
pub struct PythonSseEventProducer {
    /// Python producer instance wrapped in `Arc` for cheap cloning
    producer: Arc<Py<PyAny>>,
}

impl PythonSseEventProducer {
    /// Create a new Python SSE event producer
    pub fn new(producer: Py<PyAny>) -> Self {
        Self {
            producer: Arc::new(producer),
        }
    }
}

impl SseEventProducer for PythonSseEventProducer {
    async fn next_event(&self) -> Option<SseEvent> {
        debug!("Python SSE producer: next_event called");

        let producer = Arc::clone(&self.producer);

        let result_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            debug!("Python SSE producer: acquired GIL");
            let result = producer.bind(py).call_method0("next_event")?;
            Ok(result.unbind())
        });

        match result_py {
            Ok(result_py) => {
                let is_coroutine = Python::attach(|py| -> PyResult<bool> {
                    let asyncio = py.import("asyncio")?;
                    asyncio.call_method1("iscoroutine", (result_py.bind(py),))?.extract()
                })
                .unwrap_or(false);

                if is_coroutine {
                    debug!("Python SSE producer: result is coroutine, awaiting...");
                    let future_result =
                        Python::attach(|py| pyo3_async_runtimes::tokio::into_future(result_py.bind(py).clone()));

                    match future_result {
                        Ok(future) => match future.await {
                            Ok(result) => {
                                let is_none = Python::attach(|py| result.bind(py).is_none());
                                if is_none {
                                    debug!("Python SSE producer: received None, ending stream");
                                    return None;
                                }
                                Python::attach(|py| convert_py_to_sse_event(result.bind(py)))
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
                    let is_none = Python::attach(|py| result_py.bind(py).is_none());
                    if is_none {
                        debug!("Python SSE producer: received None, ending stream");
                        return None;
                    }
                    Python::attach(|py| convert_py_to_sse_event(result_py.bind(py)))
                }
            }
            Err(e) => {
                error!("Python error in next_event: {}", e);
                None
            }
        }
    }

    async fn on_connect(&self) {
        debug!("Python SSE producer: on_connect called");

        let producer = Arc::clone(&self.producer);

        let coroutine_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            debug!("Python SSE producer: on_connect acquired GIL");
            let coroutine = producer.bind(py).call_method0("on_connect")?;
            Ok(coroutine.unbind())
        });

        if let Ok(coroutine) = coroutine_py {
            let future_result =
                Python::attach(|py| pyo3_async_runtimes::tokio::into_future(coroutine.bind(py).clone()));

            if let Ok(future) = future_result {
                let _ = future.await;
                debug!("Python SSE producer: on_connect completed");
            } else {
                error!("Failed to convert on_connect coroutine to future");
            }
        } else {
            error!("Failed to call on_connect");
        }
    }

    async fn on_disconnect(&self) {
        debug!("Python SSE producer: on_disconnect called");

        let producer = Arc::clone(&self.producer);

        let coroutine_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
            let coroutine = producer.bind(py).call_method0("on_disconnect")?;
            Ok(coroutine.unbind())
        });

        if let Ok(coroutine) = coroutine_py {
            let future_result =
                Python::attach(|py| pyo3_async_runtimes::tokio::into_future(coroutine.bind(py).clone()));

            if let Ok(future) = future_result {
                let _ = future.await;
                debug!("Python SSE producer: on_disconnect completed");
            } else {
                error!("Failed to convert on_disconnect coroutine to future");
            }
        } else {
            error!("Failed to call on_disconnect");
        }
    }
}

/// Convert Python object to SseEvent
fn convert_py_to_sse_event(result: &Bound<'_, PyAny>) -> Option<SseEvent> {
    let data = result.getattr("data").ok()?;
    let data_json = Python::attach(|py| python_to_json(py, &data)).ok()?;

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
}

/// Create SseState from Python producer factory
pub fn create_sse_state(factory: &Bound<'_, PyAny>) -> PyResult<spikard_http::SseState<PythonSseEventProducer>> {
    let producer_instance = factory.call0()?;

    let py_producer = PythonSseEventProducer::new(producer_instance.unbind());

    Ok(spikard_http::SseState::new(py_producer))
}
