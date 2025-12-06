//! Common test utilities for spikard-http tests
//!
//! This module provides reusable test fixtures and mock handlers that implement
//! the Handler trait for testing various scenarios without requiring language bindings.
//!
//! # Examples
//!
//! ```ignore
//! use common::handlers::SuccessHandler;
//! use spikard_http::Handler;
//!
//! let handler = SuccessHandler;
//! // Use in tests...
//! ```

#![allow(dead_code)]

pub mod handlers;

#[allow(unused_imports)]
pub use handlers::{EchoHandler, ErrorHandler, JsonHandler, PanicHandler, SuccessHandler};
