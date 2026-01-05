//! Python `Request` wrapper for lifecycle hooks
//!
//! This module provides a Python-accessible `Request` type that can be used in lifecycle hooks.
//! Unlike the RequestData used in handlers, this wraps the full Axum `Request` to allow
//! modifications before routing/validation.

use axum::{body::Body, http::Request};
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};

/// Manual Clone implementation for Py`Request`
/// `PyO3`'s `Py`<T> requires clone_ref(py) but we can clone the struct outside of GIL context
/// by using Python::attach temporarily
impl Clone for PyRequest {
    fn clone(&self) -> Self {
        Python::attach(|py| Self {
            method: self.method.clone(),
            path: self.path.clone(),
            headers: self.headers.clone_ref(py),
            body: self.body.clone(),
            state: self.state.clone_ref(py),
        })
    }
}

/// Python `Request` wrapper for lifecycle hooks
///
/// This provides access to request properties and allows modifications.
/// Used primarily in lifecycle hooks (on`Request`, preValidation, preHandler).
#[pyclass]
pub struct PyRequest {
    /// HTTP method
    #[pyo3(get, set)]
    pub method: String,

    /// `Request` path
    #[pyo3(get, set)]
    pub path: String,

    /// `Request` headers (mutable)
    #[pyo3(get)]
    pub headers: Py<PyDict>,

    /// `Request` body (if available)
    body: Option<Vec<u8>>,

    /// `Request` state dictionary (for passing data between hooks)
    #[pyo3(get)]
    pub state: Py<PyDict>,
}

#[pymethods]
impl PyRequest {
    /// Get the request body as bytes
    fn body(&self, py: Python<'_>) -> PyResult<Option<Py<PyBytes>>> {
        Ok(self.body.as_ref().map(|b| PyBytes::new(py, b).into()))
    }

    /// Get the request body as a UTF-8 string
    fn text(&self) -> PyResult<Option<String>> {
        self.body
            .as_ref()
            .map(|b| {
                String::from_utf8(b.clone())
                    .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid UTF-8: {e}")))
            })
            .transpose()
    }

    fn __repr__(&self) -> String {
        format!("<Request {} {}>", self.method, self.path)
    }
}

impl PyRequest {
    /// Convert an Axum `Request` to Py`Request`
    ///
    /// This extracts all the necessary data from the Axum request and makes it
    /// accessible to Python code.
    pub fn from_request(req: Request<Body>, py: Python<'_>) -> PyResult<Self> {
        let (parts, _body) = req.into_parts();

        let method = parts.method.to_string();
        let path = parts.uri.path().to_string();

        let headers_dict = PyDict::new(py);
        for (name, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_dict.set_item(name.as_str(), value_str)?;
            }
        }

        let body_bytes = None;

        Ok(Self {
            method,
            path,
            headers: headers_dict.into(),
            body: body_bytes,
            state: PyDict::new(py).into(),
        })
    }

    /// Convert Py`Request` back to Axum `Request`
    ///
    /// This reconstructs an Axum request from the Python request data.
    /// Note: This creates a new request, so any modifications to the original body are lost.
    pub fn to_request(&self, py: Python<'_>) -> PyResult<Request<Body>> {
        let method = self
            .method
            .parse::<axum::http::Method>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid method: {e}")))?;

        let uri = self
            .path
            .parse::<axum::http::Uri>()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid path: {e}")))?;

        let mut req_builder = Request::builder().method(method).uri(uri);

        let headers_dict = self.headers.bind(py);
        for (key, value) in headers_dict.iter() {
            let key_str: String = key.extract()?;
            let value_str: String = value.extract()?;
            req_builder = req_builder.header(key_str, value_str);
        }

        let body = if let Some(ref body_bytes) = self.body {
            Body::from(body_bytes.clone())
        } else {
            Body::empty()
        };

        req_builder
            .body(body)
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to build request: {e}")))
    }
}
