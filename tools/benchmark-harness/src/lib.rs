//! Spikard Benchmark Harness
//!
//! A tool for benchmarking Spikard HTTP framework across different language bindings
//! and comparing against external frameworks.

pub mod analysis;
pub mod compare;
pub mod comparison;
pub mod error;
pub mod fixture;
pub mod framework;
pub mod generators;
pub mod load_generator;
pub mod monitor;
pub mod profile;
pub mod runner;
pub mod schema;
pub mod server;
pub mod streaming;
pub mod types;
pub mod workload;

pub use analysis::{AggregatedResult, MetricStats, aggregate_runs};
pub use compare::{CompareConfig, CompareResult, CompareRunner, CompareSummary};
pub use comparison::{
    ComparisonReport, FrameworkComparison, RegressionWarning, RelativeMetrics, compare_frameworks, detect_regressions,
    generate_json_report, generate_markdown_report,
};
pub use error::{Error, Result};
pub use fixture::{Fixture, FixtureManager};
pub use framework::{FrameworkConfig, detect_framework, get_framework, list_frameworks};
pub use load_generator::{LoadGeneratorType, LoadTestConfig};
pub use monitor::{ResourceMonitor, ResourceSample};
pub use runner::{BenchmarkRunner, RunnerConfig};
pub use server::{ServerConfig, ServerHandle};
pub use streaming::{StreamingBenchmarkRunner, StreamingFixture, StreamingRunnerConfig};
pub use types::{
    BenchmarkResult, LatencyMetrics, ResourceMetrics, StreamingBenchmarkResult, StreamingLatencyMetrics,
    StreamingMetrics, StreamingProtocol, ThroughputMetrics,
};
pub use workload::{
    JsonBodyWorkload, MultipartWorkload, ParamType, PathComplexity, PathParamWorkload, PayloadSize, QueryParamWorkload,
    UrlEncodedWorkload, Workload, WorkloadCategory, WorkloadPresets,
};
