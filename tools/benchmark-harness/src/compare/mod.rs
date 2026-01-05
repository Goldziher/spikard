//! Multi-framework comparison orchestration and reporting
//!
//! Provides `CompareRunner` for executing benchmarks across multiple frameworks
//! and generating statistical comparison reports.
//!
//! This module is distinct from the older `compare.rs` which handles post-processing
//! of existing benchmark results. The Compare Mode runner here orchestrates the
//! execution of multiple frameworks and produces `CompareResult` outputs.

mod analyzer;
mod runner;

pub use analyzer::{CompareAnalyzer, ComparisonAnalysis};
pub use runner::{CompareConfig, CompareRunner};

pub use crate::schema::compare::{CompareResult, CompareSummary, EffectSize, StatisticalTest};
