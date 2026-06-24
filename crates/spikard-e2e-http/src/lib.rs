//! spikard-e2e-http — alef extension that owns spikard's HTTP e2e test generation.
//!
//! This crate holds the HTTP-specific e2e code generation (the shared HTTP test
//! client driver, per-language HTTP test renderers, app harnesses, the Rust
//! mock-server source emission, and the HTTP fixture interpretation) that
//! previously lived in alef core. It plugs into alef via the generic
//! [`alef::Extension::emit_e2e`] hook, so alef itself stays library-agnostic.
//!
//! Registered by `spikard-alef` alongside `spikard-alef-ext::HttpExtension`.

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
        _groups: &[FixtureGroup],
        _e2e_config: &E2eConfig,
        _config: &ResolvedCrateConfig,
        _language: &str,
        _type_defs: &[TypeDef],
        _enums: &[EnumDef],
    ) -> Result<Vec<GeneratedFile>> {
        // Skeleton: the per-language HTTP e2e generation is migrated here from
        // alef core in subsequent steps. Until then this returns empty, so the
        // built-in alef e2e generators still produce the HTTP suites unchanged.
        Ok(vec![])
    }
}
