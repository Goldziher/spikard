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

    /// Contribute the ergonomic public-API re-exports to language-specific initialization files.
    ///
    /// For Python: Appends lines to `packages/python/spikard/__init__.py` with exact-line
    /// de-duplication. The appended content does not feed the generation-inputs hash,
    /// so `alef verify` remains fast. `from .app import App` is last to shadow the
    /// low-level `from .service import App` and make `from spikard import App` ergonomic.
    ///
    /// For Node (TypeScript): Appends re-export lines to the generated index file so
    /// `import { App } from '@spikard/node'` resolves to the ergonomic App (not the
    /// low-level service App).
    ///
    /// # Errors
    ///
    /// Never fails; always returns `Ok(...)`.
    fn public_api_additions(
        &self,
        _api: &ApiSurface,
        _cfg: &ExtensionConfig,
        language: Language,
    ) -> Result<Vec<String>> {
        match language {
            Language::Python => Ok(PYTHON_INIT_ADDITIONS.iter().map(|line| (*line).to_owned()).collect()),
            Language::Node => Ok(NODE_INIT_ADDITIONS.iter().map(|line| (*line).to_owned()).collect()),
            Language::Ruby => Ok(RUBY_INIT_ADDITIONS.iter().map(|line| (*line).to_owned()).collect()),
            _ => Ok(Vec::new()),
        }
    }
}

/// Raw lines appended to `packages/python/spikard/__init__.py` to expose the ergonomic surface.
///
/// Ordering matters: `from .app import App` is last among the `App` bindings so it shadows the
/// low-level `from .service import App` the backend already emitted. The final line extends the
/// backend-generated `__all__` with the ergonomic names (`App` is already listed by the backend).
const PYTHON_INIT_ADDITIONS: &[&str] = &[
    // `# noqa: F811`: the re-import deliberately shadows the low-level
    // `from .service import App` the backend already emitted, so the public
    // `App` is the ergonomic one. The shadow is intentional, not a mistake.
    "from .app import App  # noqa: F811",
    "from .params import Body, Cookie, Header, Path, Query",
    "from ._internal.converters import register_decoder",
    // `+=` (not `[*__all__, ...]`): a starred unpack inside an `__all__` list
    // literal trips ruff PLE0604 ("must contain only strings"); in-place extend
    // with string literals keeps the export list valid.
    "__all__ += [\"Body\", \"Cookie\", \"Header\", \"Path\", \"Query\", \"register_decoder\"]",
];

/// Raw lines appended to `packages/node/@spikard/node/index.ts` to expose the ergonomic surface.
///
/// Ordering matters: `export * from './app'` comes after any low-level App re-exports from the
/// backend so it shadows them, making `import { App } from '@spikard/node'` resolve to the
/// ergonomic App (which provides typed route registration with zod schema validation).
const NODE_INIT_ADDITIONS: &[&str] = &["export * from './app';"];

/// Raw lines appended to `packages/ruby/lib/spikard.rb` to expose the ergonomic surface.
///
/// Ordering matters: `require_relative 'spikard/app'` comes after the backend's low-level
/// imports so the ergonomic App class shadows the low-level one, making `require 'spikard'`
/// and `Spikard::App.new` resolve to the ergonomic typed-handler App.
const RUBY_INIT_ADDITIONS: &[&str] = &[
    "require_relative 'spikard/app'",
    "require_relative 'spikard/params'",
    "require_relative 'spikard/introspection'",
];

#[cfg(test)]
mod tests {
    use super::{HttpExtension, NODE_INIT_ADDITIONS, PYTHON_INIT_ADDITIONS, RUBY_INIT_ADDITIONS};
    use alef::Extension;
    use alef::core::config::Language;
    use alef::core::ir::ApiSurface;

    #[test]
    fn python_additions_expose_ergonomic_surface() {
        let ext = HttpExtension::new();
        let api = ApiSurface::default();
        let cfg = ext.parse_config(None).unwrap();
        let lines = ext.public_api_additions(&api, &cfg, Language::Python).unwrap();

        assert_eq!(
            lines, PYTHON_INIT_ADDITIONS,
            "python additions must match the canonical set"
        );
        // `from .app import App` (ergonomic) is present and comes after any `.service` import in
        // the additions (there is none), so on append it shadows the backend's low-level import.
        assert!(lines.iter().any(|l| l == "from .app import App  # noqa: F811"));
        assert!(
            lines
                .iter()
                .any(|l| l == "from .params import Body, Cookie, Header, Path, Query")
        );
        assert!(
            lines
                .iter()
                .any(|l| l == "from ._internal.converters import register_decoder")
        );
        assert!(lines.iter().any(|l| l.starts_with("__all__ += [")));
    }

    #[test]
    fn node_additions_expose_ergonomic_surface() {
        let ext = HttpExtension::new();
        let api = ApiSurface::default();
        let cfg = ext.parse_config(None).unwrap();
        let lines = ext.public_api_additions(&api, &cfg, Language::Node).unwrap();

        assert_eq!(
            lines, NODE_INIT_ADDITIONS,
            "node additions must match the canonical set"
        );
        // `export * from './app'` shadows the backend's low-level App re-exports
        assert!(lines.iter().any(|l| l == "export * from './app';"));
    }

    #[test]
    fn ruby_additions_expose_ergonomic_surface() {
        let ext = HttpExtension::new();
        let api = ApiSurface::default();
        let cfg = ext.parse_config(None).unwrap();
        let lines = ext.public_api_additions(&api, &cfg, Language::Ruby).unwrap();

        assert_eq!(
            lines, RUBY_INIT_ADDITIONS,
            "ruby additions must match the canonical set"
        );
        // Ergonomic app.rb is required after backend's low-level service.rb so it shadows
        assert!(lines.iter().any(|l| l == "require_relative 'spikard/app'"));
        assert!(lines.iter().any(|l| l == "require_relative 'spikard/params'"));
        assert!(lines.iter().any(|l| l == "require_relative 'spikard/introspection'"));
    }

    #[test]
    fn other_languages_additions_are_empty() {
        let ext = HttpExtension::new();
        let api = ApiSurface::default();
        let cfg = ext.parse_config(None).unwrap();
        for lang in [Language::Go, Language::Rust] {
            assert!(
                ext.public_api_additions(&api, &cfg, lang).unwrap().is_empty(),
                "{lang:?} must contribute no init additions",
            );
        }
    }
}
