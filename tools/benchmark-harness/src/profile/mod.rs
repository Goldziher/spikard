//! Profile mode - Deep analysis of Spikard implementations
//!
//! This module implements profile mode for benchmarking a single Spikard
//! implementation with deep profiling integration (py-spy, perf, etc.).

pub mod python;
pub mod runner;

pub use runner::{ProfileRunner, ProfileRunnerConfig};
