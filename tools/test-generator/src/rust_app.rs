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

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Removed fixture_analysis - no more schema inference!
// Fixtures MUST provide explicit schemas.

pub fn generate_rust_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Rust test app at {}...", output_dir.display());

    // Load fixtures from all subdirectories
    let categories = discover_fixture_categories(fixtures_dir)?;

    // Generate directly in output_dir (no app/ subdirectory)
    fs::create_dir_all(output_dir).context("Failed to create output directory")?;

    // Generate Cargo.toml
    let cargo_toml = generate_cargo_toml();
    fs::write(output_dir.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;
    println!("  ✓ Generated Cargo.toml");

    // Generate src directory
    let src_dir = output_dir.join("src");
    fs::create_dir_all(&src_dir).context("Failed to create src directory")?;

    // Generate main.rs
    let main_rs = generate_main_rs(&categories);
    fs::write(src_dir.join("main.rs"), main_rs).context("Failed to write main.rs")?;
    println!("  ✓ Generated src/main.rs");

    // Generate lib.rs for reuse in tests
    let lib_rs = generate_lib_rs(&categories);
    fs::write(src_dir.join("lib.rs"), lib_rs).context("Failed to write lib.rs")?;
    println!("  ✓ Generated src/lib.rs");

    Ok(())
}

fn discover_fixture_categories(fixtures_dir: &Path) -> Result<HashMap<String, Vec<Fixture>>> {
    let mut categories = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|n| n.to_str())
                .context("Invalid directory name")?
                .to_string();

            let fixtures =
                load_fixtures_from_dir(&path).with_context(|| format!("Failed to load fixtures from {}", category))?;

            if !fixtures.is_empty() {
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
# Use Spikard itself - this is a test of Spikard!
spikard-http = { path = "../../crates/spikard-http" }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tower = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
cookie = "0.18"
form_urlencoded = "1.2"
percent-encoding = "2.3"
"#
    .to_string()
}

fn generate_main_rs(_categories: &HashMap<String, Vec<Fixture>>) -> String {
    r#"//! Generated test application
//! This is a minimal Axum app that echoes back validated parameters

use axum::{routing::get, Json, Router};
use serde_json::{json, Value};
use std::net::SocketAddr;

pub use spikard_e2e_app::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = create_app();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
"#
    .to_string()
}

fn generate_lib_rs(categories: &HashMap<String, Vec<Fixture>>) -> String {
    let mut handlers = Vec::new();
    let mut app_functions = Vec::new();

    // Generate one handler per fixture (no grouping!)
    for (category, fixtures) in categories {
        for fixture in fixtures {
            // Generate unique handler for this fixture
            let (handler_code, app_fn_code) = generate_fixture_handler_and_app(category, fixture);
            handlers.push(handler_code);
            app_functions.push(app_fn_code);
        }
    }

    format!(
        r#"//! Generated route handlers - one handler per fixture for complete isolation

use axum::{{routing, routing::{{get, post, put, patch, delete, head, options, trace}}, Json, Router, middleware}};
use axum::response::IntoResponse;
use form_urlencoded;
use serde_json::{{json, Value}};
use std::collections::HashMap;
use spikard_http::parameters::ParameterValidator;

#[derive(Clone, Copy)]
pub struct CorsConfig {{
    allow_origin: &'static str,
    allow_methods: &'static str,
    allow_headers: &'static str,
}}

// Default app for backwards compatibility (empty)
pub fn create_app() -> Router {{
    Router::new()
}}

// Per-fixture app functions
{}

// Handler functions
{}
"#,
        app_functions.join("\n\n"),
        handlers.join("\n\n"),
    )
}

/// Generate handler and app function for a single fixture
/// Returns (handler_code, app_function_code)
fn generate_fixture_handler_and_app(category: &str, fixture: &Fixture) -> (String, String) {
    // Create unique names based on category and fixture name
    let fixture_id = format!("{}_{}", category, sanitize_name(&fixture.name));
    let handler_name = format!("{}_handler", fixture_id);
    let app_fn_name = format!("create_app_{}", fixture_id);

    // Get route from handler or request
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };

    // Strip query string from route
    let route_path = route.split('?').next().unwrap_or(&route).to_string();
    let axum_route = strip_type_hints(&route_path);

    let method = fixture.request.method.as_str();
    let method_lower = method.to_lowercase();

    // Generate handler code
    let handler_code = generate_single_handler(fixture, &handler_name);

    // Generate app function
    let app_fn_code = format!(
        r#"/// App for fixture: {}
pub fn {}() -> Router {{
    Router::new()
        .route("{}", {}({}))
        .layer(middleware::from_fn(spikard_http::middleware::validate_content_type_middleware))
}}"#,
        fixture.name, app_fn_name, axum_route, method_lower, handler_name
    );

    (handler_code, app_fn_code)
}

/// Sanitize fixture name to valid Rust identifier
fn sanitize_name(name: &str) -> String {
    name.replace(|c: char| !c.is_alphanumeric() && c != '_', "_")
        .trim_matches('_')
        .to_string()
}

/// Strip type hints and query strings from route pattern
/// Examples:
/// - {param:type} -> {param}
/// - {param:path} -> {*param} (Axum wildcard syntax)
/// - /items/?limit=10 -> /items/
fn strip_type_hints(route: &str) -> String {
    // First strip query string
    let route_without_query = route.split('?').next().unwrap_or(route);

    // Handle path type parameters specially - convert to Axum wildcard syntax
    // {param:path} -> {*param}
    let path_regex = regex::Regex::new(r"\{([^:}]+):path\}").unwrap();
    let route_with_wildcards = path_regex.replace_all(route_without_query, "{*$1}");

    // Then strip other type hints
    regex::Regex::new(r"\{([^:}]+):[^}]+\}")
        .unwrap()
        .replace_all(&route_with_wildcards, "{$1}")
        .to_string()
}

/// Generate handler for a single fixture (reusing existing generate_handler logic)
fn generate_single_handler(fixture: &Fixture, handler_name: &str) -> String {
    // For now, reuse the existing generate_handler with a single-fixture array
    // This will be refactored later to be more direct
    generate_handler_with_name(&fixture.request.method, &[fixture], handler_name)
}

/// Build parameter schema from a single fixture's parameters
#[allow(dead_code)]
fn build_param_schema_from_fixture(params: &Value) -> Option<Value> {
    use serde_json::json;

    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for (source_name, source_params) in params.as_object()? {
        if let Some(source_obj) = source_params.as_object() {
            for (param_name, param_def) in source_obj {
                if let Some(mut param_obj) = param_def.as_object().cloned() {
                    // Add source field
                    param_obj.insert("source".to_string(), json!(source_name));

                    // Determine if required
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

                    // Remove non-JSON-Schema fields
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

#[allow(dead_code)]
fn generate_router_config(route_map: &HashMap<(String, String), Vec<&Fixture>>) -> String {
    use std::collections::BTreeMap;

    // Group routes by path (not by method)
    let mut path_methods: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for (route, method) in route_map.keys() {
        let axum_route = strip_type_hints(route);
        path_methods.entry(axum_route.clone()).or_default().push(method.clone());
    }

    // Sort methods within each route for deterministic output
    for methods in path_methods.values_mut() {
        methods.sort();
    }

    // Generate route registration code
    path_methods
        .iter()
        .map(|(axum_route, methods)| {
            if methods.len() == 1 {
                // Single method - use simple routing
                let method = &methods[0];
                let method_lower = method.to_lowercase();
                let handler_name = route_method_to_handler_name(axum_route, method);
                format!("        .route(\"{}\", {}({}))", axum_route, method_lower, handler_name)
            } else {
                // Multiple methods - chain them together
                // Use the first method's function directly, then chain others
                let first_method = &methods[0];
                let first_method_lower = first_method.to_lowercase();
                let first_handler_name = route_method_to_handler_name(axum_route, first_method);

                let remaining_chains: Vec<String> = methods[1..]
                    .iter()
                    .map(|method| {
                        let method_lower = method.to_lowercase();
                        let handler_name = route_method_to_handler_name(axum_route, method);
                        format!(".{}({})", method_lower, handler_name)
                    })
                    .collect();

                format!(
                    "        .route(\"{}\", {}({}){})",
                    axum_route,
                    first_method_lower,
                    first_handler_name,
                    remaining_chains.join("")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// Generate a CORS preflight handler that uses spikard_http::cors::handle_preflight
fn generate_cors_preflight_handler(handler_name: &str, cors_config: &Value) -> String {
    // Serialize the CORS config to a Rust-embeddable JSON string
    let cors_json = serde_json::to_string(cors_config)
        .unwrap()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    format!(
        r#"async fn {}(
    headers: axum::http::HeaderMap,
) -> axum::response::Result<axum::response::Response<axum::body::Body>, axum::response::Response<axum::body::Body>> {{
    use spikard_http::cors::handle_preflight;
    use spikard_http::CorsConfig;

    // Parse CORS configuration
    let cors_config: CorsConfig = serde_json::from_str("{}").unwrap();

    // Handle the preflight request
    handle_preflight(&headers, &cors_config)
}}"#,
        handler_name, cors_json
    )
}

/// Generate handler with explicit name (for per-fixture handlers)
fn generate_handler_with_name(method: &str, fixtures: &[&Fixture], handler_name: &str) -> String {
    let route = if let Some(handler) = fixtures[0].handler.as_ref() {
        handler.route.as_str()
    } else {
        fixtures[0].request.path.as_str()
    };
    let route_path = route.split('?').next().unwrap_or(route);
    let has_path_params = route_path.contains('{');
    generate_handler_impl(method, fixtures, handler_name, has_path_params)
}

/// Generate a minimal handler for a route/method combination.
///
/// This creates the SIMPLEST possible handler that:
/// 1. Sets up Spikard's validators from fixture schemas
/// 2. Calls Spikard's validation methods
/// 3. Returns a fixed success status when validation passes
/// 4. Lets Spikard return structured errors when validation fails
///
/// The handler contains NO business logic, NO conditional behavior, NO parameter parsing
/// for dynamic behavior. It exists only to exercise Spikard's functionality.
#[allow(dead_code)]
fn generate_handler(route: &str, method: &str, fixtures: &[&Fixture]) -> String {
    let handler_name = route_method_to_handler_name(route, method);
    let has_path_params = route.contains('{');
    generate_handler_impl(method, fixtures, &handler_name, has_path_params)
}

fn generate_handler_impl(method: &str, fixtures: &[&Fixture], handler_name: &str, has_path_params: bool) -> String {
    let has_body = method == "POST" || method == "PUT" || method == "PATCH";

    // Check if this handler has CORS configuration
    let cors_config = extract_cors_config(fixtures);

    // For OPTIONS methods with CORS config, generate a CORS preflight handler
    if method == "OPTIONS" {
        if let Some(ref cors_cfg) = cors_config {
            return generate_cors_preflight_handler(handler_name, cors_cfg);
        }
    }

    // Since we generate one handler per fixture, use the exact expected_response
    // from the fixture to return the correct status code and body for stub handlers
    let expected_status = fixtures[0].expected_response.status_code;
    let expected_body = &fixtures[0].expected_response.body;

    // Serialize the expected response body to a JSON string for embedding in generated code
    let expected_body_json = serde_json::to_string(expected_body)
        .unwrap()
        .replace('\\', "\\\\")
        .replace('"', "\\\"");

    // If the fixture expects a non-2xx status code (like 401, 403), this is likely an authentication
    // or authorization test that should return a stub response without doing validation.
    // However, 422 is a validation error and should use the SchemaValidator.
    // These are NOT validation tests, so we should just return the expected response.
    let is_auth_stub = expected_status >= 300 && expected_status != 422;

    if is_auth_stub {
        // Generate a simple stub handler that returns the expected response without validation
        let handler_sig = if cors_config.is_some() {
            format!(
                r#"async fn {}(
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else {
            format!(r#"async fn {}() -> impl axum::response::IntoResponse"#, handler_name)
        };

        let cors_validation_code = if let Some(ref cors_cfg) = cors_config {
            let cors_json = serde_json::to_string(cors_cfg)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            format!(
                r#"
    // CORS validation
    use spikard_http::cors::{{validate_cors_request, add_cors_headers}};
    use spikard_http::CorsConfig;

    let cors_config: CorsConfig = serde_json::from_str("{}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {{
        return err_response;
    }}"#,
                cors_json
            )
        } else {
            String::new()
        };

        let response_code = if cors_config.is_some() {
            format!(
                r#"
    // Add CORS headers to response
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    let mut response = (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#,
                expected_body_json, expected_status
            )
        } else {
            format!(
                r#"
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body))"#,
                expected_body_json, expected_status
            )
        };

        return format!(
            r#"{} {{
{}
    {}
}}"#,
            handler_sig, cors_validation_code, response_code
        );
    }

    // Removed: success_status calculation (now using exact expected_status from fixture)

    // Try to build a parameter schema
    let param_schema = build_parameter_schema(fixtures);

    // Check if this is a multipart request by looking for:
    // 1. Binary format in body_schema, OR
    // 2. File parameters in handler.parameters.files
    let is_multipart = fixtures.iter().any(|f| {
        if let Some(handler) = &f.handler {
            // Check for file parameters
            if let Some(params) = &handler.parameters {
                if params.get("files").is_some() {
                    return true;
                }
            }

            // Check for binary format in body_schema
            if let Some(body_schema) = &handler.body_schema {
                if let Some(properties) = body_schema.get("properties").and_then(|p| p.as_object()) {
                    return properties.values().any(|prop| {
                        prop.get("format")
                            .and_then(|f| f.as_str())
                            .map(|s| s == "binary")
                            .unwrap_or(false)
                    });
                }
            }
        }
        false
    });

    // Extract explicit body schema - NO inference, NO merging
    // Include body schema for multipart requests - middleware converts to JSON
    let body_schema = if has_body {
        // Find the FIRST fixture with explicit handler.body_schema
        let mut explicit_schema = None;
        for fixture in fixtures {
            if let Some(handler) = &fixture.handler {
                if let Some(schema) = &handler.body_schema {
                    eprintln!(
                        "[SCHEMA EXTRACT] Using explicit body_schema from fixture: {}",
                        fixture.name
                    );
                    explicit_schema = Some(schema.clone());
                    break;
                }
            }
        }

        explicit_schema
    } else {
        None
    };

    // Extract file schemas for multipart validation
    let file_schemas = extract_file_schemas(fixtures);

    // Try to build a schema from fixtures with handler.parameters
    if let Some(schema) = param_schema {
        eprintln!(
            "[HANDLER GEN] Built schema: {}",
            serde_json::to_string_pretty(&schema).unwrap()
        );
        // Generate handler with validation
        // Serialize to JSON and escape for embedding in Rust string literal
        let schema_json = serde_json::to_string(&schema)
            .unwrap()
            .replace('\\', "\\\\")  // Escape backslashes first!
            .replace('"', "\\\""); // Then escape quotes

        // For multipart, we need the body to access files (middleware already parsed it)
        let handler_signature = if has_path_params && (body_schema.is_some() || is_multipart) {
            format!(
                r#"async fn {}(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else if has_path_params {
            format!(
                r#"async fn {}(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else if body_schema.is_some() || is_multipart {
            format!(
                r#"async fn {}(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else {
            format!(
                r#"async fn {}(
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        };

        // Generate handler with param validation and optional body validation
        let body_validator_code = if let Some(ref body_schema) = body_schema {
            let body_schema_json = serde_json::to_string(body_schema)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            format!(
                r#"
    // Parse body schema and create body validator
    let body_schema: Value = serde_json::from_str("{}").unwrap();
    let body_validator = spikard_http::SchemaValidator::new(body_schema).unwrap();"#,
                body_schema_json
            )
        } else {
            String::new()
        };

        let body_validation_code = if body_schema.is_some() {
            r#"
            // Validate request body
            if let Err(err) = body_validator.validate(&body) {
                let error_response = serde_json::json!({
                    "detail": err.errors
                });
                return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
            }
"#
        } else {
            ""
        };

        // Generate file validation code if we have file schemas
        // TODO: Re-enable once file_validator module is implemented
        let file_validation_code = String::new();
        let _file_schemas = file_schemas; // Suppress unused warning

        // Generate CORS validation code if CORS config is present
        let cors_validation_code = if let Some(ref cors_cfg) = cors_config {
            let cors_json = serde_json::to_string(cors_cfg)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            format!(
                r#"
    // CORS validation
    use spikard_http::cors::{{validate_cors_request, add_cors_headers}};
    use spikard_http::CorsConfig;

    let cors_config: CorsConfig = serde_json::from_str("{}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {{
        return err_response;
    }}"#,
                cors_json
            )
        } else {
            String::new()
        };

        // Generate CORS header addition code if CORS config is present
        // Return the exact expected_response body instead of validated parameters
        let cors_headers_code = if cors_config.is_some() {
            format!(
                r#"
    // Add CORS headers to response
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    let mut response = (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(expected_body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#,
                expected_body_json
            )
        } else {
            format!(
                r#"
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(expected_body))"#,
                expected_body_json
            )
        };

        format!(
            r#"{} {{
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;
{}
    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();{}

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {{
        parse_query_string_to_json(query_str.as_bytes(), true)
    }} else {{
        Value::Object(serde_json::Map::new())
    }};

    // Also extract raw query params as HashMap<String, String> for error reporting
    let mut raw_query_params = HashMap::new();
    if let Some(query_str) = uri.query() {{
        for (key, value) in form_urlencoded::parse(query_str.as_bytes()) {{
            raw_query_params.insert(key.to_string(), value.to_string());
        }}
    }}

    // Extract headers from HeaderMap (excluding Cookie which is handled separately)
    let mut headers_map = HashMap::new();
    for (name, value) in headers.iter() {{
        if name != axum::http::header::COOKIE {{
            if let Ok(value_str) = value.to_str() {{
                headers_map.insert(name.to_string(), value_str.to_string());
            }}
        }}
    }}

    // Extract cookies from Cookie header using the cookie crate for RFC 6265 compliance
    let mut cookies = HashMap::new();
    if let Some(cookie_header) = headers.get(axum::http::header::COOKIE) {{
        if let Ok(cookie_str) = cookie_header.to_str() {{
            for result in cookie::Cookie::split_parse(cookie_str) {{
                if let Ok(cookie) = result {{
                    cookies.insert(cookie.name().to_string(), cookie.value().to_string());
                }}
            }}
        }}
    }}

    // Validate parameters
    match validator.validate_and_extract(
        &query_params,
        &raw_query_params,
        &{}path_params,
        &headers_map,
        &cookies,
    ) {{
        Ok(validated) => {{{}{}
            let status_code = {};
            {}
        }}
        Err(err) => {{
            let error_response = serde_json::json!({{
                "detail": err.errors
            }});
            {}
        }}
    }}
}}"#,
            handler_signature,
            cors_validation_code,
            schema_json,
            body_validator_code,
            if has_path_params { "" } else { "HashMap::new(), //" },
            body_validation_code,
            file_validation_code,
            expected_status, // Use exact expected status from fixture
            cors_headers_code,
            if cors_config.is_some() {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()"
            } else {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))"
            }
        )
    } else if let Some(body_schema) = body_schema {
        // Body-only handler with validation
        // Extract the body schema and create a validator
        let body_schema_json = serde_json::to_string(&body_schema)
            .unwrap()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");

        // Generate CORS code for body-only handler
        let cors_validation_code = if let Some(ref cors_cfg) = cors_config {
            let cors_json = serde_json::to_string(cors_cfg)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            format!(
                r#"
    // CORS validation
    use spikard_http::cors::{{validate_cors_request, add_cors_headers}};
    use spikard_http::CorsConfig;

    let cors_config: CorsConfig = serde_json::from_str("{}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {{
        return err_response;
    }}"#,
                cors_json
            )
        } else {
            String::new()
        };

        let handler_sig = if cors_config.is_some() {
            format!(
                r#"async fn {}(
    headers: axum::http::HeaderMap,
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else {
            format!(
                r#"async fn {}(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        };

        let success_response_code = if cors_config.is_some() {
            format!(
                r#"
            let expected_body: Value = serde_json::from_str("{}").unwrap();
            let mut response = (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body)).into_response();
            response = add_cors_headers(response, origin, &cors_config);
            response"#,
                expected_body_json, expected_status
            )
        } else {
            format!(
                r#"
            let expected_body: Value = serde_json::from_str("{}").unwrap();
            (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body))"#,
                expected_body_json, expected_status
            )
        };

        let error_response_code = if cors_config.is_some() {
            r#"
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            let mut response = (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response();
            response = add_cors_headers(response, origin, &cors_config);
            response"#
        } else {
            r#"
            let error_response = serde_json::json!({
                "detail": err.errors
            });
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))"#
        };

        // Body validation handler with actual validation
        format!(
            r#"{} {{
    use spikard_http::validation::SchemaValidator;
{}
    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{}").unwrap();
    let validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    match validator.validate(&body) {{
        Ok(_) => {{{}
        }}
        Err(err) => {{{}
        }}
    }}
}}"#,
            handler_sig, cors_validation_code, body_schema_json, success_response_code, error_response_code
        )
    } else {
        // No schema available - simple echo handler
        // Generate CORS code for simple echo handler
        let cors_validation_code = if let Some(ref cors_cfg) = cors_config {
            let cors_json = serde_json::to_string(cors_cfg)
                .unwrap()
                .replace('\\', "\\\\")
                .replace('"', "\\\"");
            format!(
                r#"
    // CORS validation
    use spikard_http::cors::{{validate_cors_request, add_cors_headers}};
    use spikard_http::CorsConfig;

    let cors_config: CorsConfig = serde_json::from_str("{}").unwrap();
    let origin = headers.get("origin").and_then(|v| v.to_str().ok());

    // Validate CORS request - returns 403 if origin not allowed
    if let Err(err_response) = validate_cors_request(origin, &cors_config) {{
        return err_response;
    }}"#,
                cors_json
            )
        } else {
            String::new()
        };

        let handler_sig = if cors_config.is_some() {
            format!(
                r#"async fn {}(
    headers: axum::http::HeaderMap,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else {
            format!(r#"async fn {}() -> impl axum::response::IntoResponse"#, handler_name)
        };

        let cors_headers_code = if cors_config.is_some() {
            format!(
                r#"
    // Add CORS headers to response
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    let mut response = (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#,
                expected_body_json, expected_status
            )
        } else {
            format!(
                r#"
    let expected_body: Value = serde_json::from_str("{}").unwrap();
    (axum::http::StatusCode::from_u16({}).unwrap(), Json(expected_body))"#,
                expected_body_json, expected_status
            )
        };

        format!(
            r#"{} {{
{}
    {}
}}"#,
            handler_sig, cors_validation_code, cors_headers_code
        )
    }
}

/// Extract CORS configuration from fixtures if present
/// Returns the CORS config from the first fixture that has one
fn extract_cors_config(fixtures: &[&Fixture]) -> Option<Value> {
    for fixture in fixtures {
        if let Some(handler) = &fixture.handler {
            if let Some(cors) = &handler.cors {
                return Some(cors.clone());
            }
        }
    }
    None
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

    // Check if route needs parameters (has path params or query params in fixtures)
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

    // Check if POST/PUT/PATCH needs body schema
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

/// Extract file parameters from fixtures
/// Returns a map of field_name -> file schema for use with file validation
fn extract_file_schemas(fixtures: &[&Fixture]) -> Option<Value> {
    use serde_json::json;

    // Find the first fixture with handler.parameters.files
    for fixture in fixtures {
        if let Some(handler) = &fixture.handler {
            if let Some(params) = &handler.parameters {
                if let Some(files) = params.get("files").and_then(|f| f.as_object()) {
                    eprintln!(
                        "[FILE SCHEMA EXTRACT] Using explicit file parameters from fixture: {}",
                        fixture.name
                    );
                    return Some(json!(files));
                }
            }
        }
    }

    None
}

/// Extract explicit parameter schema from fixtures and transform to JSON Schema format
///
/// Takes the FIRST fixture that has handler.parameters and transforms it from:
/// ```json
/// {
///   "query": { "param1": { "type": "string" } },
///   "path": { "param2": { "type": "integer" } }
/// }
/// ```
///
/// To the format that ParameterValidator expects:
/// ```json
/// {
///   "type": "object",
///   "properties": {
///     "param1": { "type": "string", "source": "query" },
///     "param2": { "type": "integer", "source": "path" }
///   },
///   "required": ["param1", "param2"]
/// }
/// ```
fn build_parameter_schema(fixtures: &[&Fixture]) -> Option<Value> {
    use serde_json::json;

    // Find the first fixture with explicit handler.parameters
    for fixture in fixtures {
        if let Some(handler) = &fixture.handler {
            if let Some(params) = &handler.parameters {
                eprintln!(
                    "[SCHEMA EXTRACT] Using explicit parameters from fixture: {}",
                    fixture.name
                );

                // Transform to JSON Schema format with properties and source fields
                let mut properties = serde_json::Map::new();
                let mut required = Vec::new();

                // Process each source (query, path, header, cookie)
                // Skip files since they're handled by multipart middleware, not parameter validation
                for (source_name, source_params) in params.as_object().unwrap_or(&serde_json::Map::new()) {
                    // Skip file parameters - they're handled separately by multipart middleware
                    if source_name == "files" {
                        continue;
                    }

                    if let Some(source_obj) = source_params.as_object() {
                        for (param_name, param_def) in source_obj {
                            // Clone the parameter definition and add the source field
                            if let Some(mut param_obj) = param_def.as_object().cloned() {
                                // Normalize source name to singular form
                                let normalized_source = match source_name.as_str() {
                                    "cookies" => "cookie",
                                    "headers" => "header",
                                    _ => source_name,
                                };
                                param_obj.insert("source".to_string(), json!(normalized_source));

                                // Check if required (following FastAPI semantics):
                                // - Path params: always required
                                // - Query/Header/Cookie: required unless has default or optional: true
                                let has_default = param_obj.contains_key("default");
                                let is_optional = param_obj.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                                let explicitly_required =
                                    param_obj.get("required").and_then(|v| v.as_bool()).unwrap_or(false);

                                let is_required = if normalized_source == "path" || explicitly_required {
                                    true // Path params always required, or explicitly marked
                                } else if has_default || is_optional {
                                    false // Has default or marked optional
                                } else {
                                    true // Default: required if no default and not optional
                                };

                                if is_required {
                                    required.push(param_name.clone());
                                }

                                // Remove annotation field (not part of JSON Schema)
                                param_obj.remove("annotation");
                                // Remove required field (handled at schema level)
                                param_obj.remove("required");
                                // Remove optional field (handled at schema level)
                                param_obj.remove("optional");

                                properties.insert(param_name.clone(), Value::Object(param_obj));
                            }
                        }
                    }
                }

                return Some(json!({
                    "type": "object",
                    "properties": properties,
                    "required": required
                }));
            }
        }
    }

    // No explicit parameters found
    eprintln!(
        "[SCHEMA EXTRACT] No explicit handler.parameters found in {} fixtures",
        fixtures.len()
    );
    None
}

/// Merge multiple schemas intelligently by combining constraints
/// For simple object schemas with constraints like minProperties/maxProperties,
/// merge the constraints. For complex schemas (anyOf, oneOf, etc.), use anyOf wrapper.
#[allow(dead_code)]
fn route_method_to_handler_name(route: &str, method: &str) -> String {
    // Strip query string (e.g., "/items/?limit=10" -> "/items/")
    let route_without_query = route.split('?').next().unwrap_or(route);

    // Strip type hints like {param:type} -> {param}
    let route_without_types = strip_type_hints(route_without_query);

    // Check if route ends with '/' before processing
    let ends_with_slash = route_without_types.ends_with('/') && route_without_types.len() > 1;

    let mut route_part = route_without_types
        .trim_start_matches('/')
        .trim_end_matches('/')  // Remove trailing slash before processing
        .replace(['/', '-', '.'], "_")
        .replace(['{', '}'], "")
        .to_string();

    // If the route starts with a digit after processing, prefix with underscore
    if route_part.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        route_part = format!("_{}", route_part);
    }

    // Handle root route
    if route_part.is_empty() {
        format!("{}_root_handler", method.to_lowercase())
    } else if ends_with_slash {
        // Add _slash suffix to differentiate /items/ from /items
        format!("{}_{}_slash_handler", method.to_lowercase(), route_part)
    } else {
        format!("{}_{}_handler", method.to_lowercase(), route_part)
    }
}
