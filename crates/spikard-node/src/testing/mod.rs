//! Test client bindings for Node.js
//!
//! This module provides Node.js wrappers for Spikard's testing utilities,
//! including SSE streams and WebSocket connections for integration testing.

/// SSE stream testing client bindings
pub mod sse;
/// WebSocket connection testing client bindings
pub mod websocket;

pub use sse::*;
pub use websocket::*;
