//! TypeScript (napi-rs) emission for HTTP extension: ergonomic App layer and lifecycle hooks.
//!
//! Emits the ergonomic, typed-handler application layer for the Node.js/TypeScript binding:
//! the public `App` class (verb decorators + zod schema validation + typed handlers),
//! and optional lifecycle hook method snippets appended to the service wrapper.
//!
//! The ergonomic App assets are vendored under `templates/napi/` with a `.ts.jinja` suffix
//! and emitted verbatim as `.ts` files into `packages/node/src/`. The ergonomic layer
//! wraps the low-level service App and provides type-safe request/response handling.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use heck::ToLowerCamelCase;
use minijinja::{Environment, context};
use std::path::PathBuf;

/// Package-relative destination directory for every emitted file.
const PACKAGE_DIR: &str = "packages/node/@spikard/node";

/// A single static TypeScript asset: (package-relative `.ts` output path, file contents).
///
/// Sources carry a `.ts.jinja` suffix on disk (excluded from lint/format); the emitted
/// output path drops the `.jinja` so the file lands as ordinary `.ts` in the package.
const STATIC_FILES: &[(&str, &str)] = &[("app.ts", include_str!("../templates/napi/app.ts.jinja"))];

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "service_ts_lifecycle_hook.jinja".to_owned(),
        include_str!("../templates/napi/service_ts_lifecycle_hook.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

/// Emit TypeScript HTTP extension files: the ergonomic App layer and optional lifecycle hooks.
///
/// # Errors
///
/// Returns an error if template rendering fails.
///
/// # Panics
///
/// Panics if a built-in template fails to parse (indicates a compile-time bug).
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    let mut files = Vec::new();

    // Emit the static ergonomic app files
    for (rel, content) in STATIC_FILES {
        files.push(GeneratedFile {
            path: PathBuf::from(PACKAGE_DIR).join(rel),
            content: (*content).to_owned(),
            generated_header: true,
        });
    }

    // Emit lifecycle hook additions if present
    if !cfg.lifecycle_hooks.is_empty() {
        let env = make_env();
        let mut out = String::new();

        for hook in &cfg.lifecycle_hooks {
            let method_name = hook.name.to_lower_camel_case();
            out.push_str(
                &env.get_template("service_ts_lifecycle_hook.jinja")
                    .expect("template must exist")
                    .render(context! {
                        method_name => &method_name,
                        doc => &hook.doc,
                        is_async => hook.is_async,
                    })
                    .unwrap_or_default(),
            );
        }

        if !out.is_empty() {
            files.push(GeneratedFile {
                path: PathBuf::from("crates/spikard-node/service_http_additions.ts"),
                content: out,
                generated_header: true,
            });
        }
    }

    Ok(files)
}
