//! Go (cgo) emission for HTTP extension: Config struct, error types, lifecycle hooks,
//! WebSocket/SSE route stubs, Run method, helpers.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use heck::{ToSnakeCase, ToUpperCamelCase};
use minijinja::{Environment, context};
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
pub fn emit(api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if cfg.lifecycle_hooks.is_empty()
        && cfg.error_types.is_empty()
        && cfg.websocket_routes.is_empty()
        && cfg.sse_routes.is_empty()
    {
        return Ok(vec![]);
    }

    let env = make_env();

    let service_name = api.services.first().map_or_else(
        || api.crate_name.to_upper_camel_case(),
        |s| s.name.to_upper_camel_case(),
    );
    let service_snake = service_name.to_snake_case();
    let ffi_prefix = api.crate_name.to_upper_camel_case();
    let service_lower = api.crate_name.to_lowercase();
    let upper_prefix = ffi_prefix.to_uppercase();

    let mut out = String::new();
    out.push_str(&format!("package {service_lower}\n\n"));

    out.push_str(&render(
        &env,
        "service_config_struct.jinja",
        context! { service_name => &service_name },
    ));
    out.push_str("\n\n");

    if !cfg.error_types.is_empty() {
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
        out.push_str("\n\n");
    }

    if !cfg.lifecycle_hooks.is_empty() {
        let hook_contexts: Vec<_> = cfg
            .lifecycle_hooks
            .iter()
            .map(|h| {
                context! {
                    name => &h.name,
                    name_pascal => h.name.to_upper_camel_case(),
                    callback_type => "func(interface{}) error".to_string(),
                    doc => &h.doc,
                }
            })
            .collect();
        out.push_str(&render(
            &env,
            "service_lifecycle_hooks.jinja",
            context! {
                service_name => &service_name,
                hooks => hook_contexts,
            },
        ));
        out.push_str("\n\n");
    }

    out.push_str(&render(
        &env,
        "service_run_method.jinja",
        context! {
            service_name => &service_name,
            service_snake => &service_snake,
            service_lower => &service_lower,
            upper_prefix => &upper_prefix,
        },
    ));
    out.push_str("\n\n");

    out.push_str(&render(&env, "service_helpers.jinja", context! {}));

    Ok(vec![GeneratedFile {
        path: PathBuf::from("packages/go/service_http_additions.go"),
        content: out,
        generated_header: true,
    }])
}
