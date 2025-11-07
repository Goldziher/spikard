//! Fixture loading from testing_data

use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A test fixture from testing_data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fixture {
    pub name: String,
    pub description: String,

    #[serde(default)]
    pub category: Option<String>,

    pub handler: Handler,
    pub request: Request,
    pub expected_response: ExpectedResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Handler {
    pub route: String,
    pub method: String,

    #[serde(default)]
    pub parameters: Parameters,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Parameters {
    #[serde(default)]
    pub path: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub query: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub header: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub cookie: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub path: String,

    #[serde(default)]
    pub query_params: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub headers: HashMap<String, String>,

    #[serde(default)]
    pub cookies: HashMap<String, String>,

    #[serde(default)]
    pub body: Option<serde_json::Value>,

    #[serde(default)]
    pub body_raw: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpectedResponse {
    pub status_code: u16,

    #[serde(default)]
    pub body: Option<serde_json::Value>,

    #[serde(default)]
    pub headers: HashMap<String, String>,
}

impl Fixture {
    /// Load a fixture from a JSON file
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref();
        let contents = std::fs::read_to_string(path)?;
        let fixture: Fixture = serde_json::from_str(&contents).map_err(|e| Error::InvalidFixture {
            path: path.to_path_buf(),
            reason: format!("Failed to parse JSON: {}", e),
        })?;
        Ok(fixture)
    }

    /// Load all fixtures from a directory
    pub fn from_dir(dir: impl AsRef<Path>) -> Result<Vec<Self>> {
        let dir = dir.as_ref();
        let mut fixtures = Vec::new();

        for entry in std::fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // Skip schema files
                if path.file_name().and_then(|s| s.to_str()) == Some("schema.json") {
                    continue;
                }

                match Self::from_file(&path) {
                    Ok(fixture) => fixtures.push(fixture),
                    Err(e) => {
                        eprintln!("Warning: Failed to load fixture {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(fixtures)
    }

    /// Load fixtures matching a glob pattern
    pub fn from_glob(pattern: &str, base_dir: impl AsRef<Path>) -> Result<Vec<Self>> {
        let base_dir = base_dir.as_ref();
        let full_pattern = base_dir.join(pattern);
        let pattern_str = full_pattern.to_str().ok_or_else(|| Error::InvalidFixture {
            path: full_pattern.clone(),
            reason: "Invalid UTF-8 in path".to_string(),
        })?;

        let mut fixtures = Vec::new();

        for entry in glob::glob(pattern_str).map_err(|e| Error::InvalidFixture {
            path: full_pattern.clone(),
            reason: format!("Invalid glob pattern: {}", e),
        })? {
            let path = entry.map_err(|e| Error::InvalidFixture {
                path: PathBuf::new(),
                reason: format!("Glob error: {}", e),
            })?;

            if path.file_name().and_then(|s| s.to_str()) == Some("schema.json") {
                continue;
            }

            match Self::from_file(&path) {
                Ok(fixture) => fixtures.push(fixture),
                Err(e) => {
                    eprintln!("Warning: Failed to load fixture {}: {}", path.display(), e);
                }
            }
        }

        Ok(fixtures)
    }

    /// Get the fixture category from the file path or category field
    pub fn category(&self) -> String {
        self.category.clone().unwrap_or_else(|| "unknown".to_string())
    }
}

/// Manager for loading and organizing fixtures
pub struct FixtureManager {
    pub fixtures: Vec<Fixture>,
}

impl FixtureManager {
    pub fn new() -> Self {
        Self { fixtures: Vec::new() }
    }

    /// Load fixtures from testing_data directory
    pub fn load_from_testing_data(&mut self, testing_data_dir: impl AsRef<Path>) -> Result<()> {
        let dir = testing_data_dir.as_ref();

        // Load from each category directory
        let categories = [
            "query_params",
            "path_params",
            "json_bodies",
            "headers",
            "cookies",
            "multipart",
            "url_encoded",
            "validation_errors",
            "status_codes",
            "content_types",
            "http_methods",
            "cors",
            "edge_cases",
        ];

        for category in categories {
            let category_dir = dir.join(category);
            if category_dir.exists() {
                let mut category_fixtures = Fixture::from_dir(&category_dir)?;
                // Set category if not already set
                for fixture in &mut category_fixtures {
                    if fixture.category.is_none() {
                        fixture.category = Some(category.to_string());
                    }
                }
                self.fixtures.extend(category_fixtures);
            }
        }

        Ok(())
    }

    /// Filter fixtures by category
    pub fn by_category(&self, category: &str) -> Vec<&Fixture> {
        self.fixtures.iter().filter(|f| f.category() == category).collect()
    }

    /// Get all fixtures
    pub fn all(&self) -> &[Fixture] {
        &self.fixtures
    }

    /// Get fixture count
    pub fn len(&self) -> usize {
        self.fixtures.len()
    }

    pub fn is_empty(&self) -> bool {
        self.fixtures.is_empty()
    }
}

impl Default for FixtureManager {
    fn default() -> Self {
        Self::new()
    }
}
