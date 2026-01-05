//! `PyO3` wrapper for the core Spikard test client
//!
//! This module bridges the language-agnostic test client from spikard_http
//! to Python, providing a Pythonic API surface using `PyO3`.

use crate::conversion::{json_to_python, python_to_json};
use crate::testing::sse;
use crate::testing::websocket;
use axum::Router as AxumRouter;
use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};
use serde_json::Value;
use spikard_http::testing::{MultipartFilePart, ResponseSnapshot, TestClient as CoreTestClient};
use std::sync::Arc;

/// A test client for making requests to a Spikard application
///
/// This wraps the core TestClient from spikard_http and provides a Python-friendly interface.
#[pyclass]
pub struct TestClient {
    client: Arc<CoreTestClient>,
}

impl TestClient {
    /// Create a new test client from an Axum router
    pub fn from_router(router: AxumRouter) -> PyResult<Self> {
        let client = CoreTestClient::from_router(router)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to create test server: {}", e)))?;

        Ok(Self {
            client: Arc::new(client),
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
    ///     Test`Response`: The response from the server
    #[pyo3(signature = (path, query_params=None, headers=None, cookies=None))]
    fn get<'py>(
        &self,
        py: Python<'py>,
        path: &str,
        query_params: Option<&Bound<'py, PyDict>>,
        headers: Option<&Bound<'py, PyDict>>,
        cookies: Option<&Bound<'py, PyDict>>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let mut headers_vec = extract_dict_to_vec(headers)?;
        let client = Arc::clone(&self.client);

        if let Some(cookies_dict) = cookies {
            let cookies_vec = extract_dict_to_vec(Some(cookies_dict))?;
            if !cookies_vec.is_empty() {
                let cookie_header_value: Vec<String> =
                    cookies_vec.iter().map(|(k, v)| format!("{}={}", k, v)).collect();
                headers_vec.push(("cookie".to_string(), cookie_header_value.join("; ")));
            }
        }

        let fut = async move {
            client
                .get(&path, Some(query_params_vec), wrap_optional_vec(headers_vec))
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Make a POST request
    ///
    /// Args:
    ///     path: The path to request
    ///     json: Optional JSON body as a dict
    ///     data: Optional form data (dict, str, or bytes)
    ///     files: Optional files for multipart/form-data upload
    ///     query_params: Optional query parameters
    ///     headers: Optional headers as a dict
    ///
    /// Returns:
    ///     Test`Response`: The response from the server
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
        let json_value = python_to_json_opt(py, json)?;
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;
        let client = Arc::clone(&self.client);

        let mut form_data = Vec::new();
        let mut raw_body: Option<Vec<u8>> = None;
        if let Some(obj) = data {
            if let Ok(dict) = obj.cast::<PyDict>() {
                form_data = extract_dict_to_vec(Some(dict))?;
            } else if let Ok(py_bytes) = obj.cast::<pyo3::types::PyBytes>() {
                raw_body = Some(py_bytes.as_bytes().to_vec());
            } else if let Ok(py_str) = obj.cast::<pyo3::types::PyString>() {
                raw_body = Some(py_str.to_str()?.as_bytes().to_vec());
            } else {
                return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                    "data must be a dict, str, or bytes",
                ));
            }
        }

        let files_data = extract_files(files)?;

        let fut = async move {
            let multipart =
                if !files_data.is_empty() || (!form_data.is_empty() && raw_body.is_none() && json_value.is_none()) {
                    Some((form_data.clone(), files_data))
                } else {
                    None
                };

            let form_for_encoding = if multipart.is_none() && raw_body.is_none() && !form_data.is_empty() {
                Some(form_data)
            } else {
                None
            };

            client
                .post(
                    &path,
                    json_value,
                    form_for_encoding,
                    multipart,
                    Some(query_params_vec),
                    wrap_optional_vec(headers_vec),
                )
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let json_value = python_to_json_opt(py, json)?;
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .put(
                    &path,
                    json_value,
                    Some(query_params_vec),
                    wrap_optional_vec(headers_vec),
                )
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let json_value = python_to_json_opt(py, json)?;
        let query_params_vec = extract_dict_to_vec(query_params)?;
        let headers_vec = extract_dict_to_vec(headers)?;
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .patch(
                    &path,
                    json_value,
                    Some(query_params_vec),
                    wrap_optional_vec(headers_vec),
                )
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .delete(&path, Some(query_params_vec), wrap_optional_vec(headers_vec))
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .options(&path, Some(query_params_vec), wrap_optional_vec(headers_vec))
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .head(&path, Some(query_params_vec), wrap_optional_vec(headers_vec))
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
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
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .trace(&path, Some(query_params_vec), wrap_optional_vec(headers_vec))
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Connect to a WebSocket endpoint
    fn websocket<'py>(&self, py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let client = Arc::clone(&self.client);

        let fut = async move { websocket::connect_websocket_for_test(client.server(), &path).await };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Connect to a Server-Sent Events endpoint
    fn sse<'py>(&self, py: Python<'py>, path: &str) -> PyResult<Bound<'py, PyAny>> {
        let path = path.to_string();
        let client = Arc::clone(&self.client);

        let fut = async move {
            let axum_response = client.server().get(&path).await;
            let snapshot = spikard_http::testing::snapshot_response(axum_response)
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            sse::sse_stream_from_response(&snapshot)
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Send a GraphQL query/mutation
    ///
    /// Args:
    ///     query: GraphQL query string
    ///     variables: Optional GraphQL variables dict
    ///     operation_name: Optional operation name string
    ///
    /// Returns:
    ///     Test`Response` with GraphQL response
    #[pyo3(signature = (query, variables=None, operation_name=None))]
    fn graphql<'py>(
        &self,
        py: Python<'py>,
        query: String,
        variables: Option<&Bound<'py, PyAny>>,
        operation_name: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let variables_json = python_to_json_opt(py, variables)?;
        let client = Arc::clone(&self.client);

        let fut = async move {
            client
                .graphql(&query, variables_json, operation_name.as_deref())
                .await
                .map(|snapshot| TestResponse { snapshot })
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }

    /// Send a GraphQL query and get HTTP status separately
    ///
    /// Args:
    ///     query: GraphQL query string
    ///     variables: Optional GraphQL variables dict
    ///     operation_name: Optional operation name string
    ///
    /// Returns:
    ///     Tuple of (status_code, Test`Response`)
    #[pyo3(signature = (query, variables=None, operation_name=None))]
    fn graphql_with_status<'py>(
        &self,
        py: Python<'py>,
        query: String,
        variables: Option<&Bound<'py, PyAny>>,
        operation_name: Option<String>,
    ) -> PyResult<Bound<'py, PyAny>> {
        let variables_json = python_to_json_opt(py, variables)?;
        let client = Arc::clone(&self.client);

        let fut = async move {
            let (status, snapshot) = client
                .graphql_with_status(&query, variables_json, operation_name.as_deref())
                .await
                .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

            Ok((status, TestResponse { snapshot }))
        };

        pyo3_async_runtimes::tokio::future_into_py(py, fut)
    }
}

/// `Response` from a test request
#[pyclass]
pub struct TestResponse {
    snapshot: ResponseSnapshot,
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

        json_to_python(py, &json_value)
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

    /// Assert that the status code is 400 Bad `Request`
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

    /// Extract GraphQL data from response
    ///
    /// Returns:
    ///     The 'data' field from the GraphQL response as a Python object
    fn graphql_data<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let data = self
            .snapshot
            .graphql_data()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        json_to_python(py, &data)
    }

    /// Extract GraphQL errors from response
    ///
    /// Returns:
    ///     A list of error objects from the GraphQL response
    fn graphql_errors<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyList>> {
        let errors = self
            .snapshot
            .graphql_errors()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e.to_string()))?;

        let py_list = PyList::empty(py);
        for error in errors {
            let py_error = json_to_python(py, &error)?;
            py_list.append(py_error)?;
        }
        Ok(py_list)
    }
}

/// Convert a vector of (String, String) to Option<Vec<...>> if non-empty
/// This wraps header and query param vectors into the format expected by the test client
#[inline]
fn wrap_optional_vec(vec: Vec<(String, String)>) -> Option<Vec<(String, String)>> {
    if vec.is_empty() { None } else { Some(vec) }
}

/// Convert optional Python value to JSON, handling `None` case
/// This eliminates duplicated json conversion logic across methods
#[inline]
fn python_to_json_opt(py: Python<'_>, value: Option<&Bound<'_, PyAny>>) -> PyResult<Option<Value>> {
    value.map(|v| python_to_json(py, v)).transpose()
}

/// Extract a `PyDict` to a Vec of (String, String) tuples
/// Handles list values by creating multiple entries with the same key
fn extract_dict_to_vec(dict: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<(String, String)>> {
    if let Some(d) = dict {
        let mut result = Vec::new();
        for (key, value) in d.iter() {
            let key: String = key.extract()?;

            if let Ok(list) = value.cast::<PyList>() {
                for item in list.iter() {
                    let item_str: String = item.str()?.extract()?;
                    result.push((key.clone(), item_str));
                }
            } else {
                let value: String = value.str()?.extract()?;
                result.push((key, value));
            }
        }
        Ok(result)
    } else {
        Ok(Vec::new())
    }
}

/// Extract files from Python dict
/// Expects: {"field": ("filename", bytes), "field2": [("file1", bytes), ("file2", bytes)]}
fn extract_files(files_dict: Option<&Bound<'_, PyDict>>) -> PyResult<Vec<MultipartFilePart>> {
    let Some(files) = files_dict else {
        return Ok(Vec::new());
    };

    let mut result = Vec::new();

    for (key, value) in files.iter() {
        let field_name: String = key.extract()?;

        if let Ok(list) = value.cast::<PyList>() {
            for item in list.iter() {
                let file_data = extract_single_file(&field_name, &item)?;
                result.push(file_data);
            }
        } else {
            let file_data = extract_single_file(&field_name, &value)?;
            result.push(file_data);
        }
    }

    Ok(result)
}

/// Extract a single file from Python tuple (filename, bytes)
fn extract_single_file(field_name: &str, tuple: &Bound<'_, PyAny>) -> PyResult<MultipartFilePart> {
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

    let content_type = if tuple.len() >= 3 {
        tuple.get_item(2).ok().and_then(|v| v.extract().ok())
    } else {
        None
    };

    Ok(MultipartFilePart {
        field_name: field_name.to_string(),
        filename,
        content,
        content_type,
    })
}
