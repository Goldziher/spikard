//! PHP (ext-php-rs) emission for HTTP extension.
//!
//! Emits the ergonomic `App.php` typed-handler layer. The remaining HTTP
//! surface (lifecycle hooks, WebSocket/SSE routes, error types) is not yet
//! implemented for PHP and is logged at debug level.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use std::path::PathBuf;

/// Emit PHP HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(...)`.
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    let files = vec![GeneratedFile {
        path: PathBuf::from("packages/php/spikard/App.php"),
        content: include_str!("../templates/php/app.php.jinja").to_owned(),
        generated_header: true,
    }];

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
    Ok(files)
}
