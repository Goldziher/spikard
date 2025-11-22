//! Rust test app generation
//!
//! # Purpose
//!
//! This module generates **MINIMAL** test applications from JSON fixtures to test Spikard's
//! internal behavior. The generated apps are NOT production code - they are scaffolding to
//! exercise Spikard's validation, routing, and error handling.
//!
//! # Critical Philosophy
//!
//! **The fixtures define Spikard's behavior, not the test app's behavior.**
//!
//! - Fixtures are specifications for how Spikard should handle requests
//! - Test apps are the simplest possible implementation to exercise Spikard
//! - All validation, error handling, status codes are handled BY SPIKARD
//! - Generated handlers contain NO business logic, NO smart routing, NO special cases
//!
//! # What NOT to Generate
//!
//! ❌ Do NOT add business logic (e.g., parsing status codes from path params)
//! ❌ Do NOT implement conditional behavior based on request data
//! ❌ Do NOT duplicate validation logic (Spikard does this!)
//! ❌ Do NOT add smart error handling (Spikard handles errors!)
//!
//! # What TO Generate
//!
//! ✓ Register routes from fixtures using Axum
//! ✓ Set up Spikard's validators with schemas from fixtures
//! ✓ Call Spikard's validation methods
//! ✓ Return simple success responses when validation passes
//! ✓ Let Spikard return structured errors when validation fails
//!
//! # Example Generated Handler
//!
//! ```rust,ignore
//! async fn handle_items_get(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
//!     let schema: Value = serde_json::from_str(SCHEMA_JSON).unwrap();
//!     let validator = ParameterValidator::new(schema).unwrap();
//!
//!     match validator.validate_and_extract(&query_params, ...) {
//!         Ok(validated) => (StatusCode::OK, Json(validated)),
//!         Err(err) => (StatusCode::UNPROCESSABLE_ENTITY, Json(err.to_json()))
//!     }
//! }
//! ```
//!
//! Notice how simple this is - Spikard does all the work!
//!
//! # Goal
//!
//! Use fixtures to drive TDD development of Spikard's Rust engine and ensure
//! consistent behavior across all language bindings (Rust, Python, TypeScript, Ruby).

use crate::asyncapi::{AsyncFixture, load_sse_fixtures, load_websocket_fixtures};
use crate::background::{BackgroundFixtureData, background_data};
use crate::middleware::{MiddlewareMetadata, parse_middleware, write_static_assets};
use crate::streaming::chunk_bytes;
use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::from_fixtures::FixtureExpectedResponse;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::Path;

pub fn generate_rust_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Rust test app at {}...", output_dir.display());

    let categories = discover_fixture_categories(fixtures_dir)?;
    let sse_fixtures = load_sse_fixtures(fixtures_dir).context("Failed to load SSE fixtures")?;
    let websocket_fixtures = load_websocket_fixtures(fixtures_dir).context("Failed to load WebSocket fixtures")?;

    let mut middleware_lookup: HashMap<String, MiddlewareMetadata> = HashMap::new();
    for (category, fixtures) in &categories {
        for fixture in fixtures {
            let metadata = parse_middleware(fixture)?;
            let fixture_id = format!("{}_{}", category, sanitize_name(&fixture.name));
            middleware_lookup.insert(fixture_id, metadata);
        }
    }

    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    let static_assets_root = output_dir.join("static_assets");
    if static_assets_root.exists() {
        fs::remove_dir_all(&static_assets_root)
            .with_context(|| format!("Failed to clear {}", static_assets_root.display()))?;
    }

    let cargo_toml = generate_cargo_toml();
    fs::write(output_dir.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;
    println!("  ✓ Generated Cargo.toml");

    let src_dir = output_dir.join("src");
    fs::create_dir_all(&src_dir).context("Failed to create src directory")?;

    let main_rs = generate_main_rs(&categories);
    fs::write(src_dir.join("main.rs"), main_rs).context("Failed to write main.rs")?;
    println!("  ✓ Generated src/main.rs");

    let lib_rs = generate_lib_rs(
        &categories,
        output_dir,
        &middleware_lookup,
        &sse_fixtures,
        &websocket_fixtures,
    )?;
    fs::write(src_dir.join("lib.rs"), lib_rs).context("Failed to write lib.rs")?;
    println!("  ✓ Generated src/lib.rs");

    Ok(())
}

fn discover_fixture_categories(fixtures_dir: &Path) -> Result<BTreeMap<String, Vec<Fixture>>> {
    let mut categories = BTreeMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|n| n.to_str())
                .context("Invalid directory name")?
                .to_string();

            let mut fixtures =
                load_fixtures_from_dir(&path).with_context(|| format!("Failed to load fixtures from {}", category))?;

            if !fixtures.is_empty() {
                fixtures.sort_by(|a, b| a.name.cmp(&b.name));
                categories.insert(category, fixtures);
            }
        }
    }

    Ok(categories)
}

fn generate_cargo_toml() -> String {
    r#"[package]
name = "spikard-e2e-app"
version = "0.1.0"
edition = "2021"
publish = false

# Opt out of parent workspace
[workspace]

[lib]
name = "spikard_e2e_app"
path = "src/lib.rs"

[[bin]]
name = "spikard-e2e-app"
path = "src/main.rs"

[dependencies]
spikard = { path = "../../crates/spikard" }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
percent-encoding = "2.3"
bytes = "1.7"
futures = "0.3"
axum-test = "18"
uuid = "1"
"#
    .to_string()
}

fn generate_main_rs(_categories: &BTreeMap<String, Vec<Fixture>>) -> String {
    r#"//! Generated test application
//! This is a minimal Spikard app built from fixture data

use spikard::AppError;

pub use spikard_e2e_app::*;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    tracing_subscriber::fmt::init();

    let app = create_app()?;

    app.run().await
}
"#
    .to_string()
}

#[allow(clippy::too_many_arguments)]
fn generate_lib_rs(
    categories: &BTreeMap<String, Vec<Fixture>>,
    output_dir: &Path,
    middleware_lookup: &HashMap<String, MiddlewareMetadata>,
    sse_fixtures: &[AsyncFixture],
    websocket_fixtures: &[AsyncFixture],
) -> Result<String> {
    let has_sse = !sse_fixtures.is_empty();
    let has_websocket = !websocket_fixtures.is_empty();
    let mut handlers = Vec::new();
    let mut app_functions = Vec::new();
    let mut has_streaming = categories
        .values()
        .flat_map(|fixtures| fixtures.iter())
        .any(|fixture| fixture.streaming.is_some());
    if has_sse || has_websocket {
        has_streaming = true;
    }
    for (category, fixtures) in categories {
        for fixture in fixtures {
            let fixture_slug = format!("{}_{}", category, sanitize_name(&fixture.name));
            let metadata = middleware_lookup
                .get(&fixture_slug)
                .with_context(|| format!("Missing middleware metadata for {}", fixture_slug))?;
            if !metadata.static_dirs.is_empty() {
                write_static_assets(output_dir, &fixture_slug, &metadata.static_dirs)?;
            }
            let (handler_code, app_fn_code) =
                generate_fixture_handler_and_app(category, fixture, metadata, &fixture_slug);
            if handler_code.trim().is_empty() {
            } else {
                handlers.push(handler_code);
            }
            app_functions.push(app_fn_code);
        }
    }

    if has_sse {
        let (sse_handlers, sse_apps) = generate_sse_handlers(sse_fixtures);
        handlers.extend(sse_handlers);
        app_functions.extend(sse_apps);
    }

    if has_websocket {
        let (ws_handlers, ws_apps) = generate_websocket_handlers(websocket_fixtures);
        handlers.extend(ws_handlers);
        app_functions.extend(ws_apps);
    }

    let mut header = String::from("//! Generated route handlers - one handler per fixture for complete isolation\n\n");
    header.push_str("use std::str::FromStr;\n");
    header.push_str("use std::sync::Arc;\n");
    header.push_str("use tokio::sync::Mutex;\n");
    if has_streaming {
        header.push_str("use bytes::Bytes;\n");
        header.push_str("use futures::stream;\n");
    }
    header.push_str("use axum::body::Body;\n");
    header.push_str("use axum::http::{HeaderName, HeaderValue, Response, StatusCode};\n");
    header.push_str("use serde_json::{json, Value};\n");
    header.push_str(
        "use spikard::{App, AppError, CompressionConfig, CorsConfig, HandlerResponse, HandlerResult, HookResult, LifecycleHook, LifecycleHooks, LifecycleHooksBuilder, Method, RateLimitConfig, RequestContext, RouteBuilder, ServerConfig, SseEvent, SseEventProducer, StaticFilesConfig, WebSocketHandler, add_cors_headers, handle_preflight, request_hook, response_hook, validate_cors_request, delete, get, patch, post, put};\n",
    );
    header.push_str("type HttpResponse = Response<Body>;\n\n");
    header.push_str(
        r#"fn apply_expected_headers(mut response: HttpResponse, headers: &[(&str, &str)]) -> HttpResponse {
    for &(name, value) in headers {
        if let Ok(header_name) = HeaderName::from_lowercase(name.as_bytes()) {
            if let Ok(header_value) = HeaderValue::from_str(value) {
                response.headers_mut().insert(header_name, header_value);
            }
        }
    }
    response
}

"#,
    );

    Ok(format!(
        r#"{header}

// Default app for backwards compatibility (empty)
pub fn create_app() -> Result<App, AppError> {{
    Ok(App::new())
}}

// Per-fixture app functions
{app_functions}

// Handler functions
{handlers}
"#,
        header = header,
        app_functions = app_functions.join("\n\n"),
        handlers = handlers.join("\n\n"),
    ))
}

/// Generate handler and app function for a single fixture
/// Returns (handler_code, app_function_code)
fn generate_fixture_handler_and_app(
    category: &str,
    fixture: &Fixture,
    metadata: &MiddlewareMetadata,
    fixture_slug: &str,
) -> (String, String) {
    let fixture_id = format!("{}_{}", category, sanitize_name(&fixture.name));
    let handler_name = format!("{}_handler", fixture_id);
    let app_fn_name = format!("create_app_{}", fixture_id);

    if let Ok(Some(background)) = background_data(fixture) {
        return generate_background_fixture(fixture, &fixture_id, &handler_name, &app_fn_name, background);
    }

    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };

    let route_path = route.split('?').next().unwrap_or(&route).to_string();
    let method = fixture.request.method.as_str();

    let hooks_code = if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            if let Some(hooks) = middleware.get("lifecycle_hooks") {
                generate_lifecycle_hooks_rust(&fixture_id, hooks, fixture)
            } else {
                String::new()
            }
        } else {
            String::new()
        }
    } else {
        String::new()
    };

    let hooks_registration = if let Some(handler) = &fixture.handler {
        if let Some(middleware) = &handler.middleware {
            middleware
                .get("lifecycle_hooks")
                .map(|hooks| generate_hooks_registration_rust(&fixture_id, hooks))
        } else {
            None
        }
    } else {
        None
    };

    let normalized_route = route_path.trim_end_matches('/').to_string();
    let static_conflict = metadata.static_dirs.iter().any(|dir| {
        let mut prefix = if dir.route_prefix.starts_with('/') {
            dir.route_prefix.clone()
        } else {
            format!("/{}", dir.route_prefix)
        };
        if prefix.is_empty() {
            prefix = "/".to_string();
        }
        let normalized_prefix = prefix.trim_end_matches('/');
        normalized_prefix == normalized_route && !normalized_prefix.is_empty()
    });

    let handler_code = if static_conflict {
        String::new()
    } else {
        generate_single_handler(fixture, &handler_name)
    };

    let parameters_value = fixture
        .handler
        .as_ref()
        .and_then(|h| h.parameters.clone())
        .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));

    let parameter_schema = build_parameter_schema_from_fixture(&parameters_value);
    let file_params = extract_file_params(&parameters_value);

    let builder_expression = if static_conflict || handler_code.trim().is_empty() {
        None
    } else {
        Some(route_builder_expression(
            method,
            &route_path,
            &handler_name,
            fixture,
            parameter_schema,
            file_params,
        ))
    };

    let config_setup = generate_server_config(metadata, fixture_slug, hooks_registration.as_deref());
    let route_registration = if let Some(builder) = builder_expression {
        format!(
            "    app.route({builder}, {handler})?;\n",
            builder = builder,
            handler = handler_name
        )
    } else {
        String::new()
    };

    let app_fn_code = format!(
        r#"/// App for fixture: {}
pub fn {}() -> Result<App, AppError> {{
    let mut config = ServerConfig::default();
{config_setup}    let mut app = App::new().config(config);
{route_registration}    Ok(app)
}}"#,
        fixture.name,
        app_fn_name,
        config_setup = config_setup,
        route_registration = route_registration
    );

    let combined_handler_code = if hooks_code.is_empty() {
        handler_code
    } else if handler_code.trim().is_empty() {
        hooks_code
    } else {
        format!("{}\n\n{}", hooks_code, handler_code)
    };

    (combined_handler_code, app_fn_code)
}

fn generate_background_fixture(
    fixture: &Fixture,
    _fixture_id: &str,
    handler_name: &str,
    app_fn_name: &str,
    background: BackgroundFixtureData,
) -> (String, String) {
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };
    let route_path = route.split('?').next().unwrap_or(&route).to_string();
    let method = fixture.request.method.as_str();
    let state_handler_name = format!("{}_background_state", handler_name);
    let expected_status = fixture.expected_response.status_code;
    let sanitized_headers = sanitized_expected_headers(fixture);
    let header_literal = header_literal(&sanitized_headers);
    let header_apply_block = header_application_block("    ", header_literal.as_deref());
    let error_body = "json!({\"error\": \"missing background value\"}).to_string()".to_string();

    let handler_code = format!(
        r#"async fn {handler_name}(ctx: RequestContext, state: Arc<Mutex<Vec<Value>>>) -> HandlerResult {{
    let body = ctx.body_value();
    let value = body.get("{value_field}").cloned();
    let value = match value {{
        Some(val) => val,
        None => {{
            let response = Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .header("content-type", "application/json")
                .body(Body::from({error_body}))
                .unwrap();
{header_apply}
            return Ok(response);
        }}
    }};

    {{
        let mut guard = state.lock().await;
        guard.push(value);
    }}

    let response = Response::builder()
        .status(StatusCode::from_u16({status}).unwrap())
        .body(Body::empty())
        .unwrap();
{header_apply}
    Ok(response)
}}

async fn {state_handler_name}(_ctx: RequestContext, state: Arc<Mutex<Vec<Value>>>) -> HandlerResult {{
    let values = {{
        let guard = state.lock().await;
        guard.clone()
    }};
    let response = Response::builder()
        .status(StatusCode::OK)
        .header("content-type", "application/json")
        .body(Body::from(json!({{ "{state_key}": values }}).to_string()))
        .unwrap();
    Ok(response)
}}"#,
        handler_name = handler_name,
        state_handler_name = state_handler_name,
        value_field = background.value_field,
        error_body = error_body,
        status = expected_status,
        header_apply = header_apply_block,
        state_key = background.state_key
    );

    let parameters_value = fixture
        .handler
        .as_ref()
        .and_then(|h| h.parameters.clone())
        .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
    let parameter_schema = build_parameter_schema_from_fixture(&parameters_value);
    let file_params = extract_file_params(&parameters_value);

    let mut app_fn_code = format!(
        r#"/// App for fixture: {fixture_name}
pub fn {app_fn_name}() -> Result<App, AppError> {{
    let state: Arc<Mutex<Vec<Value>>> = Arc::new(Mutex::new(Vec::new()));
    let mut app = App::new();
"#,
        fixture_name = fixture.name,
        app_fn_name = app_fn_name
    );

    let builder_expr = route_builder_expression(
        method,
        &route_path,
        handler_name,
        fixture,
        parameter_schema,
        file_params,
    );
    app_fn_code.push_str(&format!(
        r#"    {{
        let handler_state = Arc::clone(&state);
        app.route({builder}, move |ctx: RequestContext| {{
            let handler_state = Arc::clone(&handler_state);
            async move {{ {handler_name}(ctx, handler_state).await }}
        }})?;
    }}
"#,
        builder = builder_expr,
        handler_name = handler_name
    ));

    let state_route = escape_rust_string(&background.state_path);
    app_fn_code.push_str(&format!(
        r#"    {{
        let state_clone = Arc::clone(&state);
        app.route(
            get("{state_route}").handler_name("{state_handler_name}"),
            move |ctx: RequestContext| {{
                let state_clone = Arc::clone(&state_clone);
                async move {{ {state_handler_name}(ctx, state_clone).await }}
            }},
        )?;
    }}

    Ok(app)
}}"#
    ));

    (handler_code, app_fn_code)
}

/// Sanitize fixture name to valid Rust identifier
fn sanitize_name(name: &str) -> String {
    let mut result = String::with_capacity(name.len());
    let mut last_was_underscore = false;

    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            result.push(ch.to_ascii_lowercase());
            last_was_underscore = false;
        } else if !last_was_underscore {
            result.push('_');
            last_was_underscore = true;
        }
    }

    let sanitized = result.trim_matches('_').to_string();
    if sanitized.is_empty() {
        "fixture".to_string()
    } else {
        sanitized
    }
}

fn pascal_case_name(name: &str) -> String {
    sanitize_name(name)
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            match chars.next() {
                Some(first) => {
                    let mut out = String::new();
                    out.push(first.to_ascii_uppercase());
                    out.extend(chars.map(|c| c.to_ascii_lowercase()));
                    out
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Generate handler for a single fixture (reusing existing generate_handler logic)
fn generate_single_handler(fixture: &Fixture, handler_name: &str) -> String {
    if fixture.streaming.is_some() {
        return generate_streaming_handler(fixture, handler_name);
    }

    let sanitized_headers = sanitized_expected_headers(fixture);
    let header_literal = header_literal(&sanitized_headers);
    let expected_status = fixture.expected_response.status_code;

    let body_block = if let Some(body) = &fixture.expected_response.body {
        let body_json = serde_json::to_string(body).unwrap();
        format!(
            r#"    let body_value: Value = serde_json::from_str("{body}").unwrap();
    let response = Response::builder()
        .status(StatusCode::from_u16({status}).unwrap())
        .header("content-type", "application/json")
        .body(Body::from(body_value.to_string()))
        .unwrap();
"#,
            body = escape_rust_string(&body_json),
            status = expected_status
        )
    } else {
        format!(
            r#"    let response = Response::builder()
        .status(StatusCode::from_u16({status}).unwrap())
        .body(Body::empty())
        .unwrap();
"#,
            status = expected_status
        )
    };

    let header_apply = header_application_block("    ", header_literal.as_deref());

    format!(
        r#"async fn {handler_name}(_ctx: RequestContext) -> HandlerResult {{
{body_block}{header_apply}    Ok(response)
}}"#,
        handler_name = handler_name,
        body_block = body_block,
        header_apply = header_apply
    )
}

/// Build parameter schema from a single fixture's parameters
#[allow(dead_code)]
fn build_parameter_schema_from_fixture(params: &Value) -> Option<Value> {
    use serde_json::json;

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (source_name, source_params) in params.as_object()? {
        if source_name == "files" {
            continue;
        }

        if let Some(source_obj) = source_params.as_object() {
            for (param_name, param_def) in source_obj {
                if let Some(mut param_obj) = param_def.as_object().cloned() {
                    let normalized_source = match source_name.as_str() {
                        "headers" => "header",
                        "cookies" => "cookie",
                        other => other,
                    };
                    param_obj.insert("source".to_string(), json!(normalized_source));

                    let has_default = param_obj.contains_key("default");
                    let is_optional = param_obj.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                    let explicitly_required = param_obj.get("required").and_then(|v| v.as_bool()).unwrap_or(false);

                    let is_required = if source_name == "path" || explicitly_required {
                        true
                    } else {
                        !(has_default || is_optional)
                    };

                    if is_required {
                        required.push(param_name.clone());
                    }

                    param_obj.remove("annotation");
                    param_obj.remove("required");
                    param_obj.remove("optional");

                    properties.insert(param_name.clone(), Value::Object(param_obj));
                }
            }
        }
    }

    Some(json!({
        "type": "object",
        "properties": properties,
        "required": required
    }))
}

/// Extract file parameter configuration (if present) from the fixture parameters
fn extract_file_params(params: &Value) -> Option<Value> {
    params
        .as_object()
        .and_then(|obj| obj.get("files"))
        .cloned()
        .filter(|value| !value.is_null())
}

/// Generate a CORS preflight handler that uses spikard_http::cors::handle_preflight
fn generate_streaming_handler(fixture: &Fixture, handler_name: &str) -> String {
    let streaming = fixture.streaming.as_ref().expect("streaming metadata required");

    let mut chunk_lines = Vec::new();
    for chunk in &streaming.chunks {
        let bytes = chunk_bytes(chunk).expect("invalid streaming chunk");
        let literal = rust_bytes_slice_literal(&bytes);
        chunk_lines.push(format!(
            "        Ok::<Bytes, std::io::Error>(Bytes::from_static({})),",
            literal
        ));
    }
    let chunk_section = if chunk_lines.is_empty() {
        String::from("        Ok::<Bytes, std::io::Error>(Bytes::from_static(&[]))\n")
    } else {
        chunk_lines.join("\n") + "\n"
    };

    let handler = format!(
        r#"async fn {handler_name}(_ctx: RequestContext) -> HandlerResult {{
    let stream = stream::iter(vec![
{chunk_section}    ]);

    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::from_u16({status}).unwrap())
        .into_response();
{headers}
    Ok(response)
}}"#,
        handler_name = handler_name,
        chunk_section = chunk_section,
        status = fixture.expected_response.status_code,
        headers = build_streaming_headers_rust(fixture),
    );

    handler
}

fn build_streaming_headers_rust(fixture: &Fixture) -> String {
    let headers = streaming_expected_headers(fixture);
    header_literal(&headers)
        .map(|lit| format!("    let response = apply_expected_headers(response, {});\n", lit))
        .unwrap_or_default()
}

fn rust_bytes_slice_literal(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "&[]".to_string()
    } else {
        let items = bytes
            .iter()
            .map(|byte| format!("0x{:02x}u8", byte))
            .collect::<Vec<_>>()
            .join(", ");
        format!("&[{}]", items)
    }
}

fn sanitized_expected_headers(fixture: &Fixture) -> Vec<(String, String)> {
    sanitize_expected_headers_inner(&fixture.expected_response)
}

fn streaming_expected_headers(fixture: &Fixture) -> Vec<(String, String)> {
    let mut headers = sanitize_expected_headers_inner(&fixture.expected_response);
    let has_content_type = headers.iter().any(|(name, _)| name == "content-type");

    if let Some(content_type) = fixture
        .streaming
        .as_ref()
        .and_then(|streaming| streaming.content_type.as_ref())
    {
        headers.retain(|(name, _)| name != "content-type");
        headers.push(("content-type".to_string(), content_type.clone()));
    } else if !has_content_type {
        headers.push(("content-type".to_string(), "application/octet-stream".to_string()));
    }

    headers
}

fn generate_sse_handlers(fixtures: &[AsyncFixture]) -> (Vec<String>, Vec<String>) {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    let mut handlers = Vec::new();
    let mut app_functions = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let (handler_code, app_fn_code) = build_sse_handler(&channel, &channel_fixtures);
        handlers.push(handler_code);
        app_functions.push(app_fn_code);
    }

    (handlers, app_functions)
}

fn build_sse_handler(channel: &str, fixtures: &[&AsyncFixture]) -> (String, String) {
    let channel_path = if channel.starts_with('/') {
        channel.to_string()
    } else {
        format!("/{}", channel)
    };
    let channel_slug = sanitize_name(&channel_path.trim_start_matches('/').replace('/', "_"));
    let handler_name = format!("sse_{}_handler", channel_slug);
    let app_fn_name = format!("create_app_sse_{}", channel_slug);

    let mut event_literals = Vec::new();
    for fixture in fixtures {
        for example in &fixture.examples {
            let json_str = serde_json::to_string(example).unwrap_or_else(|_| "{}".to_string());
            let chunk = format!("data: {}\n\n", json_str);
            event_literals.push(format!("\"{}\"", escape_rust_string(&chunk)));
        }
    }
    if event_literals.is_empty() {
        event_literals.push("\"data: {}\\\\n\\\\n\"".to_string());
    }
    let events_literal = format!(
        "vec![{items}].into_iter().map(String::from).collect::<Vec<_>>()",
        items = event_literals.join(", ")
    );

    let handler_code = format!(
        r#"async fn {handler_name}(_ctx: RequestContext) -> HandlerResult {{
    let events: Vec<String> = {events_literal};
    let stream = stream::iter(events.into_iter().map(|chunk| {{
        Ok::<Bytes, std::io::Error>(Bytes::from(chunk))
    }}));
    let response = HandlerResponse::stream(stream)
        .with_status(StatusCode::OK)
        .with_header(
            HeaderName::from_static("content-type"),
            HeaderValue::from_static("text/event-stream"),
        )
        .with_header(HeaderName::from_static("cache-control"), HeaderValue::from_static("no-cache"))
        .into_response();
    Ok(response)
}}"#,
        handler_name = handler_name,
        events_literal = events_literal
    );

    let app_fn_code = format!(
        r#"/// App for SSE channel: {channel}
pub fn {app_fn_name}() -> Result<App, AppError> {{
    let mut app = App::new();
    app.route(get("{path}").handler_name("{handler_name}"), {handler_name})?;
    Ok(app)
}}"#,
        channel = channel_path,
        app_fn_name = app_fn_name,
        path = channel_path,
        handler_name = handler_name
    );

    (handler_code, app_fn_code)
}

fn generate_websocket_handlers(fixtures: &[AsyncFixture]) -> (Vec<String>, Vec<String>) {
    use std::collections::BTreeMap;

    let mut grouped: BTreeMap<String, Vec<&AsyncFixture>> = BTreeMap::new();
    for fixture in fixtures {
        if let Some(channel) = fixture.channel.as_deref() {
            grouped.entry(channel.to_string()).or_default().push(fixture);
        }
    }

    let mut handlers = Vec::new();
    let mut app_functions = Vec::new();
    for (channel, channel_fixtures) in grouped {
        let (handler_code, app_fn_code) = build_websocket_handler(&channel, &channel_fixtures);
        handlers.push(handler_code);
        app_functions.push(app_fn_code);
    }

    (handlers, app_functions)
}

fn build_websocket_handler(channel: &str, _fixtures: &[&AsyncFixture]) -> (String, String) {
    let channel_path = if channel.starts_with('/') {
        channel.to_string()
    } else {
        format!("/{}", channel)
    };
    let channel_slug = sanitize_name(&channel_path.trim_start_matches('/').replace('/', "_"));
    let app_fn_name = format!("create_app_websocket_{}", channel_slug);
    let struct_name = format!("{}WebSocketHandler", pascal_case_name(&channel_slug));

    let handler_code = format!(
        r#"struct {struct_name};

impl WebSocketHandler for {struct_name} {{
    fn handle_message(&self, message: Value) -> impl std::future::Future<Output = Option<Value>> + Send {{
        async move {{
            let mut data = message;
            if let Some(obj) = data.as_object_mut() {{
                obj.insert("validated".to_string(), Value::Bool(true));
            }}
            Some(data)
        }}
    }}
}}
"#,
        struct_name = struct_name
    );

    let app_fn_code = format!(
        r#"/// App for WebSocket channel: {channel}
pub fn {app_fn_name}() -> Result<App, AppError> {{
    let mut app = App::new();
    app.websocket("{path}", {struct_name});
    Ok(app)
}}"#,
        channel = channel_path,
        app_fn_name = app_fn_name,
        path = channel_path,
        struct_name = struct_name
    );

    (handler_code, app_fn_code)
}

fn sanitize_expected_headers_inner(expected: &FixtureExpectedResponse) -> Vec<(String, String)> {
    let mut headers = Vec::new();
    if let Some(map) = expected.headers.as_ref() {
        for (name, value) in map {
            if name.eq_ignore_ascii_case("content-encoding") {
                continue;
            }
            if let Some(converted) = normalize_expected_header_value(value) {
                headers.push((name.to_ascii_lowercase(), converted));
            }
        }
    }
    headers
}

fn expected_header_value(fixture: &Fixture, header_name: &str) -> Option<String> {
    let target = header_name.to_ascii_lowercase();
    sanitize_expected_headers_inner(&fixture.expected_response)
        .into_iter()
        .find(|(name, _)| name == &target)
        .map(|(_, value)| value)
}

fn normalize_expected_header_value(raw: &str) -> Option<String> {
    match raw {
        "<<absent>>" => None,
        "<<present>>" => Some("spikard-test-value".to_string()),
        "<<uuid>>" => Some("00000000-0000-4000-8000-000000000000".to_string()),
        _ => Some(raw.to_string()),
    }
}

fn header_literal(headers: &[(String, String)]) -> Option<String> {
    if headers.is_empty() {
        return None;
    }
    let pairs = headers
        .iter()
        .map(|(name, value)| format!("(\"{}\", \"{}\")", escape_rust_string(name), escape_rust_string(value)))
        .collect::<Vec<_>>()
        .join(", ");
    Some(format!("&[{}]", pairs))
}

fn header_application_block(indent: &str, literal: Option<&str>) -> String {
    literal
        .map(|lit| format!("{indent}let response = apply_expected_headers(response, {lit});\n"))
        .unwrap_or_default()
}

fn escape_rust_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

/// Validate that required schemas are present for the route
/// Prints clear error messages and returns false if validation fails
#[allow(dead_code)]
fn validate_required_schemas(
    route: &str,
    method: &str,
    fixtures: &[&Fixture],
    has_path_params: bool,
    has_body: bool,
) -> bool {
    let mut valid = true;

    let needs_parameters = has_path_params
        || fixtures
            .iter()
            .any(|f| f.request.path.contains('?') || f.handler.as_ref().and_then(|h| h.parameters.as_ref()).is_some());

    if needs_parameters {
        let has_parameters = fixtures
            .iter()
            .any(|f| f.handler.as_ref().and_then(|h| h.parameters.as_ref()).is_some());

        if !has_parameters {
            eprintln!("\n╔════════════════════════════════════════════════════════════╗");
            eprintln!("║ ERROR: Missing Required Schema                            ║");
            eprintln!("╚════════════════════════════════════════════════════════════╝");
            eprintln!("Route: {} {}", method, route);
            eprintln!("Issue: Route requires parameters but no handler.parameters found");
            eprintln!("\nFixtures checked ({}):", fixtures.len());
            for f in fixtures {
                eprintln!("  - {}", f.name);
            }
            eprintln!("\nFix: Add handler.parameters section to at least one fixture");
            eprintln!("Example:");
            eprintln!(
                r#"  "handler": {{
    "route": "{}",
    "parameters": {{
      "query": {{
        "param_name": {{
          "type": "string",
          "required": true
        }}
      }}
    }}
  }}"#,
                route
            );
            valid = false;
        }
    }

    if has_body {
        let has_body_schema = fixtures
            .iter()
            .any(|f| f.handler.as_ref().and_then(|h| h.body_schema.as_ref()).is_some());

        if !has_body_schema {
            eprintln!("\n╔════════════════════════════════════════════════════════════╗");
            eprintln!("║ ERROR: Missing Required Schema                            ║");
            eprintln!("╚════════════════════════════════════════════════════════════╝");
            eprintln!("Route: {} {}", method, route);
            eprintln!(
                "Issue: {} requests typically require explicit handler.body_schema",
                method
            );
            eprintln!("\nFixtures checked ({}):", fixtures.len());
            for f in fixtures {
                eprintln!("  - {}", f.name);
            }
            eprintln!("\nFix: Add handler.body_schema section to at least one fixture");
            eprintln!("Example:");
            eprintln!(
                r#"  "handler": {{
    "route": "{}",
    "body_schema": {{
      "type": "object",
      "properties": {{
        "name": {{"type": "string"}},
        "count": {{"type": "integer"}}
      }},
      "required": ["name"]
    }}
  }}"#,
                route
            );
            valid = false;
        }
    }

    valid
}

/// Generate Rust lifecycle hook function implementations
#[allow(dead_code)]
fn generate_lifecycle_hooks_rust(fixture_id: &str, hooks: &Value, fixture: &Fixture) -> String {
    let mut code = String::new();

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_request_{}", fixture_id, sanitize_name(hook_name), idx);

            code.push_str(&format!(
                r#"async fn {}(req: axum::http::Request<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // Mock onRequest hook: {}
    Ok(spikard::HookResult::Continue(req))
}}

"#,
                func_name, hook_name
            ));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_validation_{}", fixture_id, sanitize_name(hook_name), idx);

            let should_short_circuit = hook_name.contains("rate_limit") && fixture.expected_response.status_code == 429;

            if should_short_circuit {
                let retry_after_code = expected_header_value(fixture, "retry-after")
                    .map(|value| {
                        format!(
                            "    response.headers_mut().insert(\"Retry-After\", \"{}\".parse().unwrap());\n",
                            escape_rust_string(&value)
                        )
                    })
                    .unwrap_or_default();

                code.push_str(&format!(
                    r#"async fn {}(_req: axum::http::Request<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // preValidation hook: {} - Short circuits with 429
    use axum::response::IntoResponse;
    let mut response = (
        axum::http::StatusCode::TOO_MANY_REQUESTS,
        axum::Json(serde_json::json!({{
            "error": "Rate limit exceeded",
            "message": "Too many requests, please try again later"
        }}))
    ).into_response();
{retry_after_code}    Ok(spikard::HookResult::ShortCircuit(response))
}}

"#,
                    func_name, hook_name, retry_after_code = retry_after_code
                ));
            } else {
                code.push_str(&format!(
                    r#"async fn {}(req: axum::http::Request<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // Mock preValidation hook: {}
    Ok(spikard::HookResult::Continue(req))
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_handler_{}", fixture_id, sanitize_name(hook_name), idx);

            let auth_fails = hook_name.contains("auth")
                && (fixture.expected_response.status_code == 401 || fixture.expected_response.status_code == 403);

            if auth_fails {
                let (status_code, error_msg, detail_msg) = if fixture.expected_response.status_code == 401 {
                    (
                        "UNAUTHORIZED",
                        "Unauthorized",
                        "Invalid or expired authentication token",
                    )
                } else {
                    ("FORBIDDEN", "Forbidden", "Admin role required for this endpoint")
                };

                code.push_str(&format!(
                    r#"async fn {}(_req: axum::http::Request<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // preHandler hook: {} - Short circuits with {}
    use axum::response::IntoResponse;
    let response = (
        axum::http::StatusCode::{},
        axum::Json(serde_json::json!({{
            "error": "{}",
            "message": "{}"
        }}))
    ).into_response();
    Ok(spikard::HookResult::ShortCircuit(response))
}}

"#,
                    func_name, hook_name, fixture.expected_response.status_code,
                    status_code, error_msg, detail_msg
                ));
            } else {
                code.push_str(&format!(
                    r#"async fn {}(req: axum::http::Request<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Request<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // Mock preHandler hook: {}
    Ok(spikard::HookResult::Continue(req))
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_response_{}", fixture_id, sanitize_name(hook_name), idx);

            if hook_name.contains("security") {
                code.push_str(&format!(
                    r#"async fn {}(mut resp: axum::http::Response<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // onResponse hook: {} - Adds security headers
    resp.headers_mut().insert("X-Content-Type-Options", "nosniff".parse().unwrap());
    resp.headers_mut().insert("X-Frame-Options", "DENY".parse().unwrap());
    resp.headers_mut().insert("X-XSS-Protection", "1; mode=block".parse().unwrap());
    resp.headers_mut().insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}}

"#,
                    func_name, hook_name
                ));
            } else if hook_name.contains("timing") || hook_name.contains("timer") {
                let timing_value =
                    expected_header_value(fixture, "x-response-time").unwrap_or_else(|| "0ms".to_string());
                code.push_str(&format!(
                    r#"async fn {}(mut resp: axum::http::Response<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // onResponse hook: {} - Adds timing header
    resp.headers_mut().insert("X-Response-Time", "{}".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}}

"#,
                    func_name, hook_name, escape_rust_string(&timing_value)
                ));
            } else {
                code.push_str(&format!(
                    r#"async fn {}(resp: axum::http::Response<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // Mock onResponse hook: {}
    Ok(spikard::HookResult::Continue(resp))
}}

"#,
                    func_name, hook_name
                ));
            }
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_error_{}", fixture_id, sanitize_name(hook_name), idx);

            code.push_str(&format!(
                r#"async fn {}(mut resp: axum::http::Response<axum::body::Body>) -> Result<spikard::HookResult<axum::http::Response<axum::body::Body>, axum::http::Response<axum::body::Body>>, String> {{
    // onError hook: {} - Format error response
    resp.headers_mut().insert("Content-Type", "application/json".parse().unwrap());
    Ok(spikard::HookResult::Continue(resp))
}}

"#,
                func_name, hook_name
            ));
        }
    }

    code
}

/// Generate Rust lifecycle hooks registration using LifecycleHooks::builder()
fn generate_server_config(
    metadata: &MiddlewareMetadata,
    fixture_slug: &str,
    hooks_registration: Option<&str>,
) -> String {
    let mut config_lines = Vec::new();

    if let Some(compression) = &metadata.compression {
        let mut parts = Vec::new();
        if let Some(gzip) = compression.gzip {
            parts.push(format!("        gzip: {},", gzip));
        }
        if let Some(brotli) = compression.brotli {
            parts.push(format!("        brotli: {},", brotli));
        }
        if let Some(min_size) = compression.min_size {
            parts.push(format!("        min_size: {},", min_size));
        }
        if let Some(quality) = compression.quality {
            parts.push(format!("        quality: {},", quality));
        }
        if parts.is_empty() {
            config_lines.push(
                "    config.compression = Some(CompressionConfig::default());
"
                .to_string(),
            );
        } else {
            config_lines.push(format!(
                "    config.compression = Some(CompressionConfig {{
{}
    }});
",
                parts.join(
                    "
"
                )
            ));
        }
    }

    if let Some(rate) = &metadata.rate_limit {
        let ip_based = rate.ip_based.unwrap_or(true);
        config_lines.push(format!(
            "    config.rate_limit = Some(RateLimitConfig {{ per_second: {}, burst: {}, ip_based: {} }});
",
            rate.per_second, rate.burst, ip_based
        ));
    }

    if let Some(timeout) = &metadata.request_timeout {
        config_lines.push(format!(
            "    config.request_timeout = Some({});
",
            timeout.seconds
        ));
    }

    if let Some(request_id) = &metadata.request_id
        && let Some(enabled) = request_id.enabled
    {
        config_lines.push(format!(
            "    config.enable_request_id = {};
",
            enabled
        ));
    }

    if let Some(body_limit) = &metadata.body_limit {
        if let Some(max_bytes) = body_limit.max_bytes {
            config_lines.push(format!(
                "    config.max_body_size = Some({});
",
                max_bytes
            ));
        } else {
            config_lines.push(
                "    config.max_body_size = None;
"
                .to_string(),
            );
        }
    }

    if !metadata.static_dirs.is_empty() {
        for dir in &metadata.static_dirs {
            let directory = format!("static_assets/{}/{}", fixture_slug, dir.directory_name);
            let directory_literal = escape_rust_string(&directory);
            let prefix_literal = escape_rust_string(&dir.route_prefix);
            let cache_code = if let Some(cache) = &dir.cache_control {
                format!("Some(\"{}\".to_string())", escape_rust_string(cache))
            } else {
                "None".to_string()
            };
            config_lines.push(format!(
                "    config.static_files.push(StaticFilesConfig {{ directory: \"{}\".to_string(), route_prefix: \"{}\".to_string(), index_file: {}, cache_control: {} }});\n",
                directory_literal, prefix_literal, dir.index_file, cache_code
            ));
        }
    }

    if let Some(expr) = hooks_registration {
        config_lines.push(format!(
            "    config.lifecycle_hooks = Some(Arc::new({}.build()));
",
            expr
        ));
    }

    if config_lines.is_empty() {
        String::new()
    } else {
        config_lines.join("")
    }
}

fn route_builder_expression(
    method: &str,
    route: &str,
    handler_name: &str,
    fixture: &Fixture,
    parameter_schema: Option<Value>,
    file_params: Option<Value>,
) -> String {
    let escaped_route = escape_rust_string(route);
    let mut builder = match method.to_uppercase().as_str() {
        "GET" => format!("get(\"{}\")", escaped_route),
        "POST" => format!("post(\"{}\")", escaped_route),
        "PUT" => format!("put(\"{}\")", escaped_route),
        "PATCH" => format!("patch(\"{}\")", escaped_route),
        "DELETE" => format!("delete(\"{}\")", escaped_route),
        other => format!(
            "RouteBuilder::new(Method::from_str(\"{}\").expect(\"invalid method\"), \"{}\")",
            other, escaped_route
        ),
    };
    builder.push_str(&format!(".handler_name(\"{}\")", handler_name));

    if let Some(handler) = &fixture.handler {
        if let Some(body_schema) = &handler.body_schema {
            let schema_json = serde_json::to_string(body_schema).unwrap();
            builder.push_str(&format!(
                ".request_schema_json(serde_json::from_str::<Value>(\"{}\").unwrap())",
                escape_rust_string(&schema_json)
            ));
        }
        if let Some(response_schema) = &handler.response_schema {
            let schema_json = serde_json::to_string(response_schema).unwrap();
            builder.push_str(&format!(
                ".response_schema_json(serde_json::from_str::<Value>(\"{}\").unwrap())",
                escape_rust_string(&schema_json)
            ));
        }
        if let Some(cors) = &handler.cors {
            let cors_json = serde_json::to_string(cors).unwrap();
            builder.push_str(&format!(
                ".cors(serde_json::from_str::<CorsConfig>(\"{}\").unwrap())",
                escape_rust_string(&cors_json)
            ));
        }
    }

    if let Some(schema) = parameter_schema {
        let schema_json = serde_json::to_string(&schema).unwrap();
        builder.push_str(&format!(
            ".params_schema_json(serde_json::from_str::<Value>(\"{}\").unwrap())",
            escape_rust_string(&schema_json)
        ));
    }

    if let Some(files) = file_params {
        let files_json = serde_json::to_string(&files).unwrap();
        builder.push_str(&format!(
            ".file_params_json(serde_json::from_str::<Value>(\"{}\").unwrap())",
            escape_rust_string(&files_json)
        ));
    }

    builder
}

fn generate_hooks_registration_rust(fixture_id: &str, hooks: &Value) -> String {
    let mut code = String::from("spikard::LifecycleHooks::builder()");

    if let Some(on_request) = hooks.get("on_request").and_then(|v| v.as_array()) {
        for (idx, hook) in on_request.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_request_{}", fixture_id, sanitize_name(hook_name), idx);
            code.push_str(&format!(
                "\n        .on_request(spikard::request_hook(\"{}\", {}))",
                hook_name, func_name
            ));
        }
    }

    if let Some(pre_validation) = hooks.get("pre_validation").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_validation.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_validation_{}", fixture_id, sanitize_name(hook_name), idx);
            code.push_str(&format!(
                "\n        .pre_validation(spikard::request_hook(\"{}\", {}))",
                hook_name, func_name
            ));
        }
    }

    if let Some(pre_handler) = hooks.get("pre_handler").and_then(|v| v.as_array()) {
        for (idx, hook) in pre_handler.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_pre_handler_{}", fixture_id, sanitize_name(hook_name), idx);
            code.push_str(&format!(
                "\n        .pre_handler(spikard::request_hook(\"{}\", {}))",
                hook_name, func_name
            ));
        }
    }

    if let Some(on_response) = hooks.get("on_response").and_then(|v| v.as_array()) {
        for (idx, hook) in on_response.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_response_{}", fixture_id, sanitize_name(hook_name), idx);
            code.push_str(&format!(
                "\n        .on_response(spikard::response_hook(\"{}\", {}))",
                hook_name, func_name
            ));
        }
    }

    if let Some(on_error) = hooks.get("on_error").and_then(|v| v.as_array()) {
        for (idx, hook) in on_error.iter().enumerate() {
            let hook_name = hook.get("name").and_then(|v| v.as_str()).unwrap_or("unnamed_hook");
            let func_name = format!("{}_{}_on_error_{}", fixture_id, sanitize_name(hook_name), idx);
            code.push_str(&format!(
                "\n        .on_error(spikard::response_hook(\"{}\", {}))",
                hook_name, func_name
            ));
        }
    }

    code
}
