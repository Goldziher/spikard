//! Kotlin (JVM) emission for HTTP extension — stub (Phase D deferred).

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;

/// Emit Kotlin HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(vec![])`.
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if !cfg.lifecycle_hooks.is_empty() {
        tracing::debug!(
            "lifecycle hook emission not implemented for kotlin ({} hooks)",
            cfg.lifecycle_hooks.len()
        );
    }
    Ok(vec![])
}
