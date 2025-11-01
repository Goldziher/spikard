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

    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create app directory")?;

    // Generate Cargo.toml
    let cargo_toml = generate_cargo_toml();
    fs::write(app_dir.join("Cargo.toml"), cargo_toml).context("Failed to write Cargo.toml")?;
    println!("  ✓ Generated Cargo.toml");

    // Generate src directory
    let src_dir = app_dir.join("src");
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

[lib]
name = "spikard_e2e_app"
path = "src/lib.rs"

[[bin]]
name = "spikard-e2e-app"
path = "src/main.rs"

[dependencies]
# Use Spikard itself - this is a test of Spikard!
spikard-http = { path = "../../../crates/spikard-http" }
axum = "0.8"
tokio = { version = "1", features = ["full"] }
tower = "0.5"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
cookie = "0.18"
form_urlencoded = "1.2"
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
    let mut routes = Vec::new();

    // Collect all unique routes, but PRIORITIZE non-validation_errors categories
    // validation_errors fixtures use generic schemas just to test error reporting
    let mut route_map: HashMap<(String, String), Vec<&Fixture>> = HashMap::new();

    for (category, fixtures) in categories {
        // Skip validation_errors category when building handler schemas
        // These fixtures are designed to test error responses, not define the handler schema
        if category == "validation_errors" {
            continue;
        }

        for fixture in fixtures {
            // Use handler.route if available, otherwise fall back to request.path
            // ALWAYS strip query strings - routes should group by path only
            let route_with_query = if let Some(handler) = &fixture.handler {
                handler.route.clone()
            } else {
                fixture.request.path.clone()
            };

            // Strip query string to ensure all fixtures for the same path are grouped together
            // E.g., "/items/?limit=10" and "/items/" both become "/items/"
            let route = route_with_query
                .split('?')
                .next()
                .unwrap_or(&route_with_query)
                .to_string();

            let method = fixture.request.method.clone();
            route_map.entry((route, method)).or_default().push(fixture);
        }
    }

    // Generate handlers
    for ((route, method), fixtures) in &route_map {
        let handler = generate_handler(route, method, fixtures);
        routes.push(handler);
    }

    format!(
        r#"//! Generated route handlers

use axum::{{routing, routing::{{get, post, put, patch, delete, head, options, trace}}, Json, Router, middleware}};
use axum::response::IntoResponse;
use form_urlencoded;
use serde_json::{{json, Value}};
use std::collections::HashMap;
use spikard_http::parameters::ParameterValidator;

pub fn create_app() -> Router {{
    Router::new()
{}
        .layer(middleware::from_fn(spikard_http::middleware::validate_content_type_middleware))
}}

{}
"#,
        generate_router_config(&route_map),
        routes.join("\n\n")
    )
}

/// Strip type hints and query strings from route pattern
/// Examples:
/// - {param:type} -> {param}
/// - /items/?limit=10 -> /items/
fn strip_type_hints(route: &str) -> String {
    // First strip query string
    let route_without_query = route.split('?').next().unwrap_or(route);

    // Then strip type hints
    regex::Regex::new(r"\{([^:}]+):[^}]+\}")
        .unwrap()
        .replace_all(route_without_query, "{$1}")
        .to_string()
}

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
fn generate_handler(route: &str, method: &str, fixtures: &[&Fixture]) -> String {
    let handler_name = route_method_to_handler_name(route, method);
    let has_path_params = route.contains('{');
    let has_body = method == "POST" || method == "PUT" || method == "PATCH";

    eprintln!(
        "[HANDLER GEN] Generating handler for {} {} with {} fixtures{}",
        method,
        route,
        fixtures.len(),
        if has_path_params { " (with path params)" } else { "" }
    );
    for f in fixtures {
        eprintln!("[HANDLER GEN]   - Fixture: {}", f.name);
    }

    // Validate that required schemas are present
    if !validate_required_schemas(route, method, fixtures, has_path_params, has_body) {
        eprintln!("\n⚠️  WARNING: Continuing with schema validation errors - handler may fail at runtime\n");
    }

    // Check if this handler has CORS configuration
    let cors_config = extract_cors_config(fixtures);

    // For OPTIONS methods with CORS config, generate a CORS preflight handler
    if method == "OPTIONS" {
        if let Some(ref cors_cfg) = cors_config {
            return generate_cors_preflight_handler(&handler_name, cors_cfg);
        }
    }

    // Determine success status code - use the most common success status (200-299)
    // from fixtures, defaulting to 200 for GET/DELETE or 201 for POST/PUT/PATCH
    //
    // IMPORTANT: This is a FIXED status code for success responses. We do NOT parse
    // status codes from path parameters or add any conditional logic. The test app
    // should be minimal - Spikard handles all validation and error status codes.
    // Error cases (400, 422, 500, etc.) are handled by Spikard's validation layer.
    let success_status = {
        let mut status_counts: std::collections::HashMap<u16, usize> = std::collections::HashMap::new();
        for fixture in fixtures {
            let status = fixture.expected_response.status_code;
            if (200..300).contains(&status) {
                *status_counts.entry(status).or_insert(0) += 1;
            }
        }
        status_counts
            .into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(status, _)| status)
            .unwrap_or_else(|| if has_body { 201 } else { 200 })
    };

    // Try to build a parameter schema
    let param_schema = build_parameter_schema(fixtures);

    // Extract explicit body schema - NO inference, NO merging
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

        let handler_signature = if has_path_params {
            format!(
                r#"async fn {}(
    axum::extract::Path(path_params): axum::extract::Path<HashMap<String, String>>,
    headers: axum::http::HeaderMap,
    uri: axum::http::Uri,
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

        let _body_validation_code = if body_schema.is_some() {
            r#"
    // Validate request body
    if let Err(err) = body_validator.validate(&body) {
        let error_response = serde_json::json!({
            "detail": err.errors
        });
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }"#
        } else {
            ""
        };

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
        let cors_headers_code = if cors_config.is_some() {
            r#"
    // Add CORS headers to response
    let mut response = (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#
        } else {
            r#"(axum::http::StatusCode::from_u16(status_code).unwrap(), Json(validated))"#
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
        &HashMap::new(),
        &cookies,
    ) {{
        Ok(validated) => {{
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
            success_status, // Fixed success status - no dynamic logic!
            cors_headers_code,
            if cors_config.is_some() {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()"
            } else {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))"
            }
        )
    } else if let Some(body_schema) = body_schema {
        // No parameter schema but we have body schema
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

        let cors_headers_code = if cors_config.is_some() {
            r#"
    // Add CORS headers to response
    let mut response = (axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body)).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#
        } else {
            r#"(axum::http::StatusCode::from_u16(status_code).unwrap(), Json(body))"#
        };

        format!(
            r#"{} {{
    use spikard_http::SchemaValidator;
{}
    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {{
        let error_response = serde_json::json!({{
            "detail": err.errors
        }});
        return {};
    }}

    let status_code = {};
    {}
}}"#,
            handler_sig,
            cors_validation_code,
            body_schema_json,
            if cors_config.is_some() {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response)).into_response()"
            } else {
                "(axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))"
            },
            success_status,
            cors_headers_code
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
    uri: axum::http::Uri,
) -> impl axum::response::IntoResponse"#,
                handler_name
            )
        } else {
            format!(
                r#"async fn {}(
    uri: axum::http::Uri,
) -> Json<Value>"#,
                handler_name
            )
        };

        let cors_headers_code = if cors_config.is_some() {
            r#"
    // Add CORS headers to response
    let mut response = Json(params).into_response();
    response = add_cors_headers(response, origin, &cors_config);
    response"#
        } else {
            r#"Json(params)"#
        };

        format!(
            r#"{} {{
    use spikard_http::query_parser::parse_query_string_to_json;
{}
    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {{
        parse_query_string_to_json(query_str.as_bytes(), true)
    }} else {{
        Value::Object(serde_json::Map::new())
    }};

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

/// Extract explicit parameter schema from fixtures
/// Takes the FIRST fixture that has handler.parameters defined and returns it as-is.
/// NO inference, NO merging, NO building - just direct extraction.
fn build_parameter_schema(fixtures: &[&Fixture]) -> Option<Value> {
    // Find the first fixture with explicit handler.parameters
    for fixture in fixtures {
        if let Some(handler) = &fixture.handler {
            if let Some(params) = &handler.parameters {
                eprintln!(
                    "[SCHEMA EXTRACT] Using explicit parameters from fixture: {}",
                    fixture.name
                );
                return Some(params.clone());
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
