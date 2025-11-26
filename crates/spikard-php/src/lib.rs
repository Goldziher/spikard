//! PHP bindings scaffold for Spikard.
//!
//! This crate will host the ext-php-rs integration that mirrors the existing
//! Python/Node/Ruby bindings. For now it exposes only metadata helpers to keep
//! the workspace aligned while the FFI surface is designed.

#![deny(clippy::unwrap_used)]

/// Return the crate version.
pub const fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}
