//! Ruby gRPC bindings for Spikard
//!
//! This module provides Ruby gRPC handler integration with full streaming support:
//! - Unary RPCs (single request, single response)
//! - Server streaming RPCs (single request, stream of responses)
//! - Client streaming RPCs (stream of requests, single response)
//! - Bidirectional streaming RPCs (stream of requests, stream of responses)

pub mod handler;

pub use handler::{RubyGrpcHandler, RubyGrpcRequest, RubyGrpcResponse};
