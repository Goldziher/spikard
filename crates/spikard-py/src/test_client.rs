//! Test client for Spikard applications
//!
//! This module provides a test client powered by axum-test for testing Spikard applications
//! without needing to start a real HTTP server.

use axum::Router as AxumRouter;
use axum::http::{HeaderName, HeaderValue, Method};
use axum_test::{TestResponse as AxumTestResponse, TestServer as AxumTestServer};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

/// A test client for making requests to a Spikard application
///
/// This wraps axum-test's TestServer and provides a Python-friendly interface
#[pyclass]
pub struct TestClient {
    server: Arc<AxumTestServer>,
}

impl TestClient {
    /// Create a new test client from an Axum router
    pub fn from_router(router: AxumRouter) -> PyResult<Self> {
        let server = AxumTestServer::new(router)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create test server: {}", e)))?;

        Ok(Self {
            server: Arc::new(server),
        })
    }
}

#[pymethods]
impl TestClient {
    /// Make a GET request
    ///
    /// Args:
    ///     path: The path to request (e.g., "/users/123")
    ///     query_params: Optional query parameters as a dict
    ///     headers: Optional headers as a dict
    ///
    /// Returns:
    ///     TestResponse: The response from the server
    #[pyo3(signature = (path, query_params=None, headers=None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // DEBUG: Log test client get() call
        let _ = std::fs::write(
            "/tmp/test_client_get.log",
            format!("TestClient.get() called: path={}\n", path),
        );
        // Extract Python data before the async block
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let _ = std::fs::write(
            "/tmp/test_client_query_params.log",
            format!("query_params_vec: {:?}\n", query_params_vec),
        );
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let _ = std::fs::write("/tmp/test_client_async_start.log", "Async block started\n");
            // Build full path with query string to properly handle arrays
            let full_path = if !query_params_vec.is_empty() {
                let query_string: Vec<String> = query_params_vec
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))  // Values already URL-safe
                    .collect();
                if path.contains('?') {
                    format!("{}&{}", path, query_string.join("&"))
                } else {
                    format!("{}?{}", path, query_string.join("&"))
                }
            } else {
                path.clone()
            };

            let _ = std::fs::write("/tmp/test_client_full_path.log", format!("full_path: {}\n", full_path));
            let mut request = server.get(&full_path);
            let _ = std::fs::write(
                "/tmp/test_client_request_created.log",
                "Request created, about to await\n",
            );

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            let _ = std::fs::write(
                "/tmp/test_client_response_received.log",
                format!("Response received, status: {}\n", response.status_code()),
            );
            let test_response = TestResponse::from_axum_response(response).await?;
            let _ = std::fs::write("/tmp/test_client_conversion_done.log", "Converted to TestResponse\n");
            Ok(test_response)
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a POST request
    ///
    /// Args:
    ///     path: The path to request
    ///     json: Optional JSON body as a dict
    ///     headers: Optional headers as a dict
    ///
    /// Returns:
    ///     TestResponse: The response from the server
    #[pyo3(signature = (path, json=None, query_params=None, headers=None))]
    fn post<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        json: Option<&Bound<'py, PyAny>>,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let json_value = if let Some(j) = json {
            Some(python_to_json_value(py, j)?)
        } else {
            None
        };
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.post(&full_path);

            if let Some(json_val) = json_value {
                request = request.json(&json_val);
            }

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a PUT request
    #[pyo3(signature = (path, json=None, query_params=None, headers=None))]
    fn put<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        json: Option<&Bound<'py, PyAny>>,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let json_value = if let Some(j) = json {
            Some(python_to_json_value(py, j)?)
        } else {
            None
        };
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.put(&full_path);

            if let Some(json_val) = json_value {
                request = request.json(&json_val);
            }

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a PATCH request
    #[pyo3(signature = (path, json=None, query_params=None, headers=None))]
    fn patch<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        json: Option<&Bound<'py, PyAny>>,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let json_value = if let Some(j) = json {
            Some(python_to_json_value(py, j)?)
        } else {
            None
        };
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.patch(&full_path);

            if let Some(json_val) = json_value {
                request = request.json(&json_val);
            }

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a DELETE request
    #[pyo3(signature = (path, query_params=None, headers=None))]
    fn delete<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.delete(&full_path);

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make an OPTIONS request
    #[pyo3(signature = (path, query_params=None, headers=None))]
    fn options<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.method(Method::OPTIONS, &full_path);

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a HEAD request
    #[pyo3(signature = (path, query_params=None, headers=None))]
    fn head<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;

        let server = Arc::clone(&self.server);

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.method(Method::HEAD, &full_path);

            for (key, value) in headers_vec {
                let header_name = HeaderName::from_bytes(key.as_bytes()).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header name: {}", e))
                })?;
                let header_value = HeaderValue::from_str(&value).map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header value: {}", e))
                })?;
                request = request.add_header(header_name, header_value);
            }

            let response = request.await;
            TestResponse::from_axum_response(response).await
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }
}

/// Response from a test request
#[pyclass]
pub struct TestResponse {
    status_code: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl TestResponse {
    /// Create a TestResponse from an axum-test response
    async fn from_axum_response(response: AxumTestResponse) -> PyResult<Self> {
        let status_code = response.status_code().as_u16();

        // Extract headers
        let mut headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Get body bytes
        let body = response.into_bytes().to_vec();

        Ok(Self {
            status_code,
            headers,
            body,
        })
    }
}

#[pymethods]
impl TestResponse {
    /// Get the response status code
    #[getter]
    fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Get response headers as a dict
    #[getter]
    fn headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.headers {
            dict.set_item(key, value)?;
        }
        Ok(dict)
    }

    /// Get the response body as bytes
    fn bytes(&self) -> Vec<u8> {
        self.body.clone()
    }

    /// Get the response body as text
    fn text(&self) -> PyResult<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("Invalid UTF-8: {}", e)))
    }

    /// Get the response body as JSON
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let json_value: Value = serde_json::from_slice(&self.body)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e)))?;

        json_value_to_python(py, &json_value)
    }

    /// Assert that the status code matches
    fn assert_status(&self, expected: u16) -> PyResult<()> {
        if self.status_code == expected {
            Ok(())
        } else {
            Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Expected status {}, got {}",
                expected, self.status_code
            )))
        }
    }

    /// Assert that the status code is 200 OK
    fn assert_status_ok(&self) -> PyResult<()> {
        self.assert_status(200)
    }

    /// Assert that the status code is 201 Created
    fn assert_status_created(&self) -> PyResult<()> {
        self.assert_status(201)
    }

    /// Assert that the status code is 400 Bad Request
    fn assert_status_bad_request(&self) -> PyResult<()> {
        self.assert_status(400)
    }

    /// Assert that the status code is 404 Not Found
    fn assert_status_not_found(&self) -> PyResult<()> {
        self.assert_status(404)
    }

    /// Assert that the status code is 500 Internal Server Error
    fn assert_status_server_error(&self) -> PyResult<()> {
        self.assert_status(500)
    }

    /// Python repr
    fn __repr__(&self) -> String {
        format!("<TestResponse status={}>", self.status_code)
    }
}

/// Extract a PyDict to a Vec of (String, String) tuples
/// Handles list values by creating multiple entries with the same key
fn extract_dict_to_vec(dict: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<(String, String)>> {
    if let Some(d) = dict {
        let mut result = Vec::new();
        for (key, value) in d.iter() {
            let key: String = key.extract()?;

            // Check if value is a list - if so, add multiple entries
            if let Ok(list) = value.cast::<pyo3::types::PyList>() {
                for item in list.iter() {
                    let item_str: String = item.str()?.extract()?;
                    result.push((key.clone(), item_str));
                }
            } else {
                // Single value - convert to string
                let value: String = value.str()?.extract()?;
                result.push((key, value));
            }
        }
        Ok(result)
    } else {
        Ok(Vec::new())
    }
}

fn build_full_path(path: &str, query_params: &[(String, String)]) -> String {
    if query_params.is_empty() {
        return path.to_string();
    }

    let query_string: Vec<String> = query_params.iter().map(|(k, v)| format!("{}={}", k, v)).collect();

    if path.contains('?') {
        format!("{}&{}", path, query_string.join("&"))
    } else {
        format!("{}?{}", path, query_string.join("&"))
    }
}

/// Convert Python object to serde_json::Value
fn python_to_json_value(py: Python<'_>, obj: &Bound<'_, PyAny>) -> PyResult<Value> {
    // Use json module to convert
    let json_module = py.import("json")?;
    let json_str: String = json_module.call_method1("dumps", (obj,))?.extract()?;

    serde_json::from_str(&json_str)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to parse JSON: {}", e)))
}

/// Convert serde_json::Value to Python object
fn json_value_to_python<'py>(py: Python<'py>, value: &Value) -> PyResult<Bound<'py, PyAny>> {
    // Use json module to convert
    let json_module = py.import("json")?;
    let json_str = serde_json::to_string(value)
        .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Failed to serialize JSON: {}", e)))?;
    let result = json_module.call_method1("loads", (json_str,))?;
    Ok(result)
}
