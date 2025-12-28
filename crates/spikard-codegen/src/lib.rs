//! Code generation utilities for Spikard
//!
//! This crate provides utilities for generating test infrastructure and type definitions
//! from fixture files and OpenAPI schemas.

pub mod error;
pub mod openapi;

pub use error::{CodegenError, Result};
