//! HTTP server runtime initialization and management.

pub mod server_runner;

pub use server_runner::{normalize_route_metadata, run_server};
