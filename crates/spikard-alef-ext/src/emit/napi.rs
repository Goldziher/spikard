//! TypeScript (napi-rs) emission for HTTP extension: lifecycle hooks.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use heck::ToLowerCamelCase;
use minijinja::{Environment, context};
use std::path::PathBuf;

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

/// Emit TypeScript HTTP extension additions.
///
/// Returns lifecycle hook method snippets to be appended to the TypeScript service wrapper.
///
/// # Errors
///
/// Returns an error if template rendering fails.
///
/// # Panics
///
/// Panics if a built-in template fails to parse (indicates a compile-time bug).
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if cfg.lifecycle_hooks.is_empty() {
        return Ok(vec![]);
    }

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

    if out.is_empty() {
        return Ok(vec![]);
    }

    Ok(vec![GeneratedFile {
        path: PathBuf::from("crates/spikard-node/service_http_additions.ts"),
        content: out,
        generated_header: true,
    }])
}
