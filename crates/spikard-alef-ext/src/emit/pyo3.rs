//! Python (`PyO3`) emission for HTTP extension — stub (Phase D deferred).

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;

/// Emit Python HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(vec![])`.
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if !cfg.lifecycle_hooks.is_empty() {
        tracing::debug!(
            "lifecycle hook emission not implemented for pyo3 ({} hooks)",
            cfg.lifecycle_hooks.len()
        );
    }
    if !cfg.error_types.is_empty() {
        tracing::debug!(
            "error type emission not implemented for pyo3 ({} types)",
            cfg.error_types.len()
        );
    }
    Ok(vec![])
}
