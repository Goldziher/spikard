//! Python bindings for Response type

use pyo3::prelude::*;
use pyo3::types::PyDict;

/// Manual Clone implementation for Response
/// PyO3's Py<T> requires clone_ref(py) but we can clone the struct outside of GIL context
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

/// HTTP Response with custom status code, headers, and content
///
/// Use this to return custom responses from route handlers with specific
/// status codes, headers, or cookies.
///
/// Examples:
///     >>> from spikard import Response
///     >>>
///     >>> # Return 201 Created
///     >>> return Response(content={"id": 1}, status_code=201)
///     >>>
///     >>> # Return 404 Not Found
///     >>> return Response(
///     ...     content={"error": "Not found"},
///     ...     status_code=404
///     ... )
///     >>>
///     >>> # Return response with custom headers
///     >>> response = Response(content={"data": "value"})
///     >>> response.headers["X-Custom"] = "header-value"
///     >>> return response
///     >>>
///     >>> # Set a cookie
///     >>> response = Response(content={"message": "Cookie set"})
///     >>> response.set_cookie("session_id", "abc123")
///     >>> return response
#[pyclass]
pub struct Response {
    /// Response body content (can be dict, list, string, or None)
    #[pyo3(get, set)]
    pub content: Option<Py<PyAny>>,

    /// HTTP status code (defaults to 200)
    #[pyo3(get, set)]
    pub status_code: u16,

    /// Response headers as a dictionary
    #[pyo3(get)]
    pub headers: Py<PyDict>,
}

#[pymethods]
impl Response {
    /// Create a new Response
    ///
    /// Args:
    ///     content: Response body (dict, list, str, bytes, or None)
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
    ///     secure: Whether cookie requires HTTPS (default: False)
    ///     httponly: Whether cookie is HTTP-only (default: False)
    ///     samesite: SameSite attribute ("Strict", "Lax", or "None")
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
            cookie_value.push_str(&format!("; Max-Age={}", age));
        }
        if let Some(d) = domain {
            cookie_value.push_str(&format!("; Domain={}", d));
        }
        // Default path to "/"
        let cookie_path = path.unwrap_or_else(|| "/".to_string());
        cookie_value.push_str(&format!("; Path={}", cookie_path));

        if secure {
            cookie_value.push_str("; Secure");
        }
        if httponly {
            cookie_value.push_str("; HttpOnly");
        }
        if let Some(ss) = samesite {
            cookie_value.push_str(&format!("; SameSite={}", ss));
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
    /// Convert an Axum Response to PyResponse
    ///
    /// This extracts response data and makes it accessible to Python.
    pub fn from_response(resp: axum::http::Response<axum::body::Body>, py: Python<'_>) -> PyResult<Self> {
        let (parts, _body) = resp.into_parts();

        let status_code = parts.status.as_u16();

        // Extract headers
        let headers_dict = PyDict::new(py);
        for (name, value) in parts.headers.iter() {
            if let Ok(value_str) = value.to_str() {
                headers_dict.set_item(name.as_str(), value_str)?;
            }
        }

        // For now, content is None (body is consumed)
        // In a full implementation, we'd need to buffer the body
        Ok(Self {
            content: None,
            status_code,
            headers: headers_dict.into(),
        })
    }

    /// Convert PyResponse to Axum Response
    ///
    /// This reconstructs an Axum response from the Python response data.
    pub fn to_response(&self, py: Python<'_>) -> PyResult<axum::http::Response<axum::body::Body>> {
        let status = axum::http::StatusCode::from_u16(self.status_code).unwrap_or(axum::http::StatusCode::OK);

        let mut resp_builder = axum::http::Response::builder().status(status);

        // Add headers from Python dict
        let headers_dict = self.headers.bind(py);
        for (key, value) in headers_dict.iter() {
            let key_str: String = key.extract()?;
            let value_str: String = value.extract()?;
            resp_builder = resp_builder.header(key_str, value_str);
        }

        // Convert content to body
        let body = if let Some(ref content) = self.content {
            // Convert Python object to JSON
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
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Failed to build response: {}", e)))
    }
}
