#![allow(deprecated)]

//! Spikard Ruby bindings using Magnus FFI.
//!
//! This crate provides Ruby bindings for the Spikard HTTP toolkit, allowing
//! Ruby developers to build and test HTTP services with Rust performance.
//!
//! ## Modules
//!
//! - `test_client`: TestClient wrapper for integration testing
//! - `handler`: RubyHandler trait implementation
//! - `di`: Dependency injection bridge for Ruby types
//! - `config`: ServerConfig extraction from Ruby objects
//! - `conversion`: Ruby ↔ Rust type conversions
//! - `server`: HTTP server setup and lifecycle management
//! - `background`: Background task management
//! - `lifecycle`: Lifecycle hook implementations
//! - `sse`: Server-Sent Events support
//! - `test_sse`: SSE testing utilities
//! - `websocket`: WebSocket support
//! - `test_websocket`: WebSocket testing utilities

mod background;
mod config;
mod conversion;
mod di;
mod handler;
mod lifecycle;
mod server;
mod sse;
mod test_client;
mod test_sse;
mod test_websocket;
mod websocket;

use magnus::prelude::*;
use magnus::{Error, Module, Ruby, function, method};

// Re-export for internal use and public API
pub use handler::RubyHandler;
pub use server::run_server;
pub use test_client::NativeTestClient;

/// Return the Spikard version.
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[magnus::init]
pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let spikard = ruby.define_module("Spikard")?;
    spikard.define_singleton_method("version", function!(version, 0))?;
    let native = match spikard.const_get("Native") {
        Ok(module) => module,
        Err(_) => spikard.define_module("Native")?,
    };

    native.define_singleton_method("run_server", function!(run_server, 6))?;
    native.define_singleton_method("background_run", function!(background::background_run, 1))?;

    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 5))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("websocket", method!(NativeTestClient::websocket, 1))?;
    class.define_method("sse", method!(NativeTestClient::sse, 1))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    let spikard_module = ruby.define_module("Spikard")?;
    test_websocket::init(ruby, &spikard_module)?;
    test_sse::init(ruby, &spikard_module)?;

    Ok(())
}
