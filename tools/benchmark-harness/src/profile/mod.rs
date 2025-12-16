//! Profile mode - Deep analysis of Spikard implementations
//!
//! This module implements profile mode for benchmarking a single Spikard
//! implementation with deep profiling integration (py-spy, perf, etc.).

pub mod node;
pub mod php;
pub mod python;
pub mod ruby;
pub mod runner;
pub mod rust;
pub mod wasm;

pub use runner::{ProfileRunner, ProfileRunnerConfig};
