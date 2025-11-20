//! AsyncAPI fixture utilities
//!
//! Provides helpers to load SSE/WebSocket fixtures that were generated
//! from AsyncAPI specifications and stored under `testing_data/`.

use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
#[allow(dead_code)]
pub struct AsyncFixture {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub channel: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub schema: Value,
    #[serde(default)]
    pub examples: Vec<Value>,
    #[serde(default)]
    pub operations: Vec<AsyncFixtureOperation>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AsyncFixtureOperation {
    #[allow(dead_code)]
    pub name: String,
    pub action: String,
    #[serde(default)]
    pub replies: Vec<String>,
}

pub fn load_sse_fixtures(fixtures_dir: &Path) -> Result<Vec<AsyncFixture>> {
    load_asyncapi_fixtures(fixtures_dir.join("sse"), "sse")
}

#[allow(dead_code)]
pub fn load_websocket_fixtures(fixtures_dir: &Path) -> Result<Vec<AsyncFixture>> {
    load_asyncapi_fixtures(fixtures_dir.join("websockets"), "websocket")
}

fn load_asyncapi_fixtures(dir: PathBuf, expected_protocol: &str) -> Result<Vec<AsyncFixture>> {
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

        let content = fs::read_to_string(&path).with_context(|| format!("Failed to read {}", path.display()))?;
        let mut fixture: AsyncFixture =
            serde_json::from_str(&content).with_context(|| format!("Failed to parse {}", path.display()))?;

        if let Some(protocol) = fixture.protocol.as_deref() {
            if protocol != expected_protocol {
                continue;
            }
        } else {
            fixture.protocol = Some(expected_protocol.to_string());
        }

        fixtures.push(fixture);
    }

    Ok(fixtures)
}
