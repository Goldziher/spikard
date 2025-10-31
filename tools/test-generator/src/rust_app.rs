//! Rust test app generation

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
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

    // Collect all unique routes
    let mut route_map: HashMap<(String, String), Vec<&Fixture>> = HashMap::new();

    for fixtures in categories.values() {
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

use axum::{{routing::{{get, post, put, patch, delete, head, options, trace}}, Json, Router}};
use serde_json::{{json, Value}};
use std::collections::HashMap;

pub fn create_app() -> Router {{
    Router::new()
{}
}}

{}
"#,
        generate_router_config(&route_map),
        routes.join("\n\n")
    )
}

fn generate_router_config(route_map: &HashMap<(String, String), Vec<&Fixture>>) -> String {
    let mut routes: Vec<_> = route_map.keys().collect();
    routes.sort();

    routes
        .iter()
        .map(|(route, method)| {
            let handler_name = route_method_to_handler_name(route, method);
            let method_lower = method.to_lowercase();
            format!("        .route(\"{}\", {}({}))", route, method_lower, handler_name)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn generate_handler(route: &str, method: &str, fixtures: &[&Fixture]) -> String {
    use std::collections::HashSet;

    let handler_name = route_method_to_handler_name(route, method);

    // Collect all query parameters from all fixtures using this route/method
    let mut query_params = HashSet::new();
    for fixture in fixtures {
        if let Some(query_map) = &fixture.request.query_params {
            for key in query_map.keys() {
                query_params.insert(key.clone());
            }
        }
    }

    // Generate a simple handler that extracts query params as a HashMap
    if !query_params.is_empty() {
        format!(
            r#"async fn {}(axum::extract::Query(params): axum::extract::Query<HashMap<String, String>>) -> Json<Value> {{
    // Extract and return query parameters
    Json(json!(params))
}}"#,
            handler_name
        )
    } else {
        // No query params - simple handler
        format!(
            r#"async fn {}() -> Json<Value> {{
    Json(json!({{
        "route": "{}",
        "method": "{}"
    }}))
}}"#,
            handler_name, route, method
        )
    }
}

fn route_method_to_handler_name(route: &str, method: &str) -> String {
    let mut route_part = route
        .trim_start_matches('/')
        .replace(['/', '-', '.'], "_")
        .replace(['{', '}'], "");

    // If the route starts with a digit after processing, prefix with underscore
    if route_part.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        route_part = format!("_{}", route_part);
    }

    format!("{}_{}_handler", method.to_lowercase(), route_part)
}
