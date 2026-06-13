//! Per-language emission dispatch for the HTTP extension.

pub mod csharp;
pub mod dart;
pub mod go;
pub mod jni;
pub mod kotlin;
pub mod magnus;
pub mod napi;
pub mod php;
pub mod pyo3;
pub mod rustler;
pub mod swift;
pub mod zig;

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::config::Language;
use alef::core::ir::ApiSurface;
use anyhow::Result;

/// Dispatch to the correct language emitter.
///
/// # Errors
///
/// Returns an error if the selected language emitter fails to render templates.
pub fn emit_for_language(
    api: &ApiSurface,
    cfg: &HttpExtensionConfig,
    language: Language,
) -> Result<Vec<GeneratedFile>> {
    match language {
        Language::Python => pyo3::emit(api, cfg),
        Language::Node => napi::emit(api, cfg),
        Language::Ruby => magnus::emit(api, cfg),
        Language::Php => php::emit(api, cfg),
        Language::Elixir => rustler::emit(api, cfg),
        Language::Go => go::emit(api, cfg),
        Language::Jni | Language::Java => jni::emit(api, cfg),
        Language::Csharp => csharp::emit(api, cfg),
        Language::Kotlin | Language::KotlinAndroid => kotlin::emit(api, cfg),
        Language::Dart => dart::emit(api, cfg),
        Language::Swift => swift::emit(api, cfg),
        Language::Zig => zig::emit(api, cfg),
        // Non-emitting targets — no HTTP extension output needed.
        Language::Ffi | Language::Wasm | Language::R | Language::Rust | Language::Gleam | Language::C => Ok(vec![]),
    }
}
