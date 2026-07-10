//! Elixir (rustler) emission for HTTP extension: lifecycle hooks, error types.

use crate::config::HttpExtensionConfig;
use crate::ir::ErrorTypeDef;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use std::fmt::Write as _;
use std::path::PathBuf;

fn prefixed_module(module_prefix: &str, module: &str) -> String {
    if module_prefix.is_empty() {
        module.to_owned()
    } else {
        format!("{module_prefix}.{module}")
    }
}

fn emit_error_types(types: &[ErrorTypeDef], module_prefix: &str) -> String {
    if types.is_empty() {
        return String::new();
    }

    let error_module = prefixed_module(module_prefix, "Errors");
    let mut out = format!("defmodule {error_module} do\n");
    out.push_str("  @moduledoc \"\"\"\n");
    out.push_str("  Generated exception types.\n");
    out.push_str("  \"\"\"\n\n");

    for error_type in types {
        let exception_name = &error_type.name;
        let status_code = error_type.http_status.as_u16();
        let doc = if error_type.doc.is_empty() {
            format!("Exception for {}.", error_type.name.to_lowercase())
        } else {
            error_type.doc.clone()
        };

        let escaped_doc = doc.replace('"', "\\\"");
        let _ = writeln!(out, "  defmodule {exception_name} do");
        let _ = writeln!(out, "    @moduledoc \"{escaped_doc}\"");
        out.push_str("    defexception [:message, :status_code, :problem_details]\n\n");
        let _ = writeln!(
            out,
            "    def new(message, status_code \\\\ {status_code}, problem_details \\\\ nil) do"
        );
        out.push_str("      %__MODULE__{\n");
        out.push_str("        message: message,\n");
        out.push_str("        status_code: status_code,\n");
        out.push_str("        problem_details: problem_details\n");
        out.push_str("      }\n");
        out.push_str("    end\n");
        out.push_str("  end\n\n");
    }

    out.push_str("end\n\n");
    out
}

fn module_prefix(api: &ApiSurface) -> String {
    let mut chars = api.crate_name.chars();
    chars
        .next()
        .map_or_else(String::new, |first| first.to_uppercase().to_string() + chars.as_str())
}

/// Emit Elixir HTTP extension files.
///
/// # Errors
///
/// Never fails; always returns `Ok(...)`.
pub fn emit(api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    let mut files = Vec::new();
    let prefix = module_prefix(api);

    let _ = &cfg.lifecycle_hooks;

    let errors_content = emit_error_types(&cfg.error_types, &prefix);
    if !errors_content.is_empty() {
        files.push(GeneratedFile {
            path: PathBuf::from("packages/elixir/lib/errors.ex"),
            content: errors_content,
            generated_header: true,
        });
    }

    Ok(files)
}
