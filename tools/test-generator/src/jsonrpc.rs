//! JSON-RPC fixture utilities
//!
//! Provides helpers to load JSON-RPC fixtures that were generated
//! from JSON-RPC specifications and stored under `testing_data/`.

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct JsonRpcFixture {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub method: String,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub transport: Option<String>,
    #[serde(default)]
    pub endpoint: Option<String>,
    pub params_schema: Value,
    pub result_schema: Value,
    #[serde(default)]
    pub examples: Vec<JsonRpcExample>,
    #[serde(default)]
    pub error_cases: Vec<JsonRpcErrorCase>,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcExample {
    pub params: Value,
    pub result: Value,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcErrorCase {
    pub name: String,
    #[serde(default)]
    pub params: Option<Value>,
    pub error: JsonRpcError,
}

#[derive(Debug, Clone, Deserialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(default)]
    pub data: Option<Value>,
}

/// Load JSON-RPC fixtures from testing_data/jsonrpc directory
pub fn load_jsonrpc_fixtures(fixtures_dir: &Path) -> Result<Vec<JsonRpcFixture>> {
    let dir = fixtures_dir.join("jsonrpc");

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let mut fixtures = Vec::new();

    for entry in fs::read_dir(&dir).with_context(|| format!("Failed to read {}", dir.display()))? {
        let entry = entry.context("Failed to read fixture entry")?;
        let path = entry.path();
        if path.extension().is_none_or(|ext| ext != "json") {
            continue;
        }

        // Skip schema.json files (they define the fixture format, not actual fixtures)
        if path.file_name().is_some_and(|name| name == "schema.json") {
            continue;
        }

        let content = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
        let mut fixture: JsonRpcFixture =
            serde_json::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

        // Set defaults as per JSON-RPC 2.0 spec
        if fixture.protocol.is_none() {
            fixture.protocol = Some("jsonrpc".to_string());
        }

        if fixture.transport.is_none() {
            fixture.transport = Some("http".to_string());
        }

        if fixture.endpoint.is_none() {
            fixture.endpoint = Some("/rpc".to_string());
        }

        fixtures.push(fixture);
    }

    // Sort by name for deterministic output
    fixtures.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(fixtures)
}
