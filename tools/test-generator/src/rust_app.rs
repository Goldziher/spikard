//! Rust test app generation

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::{load_fixtures_from_dir, Fixture};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

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
            let route = if let Some(handler) = &fixture.handler {
                handler.route.clone()
            } else {
                // Extract just the path without query string
                fixture
                    .request
                    .path
                    .split('?')
                    .next()
                    .unwrap_or(&fixture.request.path)
                    .to_string()
            };

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

/// Strip type hints from route pattern (e.g., {param:type} -> {param})
fn strip_type_hints(route: &str) -> String {
    regex::Regex::new(r"\{([^:}]+):[^}]+\}")
        .unwrap()
        .replace_all(route, "{$1}")
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

    // Determine success status code - use the most common success status (200-299)
    // from fixtures, defaulting to 200 for GET/DELETE or 201 for POST/PUT/PATCH
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

    // Try to get explicit body schema first, fallback to inference
    let body_schema = if has_body {
        // Collect all explicit body schemas from fixtures
        let explicit_schemas: Vec<Value> = fixtures
            .iter()
            .filter_map(|f| f.handler.as_ref().and_then(|h| h.body_schema.as_ref()).cloned())
            .collect();

        if !explicit_schemas.is_empty() {
            // If all explicit schemas are identical, use that one
            if explicit_schemas.iter().all(|s| s == &explicit_schemas[0]) {
                Some(explicit_schemas[0].clone())
            } else {
                // Multiple different schemas - try to merge them intelligently
                eprintln!("[HANDLER GEN] Multiple different body schemas found, attempting intelligent merge");
                Some(merge_schemas(&explicit_schemas))
            }
        } else {
            // No explicit schemas, try inference
            infer_body_schema(fixtures)
        }
    } else {
        None
    };

    if let Some(body_schema) = &body_schema {
        let is_explicit = fixtures
            .iter()
            .any(|f| f.handler.as_ref().and_then(|h| h.body_schema.as_ref()).is_some());
        eprintln!(
            "[HANDLER GEN] {} body schema: {}",
            if is_explicit { "Using explicit" } else { "Inferred" },
            serde_json::to_string_pretty(body_schema).unwrap()
        );
    }

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

        format!(
            r#"{} {{
    use spikard_http::parameters::ParameterValidator;
    use spikard_http::query_parser::parse_query_string_to_json;
    use std::collections::HashMap;

    // Parse parameter schema and create validator
    let schema: Value = serde_json::from_str("{}").unwrap();
    let validator = ParameterValidator::new(schema).unwrap();{}

    // Parse query string using Spikard's parser (auto-converts types)
    let query_params = if let Some(query_str) = uri.query() {{
        parse_query_string_to_json(query_str.as_bytes(), true)
    }} else {{
        Value::Object(serde_json::Map::new())
    }};

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
        &{}path_params,
        &HashMap::new(),
        &cookies,
    ) {{
        Ok(validated) => {{
            (axum::http::StatusCode::from_u16({}).unwrap(), Json(validated))
        }}
        Err(err) => {{
            let error_response = serde_json::json!({{
                "detail": err.errors
            }});
            (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response))
        }}
    }}
}}"#,
            handler_signature,
            schema_json,
            body_validator_code,
            if has_path_params { "" } else { "HashMap::new(), //" },
            success_status
        )
    } else if let Some(body_schema) = body_schema {
        // No parameter schema but we have body schema
        let body_schema_json = serde_json::to_string(&body_schema)
            .unwrap()
            .replace('\\', "\\\\")
            .replace('"', "\\\"");

        format!(
            r#"async fn {}(
    axum::extract::Json(body): axum::extract::Json<Value>,
) -> impl axum::response::IntoResponse {{
    use spikard_http::SchemaValidator;

    // Parse body schema and create validator
    let body_schema: Value = serde_json::from_str("{}").unwrap();
    let body_validator = SchemaValidator::new(body_schema).unwrap();

    // Validate request body
    if let Err(err) = body_validator.validate(&body) {{
        let error_response = serde_json::json!({{
            "detail": err.errors
        }});
        return (axum::http::StatusCode::UNPROCESSABLE_ENTITY, Json(error_response));
    }}

    (axum::http::StatusCode::from_u16({}).unwrap(), Json(body))
}}"#,
            handler_name, body_schema_json, success_status
        )
    } else {
        // No schema available - simple echo handler
        format!(
            r#"async fn {}(
    uri: axum::http::Uri,
) -> Json<Value> {{
    use spikard_http::query_parser::parse_query_string_to_json;

    // Parse query params using Spikard's parser
    let params = if let Some(query_str) = uri.query() {{
        parse_query_string_to_json(query_str.as_bytes(), true)
    }} else {{
        Value::Object(serde_json::Map::new())
    }};

    Json(params)
}}"#,
            handler_name
        )
    }
}

/// Parse type hints from route pattern like {param:type}
/// Returns: (param_name, type_hint)
fn parse_type_hints(route: &str) -> Vec<(String, String)> {
    let re = regex::Regex::new(r"\{([^:}]+):([^}]+)\}").unwrap();
    re.captures_iter(route)
        .map(|cap| (cap[1].to_string(), cap[2].to_string()))
        .collect()
}

/// Convert type hint to JSON Schema
fn type_hint_to_schema(type_hint: &str) -> serde_json::Map<String, Value> {
    use serde_json::json;
    let mut schema = serde_json::Map::new();

    match type_hint {
        "int" => {
            schema.insert("type".to_string(), json!("integer"));
        }
        "float" => {
            schema.insert("type".to_string(), json!("number"));
        }
        "bool" => {
            schema.insert("type".to_string(), json!("boolean"));
        }
        "uuid" => {
            schema.insert("type".to_string(), json!("string"));
            schema.insert("format".to_string(), json!("uuid"));
        }
        "date" => {
            schema.insert("type".to_string(), json!("string"));
            schema.insert("format".to_string(), json!("date"));
        }
        "datetime" => {
            schema.insert("type".to_string(), json!("string"));
            schema.insert("format".to_string(), json!("date-time"));
        }
        "string" | "path" => {
            schema.insert("type".to_string(), json!("string"));
        }
        _ => {
            // Unknown type hint, default to string
            schema.insert("type".to_string(), json!("string"));
        }
    }

    schema
}

/// Build a JSON Schema for parameter validation from fixtures
/// Merges parameter definitions from ALL fixtures for the route
fn build_parameter_schema(fixtures: &[&Fixture]) -> Option<Value> {
    use serde_json::json;
    use std::collections::HashSet;

    // Merge all parameter definitions from all fixtures
    let mut properties = serde_json::Map::new();
    // Track which params are required by ALL fixtures vs only SOME fixtures
    let mut param_fixture_count: std::collections::HashMap<String, (usize, usize)> = std::collections::HashMap::new(); // (fixtures_with_param, fixtures_requiring_param)

    // Extract route from first fixture to parse type hints
    let route = if let Some(handler) = &fixtures[0].handler {
        handler.route.clone()
    } else {
        fixtures[0]
            .request
            .path
            .split('?')
            .next()
            .unwrap_or(&fixtures[0].request.path)
            .to_string()
    };

    // Parse type hints from route and auto-generate schemas
    let type_hints = parse_type_hints(&route);
    for (param_name, type_hint) in type_hints {
        // Only auto-generate if not already explicitly defined
        // We'll check this below when processing handler.parameters
        let schema = type_hint_to_schema(&type_hint);
        let mut prop = serde_json::Map::new();
        prop.insert("source".to_string(), json!("path"));
        for (key, value) in schema {
            prop.insert(key, value);
        }
        properties.insert(param_name.clone(), Value::Object(prop));
    }

    for fixture in fixtures {
        if let Some(handler) = &fixture.handler {
            if let Some(params) = &handler.parameters {
                // Process query, path, and cookie parameters
                for (source, param_source_name) in &[("query", "query"), ("path", "path"), ("cookie", "cookies")] {
                    if let Some(source_params) = params.get(*param_source_name).and_then(|v| v.as_object()) {
                        for (param_name, param_def) in source_params {
                            if let Some(param_obj) = param_def.as_object() {
                                // Get or create property for this parameter
                                let prop = properties.entry(param_name.clone()).or_insert_with(|| {
                                    let mut p = serde_json::Map::new();
                                    p.insert("source".to_string(), json!(source));
                                    Value::Object(p)
                                });

                                if let Some(prop_map) = prop.as_object_mut() {
                                    // Merge constraint fields from this fixture's schema
                                    // Type should be consistent across fixtures, take first one
                                    if !prop_map.contains_key("type") {
                                        if let Some(param_type) = param_obj.get("type") {
                                            prop_map.insert("type".to_string(), param_type.clone());
                                        }
                                    }

                                    // Merge ALL constraint fields (union approach)
                                    for (key, value) in param_obj {
                                        if key != "annotation" && key != "type" && key != "required" {
                                            prop_map.entry(key.clone()).or_insert(value.clone());
                                        }
                                    }
                                }

                                // Track if this fixture requires this parameter
                                // Path parameters are always required by default
                                let is_required = if *source == "path" {
                                    true
                                } else {
                                    !param_obj.contains_key("default")
                                        && !param_obj.contains_key("optional")
                                        && param_obj.get("required").and_then(|v| v.as_bool()).unwrap_or(true)
                                };

                                let entry = param_fixture_count.entry(param_name.clone()).or_insert((0, 0));
                                entry.0 += 1; // fixtures with this param
                                if is_required {
                                    entry.1 += 1; // fixtures requiring this param
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Parameters are required in the schema only if ALL fixtures on the route require them
    // For parameters used by only some fixtures, we rely on the "optional" field in properties
    let total_fixtures = fixtures.len();
    let required_set: HashSet<String> = param_fixture_count
        .iter()
        .filter_map(|(param_name, (fixtures_with_param, required_count))| {
            // Only add to required set if:
            // 1. All fixtures that define this param require it (required_count == fixtures_with_param)
            // 2. AND all fixtures on this route define it (fixtures_with_param == total_fixtures)
            if required_count == fixtures_with_param && *fixtures_with_param == total_fixtures {
                Some(param_name.clone())
            } else {
                None
            }
        })
        .collect();

    // For parameters not used by all fixtures, ensure they're marked as optional if not already
    for (param_name, (fixtures_with_param, _)) in param_fixture_count.iter() {
        if *fixtures_with_param < total_fixtures {
            if let Some(prop) = properties.get_mut(param_name) {
                if let Some(prop_obj) = prop.as_object_mut() {
                    // Only add optional:true if it's not already set
                    if !prop_obj.contains_key("optional") {
                        prop_obj.insert("optional".to_string(), json!(true));
                    }
                }
            }
        }
    }

    if !properties.is_empty() {
        let required: Vec<String> = required_set.into_iter().collect();
        Some(json!({
            "type": "object",
            "properties": properties,
            "required": required
        }))
    } else {
        None
    }
}

/// Merge multiple schemas intelligently by combining constraints
/// For simple object schemas with constraints like minProperties/maxProperties,
/// merge the constraints. For complex schemas (anyOf, oneOf, etc.), use anyOf wrapper.
fn merge_schemas(schemas: &[Value]) -> Value {
    use serde_json::json;
    use std::collections::HashMap;

    // Check if all schemas are simple object schemas (can be merged)
    let all_simple_objects = schemas.iter().all(|s| {
        s.get("type") == Some(&json!("object"))
            && !s
                .as_object()
                .map(|o| o.contains_key("oneOf") || o.contains_key("anyOf") || o.contains_key("allOf"))
                .unwrap_or(false)
    });

    if all_simple_objects && schemas.len() > 1 {
        // Merge object schemas by combining constraints
        let mut merged = serde_json::Map::new();
        merged.insert("type".to_string(), json!("object"));

        // Collect all constraints from all schemas
        let mut all_properties: HashMap<String, Value> = HashMap::new();
        let mut required_sets: Vec<std::collections::HashSet<String>> = Vec::new();
        let mut min_props: Option<u64> = None;
        let mut max_props: Option<u64> = None;
        let mut additional_props: Option<Value> = None;
        let mut all_definitions: HashMap<String, Value> = HashMap::new();

        for schema in schemas {
            // Collect required fields from this schema
            let mut schema_required = std::collections::HashSet::new();
            if let Some(req) = schema
                .as_object()
                .and_then(|o| o.get("required"))
                .and_then(|r| r.as_array())
            {
                for item in req {
                    if let Some(field) = item.as_str() {
                        schema_required.insert(field.to_string());
                    }
                }
            }
            required_sets.push(schema_required);
            if let Some(obj) = schema.as_object() {
                // Merge properties (with recursive merging for nested objects)
                if let Some(props) = obj.get("properties").and_then(|p| p.as_object()) {
                    for (key, value) in props {
                        if let Some(existing) = all_properties.get(key) {
                            // Property already exists - try to merge it
                            if let (Some(existing_obj), Some(new_obj)) = (existing.as_object(), value.as_object()) {
                                // Both are objects - check if both are simple object schemas
                                let existing_is_object = existing_obj.get("type") == Some(&json!("object"));
                                let new_is_object = new_obj.get("type") == Some(&json!("object"));

                                if existing_is_object && new_is_object {
                                    // Recursively merge these two object schemas
                                    let merged_property = merge_schemas(&[existing.clone(), value.clone()]);
                                    all_properties.insert(key.clone(), merged_property);
                                } else {
                                    // Both are schema objects but not both "type": "object"
                                    // Merge their constraints (for strings, numbers, arrays, etc.)
                                    let mut merged = existing_obj.clone();
                                    for (constraint_key, constraint_value) in new_obj {
                                        // Merge constraints, keeping both
                                        if !merged.contains_key(constraint_key) {
                                            merged.insert(constraint_key.clone(), constraint_value.clone());
                                        }
                                    }
                                    all_properties.insert(key.clone(), Value::Object(merged));
                                }
                            }
                            // If not both objects, keep existing
                        } else {
                            // New property - just insert it
                            all_properties.insert(key.clone(), value.clone());
                        }
                    }
                }

                // Note: We'll handle required fields separately to take intersection, not union

                // Take the most restrictive minProperties
                if let Some(min) = obj.get("minProperties").and_then(|v| v.as_u64()) {
                    min_props = Some(min_props.map_or(min, |current| current.max(min)));
                }

                // Take the most restrictive maxProperties
                if let Some(max) = obj.get("maxProperties").and_then(|v| v.as_u64()) {
                    max_props = Some(max_props.map_or(max, |current| current.min(max)));
                }

                // additionalProperties: take false if any schema has it
                if let Some(additional) = obj.get("additionalProperties") {
                    if additional == &json!(false) {
                        additional_props = Some(json!(false));
                    }
                }

                // Collect definitions (for $ref support)
                if let Some(defs) = obj.get("definitions").and_then(|d| d.as_object()) {
                    for (key, value) in defs {
                        all_definitions.insert(key.clone(), value.clone());
                    }
                }
                // Also handle $defs (Draft 2019-09+)
                if let Some(defs) = obj.get("$defs").and_then(|d| d.as_object()) {
                    for (key, value) in defs {
                        all_definitions.insert(key.clone(), value.clone());
                    }
                }
            }
        }

        // Compute intersection of required fields (only fields required in ALL schemas)
        let required_intersection: Vec<String> = if !required_sets.is_empty() {
            // Start with the first set
            let mut intersection = required_sets[0].clone();
            // Intersect with all other sets
            for req_set in &required_sets[1..] {
                intersection.retain(|field| req_set.contains(field));
            }
            intersection.into_iter().collect()
        } else {
            Vec::new()
        };

        // Build merged schema
        if !all_properties.is_empty() {
            merged.insert("properties".to_string(), json!(all_properties));
        }
        if !required_intersection.is_empty() {
            merged.insert("required".to_string(), json!(required_intersection));
        }
        if let Some(min) = min_props {
            merged.insert("minProperties".to_string(), json!(min));
        }
        if let Some(max) = max_props {
            merged.insert("maxProperties".to_string(), json!(max));
        }
        if let Some(additional) = additional_props {
            merged.insert("additionalProperties".to_string(), additional);
        }
        if !all_definitions.is_empty() {
            merged.insert("definitions".to_string(), json!(all_definitions));
        }

        Value::Object(merged)
    } else {
        // Complex schemas or incompatible - use anyOf
        json!({
            "anyOf": schemas
        })
    }
}

/// Infer body schema from fixtures by analyzing request bodies
/// This analyzes both success and failure cases to infer constraints
fn infer_body_schema(fixtures: &[&Fixture]) -> Option<Value> {
    use serde_json::json;
    use std::collections::{HashMap, HashSet};

    // Collect all request bodies from success and failure cases
    let mut success_bodies: Vec<&Value> = Vec::new();
    let mut validation_failures: Vec<(&Value, &Value)> = Vec::new(); // (request_body, error_details)

    for fixture in fixtures {
        if let Some(body) = &fixture.request.body {
            let status = fixture.expected_response.status_code;
            if (200..300).contains(&status) {
                success_bodies.push(body);
            } else if status == 422 {
                // Validation failures help us understand constraints
                if let Some(error_body) = &fixture.expected_response.body {
                    validation_failures.push((body, error_body));
                }
            }
        }
    }

    // Infer schema by analyzing the structure of success bodies (if any)
    let mut properties = serde_json::Map::new();
    let mut required_fields = HashSet::new();

    // First, collect all fields from all success bodies
    for body in &success_bodies {
        if let Value::Object(obj) = body {
            for (key, value) in obj {
                required_fields.insert(key.clone());

                // Infer type from value
                if !properties.contains_key(key) {
                    properties.insert(key.clone(), infer_type_from_value(value));
                }
            }
        }
    }

    // If no success bodies, try to infer from failure request bodies
    if success_bodies.is_empty() && !validation_failures.is_empty() {
        for (body, _) in &validation_failures {
            if let Value::Object(obj) = body {
                for (key, value) in obj {
                    if !properties.contains_key(key) {
                        properties.insert(key.clone(), infer_type_from_value(value));
                    }
                }
            }
        }
    }

    // Check which fields are required (present in all success cases)
    for body in &success_bodies {
        if let Value::Object(obj) = body {
            for field in required_fields.clone().iter() {
                if !obj.contains_key(field) {
                    required_fields.remove(field);
                }
            }
        }
    }

    // Analyze validation failures to extract constraints
    let mut field_constraints: HashMap<String, Vec<(String, Value)>> = HashMap::new();

    for (_req_body, error_body) in validation_failures {
        if let Some(details) = error_body.get("detail").and_then(|d| d.as_array()) {
            for error in details {
                if let Some(loc) = error.get("loc").and_then(|l| l.as_array()) {
                    // Extract field name from location (e.g., ["body", "name"] -> "name")
                    if loc.len() >= 2 {
                        if let Some(field_name) = loc[1].as_str() {
                            let error_type = error.get("type").and_then(|t| t.as_str()).unwrap_or("");

                            // Extract constraint from context
                            if let Some(ctx) = error.get("ctx").and_then(|c| c.as_object()) {
                                let constraints = field_constraints.entry(field_name.to_string()).or_default();

                                match error_type {
                                    "string_too_short" => {
                                        if let Some(min_len) = ctx.get("min_length") {
                                            constraints.push(("minLength".to_string(), min_len.clone()));
                                        }
                                    }
                                    "string_too_long" => {
                                        if let Some(max_len) = ctx.get("max_length") {
                                            constraints.push(("maxLength".to_string(), max_len.clone()));
                                        }
                                    }
                                    "string_pattern_mismatch" => {
                                        if let Some(pattern) = ctx.get("pattern") {
                                            constraints.push(("pattern".to_string(), pattern.clone()));
                                        }
                                    }
                                    "enum" => {
                                        // Extract enum values from error message or context
                                        if let Some(expected) = ctx.get("expected") {
                                            // Parse "'electronics', 'clothing' or 'books'" into array
                                            if let Some(expected_str) = expected.as_str() {
                                                let values: Vec<String> = expected_str
                                                    .split(", ")
                                                    .map(|s| s.trim_matches(&['\'', ' ', 'o', 'r'][..]))
                                                    .filter(|s| !s.is_empty() && *s != "or")
                                                    .map(|s| s.to_string())
                                                    .collect();
                                                if !values.is_empty() {
                                                    constraints.push(("enum".to_string(), json!(values)));
                                                }
                                            }
                                        }
                                    }
                                    "greater_than_equal" => {
                                        if let Some(ge) = ctx.get("ge") {
                                            constraints.push(("minimum".to_string(), ge.clone()));
                                        }
                                    }
                                    "less_than_equal" => {
                                        if let Some(le) = ctx.get("le") {
                                            constraints.push(("maximum".to_string(), le.clone()));
                                        }
                                    }
                                    "greater_than" => {
                                        if let Some(gt) = ctx.get("gt") {
                                            constraints.push(("exclusiveMinimum".to_string(), gt.clone()));
                                        }
                                    }
                                    "less_than" => {
                                        if let Some(lt) = ctx.get("lt") {
                                            constraints.push(("exclusiveMaximum".to_string(), lt.clone()));
                                        }
                                    }
                                    "too_short" => {
                                        // Array min items
                                        if let Some(min_items) = ctx.get("min_length") {
                                            constraints.push(("minItems".to_string(), min_items.clone()));
                                        }
                                    }
                                    "too_long" => {
                                        // Array max items
                                        if let Some(max_items) = ctx.get("max_length") {
                                            constraints.push(("maxItems".to_string(), max_items.clone()));
                                        }
                                    }
                                    _ => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Merge constraints into properties
    for (field_name, constraints) in field_constraints {
        if let Some(field_schema) = properties.get_mut(&field_name) {
            if let Some(schema_obj) = field_schema.as_object_mut() {
                for (constraint_key, constraint_value) in constraints {
                    schema_obj.insert(constraint_key, constraint_value);
                }
            }
        }
    }

    // Handle nullable fields - if we see null values in success cases, allow null
    for body in &success_bodies {
        if let Value::Object(obj) = body {
            for (key, value) in obj {
                if value.is_null() {
                    if let Some(field_schema) = properties.get_mut(key) {
                        if let Some(schema_obj) = field_schema.as_object_mut() {
                            // Make the type nullable by converting to array if needed
                            if let Some(type_val) = schema_obj.get("type") {
                                if let Some(type_str) = type_val.as_str() {
                                    schema_obj.insert("type".to_string(), json!([type_str, "null"]));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    if properties.is_empty() {
        return None;
    }

    let required: Vec<String> = required_fields.into_iter().collect();
    Some(json!({
        "type": "object",
        "properties": properties,
        "required": required,
        "additionalProperties": true  // Allow extra fields by default
    }))
}

/// Infer JSON Schema type from a JSON value
fn infer_type_from_value(value: &Value) -> Value {
    use serde_json::json;

    match value {
        Value::Null => json!({"type": ["string", "null"]}), // Nullable
        Value::Bool(_) => json!({"type": "boolean"}),
        Value::Number(n) => {
            if n.is_f64() {
                json!({"type": "number"})
            } else {
                json!({"type": "integer"})
            }
        }
        Value::String(s) => {
            // Try to detect format from string content
            if s.contains("T") && s.contains(":") && (s.ends_with("Z") || s.contains("+")) {
                json!({"type": "string", "format": "date-time"})
            } else if s.len() == 10 && s.matches('-').count() == 2 {
                json!({"type": "string", "format": "date"})
            } else if s.len() == 36 && s.matches('-').count() == 4 {
                json!({"type": "string", "format": "uuid"})
            } else {
                json!({"type": "string"})
            }
        }
        Value::Array(arr) => {
            if let Some(first) = arr.first() {
                json!({
                    "type": "array",
                    "items": infer_type_from_value(first)
                })
            } else {
                json!({"type": "array"})
            }
        }
        Value::Object(obj) => {
            let mut props = serde_json::Map::new();
            for (key, val) in obj {
                props.insert(key.clone(), infer_type_from_value(val));
            }
            json!({
                "type": "object",
                "properties": props
            })
        }
    }
}

fn route_method_to_handler_name(route: &str, method: &str) -> String {
    // Strip type hints like {param:type} -> {param}
    let route_without_types = strip_type_hints(route);

    let mut route_part = route_without_types
        .trim_start_matches('/')
        .replace(['/', '-', '.'], "_")
        .replace(['{', '}'], "");

    // If the route starts with a digit after processing, prefix with underscore
    if route_part.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        route_part = format!("_{}", route_part);
    }

    format!("{}_{}_handler", method.to_lowercase(), route_part)
}
