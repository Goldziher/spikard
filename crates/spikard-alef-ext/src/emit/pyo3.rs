//! Python (`PyO3`) emission for the HTTP extension.
//!
//! Emits the ergonomic, typed-handler application layer for the Python binding:
//! the public `App` class (verb decorators + DTO binding), the parameter markers
//! (`Body`/`Query`/`Cookie`/`Header`/`Path`), and the pure-Python introspection and
//! type-conversion runtime that bridges the Rust `PyHandlerBridge` contract.
//!
//! The assets are vendored under `templates/pyo3/` with a `.py.jinja` suffix (so the
//! repo's `**/*.jinja` lint/format exclusion covers them without touching `poly.toml`)
//! and emitted verbatim as `.py` files into `packages/python/spikard/`. The ergonomic
//! layer introspects handlers at runtime, so no per-route wiring is required from the
//! `ApiSurface`.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use std::path::PathBuf;

/// Package-relative destination directory for every emitted file.
const PACKAGE_DIR: &str = "packages/python/spikard";

/// A single static Python asset: (package-relative `.py` output path, file contents).
///
/// Sources carry a `.py.jinja` suffix on disk (excluded from lint/format); the emitted
/// output path drops the `.jinja` so the file lands as ordinary `.py` in the package.
const STATIC_FILES: &[(&str, &str)] = &[
    ("app.py", include_str!("../templates/pyo3/app.py.jinja")),
    ("params.py", include_str!("../templates/pyo3/params.py.jinja")),
    (
        "datastructures.py",
        include_str!("../templates/pyo3/datastructures.py.jinja"),
    ),
    ("schema.py", include_str!("../templates/pyo3/schema.py.jinja")),
    (
        "introspection.py",
        include_str!("../templates/pyo3/introspection.py.jinja"),
    ),
    (
        "_internal/__init__.py",
        include_str!("../templates/pyo3/_internal/__init__.py.jinja"),
    ),
    (
        "_internal/converters.py",
        include_str!("../templates/pyo3/_internal/converters.py.jinja"),
    ),
    (
        "_internal/serialization.py",
        include_str!("../templates/pyo3/_internal/serialization.py.jinja"),
    ),
    (
        "_internal/json_schema.py",
        include_str!("../templates/pyo3/_internal/json_schema.py.jinja"),
    ),
    (
        "_internal/field_definition.py",
        include_str!("../templates/pyo3/_internal/field_definition.py.jinja"),
    ),
    (
        "_internal/parsed_signature.py",
        include_str!("../templates/pyo3/_internal/parsed_signature.py.jinja"),
    ),
    (
        "_internal/constraints.py",
        include_str!("../templates/pyo3/_internal/constraints.py.jinja"),
    ),
    (
        "_internal/types.py",
        include_str!("../templates/pyo3/_internal/types.py.jinja"),
    ),
    (
        "_internal/utils.py",
        include_str!("../templates/pyo3/_internal/utils.py.jinja"),
    ),
];

/// Emit the Python HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(...)`.
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

    let files = STATIC_FILES
        .iter()
        .map(|(rel, content)| GeneratedFile {
            path: PathBuf::from(PACKAGE_DIR).join(rel),
            content: (*content).to_owned(),
            generated_header: true,
        })
        .collect();

    Ok(files)
}
