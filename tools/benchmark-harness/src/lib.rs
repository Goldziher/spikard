//! Spikard Benchmark Harness
//!
//! A tool for benchmarking Spikard HTTP framework across different language bindings
//! and comparing against external frameworks.
#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::too_many_lines,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::unused_self,
    clippy::string_add,
    clippy::string_add_assign,
    clippy::cast_lossless,
    clippy::cast_possible_wrap,
    clippy::unnecessary_wraps,
    clippy::option_if_let_else,
    clippy::similar_names,
    clippy::format_push_string,
    clippy::type_complexity,
    clippy::single_char_pattern,
    clippy::implicit_hasher,
    clippy::match_same_arms,
    clippy::literal_string_with_formatting_args,
    clippy::or_fun_call,
    clippy::branches_sharing_code
)]

pub mod aggregate;
pub mod analysis;
pub mod compare;
pub mod comparison;
pub mod consolidate;
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
pub mod visualize;
pub mod workload;

pub use analysis::{AggregatedResult, MetricStats, aggregate_runs};
pub use compare::{CompareConfig, CompareResult, CompareRunner, CompareSummary};
pub use comparison::{
    ComparisonReport, FrameworkComparison, RegressionWarning, RelativeMetrics, compare_frameworks, detect_regressions,
    generate_json_report, generate_markdown_report,
};
pub use consolidate::{ConsolidatedProfileReport, consolidate_profile_dir, consolidate_profile_paths};
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
