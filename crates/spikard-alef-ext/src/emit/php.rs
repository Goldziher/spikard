//! PHP (ext-php-rs) emission for HTTP extension — stub (Phase D deferred).

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;

/// Emit PHP HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(vec![])`.
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if !cfg.lifecycle_hooks.is_empty() {
        tracing::debug!(
            "lifecycle hook emission not implemented for php ({} hooks)",
            cfg.lifecycle_hooks.len()
        );
    }
    if !cfg.websocket_routes.is_empty() {
        tracing::debug!(
            "WebSocket route emission not implemented for php ({} routes)",
            cfg.websocket_routes.len()
        );
    }
    if !cfg.sse_routes.is_empty() {
        tracing::debug!(
            "SSE route emission not implemented for php ({} routes)",
            cfg.sse_routes.len()
        );
    }
    if !cfg.error_types.is_empty() {
        tracing::debug!(
            "error type emission not implemented for php ({} types)",
            cfg.error_types.len()
        );
    }
    Ok(vec![])
}
