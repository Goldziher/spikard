//! Ruby gRPC bindings for Spikard
//!
//! This module provides a bridge between Ruby code and Spikard's gRPC runtime,
//! allowing Ruby handlers to process gRPC requests using protobuf serialization.

pub mod handler;

#[allow(unused_imports)]
pub use handler::{RubyGrpcHandler, RubyGrpcRequest, RubyGrpcResponse};
