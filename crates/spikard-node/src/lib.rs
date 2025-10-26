//! Node.js bindings for spikard
//!
//! This crate provides Node.js bindings using napi-rs

#![deny(clippy::all)]

use napi::Error;
use napi_derive::napi;

/// Process using spikard
#[napi]
pub fn process() -> napi::Result<()> {
    spikard::process().map_err(|e| Error::from_reason(format!("Spikard error: {}", e)))
}
