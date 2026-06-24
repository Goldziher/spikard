//! spikard-e2e-http — alef extension that owns spikard's HTTP e2e test generation.
//!
//! This crate holds the HTTP-specific e2e code generation (the shared HTTP test
//! client driver, per-language HTTP test renderers, app harnesses, the Rust
//! mock-server source emission, and the HTTP fixture interpretation) that
//! previously lived in alef core. It plugs into alef via the generic
//! [`alef::Extension::emit_e2e`] hook, so alef itself stays library-agnostic.
//!
//! Registered by `spikard-alef` alongside `spikard-alef-ext::HttpExtension`.
//!
//! The per-language renderers are staged verbatim from alef's e2e codegen, which
//! follows a less strict lint baseline than spikard's workspace pedantic/nursery
//! deny. Rather than rewrite the ports (which must stay byte-faithful to alef),
//! pedantic/nursery are allowed crate-wide here.
#![allow(clippy::pedantic, clippy::nursery)]
// `if_same_then_else` fires on a faithful port of alef's ruby SUT-body renderer
// where two branches legitimately produce the same tuple; keep it port-faithful.
#![allow(clippy::if_same_then_else)]

use alef::{E2eConfig, EnumDef, Extension, FixtureGroup, GeneratedFile, ResolvedCrateConfig, TypeDef};
use anyhow::Result;

#[allow(dead_code)]
pub mod driver;
#[allow(dead_code)]
pub mod lang;
#[allow(dead_code)]
pub mod streaming;

/// Alef extension that emits spikard's HTTP e2e test suites.
#[derive(Debug, Default, Clone, Copy)]
pub struct E2eHttpExtension;

impl E2eHttpExtension {
    /// Create a new extension instance.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Extension for E2eHttpExtension {
    fn name(&self) -> &'static str {
        "spikard-e2e-http"
    }

    fn emit_e2e(
        &self,
        groups: &[FixtureGroup],
        e2e_config: &E2eConfig,
        config: &ResolvedCrateConfig,
        language: &str,
        _type_defs: &[TypeDef],
        _enums: &[EnumDef],
    ) -> Result<Vec<GeneratedFile>> {
        // The extension owns spikard's *server-pattern* e2e generation (the
        // `app_harness` that spins up the SUT as an HTTP server, plus the
        // harness-spawn test setup). The shared client-pattern driver, per-test
        // bodies, mock-server, and project scaffolding stay generic in alef.
        match language {
            "node" => emit_node(groups, e2e_config),
            "python" => emit_python(groups, e2e_config, config),
            "php" => lang::php::emit(groups, e2e_config, config),
            "ruby" => lang::ruby::emit(groups, e2e_config, config),
            "elixir" => lang::elixir::emit(groups, e2e_config, config),
            // Remaining languages are migrated incrementally; until then alef's
            // built-in server-pattern emission still produces them.
            _ => Ok(vec![]),
        }
    }
}

/// Emit Python's server-pattern files: `app_harness.py` (SUT-as-server) and
/// the server-pattern `conftest.py` that spawns it. Gated identically to
/// alef's prior emission: HTTP fixtures present and a harness import configured.
fn emit_python(
    groups: &[FixtureGroup],
    e2e_config: &E2eConfig,
    config: &ResolvedCrateConfig,
) -> Result<Vec<GeneratedFile>> {
    use std::path::PathBuf;

    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let base = PathBuf::from(e2e_config.effective_output()).join("python");
    Ok(vec![
        GeneratedFile {
            path: base.join("app_harness.py"),
            content: lang::python::render_app_harness(e2e_config, groups, config),
            generated_header: true,
        },
        GeneratedFile {
            path: base.join("conftest.py"),
            content: lang::python::render_conftest_server(e2e_config),
            generated_header: true,
        },
    ])
}

/// Emit node's server-pattern files: the `app_harness.mjs` (SUT-as-server) and
/// the server-pattern `globalSetup.ts` that spawns it. Gated identically to
/// alef's prior emission: HTTP fixtures present and a harness import configured.
fn emit_node(groups: &[FixtureGroup], e2e_config: &E2eConfig) -> Result<Vec<GeneratedFile>> {
    use std::path::PathBuf;

    let has_http = groups.iter().flat_map(|g| g.fixtures.iter()).any(|f| f.http.is_some());
    if !has_http || e2e_config.harness.imports.is_empty() {
        return Ok(vec![]);
    }

    let base = PathBuf::from(e2e_config.effective_output()).join("node");
    Ok(vec![
        GeneratedFile {
            path: base.join("app_harness.mjs"),
            content: lang::node::render_app_harness(e2e_config, groups),
            generated_header: true,
        },
        GeneratedFile {
            path: base.join("globalSetup.ts"),
            content: lang::node::render_global_setup_server(),
            generated_header: true,
        },
    ])
}
