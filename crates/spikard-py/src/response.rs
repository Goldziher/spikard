//! Python bindings for `Response` type

use async_stream::stream;
use axum::http::{HeaderName, HeaderValue, StatusCode};
use bytes::Bytes;
use pyo3::exceptions::{PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString};
use spikard_http::HandlerResponse;
use std::io;
use std::str::FromStr;

/// Manual Clone implementation for `Response`
/// `PyO3`'s `Py`<T> requires clone_ref(py) but we can clone the struct outside of GIL context
/// by using Python::attach temporarily
impl Clone for Response {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            content: self.content.as_ref().map(|c| c.clone_ref(py)),
            status_code: self.status_code,
            headers: self.headers.clone_ref(py),
        })
    }
}

/// HTTP `Response` with custom status code, headers, and content
///
/// Use this to return custom responses from route handlers with specific
/// status codes, headers, or cookies.
///
/// Examples:
///     >>> from `spikard` import `Response`
///     >>>
///     >>> # Return 201 Created
///     >>> return `Response`(content={"id": 1}, status_code=201)
///     >>>
///     >>> # Return 404 Not Found
///     >>> return `Response`(
///     ...     content={"error": "Not found"},
///     ...     status_code=404
///     ... )
///     >>>
///     >>> # Return response with custom headers
///     >>> response = `Response`(content={"data": "value"})
///     >>> response.headers[\"X-Custom\"] = "header-value"
///     >>> return response
///     >>>
///     >>> # Set a cookie
///     >>> response = `Response`(content={"message": "Cookie set"})
///     >>> response.set_cookie("session_id", "abc123")
///     >>> return response
#[pyclass]
pub struct Response {
    /// `Response` body content (can be dict, list, string, or `None`)
    #[pyo3(get, set)]
    pub content: Option<Py<PyAny>>,

    /// HTTP status code (defaults to 200)
    #[pyo3(get, set)]
    pub status_code: u16,

    /// `Response` headers as a dictionary
    #[pyo3(get)]
    pub headers: Py<PyDict>,
}

#[pymethods]
impl Response {
    /// Create a new `Response`
    ///
    /// Args:
    ///     content: `Response` body (dict, list, str, bytes, or `None`)
    ///     status_code: HTTP status code (default: 200)
    ///     headers: Dictionary of response headers (default: {})
    #[new]
    #[pyo3(signature = (content=None, status_code=200, headers=None))]
    fn new(
        py: Python<'_>,
        content: Option<Py<PyAny>>,
        status_code: u16,
        headers: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Self> {
        let headers_dict = match headers {
            Some(h) => h.clone().unbind(),
            None => PyDict::new(py).unbind(),
        };

        Ok(Self {
            content,
            status_code,
            headers: headers_dict,
        })
    }

    /// Set a cookie in the response
    ///
    /// Args:
    ///     key: Cookie name
    ///     value: Cookie value
    ///     max_age: Maximum age in seconds (optional)
    ///     domain: Cookie domain (optional)
    ///     path: Cookie path (optional, default: "/")
    ///     secure: Whether cookie requires HTTPS (default: `False`)
    ///     httponly: Whether cookie is HTTP-only (default: `False`)
    ///     samesite: SameSite attribute ("Strict", "Lax", or "`None`")
    #[pyo3(signature = (
        key,
        value,
        max_age=None,
        domain=None,
        path=None,
        secure=false,
        httponly=false,
        samesite=None
    ))]
    #[allow(clippy::too_many_arguments)]
    fn set_cookie(
        &mut self,
        py: Python<'_>,
        key: String,
        value: String,
        max_age: Option<i64>,
        domain: Option<String>,
        path: Option<String>,
        secure: bool,
        httponly: bool,
        samesite: Option<String>,
    ) -> PyResult<()> {
        let mut cookie_value = format!("{}={}", key, value);

        if let Some(age) = max_age {
            cookie_value.push_str(&format!("; Max-Age={age}"));
        }
        if let Some(d) = domain {
            cookie_value.push_str(&format!("; Domain={d}"));
        }
        let cookie_path = path.unwrap_or_else(|| "/".to_string());
        cookie_value.push_str(&format!("; Path={cookie_path}"));

        if secure {
            cookie_value.push_str("; Secure");
        }
        if httponly {
            cookie_value.push_str("; HttpOnly");
        }
        if let Some(ss) = samesite {
            cookie_value.push_str(&format!("; SameSite={ss}"));
        }

        let headers_dict = self.headers.bind(py);
        headers_dict.set_item("set-cookie", cookie_value)?;

        Ok(())
    }

    fn __repr__(&self) -> String {
        format!("<Response status_code={}>", self.status_code)
    }
}

impl Response {
    /// Convert an Axum `Response` to Py`Response` (body will be discarded)
    ///
    /// This extracts response data and makes it accessible to Python.
    /// Note: The body is not accessible because it's an async stream.
    /// Use `from_response_parts` if you have a buffered body.
    pub fn from_response(resp: axum::http::Response<axum::body::Body>, py: Python<'_>) -> PyResult<Self> {
        let (parts, _body) = resp.into_parts();

        let status_code = parts.status.as_u16();

        let headers_dict = PyDict::new(py);
        for (name, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_dict.set_item(name.as_str(), value_str)?;
            }
        }

        Ok(Self {
            content: None,
            status_code,
            headers: headers_dict.into(),
        })
    }

    /// Convert Axum `Response` parts with buffered body to Py`Response`
    ///
    /// This is used in lifecycle hooks where we pre-buffer the body
    /// to avoid async/sync conversion issues.
    pub fn from_response_parts(
        parts: axum::http::response::Parts,
        body_bytes: bytes::Bytes,
        py: Python<'_>,
    ) -> PyResult<Self> {
        let status_code = parts.status.as_u16();

        let headers_dict = PyDict::new(py);
        for (name, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_dict.set_item(name.as_str(), value_str)?;
            }
        }

        let content = if body_bytes.is_empty() {
            None
        } else if let Ok(json_str) = std::str::from_utf8(&body_bytes) {
            match py.import("json")?.call_method1("loads", (json_str,)) {
                Ok(parsed) => Some(parsed.unbind()),
                Err(_) => Some(PyString::new(py, json_str).into_any().unbind()),
            }
        } else {
            Some(PyBytes::new(py, &body_bytes).into_any().unbind())
        };

        Ok(Self {
            content,
            status_code,
            headers: headers_dict.into(),
        })
    }

    /// Convert Py`Response` to Axum `Response`
    ///
    /// This reconstructs an Axum response from the Python response data.
    pub fn to_response(&self, py: Python<'_>) -> PyResult<axum::http::Response<axum::body::Body>> {
        let status = axum::http::StatusCode::from_u16(self.status_code).unwrap_or(axum::http::StatusCode::OK);

        let mut resp_builder = axum::http::Response::builder().status(status);

        let headers_dict = self.headers.bind(py);
        for (key, value) in headers_dict.iter() {
            let key_str: String = key.extract()?;
            let value_str: String = value.extract()?;
            resp_builder = resp_builder.header(key_str, value_str);
        }

        let body = if let Some(ref content) = self.content {
            let json_str = py
                .import("json")?
                .call_method1("dumps", (content,))?
                .extract::<String>()?;
            axum::body::Body::from(json_str)
        } else {
            axum::body::Body::empty()
        };

        resp_builder
            .body(body)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to build response: {e}")))
    }
}

#[pyclass]
pub struct StreamingResponse {
    stream: Py<PyAny>,
    #[pyo3(get, set)]
    pub status_code: u16,
    #[pyo3(get)]
    pub headers: Py<PyDict>,
}

#[pymethods]
impl StreamingResponse {
    #[new]
    #[pyo3(signature = (stream, *, status_code=200, headers=None))]
    fn new(py: Python<'_>, stream: Py<PyAny>, status_code: u16, headers: Option<&Bound<'_, PyDict>>) -> PyResult<Self> {
        let headers_dict = match headers {
            Some(h) => h.clone().unbind(),
            None => PyDict::new(py).unbind(),
        };

        let bound_stream = stream.bind(py);

        let wrapped_stream = if bound_stream.hasattr("__anext__")? {
            let wrapper_module = py.import("spikard._internal.async_generator_wrapper")?;
            let wrapper_class = wrapper_module.getattr("AsyncGeneratorWrapper")?;

            wrapper_class.call1((stream,))?.into()
        } else if bound_stream.hasattr("__next__")? || bound_stream.hasattr("__iter__")? {
            stream
        } else {
            return Err(PyTypeError::new_err(
                "StreamingResponse requires an iterator (sync or async)",
            ));
        };

        Ok(Self {
            stream: wrapped_stream,
            status_code,
            headers: headers_dict,
        })
    }

    fn __repr__(&self) -> String {
        format!("<StreamingResponse status_code={}>", self.status_code)
    }
}

impl StreamingResponse {
    /// Get a reference to the underlying stream
    pub fn get_stream(&self) -> Py<PyAny> {
        Python::attach(|py| self.stream.clone_ref(py))
    }

    pub fn to_handler_response(&self, py: Python<'_>) -> PyResult<HandlerResponse> {
        let status = StatusCode::from_u16(self.status_code)
            .map_err(|e| PyValueError::new_err(format!("Invalid status code: {e}")))?;
        let header_pairs = extract_header_pairs(py, &self.headers)?;
        let stream_object = Python::attach(|py| self.stream.clone_ref(py));

        let rust_stream = stream! {
            loop {
                let stream_clone = Python::attach(|py| stream_object.clone_ref(py));

                let next_result_py = Python::attach(|py| -> PyResult<Py<PyAny>> {
                    let bound = stream_clone.bind(py);
                    match bound.call_method0("__next__") {
                        Ok(value) => Ok(value.unbind()),
                        Err(err) => {
                            if err.is_instance_of::<pyo3::exceptions::PyStopIteration>(py) {
                                Ok(Python::None(py))
                            } else {
                                Err(err)
                            }
                        }
                    }
                });

                match next_result_py {
                    Ok(value_py) => {
                        let is_none = Python::attach(|py| value_py.bind(py).is_none());
                        if is_none {
                            break;
                        }

                        let is_coro = Python::attach(|py| -> PyResult<bool> {
                            let asyncio = py.import("asyncio")?;
                            asyncio
                                .call_method1("iscoroutine", (value_py.bind(py),))?
                                .extract()
                        }).unwrap_or(false);

                        let final_value = if is_coro {
                            let future_result = Python::attach(|py| {
                                pyo3_async_runtimes::tokio::into_future(value_py.bind(py).clone())
                            });

                            match future_result {
                                Ok(future) => {
                                    match future.await {
                                        Ok(result) => Some(result),
                                        Err(err) => {
                                            let message = format_pyerr(err);
                                            yield Err(Box::new(io::Error::other(message)));
                                            break;
                                        }
                                    }
                                }
                                Err(err) => {
                                    let message = format_pyerr(err);
                                    yield Err(Box::new(io::Error::other(message)));
                                    break;
                                }
                            }
                        } else {
                            Some(value_py)
                        };

                        if let Some(final_val) = final_value {
                            match Python::attach(|py| convert_chunk_to_bytes(final_val.bind(py))) {
                                Ok(bytes) => {
                                    yield Ok(bytes);
                                }
                                Err(err) => {
                                    let message = format_pyerr(err);
                                    yield Err(Box::new(io::Error::other(message)));
                                    break;
                                }
                            }
                        }
                    }
                    Err(err) => {
                        let message = format_pyerr(err);
                        yield Err(Box::new(io::Error::other(message)));
                        break;
                    }
                }
            }
        };

        let mut response = HandlerResponse::stream(rust_stream).with_status(status);
        for (name, value) in header_pairs {
            response = response.with_header(name, value);
        }
        Ok(response)
    }
}

fn extract_header_pairs(py: Python<'_>, headers: &Py<PyDict>) -> PyResult<Vec<(HeaderName, HeaderValue)>> {
    let mut pairs = Vec::new();
    let dict = headers.bind(py);
    for (key, value) in dict.iter() {
        let key_str: String = key.extract()?;
        let value_str: String = value.extract()?;
        let header_name = HeaderName::from_str(&key_str)
            .map_err(|e| PyValueError::new_err(format!("Invalid header '{}': {}", key_str, e)))?;
        let header_value = HeaderValue::from_str(&value_str)
            .map_err(|e| PyValueError::new_err(format!("Invalid header value '{}': {}", value_str, e)))?;
        pairs.push((header_name, header_value));
    }
    Ok(pairs)
}

fn convert_chunk_to_bytes(obj: &Bound<'_, PyAny>) -> PyResult<Bytes> {
    if let Ok(py_bytes) = obj.cast::<PyBytes>() {
        Ok(Bytes::copy_from_slice(py_bytes.as_bytes()))
    } else if obj.cast::<PyString>().is_ok() {
        let text: String = obj.extract()?;
        Ok(Bytes::from(text.into_bytes()))
    } else {
        Err(PyTypeError::new_err("StreamingResponse chunks must be str or bytes"))
    }
}

fn format_pyerr(err: PyErr) -> String {
    Python::attach(|py| {
        err.into_value(py)
            .bind(py)
            .repr()
            .ok()
            .and_then(|repr| repr.extract::<String>().ok())
            .unwrap_or_else(|| "Streaming error".to_string())
    })
}
