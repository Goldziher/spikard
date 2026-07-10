//! Go (cgo) emission for HTTP extension: Config struct, error types, lifecycle hooks,
//! WebSocket/SSE route stubs, Run method, helpers.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use minijinja::{Environment, context};
use std::fmt::Write as _;
use std::path::PathBuf;

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "service_config_struct.jinja".to_owned(),
        include_str!("../templates/go/service_config_struct.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_error_types.jinja".to_owned(),
        include_str!("../templates/go/service_error_types.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_lifecycle_hooks.jinja".to_owned(),
        include_str!("../templates/go/service_lifecycle_hooks.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_run_method.jinja".to_owned(),
        include_str!("../templates/go/service_run_method.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_helpers.jinja".to_owned(),
        include_str!("../templates/go/service_helpers.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "app.go.jinja".to_owned(),
        include_str!("../templates/go/app.go.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

/// Emit Go HTTP extension additions.
///
/// # Errors
///
/// Never fails; always returns `Ok(...)`.
pub fn emit(_api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    let env = make_env();

    let mut files = vec![GeneratedFile {
        path: PathBuf::from("packages/go/app.go"),
        content: render(&env, "app.go.jinja", context! {}),
        generated_header: true,
    }];

    if cfg.error_types.is_empty() {
        return Ok(files);
    }

    let mut out = String::new();
    let _ = writeln!(out, "package spikard\n");

    let error_contexts: Vec<_> = cfg
        .error_types
        .iter()
        .map(|e| {
            context! {
                name => &e.name,
                http_status => e.http_status.as_u16(),
                problem_details_type => e.problem_details_type.as_deref().unwrap_or(""),
                doc => &e.doc,
            }
        })
        .collect();
    out.push_str(&render(
        &env,
        "service_error_types.jinja",
        context! { error_types => error_contexts },
    ));

    files.push(GeneratedFile {
        path: PathBuf::from("packages/go/service_http_additions.go"),
        content: out,
        generated_header: true,
    });

    Ok(files)
}
