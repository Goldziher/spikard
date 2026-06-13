//! spikard-alef-ext — Alef HTTP extension for the spikard polyglot framework.
//!
//! This crate implements [`alef::Extension`] for spikard's HTTP-domain IR:
//! lifecycle hooks, WebSocket/SSE routes, and cross-binding error types.
//! It reads the `[extensions.spikard-http]` section from `alef.toml` at
//! generation time and emits per-language files for the configured IR sections.
//!
//! # Usage
//!
//! Wire `HttpExtension` into the alef pipeline via the `spikard-alef` bin crate:
//!
//! ```rust,no_run
//! fn main() -> std::process::ExitCode {
//!     alef::run_with_extensions(vec![
//!         Box::new(spikard_alef_ext::HttpExtension::new()),
//!     ])
//! }
//! ```

pub mod config;
pub mod emit;
pub mod ir;

use alef::core::backend::GeneratedFile;
use alef::core::config::Language;
use alef::core::ir::ApiSurface;
use alef::core::template_env::TemplateEnv;
use alef::{Extension, ExtensionConfig};
use anyhow::Result;
use config::HttpExtensionConfig;

/// Alef extension that adds spikard's HTTP-domain IR to the generation pipeline.
///
/// Reads `[extensions.spikard-http]` from `alef.toml` (in the working directory)
/// and emits per-language lifecycle hooks, error classes, and WebSocket/SSE methods.
pub struct HttpExtension {
    /// Cached config loaded on first access.
    config: std::sync::OnceLock<HttpExtensionConfig>,
}

impl HttpExtension {
    /// Construct a new `HttpExtension`.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            config: std::sync::OnceLock::new(),
        }
    }

    fn loaded_config(&self) -> &HttpExtensionConfig {
        self.config.get_or_init(|| {
            HttpExtensionConfig::load_from_alef_toml()
                .inspect_err(|e| tracing::warn!("spikard-http extension: config load error: {e}"))
                .unwrap_or_default()
        })
    }
}

impl Default for HttpExtension {
    fn default() -> Self {
        Self::new()
    }
}

impl Extension for HttpExtension {
    fn name(&self) -> &'static str {
        "spikard-http"
    }

    /// Parse is a no-op here — config is loaded lazily from disk in [`augment_surface`]
    /// and [`emit_for_language`]. The alef pipeline currently passes `None` for
    /// per-extension TOML sections; we read `alef.toml` directly instead.
    ///
    /// # Errors
    ///
    /// Never fails; always returns `Ok(ExtensionConfig::empty())`.
    fn parse_config(&self, _raw: Option<&toml::Value>) -> Result<ExtensionConfig> {
        Ok(ExtensionConfig::empty())
    }

    /// Augment the API surface by injecting HTTP-domain IR loaded from `alef.toml`.
    ///
    /// Currently a no-op: the HTTP IR is self-contained in the extension config and
    /// does not need to be reflected back into `ApiSurface` fields (those fields were
    /// removed from alef core). Extension files are emitted directly from
    /// [`emit_for_language`] without an intermediate surface mutation step.
    ///
    /// # Errors
    ///
    /// Never fails; always returns `Ok(())`.
    fn augment_surface(&self, _api: &mut ApiSurface, _cfg: &ExtensionConfig) -> Result<()> {
        // Pre-load config so it's available when emit_for_language is called.
        let _ = self.loaded_config();
        Ok(())
    }

    /// Emit extra files for the given language using the HTTP IR from `alef.toml`.
    ///
    /// # Errors
    ///
    /// Returns an error if the per-language emitter fails to render templates.
    fn emit_for_language(
        &self,
        api: &ApiSurface,
        _cfg: &ExtensionConfig,
        language: Language,
        _env: &TemplateEnv,
    ) -> Result<Vec<GeneratedFile>> {
        let cfg = self.loaded_config();
        emit::emit_for_language(api, cfg, language)
    }
}
