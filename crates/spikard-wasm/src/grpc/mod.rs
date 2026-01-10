//! gRPC support for WebAssembly bindings
//!
//! This module provides gRPC streaming bindings for WASM targets, enabling
//! JavaScript code to implement gRPC service handlers with full streaming support.
//!
//! # Overview
//!
//! The WASM gRPC implementation follows the same architecture as the Python and Node.js bindings:
//!
//! 1. **Handler bridging**: The `WasmGrpcHandler` struct implements Rust's `GrpcHandler` trait
//!    while delegating actual handler logic to JavaScript functions.
//!
//! 2. **Type conversion**: All data crossing the WASM boundary uses JavaScript-friendly types:
//!    - Protobuf messages: `Uint8Array`
//!    - Metadata: Plain JavaScript objects
//!    - Streaming: JavaScript async iterators and generators
//!
//! 3. **Single-threaded design**: WASM has no threads, so all operations use `wasm-bindgen-futures`
//!    for async task scheduling instead of Tokio.
//!
//! # Handler Patterns
//!
//! JavaScript handlers should follow these patterns:
//!
//! ## Unary RPC
//! ```javascript
//! async function myUnaryHandler(request) {
//!   // request: { service_name, method_name, payload: Uint8Array, metadata: {} }
//!   const response_payload = await processRequest(request.payload);
//!   return {
//!     payload: new Uint8Array(response_payload),
//!     metadata: { 'content-type': 'application/grpc' }
//!   };
//! }
//! ```
//!
//! ## Server Streaming RPC
//! ```javascript
//! async function* myServerStreamHandler(request) {
//!   for await (const item of fetchItems()) {
//!     const payload = serializeItem(item);
//!     yield new Uint8Array(payload);
//!   }
//! }
//! ```
//!
//! ## Client Streaming RPC
//! ```javascript
//! async function myClientStreamHandler(stream) {
//!   // stream has .next() method returning Promise<Uint8Array | null>
//!   const messages = [];
//!   for await (const message of stream) {
//!     messages.push(message);
//!   }
//!   const response_payload = aggregateMessages(messages);
//!   return {
//!     payload: new Uint8Array(response_payload),
//!     metadata: {}
//!   };
//! }
//! ```
//!
//! ## Bidirectional Streaming RPC
//! ```javascript
//! async function* myBidiHandler(stream) {
//!   for await (const request of stream) {
//!     const response = await processMessage(request);
//!     yield new Uint8Array(response);
//!   }
//! }
//! ```
//!
//! # Streaming Implementation Details
//!
//! ## Server Streaming
//! - JavaScript handler returns an async generator (`async function*`)
//! - Generator yields `Uint8Array` messages
//! - Stream terminates when generator ends or throws
//!
//! ## Client Streaming
//! - JavaScript handler receives a `GrpcMessageStream` object
//! - Can be used in `for await` loops: `for await (const msg of stream)`
//! - Call `.next()` to manually get the next message as a Promise
//! - Returns Promise<Uint8Array | null>
//!
//! ## Bidirectional Streaming
//! - JavaScript handler receives a `GrpcMessageStream` for input
//! - Returns an async generator for output
//! - Messages flow independently in both directions
//!
//! # Error Handling
//!
//! All errors in JavaScript handlers should be thrown as regular errors.
//! They will be caught and converted to gRPC status codes:
//!
//! ```javascript
//! async function handler(request) {
//!   if (!request.payload.length) {
//!     throw new Error("Empty payload");
//!   }
//!   // ... rest of handler
//! }
//! ```

pub mod handler;

pub use handler::{GrpcRequest, GrpcResponse, GrpcMessageStream, GrpcStatus, MessageStream};
