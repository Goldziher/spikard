//! Spikard Benchmark Harness
//!
//! A tool for benchmarking Spikard HTTP framework across different language bindings
//! and comparing against external frameworks.

pub mod analysis;
pub mod compare;
pub mod error;
pub mod fixture;
pub mod load_generator;
pub mod monitor;
pub mod runner;
pub mod server;
pub mod types;

pub use analysis::{aggregate_runs, AggregatedResult, MetricStats};
pub use compare::{
    compare_frameworks, detect_regressions, generate_json_report, generate_markdown_report,
    ComparisonReport, FrameworkComparison, RegressionWarning, RelativeMetrics,
};
pub use error::{Error, Result};
pub use fixture::{Fixture, FixtureManager};
pub use load_generator::{LoadGeneratorType, LoadTestConfig};
pub use monitor::{ResourceMonitor, ResourceSample};
pub use runner::{BenchmarkRunner, RunnerConfig};
pub use server::{ServerConfig, ServerHandle};
pub use types::{BenchmarkResult, LatencyMetrics, ResourceMetrics, ThroughputMetrics};
