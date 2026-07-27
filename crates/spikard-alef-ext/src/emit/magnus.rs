//! Ruby (magnus) emission for HTTP extension: lifecycle hooks, error classes,
//! WebSocket/SSE methods.

use crate::config::HttpExtensionConfig;
use crate::ir::{LifecycleHookDef, SseRouteDef, WebSocketRouteDef};
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use heck::ToUpperCamelCase;
use minijinja::{Environment, context};
use std::fmt::Write as _;
use std::path::PathBuf;

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "service_rb_error_base_class.rb.jinja".to_owned(),
        include_str!("../templates/magnus/service_rb_error_base_class.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_rb_error_subclass.rb.jinja".to_owned(),
        include_str!("../templates/magnus/service_rb_error_subclass.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_rb_lifecycle_hook.rb.jinja".to_owned(),
        include_str!("../templates/magnus/service_rb_lifecycle_hook.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_rb_websocket_method.rb.jinja".to_owned(),
        include_str!("../templates/magnus/service_rb_websocket_method.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "service_rb_sse_method.rb.jinja".to_owned(),
        include_str!("../templates/magnus/service_rb_sse_method.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "app.rb.jinja".to_owned(),
        include_str!("../templates/magnus/app.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "params.rb.jinja".to_owned(),
        include_str!("../templates/magnus/params.rb.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "introspection.rb.jinja".to_owned(),
        include_str!("../templates/magnus/introspection.rb.jinja").to_owned(),
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

fn format_ruby_comment(text: &str, indent: usize) -> String {
    let trimmed = text.trim();
    let pad = " ".repeat(indent);
    if trimmed.is_empty() {
        return String::new();
    }
    let mut out = String::new();
    for line in trimmed.lines() {
        out.push_str(&pad);
        if line.trim().is_empty() {
            out.push_str("#\n");
        } else {
            out.push_str("# ");
            out.push_str(line);
            out.push('\n');
        }
    }
    out
}

/// Indent every non-blank line of `s` by `n` spaces, leaving blank lines empty
/// (so no trailing whitespace is emitted). Used to nest col-0 class templates
/// inside the `module {Crate}` / `module Errors` wrapper.
fn indent_block(s: &str, n: usize) -> String {
    let pad = " ".repeat(n);
    let mut out = String::new();
    for line in s.split_inclusive('\n') {
        match line.strip_suffix('\n') {
            Some("") => out.push('\n'),
            Some(content) => {
                out.push_str(&pad);
                out.push_str(content);
                out.push('\n');
            }
            None if line.is_empty() => {}
            None => {
                out.push_str(&pad);
                out.push_str(line);
            }
        }
    }
    out
}

fn gen_lifecycle_hooks(env: &Environment<'static>, out: &mut String, hooks: &[LifecycleHookDef]) {
    for hook in hooks {
        let doc_comment = format_ruby_comment(&hook.doc, 6);
        out.push_str(&render(
            env,
            "service_rb_lifecycle_hook.rb.jinja",
            context! {
                method_name => &hook.name,
                doc_comment => doc_comment,
            },
        ));
    }
}

fn gen_websocket_methods(env: &Environment<'static>, out: &mut String, routes: &[WebSocketRouteDef]) {
    for ws in routes {
        let doc_comment = format_ruby_comment(&ws.doc, 6);
        out.push_str(&render(
            env,
            "service_rb_websocket_method.rb.jinja",
            context! {
                doc_comment => doc_comment,
            },
        ));
    }
}

fn gen_sse_methods(env: &Environment<'static>, out: &mut String, routes: &[SseRouteDef]) {
    for sse in routes {
        let doc_comment = format_ruby_comment(&sse.doc, 6);
        out.push_str(&render(
            env,
            "service_rb_sse_method.rb.jinja",
            context! {
                doc_comment => doc_comment,
            },
        ));
    }
}

fn gen_error_classes(env: &Environment<'static>, api: &ApiSurface, cfg: &HttpExtensionConfig) -> String {
    if cfg.error_types.is_empty() {
        return String::new();
    }

    let crate_module = api.crate_name.to_upper_camel_case();
    let mut out = String::from("# frozen_string_literal: true\n\n");
    let _ = writeln!(out, "module {crate_module}");
    out.push_str("  module Errors\n");
    out.push_str(&indent_block(
        &render(env, "service_rb_error_base_class.rb.jinja", context! {}),
        4,
    ));

    for error in &cfg.error_types {
        let http_status = error.http_status.as_u16();
        let problem_details_type = error
            .problem_details_type
            .as_deref()
            .map_or_else(|| "nil".to_string(), |s| format!("\"{s}\""));
        let doc_comment = format_ruby_comment(&error.doc, 2);
        out.push('\n');
        out.push_str(&indent_block(
            &render(
                env,
                "service_rb_error_subclass.rb.jinja",
                context! {
                    class_name => &error.name,
                    http_status => http_status,
                    problem_details_type => problem_details_type,
                    doc_comment => doc_comment,
                },
            ),
            4,
        ));
    }

    out.push_str("  end\n");
    out.push_str("end\n");
    out
}

fn gen_service_additions(env: &Environment<'static>, cfg: &HttpExtensionConfig) -> String {
    let mut out = String::new();
    gen_lifecycle_hooks(env, &mut out, &cfg.lifecycle_hooks);
    gen_websocket_methods(env, &mut out, &cfg.websocket_routes);
    gen_sse_methods(env, &mut out, &cfg.sse_routes);
    out
}

fn emit_ergonomic(env: &Environment<'static>) -> Vec<GeneratedFile> {
    let mut files = Vec::new();

    let app_content = render(env, "app.rb.jinja", context! {});
    files.push(GeneratedFile {
        path: PathBuf::from("packages/ruby/lib/spikard/app.rb"),
        content: app_content,
        generated_header: true,
    });

    let params_content = render(env, "params.rb.jinja", context! {});
    files.push(GeneratedFile {
        path: PathBuf::from("packages/ruby/lib/spikard/params.rb"),
        content: params_content,
        generated_header: true,
    });

    let introspection_content = render(env, "introspection.rb.jinja", context! {});
    files.push(GeneratedFile {
        path: PathBuf::from("packages/ruby/lib/spikard/introspection.rb"),
        content: introspection_content,
        generated_header: true,
    });

    files
}

/// Emit Ruby HTTP extension files.
///
/// # Errors
///
/// Returns an error if template rendering fails.
pub fn emit(api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    let env = make_env();
    let mut files = Vec::new();

    files.extend(emit_ergonomic(&env));

    if !cfg.lifecycle_hooks.is_empty()
        || !cfg.websocket_routes.is_empty()
        || !cfg.sse_routes.is_empty()
        || !cfg.error_types.is_empty()
    {
        let errors_content = gen_error_classes(&env, api, cfg);
        if !errors_content.is_empty() {
            files.push(GeneratedFile {
                path: PathBuf::from("packages/ruby/lib/spikard/errors.rb"),
                content: errors_content,
                generated_header: true,
            });
        }

        let additions = gen_service_additions(&env, cfg);
        if !additions.is_empty() {
            files.push(GeneratedFile {
                path: PathBuf::from("packages/ruby/lib/spikard/service_http_additions.rb"),
                content: additions,
                generated_header: true,
            });
        }
    }

    Ok(files)
}

/// Ruby impl files that reopen the native `Spikard::App` class and call
/// runtime-only native methods. Steep type-checks against RBS and cannot see the
/// native surface, so every one of them yields false positives; they must be
/// steered to their RBS coverage via Steep `ignore` directives.
///
/// `service.rb` is emitted by alef core's magnus backend; the remaining five are
/// emitted by this extension (see [`emit`]). Hardcoding spikard's paths here is
/// intentional — this extension is spikard-specific.
const STEEP_IGNORED_IMPL_FILES: &[&str] = &[
    "lib/spikard/service.rb",
    "lib/spikard/app.rb",
    "lib/spikard/params.rb",
    "lib/spikard/introspection.rb",
    "lib/spikard/errors.rb",
    "lib/spikard/service_http_additions.rb",
];

/// Anchor line the alef ruby scaffold already emits in the `Steepfile`. The impl
/// ignores are inserted immediately after it so they sit inside the `target :lib`
/// block alongside the scaffold's own `native.rb` ignore.
const STEEP_NATIVE_IGNORE: &str = "  ignore \"lib/spikard/native.rb\"\n";

/// Extend the alef-scaffolded `packages/ruby/Steepfile` so Steep ignores the six
/// generated impl files in addition to the scaffold's `native.rb`.
///
/// Runs from [`crate::HttpExtension::transform_scaffold_files`]. The alef ruby
/// scaffold owns the `Steepfile` and only ignores `native.rb`; the scaffold is
/// project-agnostic and cannot hardcode spikard's impl filenames, so this
/// spikard-specific extension appends them. Idempotent: a `Steepfile` already
/// carrying the impl ignores is left untouched.
pub fn wire_steep_ignores(files: &mut [GeneratedFile]) {
    for file in files.iter_mut() {
        if file.path.file_name().and_then(|n| n.to_str()) != Some("Steepfile") {
            continue;
        }
        if file.content.contains(STEEP_IGNORED_IMPL_FILES[0]) {
            return;
        }
        let Some(anchor_end) = file
            .content
            .find(STEEP_NATIVE_IGNORE)
            .map(|pos| pos + STEEP_NATIVE_IGNORE.len())
        else {
            continue;
        };
        let mut additions = String::new();
        for path in STEEP_IGNORED_IMPL_FILES {
            let _ = writeln!(additions, "  ignore \"{path}\"");
        }
        file.content.insert_str(anchor_end, &additions);
    }
}
