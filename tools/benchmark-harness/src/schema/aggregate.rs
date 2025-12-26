use serde::{Deserialize, Serialize};

use super::profile::ProfileResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatedBenchmarkResults {
    pub metadata: AggregationMetadata,
    pub frameworks: Vec<FrameworkResult>,
    pub summary: AggregationSummary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationMetadata {
    pub run_id: String,
    pub run_url: String,
    pub workflow: String,
    pub commit: Option<String>,
    pub branch: Option<String>,
    pub aggregated_at: String,
    pub artifact_count: usize,
    pub artifacts: Vec<ArtifactInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtifactInfo {
    pub name: String,
    pub framework: String,
    pub size_bytes: u64,
    pub downloaded: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkResult {
    pub framework: String,
    pub profile: ProfileResult,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregationSummary {
    pub total_frameworks: usize,
    pub completed: usize,
    pub failed: usize,
    pub total_requests: u64,
    pub total_duration_secs: f64,
}
