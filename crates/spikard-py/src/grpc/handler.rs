//! Python gRPC handler implementation
//!
//! This module provides PyO3 bindings for gRPC request/response handling,
//! enabling Python code to implement gRPC service handlers.

use bytes::Bytes;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tonic::metadata::MetadataMap;

/// Python-side gRPC request
///
/// Represents a gRPC request that is passed to Python handlers.
/// Contains the service name, method name, serialized protobuf payload,
/// and metadata (gRPC headers).
#[pyclass(name = "GrpcRequest")]
pub struct PyGrpcRequest {
    /// Fully qualified service name (e.g., "mypackage.MyService")
    #[pyo3(get)]
    pub service_name: String,

    /// Method name (e.g., "GetUser")
    #[pyo3(get)]
    pub method_name: String,

    /// Serialized protobuf message as bytes
    #[pyo3(get)]
    pub payload: Py<PyBytes>,

    /// gRPC metadata (headers) as a dictionary
    #[pyo3(get)]
    pub metadata: Py<PyDict>,
}

#[pymethods]
impl PyGrpcRequest {
    /// Create a new gRPC request
    #[new]
    #[pyo3(signature = (service_name, method_name, payload, metadata = None))]
    pub fn new(
        py: Python<'_>,
        service_name: String,
        method_name: String,
        payload: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<Self> {
        let py_bytes = PyBytes::new(py, &payload).into();

        let py_metadata = PyDict::new(py);
        if let Some(meta) = metadata {
            for (key, value) in meta {
                py_metadata.set_item(key, value)?;
            }
        }

        Ok(Self {
            service_name,
            method_name,
            payload: py_bytes,
            metadata: py_metadata.into(),
        })
    }

    /// Get metadata value by key
    pub fn get_metadata(&self, py: Python<'_>, key: &str) -> PyResult<Option<String>> {
        let metadata = self.metadata.bind(py);
        match metadata.get_item(key)? {
            Some(value) => Ok(Some(value.extract()?)),
            None => Ok(None),
        }
    }

    /// String representation for debugging
    fn __repr__(&self) -> String {
        format!(
            "GrpcRequest(service_name='{}', method_name='{}', payload_size={})",
            self.service_name,
            self.method_name,
            Python::attach(|py| self.payload.bind(py).len().unwrap_or(0))
        )
    }
}

/// Python-side gRPC response
///
/// Represents a gRPC response returned from Python handlers.
/// Contains the serialized protobuf payload and optional metadata.
#[pyclass(name = "GrpcResponse")]
pub struct PyGrpcResponse {
    /// Serialized protobuf message as bytes
    #[pyo3(get, set)]
    pub payload: Py<PyBytes>,

    /// gRPC metadata (headers) to include in response
    #[pyo3(get, set)]
    pub metadata: Py<PyDict>,
}

#[pymethods]
impl PyGrpcResponse {
    /// Create a new gRPC response
    #[new]
    #[pyo3(signature = (payload, metadata = None))]
    pub fn new(
        py: Python<'_>,
        payload: Vec<u8>,
        metadata: Option<HashMap<String, String>>,
    ) -> PyResult<Self> {
        let py_bytes = PyBytes::new(py, &payload).into();

        let py_metadata = PyDict::new(py);
        if let Some(meta) = metadata {
            for (key, value) in meta {
                py_metadata.set_item(key, value)?;
            }
        }

        Ok(Self {
            payload: py_bytes,
            metadata: py_metadata.into(),
        })
    }

    /// String representation for debugging
    fn __repr__(&self) -> String {
        format!(
            "GrpcResponse(payload_size={})",
            Python::attach(|py| self.payload.bind(py).len().unwrap_or(0))
        )
    }
}

/// Python gRPC handler that bridges Python code to Rust's GrpcHandler trait
///
/// This handler wraps a Python callable (async function or class with handle_request method)
/// and implements the GrpcHandler trait, allowing it to be used in Spikard's gRPC runtime.
pub struct PyGrpcHandler {
    /// Python handler object (callable or object with handle_request method)
    handler: Py<PyAny>,

    /// Fully qualified service name this handler serves
    service_name: String,
}

impl PyGrpcHandler {
    /// Create a new Python gRPC handler
    ///
    /// # Arguments
    ///
    /// * `handler` - Python callable or object with async handle_request method
    /// * `service_name` - Fully qualified service name (e.g., "mypackage.MyService")
    pub fn new(handler: Py<PyAny>, service_name: String) -> Self {
        Self {
            handler,
            service_name,
        }
    }

    /// Convert Rust GrpcRequestData to Python PyGrpcRequest
    fn to_py_request(py: Python<'_>, request: &GrpcRequestData) -> PyResult<PyGrpcRequest> {
        let payload_bytes = request.payload.to_vec();
        let py_bytes = PyBytes::new(py, &payload_bytes).into();

        let py_metadata = PyDict::new(py);
        for key_value in request.metadata.iter() {
            if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value {
                let key_str = key.as_str();
                let value_str = value.to_str().map_err(|e| {
                    PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                        "Invalid metadata value for key {}: {}",
                        key_str, e
                    ))
                })?;
                py_metadata.set_item(key_str, value_str)?;
            }
        }

        Ok(PyGrpcRequest {
            service_name: request.service_name.clone(),
            method_name: request.method_name.clone(),
            payload: py_bytes,
            metadata: py_metadata.into(),
        })
    }

}

impl GrpcHandler for PyGrpcHandler {
    fn call(&self, request: GrpcRequestData) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        let handler = Python::attach(|py| self.handler.clone_ref(py));

        Box::pin(async move {
            // Create Python request object
            let py_request = Python::attach(|py| -> PyResult<PyGrpcRequest> {
                Self::to_py_request(py, &request)
            })
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to create Python request: {}", e))
            })?;

            // Call Python handler and get future
            let coroutine_future = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

                // Create a Python object from our PyGrpcRequest
                let req_obj = Py::new(py, py_request)?;

                // Check if handler is callable or has handle_request method
                let coroutine = if handler_obj.is_callable() {
                    handler_obj.call1((req_obj.clone_ref(py),))?
                } else if handler_obj.hasattr("handle_request")? {
                    let method = handler_obj.getattr("handle_request")?;
                    method.call1((req_obj.clone_ref(py),))?
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be callable or have a handle_request method"
                    ));
                };

                // Check if it's a coroutine (async)
                if !coroutine.hasattr("__await__")? {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be async (return a coroutine)"
                    ));
                }

                // Get the Python event loop task locals
                let task_locals = crate::handler::PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Python async context not initialized. Call init_python_event_loop() first."
                        )
                    })?
                    .clone();

                // Schedule the coroutine on the Python event loop
                pyo3_async_runtimes::into_future_with_locals(&task_locals, coroutine.clone())
            })
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to call Python handler: {}", e))
            })?;

            // Await the Python coroutine
            let result = coroutine_future.await
                .map_err(|e| {
                    tonic::Status::internal(format!("Python handler error: {}", e))
                })?;

            // Convert Python response to Rust response
            let response = Python::attach(|py| -> PyResult<GrpcResponseData> {
                // Get the bound PyGrpcResponse from the result
                let response_obj = result.bind(py);

                // Extract payload bytes
                let payload_obj = response_obj.getattr("payload")?;
                let payload_bytes = payload_obj.downcast::<PyBytes>()?.as_bytes();
                let payload = Bytes::copy_from_slice(payload_bytes);

                // Extract metadata
                let mut metadata = MetadataMap::new();
                let metadata_obj = response_obj.getattr("metadata")?;
                let metadata_dict = metadata_obj.downcast_into::<pyo3::types::PyDict>()?;

                for (key, value) in metadata_dict.iter() {
                    let key_str: String = key.extract()?;
                    let value_str: String = value.extract()?;

                    let metadata_key = key_str.parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Invalid metadata key '{}': {}",
                                key_str, e
                            ))
                        })?;

                    let metadata_value = value_str.parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
                        .map_err(|e| {
                            PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                                "Invalid metadata value for key '{}': {}",
                                key_str, e
                            ))
                        })?;

                    metadata.insert(metadata_key, metadata_value);
                }

                Ok(GrpcResponseData { payload, metadata })
            })
            .map_err(|e| {
                tonic::Status::internal(format!("Failed to convert Python response: {}", e))
            })?;

            Ok(response)
        })
    }

    fn service_name(&self) -> &'static str {
        // This is a limitation: we need to return a static string, but we have a dynamic String.
        // For now, we'll leak the string to get a 'static reference.
        // In production, service names should be known at compile time or managed differently.
        Box::leak(self.service_name.clone().into_boxed_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_grpc_request_creation() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let request = PyGrpcRequest::new(
                py,
                "test.TestService".to_string(),
                "TestMethod".to_string(),
                vec![1, 2, 3, 4],
                None,
            )
            .unwrap();

            assert_eq!(request.service_name, "test.TestService");
            assert_eq!(request.method_name, "TestMethod");
            assert_eq!(request.payload.bind(py).as_bytes(), &[1, 2, 3, 4]);
        });
    }

    #[test]
    fn test_py_grpc_request_with_metadata() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut metadata = HashMap::new();
            metadata.insert("authorization".to_string(), "Bearer token".to_string());

            let request = PyGrpcRequest::new(
                py,
                "test.TestService".to_string(),
                "TestMethod".to_string(),
                vec![],
                Some(metadata),
            )
            .unwrap();

            let auth = request.get_metadata(py, "authorization").unwrap();
            assert_eq!(auth, Some("Bearer token".to_string()));
        });
    }

    #[test]
    fn test_py_grpc_response_creation() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let response = PyGrpcResponse::new(
                py,
                vec![5, 6, 7, 8],
                None,
            )
            .unwrap();

            assert_eq!(response.payload.bind(py).as_bytes(), &[5, 6, 7, 8]);
        });
    }

    #[test]
    fn test_py_grpc_response_with_metadata() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("content-type".to_string(), "application/grpc".to_string());

            let response = PyGrpcResponse::new(
                py,
                vec![],
                Some(metadata),
            )
            .unwrap();

            let metadata_dict = response.metadata.bind(py);
            let value: String = metadata_dict.get_item("content-type").unwrap().unwrap().extract().unwrap();
            assert_eq!(value, "application/grpc");
        });
    }

    #[test]
    fn test_py_grpc_request_repr() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let request = PyGrpcRequest::new(
                py,
                "test.Service".to_string(),
                "Method".to_string(),
                vec![1, 2, 3],
                None,
            )
            .unwrap();

            let repr = request.__repr__();
            assert!(repr.contains("test.Service"));
            assert!(repr.contains("Method"));
            assert!(repr.contains("payload_size=3"));
        });
    }

    #[test]
    fn test_py_grpc_response_repr() {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let response = PyGrpcResponse::new(
                py,
                vec![1, 2, 3, 4, 5],
                None,
            )
            .unwrap();

            let repr = response.__repr__();
            assert!(repr.contains("payload_size=5"));
        });
    }
}
