//! Python gRPC bindings for Spikard
//!
//! This module provides Python FFI bindings for gRPC functionality,
//! allowing Python code to implement gRPC handlers and connect to
//! Spikard's gRPC runtime.

pub mod handler;

// Re-export main types
pub use handler::{PyGrpcHandler, PyGrpcRequest, PyGrpcResponse};

use pyo3::types::PyModuleMethods;

/// Initialize Python gRPC module
///
/// This should be called during module initialization to register
/// the gRPC types with Python.
pub fn init_module(module: &pyo3::Bound<'_, pyo3::types::PyModule>) -> pyo3::PyResult<()> {
    module.add_class::<PyGrpcRequest>()?;
    module.add_class::<PyGrpcResponse>()?;
    Ok(())
}
