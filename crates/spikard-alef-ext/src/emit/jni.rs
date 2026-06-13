//! JNI (Java) emission for HTTP extension: lifecycle hooks, error types.

use crate::config::HttpExtensionConfig;
use crate::ir::{ErrorTypeDef, LifecycleHookDef};
use alef::core::backend::GeneratedFile;
use alef::core::ir::ApiSurface;
use anyhow::Result;
use heck::ToUpperCamelCase;
use minijinja::{Environment, context};
use std::path::PathBuf;

fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "lifecycle_hook_registration.rs.jinja".to_owned(),
        include_str!("../templates/jni/lifecycle_hook_registration.rs.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "error_type_constructor.rs.jinja".to_owned(),
        include_str!("../templates/jni/error_type_constructor.rs.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

fn jni_package(crate_name: &str) -> String {
    format!("dev.{}", crate_name.replace('-', ""))
}

fn jni_symbol(package: &str, class: &str, method: &str) -> String {
    let pkg = package.replace('.', "_");
    format!("Java_{pkg}_{class}_{method}")
}

fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

fn gen_lifecycle_hooks(env: &Environment<'static>, hooks: &[LifecycleHookDef], api: &ApiSurface) -> String {
    if hooks.is_empty() {
        return String::new();
    }
    let package = jni_package(&api.crate_name);
    let core_import = api.crate_name.replace('-', "");
    let mut out = String::new();

    for service in &api.services {
        let service_bridge_class = format!("{}Bridge", service.name.to_upper_camel_case());
        let opaque_name = format!("{}Opaque", service.name.to_upper_camel_case());
        for hook in hooks {
            let hook_pascal = hook.name.to_upper_camel_case();
            let service_pascal = service.name.to_upper_camel_case();
            let hook_method = format!("register{service_pascal}{hook_pascal}");
            let symbol = jni_symbol(&package, &service_bridge_class, &hook_method);
            let bridge_name = format!("Jni{}Bridge", hook.callback_contract.to_upper_camel_case());

            if api
                .handler_contracts
                .iter()
                .any(|c| c.trait_name == hook.callback_contract)
            {
                out.push_str(&render(
                    env,
                    "lifecycle_hook_registration.rs.jinja",
                    context! {
                        service_pascal => &service_pascal,
                        hook_pascal => &hook_pascal,
                        hook_name => &hook.name,
                        symbol => &symbol,
                        bridge_name => &bridge_name,
                        core_import => &core_import,
                        contract_name => &hook.callback_contract,
                        opaque_name => &opaque_name,
                        is_async => hook.is_async,
                    },
                ));
            }
        }
    }
    out
}

fn gen_error_types(env: &Environment<'static>, types: &[ErrorTypeDef], api: &ApiSurface) -> String {
    if types.is_empty() {
        return String::new();
    }
    let package = jni_package(&api.crate_name);
    let mut out = String::new();

    for error_type in types {
        let error_pascal = &error_type.name;
        let status_code = error_type.http_status.as_u16();
        let problem_details_type = error_type.problem_details_type.as_deref().unwrap_or("");
        let method = format!("create{error_pascal}");
        let symbol = jni_symbol(&package, "Errors", &method);
        let error_class_path = format!("{}/errors/{error_pascal}", package.replace('.', "/"));

        out.push_str(&render(
            env,
            "error_type_constructor.rs.jinja",
            context! {
                error_pascal => error_pascal,
                error_name => &error_type.name,
                error_class_path => &error_class_path,
                symbol => &symbol,
                status_code => status_code,
                problem_details_type => problem_details_type,
                doc => &error_type.doc,
            },
        ));
    }
    out
}

/// Emit JNI HTTP extension Rust shim additions.
///
/// # Errors
///
/// Never fails; always returns `Ok(...)`.
pub fn emit(api: &ApiSurface, cfg: &HttpExtensionConfig) -> Result<Vec<GeneratedFile>> {
    if cfg.lifecycle_hooks.is_empty() && cfg.error_types.is_empty() {
        return Ok(vec![]);
    }

    let env = make_env();
    let mut out = String::new();
    out.push_str(&gen_lifecycle_hooks(&env, &cfg.lifecycle_hooks, api));
    out.push_str(&gen_error_types(&env, &cfg.error_types, api));

    if out.is_empty() {
        return Ok(vec![]);
    }

    Ok(vec![GeneratedFile {
        path: PathBuf::from("packages/java/rust/src/service_http_additions.rs"),
        content: out,
        generated_header: true,
    }])
}
