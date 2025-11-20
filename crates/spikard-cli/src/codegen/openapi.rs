//! OpenAPI schema parsing

use anyhow::{Context, Result};
use openapiv3::OpenAPI;
use std::path::Path;

/// Parse an OpenAPI schema from a file (JSON or YAML)
pub fn parse_openapi_schema(path: &Path) -> Result<OpenAPI> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read OpenAPI schema from {}", path.display()))?;

    let spec: OpenAPI = serde_json::from_str(&content)
        .or_else(|_| serde_yaml::from_str(&content).context("Failed to parse as JSON or YAML"))
        .with_context(|| format!("Failed to parse OpenAPI schema from {}", path.display()))?;

    Ok(spec)
}
