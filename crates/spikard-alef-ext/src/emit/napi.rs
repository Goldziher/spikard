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
///
/// This is the real napi-rs package the alef Node backend emits into (holds
/// `index-wrapper.cjs`, `service.cjs`, `index.js`), so the ergonomic assets land
/// alongside the low-level binding and the entry re-export can reach them.
const PACKAGE_DIR: &str = "crates/spikard-node";

/// Static ergonomic assets: (package-relative output path, file contents).
///
/// Sources carry a `.jinja` suffix on disk (excluded from lint/format); the emitted
/// output path drops the `.jinja`. `app.cjs` is the `CommonJS` ergonomic runtime and
/// `app.d.ts` its TypeScript declarations. Both are static — no per-surface context.
const STATIC_FILES: &[(&str, &str)] = &[
    ("app.cjs", include_str!("../templates/napi/app.cjs.jinja")),
    ("app.d.ts", include_str!("../templates/napi/app.d.ts.jinja")),
];

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

    for (rel, content) in STATIC_FILES {
        files.push(GeneratedFile {
            path: PathBuf::from(PACKAGE_DIR).join(rel),
            content: (*content).to_owned(),
            generated_header: true,
        });
    }

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

/// Rewrite the backend-generated Node entry files so `require('@spikard/node').App`
/// (and the TypeScript types) resolve to the ergonomic `app.cjs` App.
///
/// Three idempotent rewrites, each a no-op if already applied:
/// - `index-wrapper.cjs` (the package `main`): also require `./app.cjs` and spread its
///   exports last, shadowing the low-level `App`.
/// - `index.d.ts` (the types entry): re-export the ergonomic declarations.
/// - `package.json`: add the `zod` / `zod-to-json-schema` runtime dependencies the
///   ergonomic layer needs to derive JSON Schema from DTOs.
pub fn wire_ergonomic_entry(files: &mut [GeneratedFile]) {
    for file in files.iter_mut() {
        match file.path.file_name().and_then(|n| n.to_str()) {
            Some("index-wrapper.cjs") => wire_cjs_wrapper(&mut file.content),
            Some("index.d.ts") => wire_types_entry(&mut file.content),
            Some("package.json") => wire_package_json(&mut file.content),
            _ => {}
        }
    }
}

/// Inject `const _app = require("./app.cjs");` and spread `..._app` last in the
/// `CommonJS` wrapper's `module.exports`. No-op if already wired.
fn wire_cjs_wrapper(content: &mut String) {
    if content.contains("require(\"./app.cjs\")") {
        return;
    }
    let service_require = "const _service = require(\"./service.cjs\");";
    if let Some(pos) = content.find(service_require) {
        let insert_at = pos + service_require.len();
        content.insert_str(insert_at, "\nconst _app = require(\"./app.cjs\");");
    }
    *content = content.replace("..._service };", "..._service, ..._app };");
}

/// Append the ergonomic type re-export to `index.d.ts`. No-op if already present.
fn wire_types_entry(content: &mut String) {
    let export = "export * from './app';";
    if content.contains(export) {
        return;
    }
    if !content.ends_with('\n') {
        content.push('\n');
    }
    content.push_str(export);
    content.push('\n');
}

/// Publish the ergonomic assets and add the zod runtime dependencies to
/// `package.json`. The napi `package.json` has no `dependencies` block by
/// default, so one is inserted before `optionalDependencies`. Both edits are
/// idempotent (guarded on the ergonomic file / dependency already being present).
fn wire_package_json(content: &mut String) {
    if !content.contains("\"app.cjs\"") {
        *content = content.replace(
            "\"files\": [\"index.js\",",
            "\"files\": [\"app.cjs\", \"app.d.ts\", \"index.js\",",
        );
    }
    if !content.contains("\"zod\"") {
        let dep_block = "\"dependencies\": {\n    \"zod\": \"^3.25.0 || ^4.0.0\"\n  },\n  ";
        if let Some(pos) = content.find("\"optionalDependencies\":") {
            content.insert_str(pos, dep_block);
        }
    }
}
