//! Go (cgo) emission for HTTP extension: Config struct, error types, lifecycle hooks,
//! WebSocket/SSE route stubs, Run method, helpers.

use crate::config::HttpExtensionConfig;
use alef::core::backend::GeneratedFile;
use alef::core::hash::{CommentStyle, header};
use alef::core::ir::ApiSurface;
use anyhow::Result;
use minijinja::{Environment, context};
use std::fmt::Write as _;
use std::path::PathBuf;

/// Go binding files that carry lint-sensitive generated code but are emitted
/// without the standard alef generated header: `service.go` (emitted by alef
/// core's Go backend) plus this extension's `app.go` and
/// `service_http_additions.go`.
///
/// golangci-lint's `generated: lax` exclusion (see `packages/go/.golangci.yml`)
/// skips files carrying the alef header — the same mechanism that already excludes
/// `binding.go` and `native_setup.go`. Without the header these three files are
/// linted and trip `goconst` (`"status"`/`"title"`/`"object"`), `govet`
/// (`reflect.Ptr`), and `revive` var-naming on alef-core-owned code we do not
/// control. Injecting the header keeps the whole `package spikard` consistent and
/// avoids per-finding churn in templates outside this repository.
const HEADERLESS_GO_FILES: &[&str] = &["service.go", "app.go", "service_http_additions.go"];

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

/// Prepend the standard alef generated header to headerless Go binding files.
///
/// Targets the files in [`HEADERLESS_GO_FILES`] so golangci-lint's
/// `generated: lax` exclusion treats them as generated and skips them.
///
/// Runs from [`crate::HttpExtension::transform_emitted_files`], which sees both
/// the alef-core Go backend files (`service.go`) and this extension's own files
/// (`app.go`, `service_http_additions.go`). Idempotent: files already carrying the
/// alef marker are left untouched, so the per-file `alef:hash:` line injected later
/// by the pipeline stays stable.
pub fn add_generated_headers(files: &mut [GeneratedFile]) {
    for file in files.iter_mut() {
        let Some(name) = file.path.file_name().and_then(|n| n.to_str()) else {
            continue;
        };
        if !HEADERLESS_GO_FILES.contains(&name) {
            continue;
        }
        if file.content.contains("auto-generated by alef") {
            continue;
        }
        let mut content = header(CommentStyle::DoubleSlash);
        content.push('\n');
        content.push_str(&file.content);
        file.content = content;
    }
}

/// Anchor in the alef go scaffold's `.golangci.yml`: the sole `rules:` list under
/// the linters `exclusions:` block. The service.go exclusion is inserted right
/// after it. (The `formatters.exclusions` block has no `rules:` key, so this
/// substring is unique.)
const GOLANGCI_RULES_ANCHOR: &str = "\n    rules:\n";

/// golangci-lint exclusion added for `service.go`. Matches the entry style of the
/// scaffold's existing rules (6-space list item, 10-space linter name, 8-space keys).
const GOLANGCI_SERVICE_EXCLUSION: &str = "      - linters:\n          - revive\n        path: (^|/)service\\.go$\n";

/// Rewrite the alef-scaffolded `packages/go/.golangci.yml` so `revive` is excluded
/// on `service.go`.
///
/// `service.go` is a fully alef-generated binding, but alef core emits it from the
/// service-api pass *without* the standard generated header and with no extension
/// hook over that pass (unlike `binding.go`/`app.go`, which carry the header and are
/// skipped by `exclusions.generated: lax`). Its alef-generated `Config` method uses
/// C-idiomatic `c_config`/`new_owner` names that `revive`'s `var-naming` rule flags —
/// names owned by alef-core templates this repository does not control. This
/// exclusion emulates exactly the generated-file skip the header would have provided,
/// scoped to the one linter that fires on `service.go`. Idempotent.
pub fn wire_golangci_service_exclusion(files: &mut [GeneratedFile]) {
    for file in files.iter_mut() {
        if file.path.file_name().and_then(|n| n.to_str()) != Some(".golangci.yml") {
            continue;
        }
        if file.content.contains("(^|/)service\\.go$") {
            return;
        }
        if let Some(pos) = file.content.find(GOLANGCI_RULES_ANCHOR) {
            let insert_at = pos + GOLANGCI_RULES_ANCHOR.len();
            file.content.insert_str(insert_at, GOLANGCI_SERVICE_EXCLUSION);
        }
    }
}
