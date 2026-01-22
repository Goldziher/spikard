//! Python-specific implementation of ResponseInterpreter trait
//!
//! This module provides the concrete implementation of the ResponseInterpreter trait
//! for Python, eliminating duplicate response detection logic that was previously
//! hand-coded in handler.rs.

use crate::conversion::python_to_json;
use crate::response::StreamingResponse;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use spikard_bindings_shared::{InterpretedResponse, ResponseInterpreter, StreamSource};
use std::collections::HashMap;

/// Python-specific implementation of StreamSource for iterators and generators
///
/// Wraps a Python iterator/generator and provides the StreamSource interface
/// to yield chunks of data.
pub struct PyStreamSource {
    iterator: Py<PyAny>,
}

impl PyStreamSource {
    /// Create a new PyStreamSource from a Python iterator
    pub fn new(iterator: Py<PyAny>) -> Self {
        Self { iterator }
    }
}

impl StreamSource for PyStreamSource {
    fn next_chunk(&mut self) -> Option<Vec<u8>> {
        Python::attach(|py| -> Option<Vec<u8>> {
            let bound = self.iterator.bind(py);

            // Call __next__() on the Python iterator
            match bound.call_method0("__next__") {
                Ok(value) => {
                    // Convert the chunk to bytes
                    convert_chunk_to_bytes(&value).ok().map(|bytes| bytes.to_vec())
                }
                Err(err) => {
                    // End of iteration or error - treat as end of stream
                    if !err.is_instance_of::<pyo3::exceptions::PyStopIteration>(py) {
                        // Log other errors silently
                    }
                    None
                }
            }
        })
    }
}

/// Python-specific implementation of ResponseInterpreter
///
/// Detects streaming responses (StreamingResponse class or iterators),
/// custom responses (objects with status_code/headers attributes),
/// and plain JSON responses.
///
/// Since PyO3's Bound types have lifetime parameters, we use Py<PyAny> (which is 'static)
/// as the LanguageValue type, and convert Bound references to Py when needed.
pub struct PyResponseInterpreter;

impl ResponseInterpreter for PyResponseInterpreter {
    type LanguageValue = Py<PyAny>;
    type Error = PyErr;

    fn is_streaming(&self, value: &Self::LanguageValue) -> bool {
        Python::attach(|py| {
            let bound = value.bind(py);
            Self::is_streaming_impl(bound)
        })
    }

    fn is_custom_response(&self, value: &Self::LanguageValue) -> bool {
        Python::attach(|py| {
            let bound = value.bind(py);
            Self::is_custom_response_impl(bound)
        })
    }

    fn interpret(&self, value: &Self::LanguageValue) -> Result<InterpretedResponse, Self::Error> {
        Python::attach(|py| {
            let bound = value.bind(py);
            Self::interpret_impl(bound)
        })
    }
}

impl PyResponseInterpreter {
    fn is_streaming_impl(value: &Bound<'_, PyAny>) -> bool {
        // Check if it's a StreamingResponse instance
        if value.is_instance_of::<StreamingResponse>() {
            return true;
        }

        // Check for iterator protocol (__iter__ and __next__)
        if let Ok(has_iter) = value.hasattr("__iter__")
            && has_iter
            && !value.is_instance_of::<PyDict>()
        {
            // Don't treat dicts as iterators for streaming
            // Check for __next__ to confirm it's an iterator/generator
            if let Ok(has_next) = value.hasattr("__next__") {
                return has_next;
            }
        }

        false
    }

    fn is_custom_response_impl(value: &Bound<'_, PyAny>) -> bool {
        // Check for custom response attributes (status_code, headers, or content)
        // This matches the Response class from response.rs
        matches!(
            (
                value.hasattr("status_code"),
                value.hasattr("content"),
                value.hasattr("headers")
            ),
            (Ok(true), Ok(true), Ok(true))
        )
    }

    fn interpret_impl(value: &Bound<'_, PyAny>) -> Result<InterpretedResponse, PyErr> {
        // Check for streaming response first
        if Self::is_streaming_impl(value) {
            // If it's a StreamingResponse instance, extract its properties
            if value.is_instance_of::<StreamingResponse>() {
                let streaming: Py<StreamingResponse> = value.extract()?;
                let py = value.py();

                let streaming_ref = streaming.borrow(py);
                let status = streaming_ref.status_code;

                let mut headers = HashMap::new();
                let headers_dict = streaming_ref.headers.bind(py);
                for (key, val) in headers_dict.iter() {
                    let key_str: String = key.extract()?;
                    let value_str: String = val.extract()?;
                    headers.insert(key_str, value_str);
                }

                // Create a PyStreamSource from the iterator
                let stream_py = streaming_ref.get_stream();

                let enumerator = Box::new(PyStreamSource::new(stream_py));

                return Ok(InterpretedResponse::Streaming {
                    enumerator,
                    status,
                    headers,
                });
            }
            // Generic Python iterator/generator
            let stream_py: Py<PyAny> = value.extract()?;
            let enumerator = Box::new(PyStreamSource::new(stream_py));

            return Ok(InterpretedResponse::Streaming {
                enumerator,
                status: 200,
                headers: HashMap::new(),
            });
        }

        // Check for custom response (Response class or object with status_code/headers)
        if Self::is_custom_response_impl(value) {
            let status_code: u16 = value.getattr("status_code")?.extract()?;

            let content_attr = value.getattr("content")?;
            let body = if content_attr.is_none() {
                None
            } else {
                Some(python_to_json(value.py(), &content_attr)?)
            };

            let headers_dict = value.getattr("headers")?;
            let mut headers = HashMap::new();

            #[allow(deprecated)]
            if let Ok(dict) = headers_dict.downcast::<PyDict>() {
                for (key, val) in dict.iter() {
                    let key_str: String = key.extract()?;
                    let value_str: String = val.extract()?;
                    headers.insert(key_str, value_str);
                }
            }

            return Ok(InterpretedResponse::Custom {
                status: status_code,
                headers,
                body,
                raw_body: None,
            });
        }

        // Plain JSON response
        let body = python_to_json(value.py(), value)?;
        Ok(InterpretedResponse::Plain { body })
    }
}

/// Convert a Python chunk (from iterator) to bytes
///
/// Handles both str and bytes types.
fn convert_chunk_to_bytes(obj: &Bound<'_, PyAny>) -> PyResult<bytes::Bytes> {
    use pyo3::types::PyBytes;
    use pyo3::types::PyString;

    if let Ok(py_bytes) = obj.cast::<PyBytes>() {
        Ok(bytes::Bytes::copy_from_slice(py_bytes.as_bytes()))
    } else if obj.cast::<PyString>().is_ok() {
        let text: String = obj.extract()?;
        Ok(bytes::Bytes::from(text.into_bytes()))
    } else {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Streaming chunks must be str or bytes",
        ))
    }
}

// Tests for PyResponseInterpreter are tested through the Python handler integration tests
// See packages/python/tests for comprehensive test coverage
