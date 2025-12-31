//! PHP gRPC bindings for Spikard
//!
//! This module provides PHP FFI bindings for gRPC functionality using ext-php-rs,
//! allowing PHP code to implement gRPC handlers and connect to Spikard's gRPC runtime.

pub mod handler;

// Re-export main types
pub use handler::{PhpGrpcHandler, PhpGrpcRequest, PhpGrpcResponse};
