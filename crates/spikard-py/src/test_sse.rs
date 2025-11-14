//! SSE test client bindings for Python

use pyo3::prelude::*;
use pyo3::types::PyList;
use serde_json::Value;
use spikard_http::testing::{ResponseSnapshot, SseEvent as RustSseEvent, SseStream as RustSseStream};

/// Python wrapper for SSE stream
#[pyclass]
pub struct SseStream {
    inner: RustSseStream,
}

impl SseStream {
    pub fn new(inner: RustSseStream) -> Self {
        Self { inner }
    }
}

#[pymethods]
impl SseStream {
    /// Get the raw body of the SSE response
    fn body(&self) -> String {
        self.inner.body().to_string()
    }

    /// Get all events from the stream
    fn events<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let list = PyList::empty(py);
        for event in self.inner.events() {
            let py_event = SseEvent::from_rust(event.clone());
            list.append(Py::new(py, py_event)?)?;
        }
        Ok(list)
    }

    /// Get events as JSON values
    fn events_as_json(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let json_events = self
            .inner
            .events_as_json()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;

        // Convert Vec<Value> to Python list
        let list = PyList::empty(py);
        for value in json_events {
            list.append(json_to_python(py, &value)?)?;
        }
        Ok(list.into_any().unbind())
    }

    fn __repr__(&self) -> String {
        format!("SseStream({} events)", self.inner.events().len())
    }
}

/// Python wrapper for SSE event
#[pyclass]
#[derive(Clone)]
pub struct SseEvent {
    inner: RustSseEvent,
}

impl SseEvent {
    fn from_rust(event: RustSseEvent) -> Self {
        Self { inner: event }
    }
}

#[pymethods]
impl SseEvent {
    /// Get the data field of the event
    #[getter]
    fn data(&self) -> String {
        self.inner.data.clone()
    }

    /// Parse the event data as JSON
    fn as_json(&self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        let value = self
            .inner
            .as_json()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;

        Ok(json_to_python(py, &value)?.unbind())
    }

    fn __repr__(&self) -> String {
        format!("SseEvent(data='{}')", self.inner.data)
    }
}

/// Create an SSE stream from a response snapshot
pub fn sse_stream_from_response(response: &ResponseSnapshot) -> PyResult<SseStream> {
    let stream =
        RustSseStream::from_response(response).map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(e))?;
    Ok(SseStream::new(stream))
}

/// Helper to convert JSON Value to Python object
fn json_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
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
                list.append(json_to_python(py, item)?)?;
            }
            Ok(list.into_any())
        }
        Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (key, val) in obj {
                dict.set_item(key, json_to_python(py, val)?)?;
            }
            Ok(dict.into_any())
        }
    }
}
