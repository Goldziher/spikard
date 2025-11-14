//! Test client for Spikard applications
//!
//! This module provides a test client powered by axum-test for testing Spikard applications
//! without needing to start a real HTTP server.

use crate::test_sse;
use crate::test_websocket;
use axum::Router as AxumRouter;
use axum::http::{HeaderName, HeaderValue, Method};
use axum_test::{TestResponse as AxumTestResponse, TestServer as AxumTestServer};
use once_cell::sync::Lazy;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyString};
use serde_json::Value;
use spikard_http::testing::{ResponseSnapshot, SnapshotError, snapshot_response};
use std::sync::Arc;
use tokio::runtime::{Builder, Runtime};
use urlencoding::encode;

// Global Tokio runtime for synchronous test client creation with HTTP transport
// Must be multi-threaded to support HTTP transport background tasks
pub(crate) static GLOBAL_RUNTIME: Lazy<Runtime> = Lazy::new(|| {
    Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .expect("Failed to create Tokio runtime")
});

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
        // Use default transport (no HTTP) since WebSocket support in axum-test
        // seems to have issues with HTTP transport
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
    ///     cookies: Optional cookies as a dict
    ///
    /// Returns:
    ///     TestResponse: The response from the server
    #[pyo3(signature = (path, query_params=None, headers=None, cookies=None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
        cookies: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        // Extract Python data before the async block
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let mut headers_vec = extract_dict_to_vec(headers)?;

        // Convert cookies dict to Cookie header if present
        if let Some(cookies_dict) = cookies {
            let cookies_vec = extract_dict_to_vec(Some(cookies_dict))?;
            if !cookies_vec.is_empty() {
                // Format as "name1=value1; name2=value2"
                let cookie_header_value: Vec<String> =
                    cookies_vec.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
                headers_vec.push(("cookie".to_string(), cookie_header_value.join("; ")));
            }
        }

        let server = Arc::clone(&self.server);

        let fut = async move {
            // Build full path with query string to properly handle arrays
            let full_path = if !query_params_vec.is_empty() {
                let query_string: Vec<String> = query_params_vec
                    .iter()
                    .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
                    .collect();
                if path.contains('?') {
                    format!("{}&{}", path, query_string.join("&"))
                } else {
                    format!("{}?{}", path, query_string.join("&"))
                }
            } else {
                path.clone()
            };

            let mut request = server.get(&full_path);

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

    /// Make a POST request
    ///
    /// Args:
    ///     path: The path to request
    ///     json: Optional JSON body as a dict
    ///     files: Optional files for multipart/form-data upload
    ///     query_params: Optional query parameters
    ///     headers: Optional headers as a dict
    ///
    /// Returns:
    ///     TestResponse: The response from the server
    #[allow(clippy::too_many_arguments)]
    #[pyo3(signature = (path, json=None, data=None, files=None, query_params=None, headers=None))]
    fn post<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        json: Option<&Bound<'py, PyAny>>,
        data: Option<&Bound<'py, PyAny>>,
        files: Option<&Bound<'py, PyDict>>,
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

        // Extract form data for multipart or capture raw body content
        let mut form_data = Vec::new();
        let mut raw_body: Option<Vec<u8>> = None;
        if let Some(obj) = data {
            if let Ok(dict) = obj.cast::<PyDict>() {
                #[allow(clippy::needless_borrow)]
                {
                    form_data = extract_dict_to_vec(Some(&dict))?;
                }
            } else if let Ok(py_bytes) = obj.cast::<PyBytes>() {
                raw_body = Some(py_bytes.as_bytes().to_vec());
            } else if let Ok(py_str) = obj.cast::<PyString>() {
                raw_body = Some(py_str.to_str()?.as_bytes().to_vec());
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "data must be a dict, str, or bytes",
                ));
            }
        }

        // Extract files data for multipart
        let files_data = extract_files(files)?;

        let server = Arc::clone(&self.server);
        let raw_body = raw_body;

        let fut = async move {
            let full_path = build_full_path(&path, &query_params_vec);
            let mut request = server.post(&full_path);

            // Check if this is a multipart request (files or form data provided)
            let is_multipart = !files_data.is_empty() || !form_data.is_empty();

            // Check if this is a URL-encoded form request
            let is_form_encoded = headers_vec.iter().any(|(k, v)| {
                k.eq_ignore_ascii_case("content-type") && v.contains("application/x-www-form-urlencoded")
            });

            // Add headers and body
            if is_multipart {
                // Build multipart/form-data body
                let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
                let multipart_body = build_multipart_body(&form_data, &files_data, boundary);

                // Set Content-Type with boundary
                request = request.add_header(
                    HeaderName::from_static("content-type"),
                    HeaderValue::from_str(&format!("multipart/form-data; boundary={}", boundary)).map_err(|e| {
                        PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid header: {}", e))
                    })?,
                );

                // Set body
                request = request.bytes(bytes::Bytes::from(multipart_body));
            } else if let Some(body) = raw_body {
                request = request.bytes(bytes::Bytes::from(body));
            } else if let Some(json_val) = json_value {
                if is_form_encoded {
                    // For form-encoded requests, manually set Content-Type header and body bytes
                    let form_body = match &json_val {
                        serde_json::Value::String(s) => {
                            // If json is a string, use it directly as the form body
                            s.clone()
                        }
                        serde_json::Value::Object(map) => {
                            // If json is an object, encode it as form data
                            let mut form_data: Vec<String> = Vec::new();
                            for (k, v) in map.iter() {
                                match v {
                                    serde_json::Value::Array(arr) => {
                                        // For arrays, repeat the key for each value
                                        for item in arr {
                                            let item_str = match item {
                                                serde_json::Value::String(s) => s.clone(),
                                                serde_json::Value::Number(n) => n.to_string(),
                                                serde_json::Value::Bool(b) => b.to_string(),
                                                serde_json::Value::Null => String::new(),
                                                _ => serde_json::to_string(item).unwrap_or_default(),
                                            };
                                            form_data.push(format!(
                                                "{}={}",
                                                urlencoding::encode(k),
                                                urlencoding::encode(&item_str)
                                            ));
                                        }
                                    }
                                    _ => {
                                        // For scalar values, add single key=value pair
                                        let value_str = match v {
                                            serde_json::Value::String(s) => s.clone(),
                                            serde_json::Value::Number(n) => n.to_string(),
                                            serde_json::Value::Bool(b) => b.to_string(),
                                            serde_json::Value::Null => String::new(),
                                            _ => serde_json::to_string(v).unwrap_or_default(),
                                        };
                                        form_data.push(format!(
                                            "{}={}",
                                            urlencoding::encode(k),
                                            urlencoding::encode(&value_str)
                                        ));
                                    }
                                }
                            }
                            form_data.join("&")
                        }
                        _ => {
                            return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                                "Form-encoded body must be a dict or string",
                            ));
                        }
                    };

                    // Set Content-Type header
                    request = request.add_header(
                        HeaderName::from_static("content-type"),
                        HeaderValue::from_static("application/x-www-form-urlencoded"),
                    );

                    // Set body as bytes (doesn't set Content-Type)
                    request = request.bytes(bytes::Bytes::from(form_body));
                } else {
                    request = request.json(&json_val);
                }
            }

            // Add remaining headers (skip Content-Type for form-encoded since we set it above)
            for (key, value) in headers_vec {
                if is_form_encoded && key.eq_ignore_ascii_case("content-type") {
                    continue;
                }

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

    /// Make a TRACE request
    #[pyo3(signature = (path, query_params=None, headers=None))]
    fn trace<'py>(
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
            let mut request = server.method(Method::TRACE, &full_path);

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

    /// Connect to a WebSocket endpoint
    ///
    /// Args:
    ///     path: The WebSocket endpoint path (e.g., "/ws")
    ///
    /// Returns:
    ///     WebSocketTestConnection: A WebSocket connection for testing
    fn websocket<'py>(&self, py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let server = Arc::clone(&self.server);

        // Run the WebSocket connection in the GLOBAL_RUNTIME, then convert to Python
        let fut = GLOBAL_RUNTIME.spawn(async move { test_websocket::connect_websocket_for_test(&server, &path).await });

        // Convert the JoinHandle into a Python awaitable
        pyo3_async_runtimes::tokio::future_into_py(py, async move {
            fut.await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("WebSocket task failed: {}", e)))?
        })
    }

    /// Connect to a Server-Sent Events endpoint
    ///
    /// Args:
    ///     path: The SSE endpoint path (e.g., "/events")
    ///
    /// Returns:
    ///     SseStream: An SSE stream for testing
    fn sse<'py>(&self, py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let server = Arc::clone(&self.server);

        let fut = async move {
            // Make GET request to SSE endpoint
            let axum_response = server.get(&path).await;
            let snapshot = snapshot_response(axum_response).await.map_err(snapshot_err_to_py)?;

            // Parse SSE stream from response
            let sse_stream = test_sse::sse_stream_from_response(&snapshot)?;

            Ok(sse_stream)
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }
}

/// Response from a test request
#[pyclass]
pub struct TestResponse {
    snapshot: ResponseSnapshot,
}

impl TestResponse {
    /// Create a TestResponse from an axum-test response
    async fn from_axum_response(response: AxumTestResponse) -> PyResult<Self> {
        let snapshot = snapshot_response(response).await.map_err(snapshot_err_to_py)?;
        Ok(Self { snapshot })
    }
}

#[pymethods]
impl TestResponse {
    /// Get the response status code
    #[getter]
    fn status_code(&self) -> u16 {
        self.snapshot.status
    }

    /// Get response headers as a dict
    #[getter]
    fn headers<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new(py);
        for (key, value) in &self.snapshot.headers {
            dict.set_item(key, value)?;
        }
        Ok(dict)
    }

    /// Get the response body as bytes
    fn bytes(&self) -> Vec<u8> {
        self.snapshot.body.clone()
    }

    /// Get the response body as text
    fn text(&self) -> PyResult<String> {
        self.snapshot
            .text()
            .map_err(|e| pyo3::exceptions::PyUnicodeDecodeError::new_err(format!("Invalid UTF-8: {}", e)))
    }

    /// Get the response body as JSON
    fn json<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let json_value: Value = self
            .snapshot
            .json()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("Invalid JSON: {}", e)))?;

        json_value_to_python(py, &json_value)
    }

    /// Assert that the status code matches
    fn assert_status(&self, expected: u16) -> PyResult<()> {
        let actual = self.status_code();
        if actual == expected {
            Ok(())
        } else {
            Err(pyo3::exceptions::PyAssertionError::new_err(format!(
                "Expected status {}, got {}",
                expected, actual
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
        format!("<TestResponse status={}>", self.snapshot.status)
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

    let query_string: Vec<String> = query_params
        .iter()
        .map(|(k, v)| format!("{}={}", encode(k), encode(v)))
        .collect();

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

fn snapshot_err_to_py(err: SnapshotError) -> PyErr {
    pyo3::exceptions::PyRuntimeError::new_err(err.to_string())
}

/// File data for multipart upload
#[derive(Debug, Clone)]
struct FileData {
    field_name: String,
    filename: String,
    content: Vec<u8>,
    content_type: Option<String>,
}

/// Extract files from Python dict
/// Expects: {"field": ("filename", bytes), "field2": [("file1", bytes), ("file2", bytes)]}
fn extract_files(files_dict: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<FileData>> {
    let Some(files) = files_dict else {
        return Ok(Vec::new());
    };

    let mut result = Vec::new();

    for (key, value) in files.iter() {
        let field_name: String = key.extract()?;

        // Check if value is a list (multiple files for same field)
        if let Ok(list) = value.cast::<pyo3::types::PyList>() {
            for item in list.iter() {
                let file_data = extract_single_file(&field_name, &item)?;
                result.push(file_data);
            }
        } else {
            // Single file tuple
            let file_data = extract_single_file(&field_name, &value)?;
            result.push(file_data);
        }
    }

    Ok(result)
}

/// Extract a single file from Python tuple (filename, bytes)
fn extract_single_file(field_name: &str, tuple: &Bound<'_, PyAny>) -> PyResult<FileData> {
    use pyo3::types::PyTuple;

    let tuple = tuple
        .cast::<PyTuple>()
        .map_err(|_| PyErr::new::<pyo3::exceptions::PyTypeError, _>("File must be a tuple (filename, bytes)"))?;

    if tuple.len() < 2 {
        return Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            "File tuple must have at least 2 elements: (filename, bytes)",
        ));
    }

    let filename: String = tuple.get_item(0)?.extract()?;
    let content: Vec<u8> = tuple.get_item(1)?.extract()?;

    // Optional content_type (3rd element if provided)
    let content_type = if tuple.len() >= 3 {
        tuple.get_item(2).ok().and_then(|v| v.extract().ok())
    } else {
        None
    };

    Ok(FileData {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}

/// Build multipart/form-data body
fn build_multipart_body(form_data: &[(String, String)], files: &[FileData], boundary: &str) -> Vec<u8> {
    let mut body = Vec::new();

    // Add form fields first
    for (field_name, field_value) in form_data {
        // Boundary line
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");

        // Content-Disposition header (no filename for regular fields)
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(field_name.as_bytes());
        body.extend_from_slice(b"\"\r\n");

        // Empty line before content
        body.extend_from_slice(b"\r\n");

        // Field value
        body.extend_from_slice(field_value.as_bytes());

        // CRLF after content
        body.extend_from_slice(b"\r\n");
    }

    // Add files
    for file in files {
        // Boundary line
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(b"\r\n");

        // Content-Disposition header
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        body.extend_from_slice(file.field_name.as_bytes());
        body.extend_from_slice(b"\"; filename=\"");
        body.extend_from_slice(file.filename.as_bytes());
        body.extend_from_slice(b"\"\r\n");

        // Content-Type header (if specified)
        if let Some(ref content_type) = file.content_type {
            body.extend_from_slice(b"Content-Type: ");
            body.extend_from_slice(content_type.as_bytes());
            body.extend_from_slice(b"\r\n");
        }

        // Empty line before content
        body.extend_from_slice(b"\r\n");

        // File content
        body.extend_from_slice(&file.content);

        // CRLF after content
        body.extend_from_slice(b"\r\n");
    }

    // Final boundary
    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--\r\n");

    body
}
