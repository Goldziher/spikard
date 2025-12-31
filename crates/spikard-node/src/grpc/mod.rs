//! Node.js bindings for Spikard gRPC support
//!
//! This module provides the TypeScript/Node.js FFI layer for gRPC handlers,
//! allowing JavaScript code to handle gRPC requests via the napi-rs bridge.
//!
//! ## Architecture
//!
//! The gRPC binding follows the same pattern as the HTTP handler:
//!
//! 1. **NodeGrpcHandler**: Implements `spikard_http::grpc::GrpcHandler` trait using ThreadsafeFunction
//! 2. **GrpcRequest/GrpcResponse**: napi-rs objects for FFI data transfer
//! 3. **Service registration**: Maps service names to handlers
//!
//! ## Thread Safety
//!
//! The implementation uses:
//! - ThreadsafeFunction to safely call JavaScript from Rust async tasks
//! - Arc-wrapped handlers for safe sharing across services
//! - Proper metadata conversion between tonic and napi types
//!
//! ## Example
//!
//! ```typescript
//! import { GrpcService, GrpcRequest, GrpcResponse } from 'spikard';
//! import * as $protobuf from 'protobufjs';
//!
//! class UserServiceHandler {
//!     async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
//!         // Deserialize using protobufjs
//!         const req = UserService.GetUserRequest.decode(request.payload);
//!
//!         // Process request
//!         const user = { id: req.id, name: 'John Doe' };
//!
//!         // Serialize response
//!         return {
//!             payload: Buffer.from(UserService.User.encode(user).finish())
//!         };
//!     }
//! }
//! ```

pub mod handler;
#[cfg(test)]
mod test_grpc;

// Re-export main types
pub use handler::{GrpcMetadata, GrpcRequest, GrpcResponse, NodeGrpcHandler};
