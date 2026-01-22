//! Python request wrapper passed to handlers.
//!
//! This is separate from `request::PyRequest`, which is used for lifecycle hooks and wraps
//! a full Axum request. `PyHandlerRequest` is focused on `handler` inputs and keeps conversions
//! lazy to avoid per-request Python object allocations when handlers don't need them.

use crate::conversion::json_to_python;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use serde_json::Value;
use spikard_http::RequestData;
use std::collections::HashMap;
use std::sync::OnceLock;

#[pyclass]
pub struct PyHandlerRequest {
    method: String,
    path: String,
    is_json: bool,
    request_data: RequestData,
    cached_path_params: OnceLock<Py<PyAny>>,
    cached_query_params: OnceLock<Py<PyAny>>,
    cached_headers: OnceLock<Py<PyAny>>,
    cached_cookies: OnceLock<Py<PyAny>>,
    cached_body: OnceLock<Py<PyAny>>,
}

#[pymethods]
impl PyHandlerRequest {
    #[getter]
    fn method(&self) -> &str {
        &self.method
    }

    #[getter]
    fn path(&self) -> &str {
        &self.path
    }

    #[getter]
    fn path_params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // PERFORMANCE: Check cache first to avoid rebuilding on repeated access
        if let Some(obj) = self.cached_path_params.get() {
            return Ok(obj.bind(py).clone());
        }

        let dict = PyDict::new(py);
        for (key, value) in self.request_data.path_params.iter() {
            if let Ok(int_val) = value.parse::<i64>() {
                dict.set_item(key, int_val)?;
            } else if let Ok(float_val) = value.parse::<f64>() {
                dict.set_item(key, float_val)?;
            } else if value == "true" || value == "false" {
                dict.set_item(key, value == "true")?;
            } else {
                dict.set_item(key, value)?;
            }
        }

        // PERFORMANCE: Store unbind() result and return immediately; cache() always succeeds
        let obj = dict.into_any().unbind();
        let _ = self.cached_path_params.set(obj.clone_ref(py));
        Ok(obj.bind(py).clone())
    }

    #[getter]
    fn query_params<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // PERFORMANCE: Check cache first to avoid rebuilding on repeated access
        if let Some(obj) = self.cached_query_params.get() {
            return Ok(obj.bind(py).clone());
        }

        let py_value = match &*self.request_data.query_params {
            Value::Object(map) => {
                let dict = PyDict::new(py);
                for (key, value) in map {
                    dict.set_item(key.as_str(), json_to_python(py, value)?)?;
                }
                dict.into_any()
            }
            _ => PyDict::new(py).into_any(),
        };

        // PERFORMANCE: Store unbind() result and return immediately; cache() always succeeds
        let obj = py_value.unbind();
        let _ = self.cached_query_params.set(obj.clone_ref(py));
        Ok(obj.bind(py).clone())
    }

    #[getter]
    fn headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // PERFORMANCE: Check cache first to avoid rebuilding on repeated access
        if let Some(obj) = self.cached_headers.get() {
            return Ok(obj.bind(py).clone());
        }

        let dict = PyDict::new(py);
        for (key, value) in self.request_data.headers.iter() {
            dict.set_item(key, value)?;
        }

        // PERFORMANCE: Store unbind() result and return immediately; cache() always succeeds
        let obj = dict.into_any().unbind();
        let _ = self.cached_headers.set(obj.clone_ref(py));
        Ok(obj.bind(py).clone())
    }

    #[getter]
    fn cookies<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // PERFORMANCE: Check cache first to avoid rebuilding on repeated access
        if let Some(obj) = self.cached_cookies.get() {
            return Ok(obj.bind(py).clone());
        }

        let dict = PyDict::new(py);
        for (key, value) in self.request_data.cookies.iter() {
            dict.set_item(key, value)?;
        }

        // PERFORMANCE: Store unbind() result and return immediately; cache() always succeeds
        let obj = dict.into_any().unbind();
        let _ = self.cached_cookies.set(obj.clone_ref(py));
        Ok(obj.bind(py).clone())
    }

    #[getter]
    fn body<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        // PERFORMANCE: Check cache first to avoid rebuilding on repeated access
        if let Some(obj) = self.cached_body.get() {
            return Ok(obj.bind(py).clone());
        }

        let body_obj = if let Some(raw) = &self.request_data.raw_body {
            if self.is_json {
                PyBytes::new(py, raw).into_any()
            } else if !self.request_data.body.is_null() {
                json_to_python(py, &self.request_data.body)?
            } else {
                PyBytes::new(py, raw).into_any()
            }
        } else {
            json_to_python(py, &self.request_data.body)?
        };

        // PERFORMANCE: Store unbind() result and return immediately; cache() always succeeds
        let obj = body_obj.unbind();
        let _ = self.cached_body.set(obj.clone_ref(py));
        Ok(obj.bind(py).clone())
    }

    /// Expose raw body bytes when available.
    #[getter]
    fn raw_body<'py>(&self, py: Python<'py>) -> PyResult<Option<Bound<'py, PyBytes>>> {
        Ok(self.request_data.raw_body.as_ref().map(|raw| PyBytes::new(py, raw)))
    }

    /// `True` when `.body` represents raw JSON bytes (to be decoded in Python).
    #[getter]
    fn raw_json(&self) -> bool {
        self.is_json && self.request_data.raw_body.is_some()
    }

    fn __repr__(&self) -> String {
        format!("<HandlerRequest {} {}>", self.method, self.path)
    }
}

fn is_json_content_type(headers: &HashMap<String, String>) -> bool {
    let ct = headers
        .get("content-type")
        .or_else(|| headers.get("Content-Type"))
        .map(|s| s.as_str())
        .unwrap_or("");

    let ct = ct.to_ascii_lowercase();
    ct.contains("application/json") || ct.contains("+json")
}

impl PyHandlerRequest {
    pub fn new(request_data: RequestData) -> Self {
        let is_json = is_json_content_type(&request_data.headers);
        Self {
            method: request_data.method.clone(),
            path: request_data.path.clone(),
            is_json,
            request_data,
            cached_path_params: OnceLock::new(),
            cached_query_params: OnceLock::new(),
            cached_headers: OnceLock::new(),
            cached_cookies: OnceLock::new(),
            cached_body: OnceLock::new(),
        }
    }
}
