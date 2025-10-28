//! Python bindings for Response type

use pyo3::prelude::*;
use pyo3::types::PyDict;

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
