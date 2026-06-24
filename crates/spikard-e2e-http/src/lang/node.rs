//! TypeScript/Node HTTP e2e test generation (verbatim copy from alef core).
//!
//! This module is a staged, verbatim copy of alef's TypeScript HTTP-producing
//! code: the server-pattern `app_harness.mjs` renderer and the per-fixture HTTP
//! `it(...)` test-case renderer. Generic TypeScript scaffolding (package.json,
//! tsconfig, vitest config, global setup, and the non-HTTP `render_test_file`
//! dispatcher) is intentionally NOT copied — only the HTTP slice lives here.
//!
//! Sources (alef `src/e2e/codegen/typescript/`):
//! - `config.rs::render_app_harness`
//! - `test_file/http.rs::{render_http_test_case, synthesize_multipart_body_from_schema}`
//! - `json.rs::{json_to_js, json_to_js_multiline}` (the JSON-literal helpers the
//!   HTTP path consumes)
//!
//! Nothing in this crate calls these yet — `emit_e2e` still returns empty. The
//! module is `#[allow(dead_code)]` until cutover.
#![allow(dead_code)]

use alef::core::hash::{self, CommentStyle};
use alef::e2e::config::E2eConfig;
use alef::e2e::escape::sanitize_ident;
use alef::e2e::escape::{escape_js, expand_fixture_templates};
use alef::e2e::fixture::{Fixture, FixtureGroup};
use minijinja::{Environment, context};

/// Build the private template environment holding the TypeScript HTTP templates.
///
/// Mirrors the `make_env` pattern in `spikard-alef-ext::emit::magnus`: each
/// template is embedded via `include_str!` so the crate is self-contained and
/// does not depend on alef's shared `crate::e2e::template_env` registry.
fn make_env() -> Environment<'static> {
    let mut env = Environment::new();
    env.set_trim_blocks(true);
    env.set_lstrip_blocks(true);
    env.set_keep_trailing_newline(true);
    env.add_template_owned(
        "typescript/app_harness.mjs.jinja".to_owned(),
        include_str!("../../templates/typescript/app_harness.mjs.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "typescript/http_test.jinja".to_owned(),
        include_str!("../../templates/typescript/http_test.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "typescript/http_test_skip_101.jinja".to_owned(),
        include_str!("../../templates/typescript/http_test_skip_101.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env.add_template_owned(
        "typescript/globalSetup_server.ts.jinja".to_owned(),
        include_str!("../../templates/typescript/globalSetup_server.ts.jinja").to_owned(),
    )
    .expect("built-in template parse failed");
    env
}

/// Render the server-pattern `globalSetup.ts` that spawns the `app_harness.mjs`
/// subprocess and exposes it to the vitest suite. Ported verbatim from alef's
/// `typescript/config.rs::render_global_setup(true)`.
#[must_use]
pub fn render_global_setup_server() -> String {
    let header = hash::header(CommentStyle::DoubleSlash);
    let env = make_env();
    render(
        &env,
        "typescript/globalSetup_server.ts.jinja",
        context! { header => header },
    )
}

/// Render a named template from the local environment.
fn render(env: &Environment<'static>, name: &str, ctx: minijinja::Value) -> String {
    env.get_template(name)
        .expect("template must exist")
        .render(ctx)
        .unwrap_or_default()
}

// ---------------------------------------------------------------------------
// JSON-to-JavaScript literal helpers (from alef `typescript/json.rs`).
// Only the helpers consumed by the HTTP path are copied.
// ---------------------------------------------------------------------------

/// Convert a `serde_json::Value` to a JavaScript literal string.
fn json_to_js(value: &serde_json::Value) -> String {
    match value {
        serde_json::Value::String(s) => {
            let expanded = expand_fixture_templates(s);
            format!("\"{}\"", escape_js(&expanded))
        }
        serde_json::Value::Bool(b) => b.to_string(),
        serde_json::Value::Number(n) => {
            // For integers outside JS safe range, emit as string to avoid precision loss.
            if let Some(i) = n.as_i64()
                && !(-9_007_199_254_740_991..=9_007_199_254_740_991).contains(&i)
            {
                return format!("Number(\"{i}\")");
            }
            if let Some(u) = n.as_u64()
                && u > 9_007_199_254_740_991
            {
                return format!("Number(\"{u}\")");
            }
            n.to_string()
        }
        serde_json::Value::Null => "null".to_string(),
        serde_json::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_js).collect();
            format!("[{}]", items.join(", "))
        }
        serde_json::Value::Object(map) => {
            let entries: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    // Quote keys that aren't valid JS identifiers (contain hyphens, spaces, etc.)
                    let key = if k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
                        && !k.starts_with(|c: char| c.is_ascii_digit())
                    {
                        k.clone()
                    } else {
                        format!("\"{}\"", escape_js(k))
                    };
                    format!("{key}: {}", json_to_js(v))
                })
                .collect();
            format!("{{ {} }}", entries.join(", "))
        }
    }
}

/// Convert a `serde_json::Value` to an indented multi-line JavaScript literal.
///
/// Top-level objects are always expanded to multi-line form with trailing commas
/// so that formatters (e.g. oxfmt) leave the output unchanged. Scalar values and
/// arrays are emitted inline. Nested objects are also expanded to multi-line.
///
/// The `indent` parameter controls the base indentation in spaces for all but
/// the outermost `{`/`}`. Pass 4 for a top-level `expect(data).toEqual({...})`
/// inside a two-space-indented test body.
fn json_to_js_multiline(value: &serde_json::Value, indent: usize) -> String {
    match value {
        serde_json::Value::Object(map) => {
            if map.is_empty() {
                return "{}".to_string();
            }
            let pad = " ".repeat(indent);
            let inner_pad = " ".repeat(indent + 2);
            let entries: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let key = if k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '$')
                        && !k.starts_with(|c: char| c.is_ascii_digit())
                    {
                        k.clone()
                    } else {
                        format!("\"{}\"", escape_js(k))
                    };
                    format!("{inner_pad}{key}: {},", json_to_js_multiline(v, indent + 2))
                })
                .collect();
            format!("{{\n{}\n{pad}}}", entries.join("\n"))
        }
        // Non-object values are emitted inline.
        other => json_to_js(other),
    }
}

// ---------------------------------------------------------------------------
// App harness renderer (from alef `typescript/config.rs::render_app_harness`).
// ---------------------------------------------------------------------------

#[must_use]
pub fn render_app_harness(e2e_config: &E2eConfig, groups: &[FixtureGroup]) -> String {
    // Collect all HTTP fixtures from all groups.
    let mut fixtures_map = serde_json::Map::new();

    for group in groups {
        for fixture in &group.fixtures {
            if fixture.http.is_none() {
                continue;
            }
            // Convert the fixture to JSON for the harness to load.
            // We only need the http field, handler, request, and expected_response.
            let http_data = &fixture.http.as_ref().unwrap();
            let mut handler_obj = serde_json::json!({
                "route": &http_data.handler.route,
                "method": &http_data.handler.method,
                "body_schema": http_data.handler.body_schema.clone(),
            });
            // Include middleware if present for CORS preflight registration
            if let Some(middleware) = &http_data.handler.middleware
                && let Ok(middleware_json) = serde_json::to_value(middleware)
                && let serde_json::Value::Object(ref obj) = handler_obj
            {
                let mut handler_map = obj.clone();
                handler_map.insert("middleware".to_string(), middleware_json);
                handler_obj = serde_json::Value::Object(handler_map);
            }
            let mut request_obj = serde_json::json!({
                "path": &http_data.request.path,
            });
            // Include content_type if present for multipart/form-encoded detection
            if let Some(ct) = &http_data.request.content_type
                && let serde_json::Value::Object(ref obj) = request_obj
            {
                let mut request_map = obj.clone();
                request_map.insert("content_type".to_string(), serde_json::Value::String(ct.clone()));
                request_obj = serde_json::Value::Object(request_map);
            }
            let fixture_json = serde_json::json!({
                "http": {
                    "handler": handler_obj,
                    "request": request_obj,
                    "expected_response": {
                        "status_code": http_data.expected_response.status_code,
                        "body": &http_data.expected_response.body,
                        "headers": &http_data.expected_response.headers,
                    }
                }
            });
            fixtures_map.insert(fixture.id.clone(), fixture_json);
        }
    }

    let fixtures_json = serde_json::to_string(&fixtures_map).unwrap_or_default();
    let host = &e2e_config.harness.host;
    let port = e2e_config.harness.port;
    let header = hash::header(CommentStyle::DoubleSlash);

    let imports = e2e_config.harness.imports_for_lang("node");
    let app_class = e2e_config.harness.app_class_for_lang("node");
    let method_enum = &e2e_config.harness.method_enum;
    let run_method = e2e_config.harness.run_method_for_lang("node");
    // Node.js NAPI-RS binding has two route-registration forms:
    // - route() is a single-arg decorator returning a callable
    // - register_route() is a two-arg direct method
    // The harness uses two-arg registration, so always use registerRoute (camelCased).
    let register_method = "registerRoute".to_string();

    // For NAPI-RS bindings (Node.js/WASM), detect the constructor pattern.
    // If imports include "/node" or "wasm", use App.new() factory method.
    // Otherwise, use traditional new App() constructor.
    let constructor_method = if imports.iter().any(|imp| imp.contains("/node") || imp.contains("wasm")) {
        ".new()"
    } else {
        "new"
    };

    // napi-rs does not expose `new`-able JS constructors for Rust types — Rust
    // constructors become static factory methods (`RouteBuilder.new(...)`).
    // wasm-bindgen DOES expose them as proper JS constructors, so it keeps the
    // `new RouteBuilder(...)` syntax. Detect by import path the same way as the
    // App constructor selection above; the value is consumed by the harness
    // template's `{% if route_builder_constructor_method == ".new" %}` branch.
    let route_builder_constructor_method = if imports.iter().any(|imp| imp.contains("/node")) {
        ".new"
    } else {
        "new"
    };

    let route_builder_class = e2e_config.harness.route_builder.as_deref().unwrap_or("RouteBuilder");

    // Determine which ServerConfig factory expression to use (backend-specific defaults).
    // Node uses `serverConfigDefault()` factory; wasm-bindgen exposes the
    // `WasmServerConfig` class with a default constructor; generic TypeScript
    // bindings fall back to `new ServerConfig()`.
    let factory_lang = if imports.iter().any(|imp| imp.contains("/node")) {
        "node"
    } else if imports.iter().any(|imp| imp.contains("wasm")) {
        "wasm"
    } else {
        "typescript"
    };
    let server_config_factory = e2e_config.harness.server_config_factory_for_lang(factory_lang);
    // Companion import identifier: when the factory is a bare-identifier call,
    // the destructure import must include that identifier.
    let server_config_factory_import = e2e_config
        .harness
        .server_config_factory_import_for_lang(factory_lang)
        .unwrap_or_else(|| "ServerConfig".to_string());
    let import_style = e2e_config.harness.import_style_for_lang(factory_lang);

    let env = make_env();
    render(
        &env,
        "typescript/app_harness.mjs.jinja",
        context! {
            header => header,
            host => host,
            port => port,
            response_body_field => e2e_config.harness.response_body_field.as_str(),
            fixtures_json => fixtures_json,
            imports => imports,
            app_class => app_class.as_deref().unwrap_or("App"),
            method_enum => method_enum.as_deref().unwrap_or("Method"),
            route_builder_class => route_builder_class,
            route_builder_constructor_method => route_builder_constructor_method,
            run_method => run_method.as_deref().unwrap_or("run"),
            register_route_method => register_method.as_str(),
            constructor_method => constructor_method,
            server_config_factory => server_config_factory,
            server_config_factory_import => server_config_factory_import,
            import_style => import_style,
        },
    )
}

// ---------------------------------------------------------------------------
// Per-fixture HTTP test-case renderer
// (from alef `typescript/test_file/http.rs`).
// ---------------------------------------------------------------------------

pub fn render_http_test_case(out: &mut String, fixture: &Fixture) {
    let Some(http) = &fixture.http else {
        return;
    };

    let test_name = sanitize_ident(&fixture.id);
    // Escape backslashes and double quotes for use in a double-quoted JS string.
    let description = fixture.description.replace('\\', "\\\\").replace('"', "\\\"");

    if http.expected_response.status_code == 101 {
        return;
    }

    let method = http.request.method.to_uppercase();

    // Detect content-type so the renderer can decide between JSON-encoded and
    // raw (form-urlencoded / multipart) body emission.
    let content_type_lower = http
        .request
        .headers
        .iter()
        .find(|(k, _)| k.eq_ignore_ascii_case("content-type"))
        .map_or_else(
            || {
                http.request
                    .content_type
                    .as_ref()
                    .map(|ct| ct.to_ascii_lowercase())
                    .unwrap_or_default()
            },
            |(_, v)| v.to_ascii_lowercase(),
        );
    let is_form_body = content_type_lower
        .split(';')
        .next()
        .map(str::trim)
        .is_some_and(|t| t.eq_ignore_ascii_case("application/x-www-form-urlencoded"));
    let is_multipart = content_type_lower
        .split(';')
        .next()
        .map(str::trim)
        .is_some_and(|t| t.eq_ignore_ascii_case("multipart/form-data"));

    // If multipart but no request body, synthesize from body_schema
    let effective_body = if is_multipart && http.request.body.is_none() && http.handler.body_schema.is_some() {
        // Synthesize a minimal multipart body from the schema
        Some(synthesize_multipart_body_from_schema(&http.handler.body_schema))
    } else {
        http.request.body.clone()
    };

    // Determine if we need to auto-add Content-Type header for JSON body.
    let has_body = effective_body.is_some();
    let has_content_type = !content_type_lower.is_empty();
    let needs_json_content_type = has_body && !is_form_body && !is_multipart && !has_content_type;

    let has_headers = !http.request.headers.is_empty() || needs_json_content_type || is_multipart && has_body;

    // Build the body entry if present.
    let body_entry: Option<String> = effective_body.as_ref().map(|body| {
        let js_body = json_to_js(body);
        let body_is_string = matches!(body, serde_json::Value::String(_));

        // For multipart/form-data or form-urlencoded, the body is raw bytes as a string.
        // Wrap in Buffer.from() to send as UTF-8 bytes without JSON.stringify.
        if (is_form_body || is_multipart) && body_is_string {
            // Raw form-urlencoded or multipart: wrap string in Buffer.from() to send as bytes
            format!("body: Buffer.from({js_body}, 'utf-8')")
        } else {
            format!("body: JSON.stringify({js_body})")
        }
    });

    // Build the fetch init object. Use multi-line form when headers or body
    // are present so the output matches what oxfmt would produce; use inline
    // form for the simple method+redirect-only case.
    let fetch_init: String = if has_headers || body_entry.is_some() {
        // Multi-line object: each entry on its own line, trailing commas.
        let mut lines: Vec<String> = Vec::new();
        lines.push(format!("      method: \"{method}\","));
        lines.push("      redirect: \"manual\",".to_string());
        if has_headers {
            let mut header_lines: Vec<String> = http
                .request
                .headers
                .iter()
                // Skip Content-Type for multipart fixtures — we'll add the correct one below
                .filter(|(k, _)| !(is_multipart && k.eq_ignore_ascii_case("content-type")))
                .map(|(k, v)| {
                    let expanded_v = expand_fixture_templates(v);
                    format!("        \"{}\": \"{}\",", escape_js(k), escape_js(&expanded_v))
                })
                .collect();
            if needs_json_content_type {
                header_lines.push("        \"Content-Type\": \"application/json\",".to_string());
            }
            if is_multipart && has_body {
                // For multipart bodies, add the correct Content-Type with boundary
                header_lines
                    .push("        \"Content-Type\": \"multipart/form-data; boundary=alef-boundary\",".to_string());
            }
            lines.push("      headers: {".to_string());
            lines.extend(header_lines);
            lines.push("      },".to_string());
        }
        if let Some(body) = body_entry {
            lines.push(format!("      {body},"));
        }
        format!("{{\n{}\n    }}", lines.join("\n"))
    } else {
        // Inline: no headers, no body — only method and redirect.
        format!("{{ method: \"{method}\", redirect: \"manual\" }}")
    };

    let init_str = fetch_init;
    // Server-pattern: construct path as /fixtures/{fixture_id}{request_path}
    let path = format!("/fixtures/{}{}", &fixture.id, &http.request.path);

    let status = http.expected_response.status_code;

    // Determine body type and prepare context
    let (has_text_body, text_body) = if let Some(expected_body) = &http.expected_response.body {
        if expected_body.is_null() || expected_body.is_string() && expected_body.as_str() == Some("") {
            (false, String::new())
        } else if let serde_json::Value::String(s) = expected_body {
            (true, escape_js(s))
        } else {
            (false, String::new())
        }
    } else {
        (false, String::new())
    };

    let (has_json_body, json_val) = if let Some(expected_body) = &http.expected_response.body {
        if expected_body.is_null() || expected_body.is_string() && expected_body.as_str() == Some("") {
            (false, String::new())
        } else if let serde_json::Value::String(_) = expected_body {
            (false, String::new())
        } else {
            // Use multi-line form for objects so the output is stable under
            // oxfmt (formatters leave properly-indented multi-line objects
            // unchanged). Scalar and array values stay inline.
            (true, json_to_js_multiline(expected_body, 4))
        }
    } else {
        (false, String::new())
    };

    let (has_partial_body, partial_body_checks) = if let Some(partial) = &http.expected_response.body_partial {
        if let Some(obj) = partial.as_object() {
            let checks: Vec<minijinja::Value> = obj
                .iter()
                .map(|(key, val)| {
                    minijinja::context! {
                        key => escape_js(key),
                        js_val => json_to_js(val),
                    }
                })
                .collect();
            (true, checks)
        } else {
            (false, Vec::new())
        }
    } else {
        (false, Vec::new())
    };

    // Build header assertions
    let mut header_assertions: Vec<minijinja::Value> = Vec::new();
    for (header_name, header_value) in &http.expected_response.headers {
        let lower_name = header_name.to_lowercase();
        if lower_name == "content-encoding" {
            continue;
        }
        let escaped_name = escape_js(&lower_name);
        let (assertion_type, value) = match header_value.as_str() {
            "<<present>>" => ("present", String::new()),
            "<<absent>>" => ("absent", String::new()),
            "<<uuid>>" => ("uuid", String::new()),
            exact => ("exact", escape_js(exact)),
        };
        header_assertions.push(minijinja::context! {
            name => escaped_name,
            assertion_type => assertion_type,
            value => value,
        });
    }

    // Build validation error assertions
    let body_has_content = matches!(&http.expected_response.body, Some(v)
        if !(v.is_null() || (v.is_string() && v.as_str() == Some(""))));
    let (has_validation_errors, validation_errors) =
        if let Some(validation_errors) = &http.expected_response.validation_errors {
            if !validation_errors.is_empty() && !body_has_content {
                let errors: Vec<minijinja::Value> = validation_errors
                    .iter()
                    .map(|ve| {
                        let loc_js: Vec<String> = ve.loc.iter().map(|s| format!("\"{}\"", escape_js(s))).collect();
                        let loc_str = loc_js.join(", ");
                        let expanded_msg = expand_fixture_templates(&ve.msg);
                        let escaped_msg = escape_js(&expanded_msg);
                        minijinja::context! {
                            loc_js => loc_str,
                            escaped_msg => escaped_msg,
                        }
                    })
                    .collect();
                (true, errors)
            } else {
                (false, Vec::new())
            }
        } else {
            (false, Vec::new())
        };

    let ctx = minijinja::context! {
        test_name => test_name,
        description => description,
        method => method,
        init_str => init_str,
        path => path,
        expected_status => status,
        has_text_body => has_text_body,
        text_body => text_body,
        has_json_body => has_json_body,
        json_val => json_val,
        has_partial_body => has_partial_body,
        partial_body_checks => partial_body_checks,
        header_assertions => header_assertions,
        has_validation_errors => has_validation_errors,
        validation_errors => validation_errors,
        is_multipart => is_multipart,
    };
    let env = make_env();
    let rendered = render(&env, "typescript/http_test.jinja", ctx);
    out.push_str(&rendered);
}

/// Synthesize a minimal multipart/form-data body from a JSON schema.
/// RFC 2388 requires boundaries to be prefixed with CRLF and the final boundary
/// to end with CRLF followed by `--` (i.e., `\r\n--boundary--\r\n`).
fn synthesize_multipart_body_from_schema(schema: &Option<serde_json::Value>) -> serde_json::Value {
    let Some(schema_val) = schema else {
        return serde_json::Value::String(String::new());
    };

    let mut body = String::new();
    let boundary = "alef-boundary";

    if let Some(props) = schema_val.get("properties").and_then(|p| p.as_object()) {
        for (key, prop_schema) in props {
            // Check if this is a binary/file field
            let is_binary = prop_schema
                .get("format")
                .and_then(|f| f.as_str())
                .is_some_and(|f| f == "binary");

            body.push_str(&format!("--{boundary}\r\n"));

            if is_binary {
                body.push_str(&format!(
                    "Content-Disposition: form-data; name=\"{}\"; filename=\"{}.txt\"\r\nContent-Type: text/plain\r\n\r\n<file content>",
                    escape_js(key),
                    escape_js(key)
                ));
            } else {
                body.push_str(&format!(
                    "Content-Disposition: form-data; name=\"{}\"\r\n\r\ntest_value",
                    escape_js(key)
                ));
            }

            body.push_str("\r\n");
        }
    }

    // RFC 2388: final boundary must be terminated with `--` and CRLF
    body.push_str(&format!("--{boundary}--\r\n"));
    serde_json::Value::String(body)
}
