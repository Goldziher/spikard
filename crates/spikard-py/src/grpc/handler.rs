//! Python gRPC `handler` implementation
//!
//! This module provides `PyO3` bindings for gRPC request/response handling,
//! enabling Python code to implement gRPC service handlers.

use bytes::Bytes;
use futures::stream::StreamExt;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict};
use spikard_http::grpc::streaming::{MessageStream, StreamingRequest};
use spikard_http::grpc::{GrpcHandler, GrpcHandlerResult, GrpcRequestData, GrpcResponseData};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tonic::metadata::MetadataMap;

/// Python-side gRPC message stream for client streaming RPC
///
/// Wraps a Rust `MessageStream` and exposes Python async iterator protocol
/// so Python handlers can consume incoming messages as an async iterable.
#[pyclass(name = "GrpcMessageStream")]
pub struct PyGrpcMessageStream {
    /// The underlying Rust message stream wrapped for sharing across FFI boundary
    stream: Arc<tokio::sync::Mutex<Option<MessageStream>>>,
}

#[pymethods]
impl PyGrpcMessageStream {
    /// String representation for debugging
    fn __repr__(&self) -> String {
        "GrpcMessageStream(async_iterator)".to_string()
    }

    /// Implement async iterator protocol: __aiter__ returns self
    fn __aiter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    /// Implement async iterator protocol: __anext__ returns awaitable
    ///
    /// Returns a coroutine that yields the next message or raises StopAsyncIteration
    fn __anext__(&mut self, py: Python<'_>) -> PyResult<Py<PyAny>> {
        // Clone the Arc to access the stream
        let stream_arc = self.stream.clone();

        // Create a future that fetches the next message
        let future = async move {
            let mut stream_opt = stream_arc.lock().await;

            if let Some(stream) = stream_opt.as_mut() {
                match stream.next().await {
                    Some(Ok(msg)) => {
                        // Return message as Python bytes
                        Python::attach(|py| {
                            let py_bytes = PyBytes::new(py, &msg);
                            Ok(py_bytes.into_any().unbind())
                        })
                    }
                    Some(Err(status)) => {
                        // Convert tonic::Status to Python exception
                        Err(PyErr::new::<pyo3::exceptions::PyException, _>(format!(
                            "gRPC error: {}",
                            status.message()
                        )))
                    }
                    None => {
                        // Stream exhausted
                        Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(""))
                    }
                }
            } else {
                // Stream was already consumed
                Err(PyErr::new::<pyo3::exceptions::PyStopAsyncIteration, _>(""))
            }
        };

        // Schedule the future on the Python event loop and return the coroutine object
        pyo3_async_runtimes::tokio::future_into_py(py, future).map(|bound| bound.unbind())
    }
}

/// Helper function to convert Option<HashMap> to `PyDict` (DRY)
fn option_hashmap_to_pydict(py: Python<'_>, map: Option<HashMap<String, String>>) -> PyResult<Bound<'_, PyDict>> {
    let py_dict = PyDict::new(py);
    if let Some(metadata) = map {
        for (key, value) in metadata {
            py_dict.set_item(key, value)?;
        }
    }
    Ok(py_dict)
}

/// Helper function to convert MetadataMap to `PyDict` (DRY)
fn metadata_map_to_pydict<'py>(py: Python<'py>, metadata: &MetadataMap) -> PyResult<Bound<'py, PyDict>> {
    let py_dict = PyDict::new(py);
    for key_value in metadata.iter() {
        if let tonic::metadata::KeyAndValueRef::Ascii(key, value) = key_value {
            let key_str = key.as_str();
            let value_str = value.to_str().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid metadata value for key {key_str}: {e}"
                ))
            })?;
            py_dict.set_item(key_str, value_str)?;
        }
    }
    Ok(py_dict)
}

/// Helper function to convert `PyDict` to MetadataMap (DRY)
fn pydict_to_metadata_map(_py: Python<'_>, py_dict: &Bound<'_, PyDict>) -> PyResult<MetadataMap> {
    let mut metadata = MetadataMap::new();
    for (key, value) in py_dict.iter() {
        let key_str: String = key.extract()?;
        let value_str: String = value.extract()?;

        let metadata_key = key_str
            .parse::<tonic::metadata::MetadataKey<tonic::metadata::Ascii>>()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("Invalid metadata key '{key_str}': {e}"))
            })?;

        let metadata_value = value_str
            .parse::<tonic::metadata::MetadataValue<tonic::metadata::Ascii>>()
            .map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyValueError, _>(format!(
                    "Invalid metadata value for key '{key_str}': {e}"
                ))
            })?;

        metadata.insert(metadata_key, metadata_value);
    }
    Ok(metadata)
}

/// Convert Python async generator to MessageStream
///
/// Takes a Python async generator (with `__anext__` method) and converts it to a Rust `MessageStream`.
/// The generator should yield bytes objects representing serialized protobuf messages.
fn python_async_generator_to_message_stream(
    py_generator: Py<PyAny>,
    task_locals: pyo3_async_runtimes::TaskLocals,
) -> PyResult<MessageStream> {
    use async_stream::stream;

    let message_stream = stream! {
        loop {
            // Call `__anext__()` on the Python generator to get the next value
            let next_result = Python::attach(|py| -> PyResult<Py<PyAny>> {
                let generator_obj = py_generator.bind(py);
                let next_coro = generator_obj.call_method0("__anext__")?;
                Ok(next_coro.into())
            });

            match next_result {
                Ok(coro_py) => {
                    // Schedule the coroutine on the Python event loop
                    let future_result = Python::attach(|py| -> PyResult<_> {
                        pyo3_async_runtimes::into_future_with_locals(&task_locals, coro_py.bind(py).clone())
                    });

                    match future_result {
                        Ok(fut) => {
                            // Await the coroutine to get the next message
                            match fut.await {
                                Ok(value_py) => {
                                    // Convert Python bytes to Rust Bytes
                                    let bytes_result = Python::attach(|py| -> PyResult<Bytes> {
                                        let value_obj = value_py.bind(py);
                                        let py_bytes = value_obj.cast::<PyBytes>()?;
                                        Ok(Bytes::copy_from_slice(py_bytes.as_bytes()))
                                    });

                                    match bytes_result {
                                        Ok(bytes) => yield Ok(bytes),
                                        Err(e) => {
                                            // Conversion error
                                            yield Err(tonic::Status::internal(format!(
                                                "Failed to convert Python value to bytes: {}", e
                                            )));
                                            break;
                                        }
                                    }
                                }
                                Err(e) => {
                                    // Check if it's StopAsyncIteration (normal end of stream)
                                    let is_stop_iteration = Python::attach(|py| {
                                        e.is_instance_of::<pyo3::exceptions::PyStopAsyncIteration>(py)
                                    });

                                    if !is_stop_iteration {
                                        // Error during iteration
                                        yield Err(pyerr_to_grpc_status(e));
                                    }
                                    break;
                                }
                            }
                        }
                        Err(e) => {
                            yield Err(pyerr_to_grpc_status(e));
                            break;
                        }
                    }
                }
                Err(e) => {
                    yield Err(pyerr_to_grpc_status(e));
                    break;
                }
            }
        }
    };

    Ok(Box::pin(message_stream))
}

/// Create a Python async iterator from a Rust MessageStream
///
/// Returns a Python object that implements the async iterator protocol (`__aiter__` and `__anext__`),
/// allowing Python code to consume messages from a Rust stream using `async for`.
fn create_py_stream_iterator(py: Python<'_>, stream: MessageStream) -> PyResult<Py<PyAny>> {
    let stream_wrapper = PyGrpcMessageStream {
        stream: Arc::new(tokio::sync::Mutex::new(Some(stream))),
    };
    Py::new(py, stream_wrapper).map(|p| p.into_any())
}

/// Convert Python exception to appropriate gRPC Status
///
/// Maps common Python exceptions to gRPC status codes for better error handling:
/// - `ValueError` -> INVALID_ARGUMENT
/// - PermissionError -> PERMISSION_DENIED
/// - `NotImplementedError` -> UNIMPLEMENTED
/// - TimeoutError -> DEADLINE_EXCEEDED
/// - FileNotFoundError/`KeyError` -> NOT_FOUND
/// - Default -> INTERNAL
fn pyerr_to_grpc_status(err: PyErr) -> tonic::Status {
    Python::attach(|py| {
        let err_type = err.get_type(py);
        let err_msg = err.to_string();

        // Check exception type and map to appropriate gRPC code
        if err_type
            .is_subclass_of::<pyo3::exceptions::PyValueError>()
            .unwrap_or(false)
        {
            tonic::Status::invalid_argument(err_msg)
        } else if err_type
            .is_subclass_of::<pyo3::exceptions::PyPermissionError>()
            .unwrap_or(false)
        {
            tonic::Status::permission_denied(err_msg)
        } else if err_type
            .is_subclass_of::<pyo3::exceptions::PyNotImplementedError>()
            .unwrap_or(false)
        {
            tonic::Status::unimplemented(err_msg)
        } else if err_type
            .is_subclass_of::<pyo3::exceptions::PyTimeoutError>()
            .unwrap_or(false)
        {
            tonic::Status::deadline_exceeded(err_msg)
        } else if err_type
            .is_subclass_of::<pyo3::exceptions::PyFileNotFoundError>()
            .unwrap_or(false)
            || err_type
                .is_subclass_of::<pyo3::exceptions::PyKeyError>()
                .unwrap_or(false)
        {
            tonic::Status::not_found(err_msg)
        } else {
            // Default to INTERNAL for unknown exception types
            tonic::Status::internal(format!("Python handler error: {err_msg}"))
        }
    })
}

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
        let py_metadata = option_hashmap_to_pydict(py, metadata)?;

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
    pub fn new(py: Python<'_>, payload: Vec<u8>, metadata: Option<HashMap<String, String>>) -> PyResult<Self> {
        let py_bytes = PyBytes::new(py, &payload).into();
        let py_metadata = option_hashmap_to_pydict(py, metadata)?;

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

/// Python gRPC `handler` that bridges Python code to Rust's GrpcHandler trait
///
/// This `handler` wraps a Python callable (async function or class with handle_request method)
/// and implements the GrpcHandler trait, allowing it to be used in Spikard's gRPC runtime.
pub struct PyGrpcHandler {
    /// Python `handler` object (callable or object with handle_request method)
    handler: Py<PyAny>,

    /// Fully qualified service name this `handler` serves
    /// Using `Arc`<str> instead of String to avoid memory leak when converting to &'static str
    service_name: Arc<str>,
}

impl PyGrpcHandler {
    /// Create a new Python gRPC `handler`
    ///
    /// # Arguments
    ///
    /// * `handler` - Python callable or object with async handle_request method
    /// * `service_name` - Fully qualified service name (e.g., "mypackage.MyService")
    pub fn new(handler: Py<PyAny>, service_name: String) -> Self {
        Self {
            handler,
            service_name: Arc::from(service_name.as_str()),
        }
    }

    /// Convert Rust GrpcRequestData to Python PyGrpcRequest
    fn to_py_request(py: Python<'_>, request: &GrpcRequestData) -> PyResult<PyGrpcRequest> {
        // Optimize: Use direct slice access without intermediate Vec allocation
        let py_bytes = PyBytes::new(py, &request.payload).into();
        let py_metadata = metadata_map_to_pydict(py, &request.metadata)?;

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
            let py_request = Python::attach(|py| -> PyResult<PyGrpcRequest> { Self::to_py_request(py, &request) })
                .map_err(pyerr_to_grpc_status)?;

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
                        "Handler must be callable or have a handle_request method",
                    ));
                };

                // Check if it's a coroutine (async)
                if !coroutine.hasattr("__await__")? {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be async (return a coroutine)",
                    ));
                }

                // Get the Python event loop task locals
                let task_locals = crate::handler::PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Python async context not initialized. Call init_python_event_loop() first.",
                        )
                    })?
                    .clone();

                // Schedule the coroutine on the Python event loop
                pyo3_async_runtimes::into_future_with_locals(&task_locals, coroutine.clone())
            })
            .map_err(pyerr_to_grpc_status)?;

            // Await the Python coroutine
            let result = coroutine_future.await.map_err(pyerr_to_grpc_status)?;

            // Convert Python response to Rust response
            let response = Python::attach(|py| -> PyResult<GrpcResponseData> {
                // Get the bound PyGrpcResponse from the result
                let response_obj = result.bind(py);

                // Extract payload bytes - use cast instead of deprecated downcast
                let payload_obj = response_obj.getattr("payload")?;
                let payload_bytes = payload_obj.cast::<PyBytes>()?.as_bytes();
                // Optimize: Bytes can be created from slice without intermediate Vec
                let payload = Bytes::copy_from_slice(payload_bytes);

                // Extract metadata - use cast_into instead of deprecated downcast_into
                let metadata_obj = response_obj.getattr("metadata")?;
                let metadata_dict = metadata_obj.cast_into::<pyo3::types::PyDict>()?;

                // Use helper function for DRY
                let metadata = pydict_to_metadata_map(py, &metadata_dict)?;

                Ok(GrpcResponseData { payload, metadata })
            })
            .map_err(pyerr_to_grpc_status)?;

            Ok(response)
        })
    }

    fn service_name(&self) -> &str {
        self.service_name.as_ref()
    }

    fn call_server_stream(
        &self,
        request: GrpcRequestData,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = Python::attach(|py| self.handler.clone_ref(py));

        Box::pin(async move {
            // Create Python request object
            let py_request = Python::attach(|py| -> PyResult<PyGrpcRequest> { Self::to_py_request(py, &request) })
                .map_err(pyerr_to_grpc_status)?;

            // Call Python handler and get async generator
            let py_generator_and_locals = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

                // Create a Python object from our PyGrpcRequest
                let req_obj = Py::new(py, py_request)?;

                // Check if handler has handle_server_stream method (for server streaming)
                let async_gen = if handler_obj.hasattr("handle_server_stream")? {
                    let method = handler_obj.getattr("handle_server_stream")?;
                    method.call1((req_obj.clone_ref(py),))?
                } else if handler_obj.is_callable() {
                    // Fallback to callable handler for backward compatibility
                    handler_obj.call1((req_obj.clone_ref(py),))?
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be callable or have a handle_server_stream method for server streaming",
                    ));
                };

                // Check if it's an async generator (has __anext__)
                if !async_gen.hasattr("__anext__")? {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must return an async generator for server streaming",
                    ));
                }

                // Get the Python event loop task locals
                let task_locals = crate::handler::PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Python async context not initialized. Call init_python_event_loop() first.",
                        )
                    })?
                    .clone();

                Ok((async_gen.unbind(), task_locals))
            })
            .map_err(pyerr_to_grpc_status)?;

            let (py_generator, task_locals) = py_generator_and_locals;

            // Convert Python async generator to MessageStream
            let message_stream =
                python_async_generator_to_message_stream(py_generator, task_locals).map_err(pyerr_to_grpc_status)?;

            Ok(message_stream)
        })
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<GrpcResponseData, tonic::Status>> + Send>> {
        let handler = Python::attach(|py| self.handler.clone_ref(py));

        Box::pin(async move {
            // Create a Python async iterator wrapper around the message stream
            let py_stream = Python::attach(|py| -> PyResult<Py<PyGrpcMessageStream>> {
                let stream_wrapper = PyGrpcMessageStream {
                    stream: Arc::new(tokio::sync::Mutex::new(Some(request.message_stream))),
                };
                Py::new(py, stream_wrapper)
            })
            .map_err(pyerr_to_grpc_status)?;

            // Call Python handler with the async iterator stream
            let coroutine_future = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

                // Check if handler has handle_client_stream method (for client streaming)
                let coroutine = if handler_obj.hasattr("handle_client_stream")? {
                    let method = handler_obj.getattr("handle_client_stream")?;
                    method.call1((py_stream.clone_ref(py),))?
                } else if handler_obj.is_callable() {
                    // Fallback to callable handler for backward compatibility
                    handler_obj.call1((py_stream.clone_ref(py),))?
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be callable or have a handle_client_stream method for client streaming",
                    ));
                };

                // Check if it's a coroutine (async)
                if !coroutine.hasattr("__await__")? {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be async (return a coroutine)",
                    ));
                }

                // Get the Python event loop task locals
                let task_locals = crate::handler::PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Python async context not initialized. Call init_python_event_loop() first.",
                        )
                    })?
                    .clone();

                // Schedule the coroutine on the Python event loop
                pyo3_async_runtimes::into_future_with_locals(&task_locals, coroutine.clone())
            })
            .map_err(pyerr_to_grpc_status)?;

            // Await the Python coroutine
            let result = coroutine_future.await.map_err(pyerr_to_grpc_status)?;

            // Convert Python response to Rust response
            let response = Python::attach(|py| -> PyResult<GrpcResponseData> {
                // Get the bound PyGrpcResponse from the result
                let response_obj = result.bind(py);

                // Extract payload bytes
                let payload_obj = response_obj.getattr("payload")?;
                let payload_bytes = payload_obj.cast::<PyBytes>()?.as_bytes();
                let payload = Bytes::copy_from_slice(payload_bytes);

                // Extract metadata
                let metadata_obj = response_obj.getattr("metadata")?;
                let metadata_dict = metadata_obj.cast_into::<pyo3::types::PyDict>()?;

                // Use helper function for DRY
                let metadata = pydict_to_metadata_map(py, &metadata_dict)?;

                Ok(GrpcResponseData { payload, metadata })
            })
            .map_err(pyerr_to_grpc_status)?;

            Ok(response)
        })
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
        let handler = Python::attach(|py| self.handler.clone_ref(py));

        Box::pin(async move {
            // Convert the incoming message stream to a Python async iterator
            let py_iterator = Python::attach(|py| -> PyResult<Py<PyAny>> {
                // Import the PyStreamIterator wrapper
                create_py_stream_iterator(py, request.message_stream)
            })
            .map_err(pyerr_to_grpc_status)?;

            // Call Python handler with the iterator to get an async generator
            let generator_future = Python::attach(|py| -> PyResult<_> {
                let handler_obj = handler.bind(py);

                // Check if handler is callable or has handle_bidi_stream method
                let async_gen = if handler_obj.hasattr("handle_bidi_stream")? {
                    let method = handler_obj.getattr("handle_bidi_stream")?;
                    method.call1((py_iterator.bind(py),))?
                } else if handler_obj.is_callable() {
                    handler_obj.call1((py_iterator.bind(py),))?
                } else {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must be callable or have a handle_bidi_stream method",
                    ));
                };

                // Check if it's an async generator (has __anext__)
                if !async_gen.hasattr("__anext__")? {
                    return Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
                        "Handler must return an async generator for bidirectional streaming",
                    ));
                }

                // Get the Python event loop task locals
                let task_locals = crate::handler::PYTHON_TASK_LOCALS
                    .get()
                    .ok_or_else(|| {
                        PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                            "Python async context not initialized. Call init_python_event_loop() first.",
                        )
                    })?
                    .clone();

                Ok((async_gen.unbind(), task_locals))
            })
            .map_err(pyerr_to_grpc_status)?;

            let (py_generator, task_locals) = generator_future;

            // Convert Python async generator to MessageStream
            let message_stream =
                python_async_generator_to_message_stream(py_generator, task_locals).map_err(pyerr_to_grpc_status)?;

            Ok(message_stream)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require the Python interpreter to be initialized and are
    // tested via Python integration tests instead of unit tests. They are marked
    // as ignored here to prevent failures when running `cargo test --workspace`
    // without the extension-module feature enabled. Run with `cargo test --ignored --lib -p spikard-py`
    // to execute these tests when the Python environment is properly initialized.

    #[test]
    #[ignore = "requires Python environment"]
    fn test_py_grpc_request_creation() {
        Python::attach(|py| {
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
    #[ignore = "requires Python environment"]
    fn test_py_grpc_request_with_metadata() {
        Python::attach(|py| {
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
    #[ignore = "requires Python environment"]
    fn test_py_grpc_response_creation() {
        Python::attach(|py| {
            let response = PyGrpcResponse::new(py, vec![5, 6, 7, 8], None).unwrap();

            assert_eq!(response.payload.bind(py).as_bytes(), &[5, 6, 7, 8]);
        });
    }

    #[test]
    #[ignore = "requires Python environment"]
    fn test_py_grpc_response_with_metadata() {
        Python::attach(|py| {
            let mut metadata = std::collections::HashMap::new();
            metadata.insert("content-type".to_string(), "application/grpc".to_string());

            let response = PyGrpcResponse::new(py, vec![], Some(metadata)).unwrap();

            let metadata_dict = response.metadata.bind(py);
            let value: String = metadata_dict
                .get_item("content-type")
                .unwrap()
                .unwrap()
                .extract()
                .unwrap();
            assert_eq!(value, "application/grpc");
        });
    }

    #[test]
    #[ignore = "requires Python environment"]
    fn test_py_grpc_request_repr() {
        Python::attach(|py| {
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
    #[ignore = "requires Python environment"]
    fn test_py_grpc_response_repr() {
        Python::attach(|py| {
            let response = PyGrpcResponse::new(py, vec![1, 2, 3, 4, 5], None).unwrap();

            let repr = response.__repr__();
            assert!(repr.contains("payload_size=5"));
        });
    }
}
