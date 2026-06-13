//! TOML config deserialization for the HTTP extension.
//!
//! Config is stored in `spikard-http.toml` in the workspace root (the current
//! working directory when `spikard-alef` runs). This is separate from `alef.toml`
//! because alef's top-level TOML schema uses `deny_unknown_fields` and cannot
//! accommodate consumer-specific extension sections until alef's config schema
//! is relaxed.

use crate::ir::{ErrorTypeDef, LifecycleHookDef, SseRouteDef, WebSocketRouteDef};
use anyhow::{Context as _, Result};
use serde::Deserialize;

/// Parsed content of `spikard-http.toml`.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct HttpExtensionConfig {
    #[serde(default, rename = "lifecycle_hooks")]
    pub lifecycle_hooks: Vec<LifecycleHookDef>,
    #[serde(default, rename = "websocket_routes")]
    pub websocket_routes: Vec<WebSocketRouteDef>,
    #[serde(default, rename = "sse_routes")]
    pub sse_routes: Vec<SseRouteDef>,
    #[serde(default, rename = "error_types")]
    pub error_types: Vec<ErrorTypeDef>,
}

impl HttpExtensionConfig {
    /// Parse from a raw `toml::Value`.
    ///
    /// # Errors
    ///
    /// Returns an error if the TOML value cannot be deserialized into
    /// [`HttpExtensionConfig`].
    pub fn from_toml(raw: &toml::Value) -> Result<Self> {
        raw.clone()
            .try_into()
            .context("failed to parse spikard-http.toml config")
    }

    /// Load from `spikard-http.toml` in the current working directory.
    ///
    /// Returns `Ok(Default::default())` if the file does not exist.
    ///
    /// # Errors
    ///
    /// Returns an error if the file exists but cannot be read or parsed.
    pub fn load_from_alef_toml() -> Result<Self> {
        let path = std::path::Path::new("spikard-http.toml");
        if !path.exists() {
            return Ok(Self::default());
        }
        let raw = std::fs::read_to_string(path).context("failed to read spikard-http.toml")?;
        let doc: toml::Value = toml::from_str(&raw).context("failed to parse spikard-http.toml")?;
        Self::from_toml(&doc)
    }
}
