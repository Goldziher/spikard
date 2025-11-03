//! Node.js test app generator
//!
//! Generates a Spikard Node.js/TypeScript application from fixtures for e2e testing.

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Generate Node.js test application from fixtures
pub fn generate_node_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Node.js test app...");

    // Create output directory structure
    let app_dir = output_dir.join("app");
    fs::create_dir_all(&app_dir).context("Failed to create app directory")?;

    // Load all fixtures by category
    let mut fixtures_by_category: HashMap<String, Vec<Fixture>> = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path.file_name().unwrap().to_str().unwrap().to_string();
            let fixtures = load_fixtures_from_dir(&path)?;

            if !fixtures.is_empty() {
                fixtures_by_category.insert(category, fixtures);
            }
        }
    }

    // Generate main app file with per-fixture app factories
    let app_content = generate_app_file_per_fixture(&fixtures_by_category)?;
    fs::write(app_dir.join("main.ts"), app_content).context("Failed to write main.ts")?;

    // Generate package.json for the e2e app
    let package_json = generate_package_json();
    fs::write(output_dir.join("package.json"), package_json).context("Failed to write package.json")?;

    // Generate tsconfig.json
    let tsconfig = generate_tsconfig();
    fs::write(output_dir.join("tsconfig.json"), tsconfig).context("Failed to write tsconfig.json")?;

    // Generate vitest.config.ts
    let vitest_config = generate_vitest_config();
    fs::write(output_dir.join("vitest.config.ts"), vitest_config).context("Failed to write vitest.config.ts")?;

    println!("  ✓ Generated app/main.ts");
    println!("  ✓ Generated package.json");
    println!("  ✓ Generated tsconfig.json");
    println!("  ✓ Generated vitest.config.ts");
    Ok(())
}

/// Generate package.json for the e2e Node.js project
fn generate_package_json() -> String {
    r#"{
	"name": "spikard-e2e-node",
	"version": "0.1.0",
	"private": true,
	"type": "module",
	"scripts": {
		"test": "vitest run",
		"test:watch": "vitest"
	},
	"devDependencies": {
		"@spikard/node": "workspace:*",
		"@types/node": "^24.9.2",
		"@vitest/coverage-v8": "^4.0.6",
		"typescript": "^5.9.3",
		"vitest": "^4.0.6"
	}
}
"#
    .to_string()
}

/// Generate tsconfig.json for TypeScript compilation
fn generate_tsconfig() -> String {
    r#"{
	"compilerOptions": {
		"target": "ES2022",
		"module": "ES2022",
		"lib": ["ES2022"],
		"moduleResolution": "bundler",
		"strict": true,
		"esModuleInterop": true,
		"skipLibCheck": true,
		"forceConsistentCasingInFileNames": true,
		"resolveJsonModule": true,
		"types": ["vitest/globals", "node"]
	},
	"include": ["app/**/*", "tests/**/*"]
}
"#
    .to_string()
}

/// Generate vitest.config.ts for test configuration
fn generate_vitest_config() -> String {
    r#"import { defineConfig } from "vitest/config";

export default defineConfig({
	test: {
		globals: true,
		environment: "node",
	},
});
"#
    .to_string()
}

/// Generate app file with per-fixture app factory functions
fn generate_app_file_per_fixture(fixtures_by_category: &HashMap<String, Vec<Fixture>>) -> Result<String> {
    let mut code = String::new();

    // File header
    code.push_str("/**\n");
    code.push_str(" * Generated E2E test application with per-fixture app factories.\n");
    code.push_str(" * @generated\n");
    code.push_str(" */\n\n");

    // Imports
    code.push_str("import type { SpikardApp, RouteMetadata } from \"@spikard/node\";\n\n");

    // Track handler names for uniqueness
    let mut handler_names = HashMap::new();

    // Collect all fixtures and generate per-fixture functions
    let mut all_app_factories = Vec::new();

    for (category, fixtures) in fixtures_by_category.iter() {
        for fixture in fixtures.iter() {
            // Generate unique identifier for this fixture
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            let handler_name = make_unique_name(&fixture_id, &mut handler_names);

            // Generate handler and app factory for this fixture
            let (handler_code, app_factory_code) = generate_fixture_handler_and_app_node(fixture, &handler_name)?;

            code.push_str(&handler_code);
            code.push_str("\n\n");
            code.push_str(&app_factory_code);
            code.push_str("\n\n");

            all_app_factories.push((
                category.clone(),
                fixture.name.clone(),
                format!("createApp{}", to_pascal_case(&handler_name)),
            ));
        }
    }

    // Add a comment listing all app factories
    code.push_str("// App factory functions:\n");
    for (category, fixture_name, factory_fn) in all_app_factories {
        code.push_str(&format!("// - {}() for {} / {}\n", factory_fn, category, fixture_name));
    }

    Ok(code)
}

/// Generate handler and app factory for a single fixture (Node.js version)
fn generate_fixture_handler_and_app_node(fixture: &Fixture, handler_name: &str) -> Result<(String, String)> {
    // Get route from handler or request
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };

    // Strip query string from route
    let route_path = route.split('?').next().unwrap_or(&route);
    let method = fixture.request.method.as_str();

    // Generate handler function
    let handler_func = generate_handler_function(fixture, route_path, method, handler_name)?;

    // Generate app factory function
    let app_factory_name = format!("createApp{}", to_pascal_case(handler_name));

    // Extract metadata for route registration
    let body_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(schema) = &handler.body_schema {
            serde_json::to_string(schema)?
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    let parameter_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            build_parameter_schema_json(params)?
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    let file_params_str = if let Some(handler) = &fixture.handler {
        if let Some(params) = &handler.parameters {
            extract_file_params_json(params).unwrap_or_else(|| "undefined".to_string())
        } else {
            "undefined".to_string()
        }
    } else {
        "undefined".to_string()
    };

    // Generate app factory
    let app_factory_code = format!(
        r#"export function {}(): SpikardApp {{
	const route: RouteMetadata = {{
		method: "{}",
		path: "{}",
		handler_name: "{}",
		request_schema: {},
		response_schema: undefined,
		parameter_schema: {},
		file_params: {},
		is_async: true,
	}};

	return {{
		routes: [route],
		handlers: {{
			{}: {},
		}},
	}};
}}"#,
        app_factory_name,
        method.to_uppercase(),
        route_path,
        handler_name,
        body_schema_str,
        parameter_schema_str,
        file_params_str,
        handler_name,
        to_camel_case(handler_name) // Fix: use camelCase for function reference
    );

    Ok((handler_func, app_factory_code))
}

/// Generate handler function for a fixture
fn generate_handler_function(fixture: &Fixture, route: &str, method: &str, handler_name: &str) -> Result<String> {
    // Extract handler info from fixture
    let handler_opt = fixture.handler.as_ref();

    // Extract parameters
    let params = if let Some(handler) = handler_opt {
        if let Some(ref param_schema) = handler.parameters {
            extract_parameters_ts(param_schema)?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Check if body is expected
    let has_body = handler_opt.and_then(|h| h.body_schema.as_ref()).is_some();

    // Get expected response status code and body
    let expected_status = fixture.expected_response.status_code;
    let expected_body = fixture.expected_response.body.as_ref();

    // Generate handler function
    let mut code = String::new();

    code.push_str(&format!(
        "/**\n * Handler for {} {}\n */\n",
        method.to_uppercase(),
        route
    ));
    code.push_str(&format!("async function {}(", to_camel_case(handler_name)));

    // Build parameter list
    let mut param_list = Vec::new();

    if has_body {
        param_list.push("body: any".to_string());
    }

    for (param_name, param_type, _is_required) in &params {
        param_list.push(format!("{}: {}", param_name, param_type));
    }

    if param_list.is_empty() {
        code.push_str("): Promise<any> {\n");
    } else {
        code.push_str(&param_list.join(", "));
        code.push_str("): Promise<any> {\n");
    }

    // Function body - handle different response scenarios
    let should_echo_params = (expected_status == 200 && expected_body.is_none()) || expected_status == 422;
    let should_return_expected = expected_body.is_some() && expected_status != 422;

    if should_return_expected {
        // Return the expected response body
        if let Some(body_json) = expected_body {
            let json_str = serde_json::to_string(body_json)?;
            code.push_str(&format!("\treturn {};\n", json_str));
        } else {
            code.push_str(&format!("\treturn {{ status: {} }};\n", expected_status));
        }
    } else if should_echo_params {
        // Echo parameters to prove extraction/validation worked
        code.push_str("\t// Echo back parameters for testing\n");
        code.push_str("\tconst result: Record<string, any> = {};\n");

        if has_body {
            code.push_str("\tif (body !== null && body !== undefined) {\n");
            code.push_str("\t\tObject.assign(result, body);\n");
            code.push_str("\t}\n");
        }

        for (param_name, param_type, _) in &params {
            code.push_str(&format!(
                "\tif ({} !== null && {} !== undefined) {{\n",
                param_name, param_name
            ));
            // Convert non-JSON-serializable types to strings
            if param_type.contains("Date") {
                code.push_str(&format!(
                    "\t\tresult[\"{}\"] = {}.toISOString();\n",
                    param_name, param_name
                ));
            } else {
                code.push_str(&format!("\t\tresult[\"{}\"] = {};\n", param_name, param_name));
            }
            code.push_str("\t}\n");
        }

        code.push_str("\treturn result;\n");
    } else {
        code.push_str(&format!("\treturn {{ status: {} }};\n", expected_status));
    }

    code.push('}');

    Ok(code)
}

/// Extract parameters from parameter schema (TypeScript types)
fn extract_parameters_ts(schema: &Value) -> Result<Vec<(String, String, bool)>> {
    let mut params = Vec::new();

    if let Some(obj) = schema.as_object() {
        // Extract path parameters
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let param_type = json_type_to_typescript(param_schema)?;
                params.push((to_camel_case(name), param_type, true));
            }
        }

        // Extract query parameters
        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let param_type = json_type_to_typescript(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                params.push((to_camel_case(name), param_type, is_required));
            }
        }

        // Extract header parameters
        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let param_type = json_type_to_typescript(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                params.push((to_camel_case(name), param_type, is_required));
            }
        }

        // Extract cookie parameters
        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let param_type = json_type_to_typescript(param_schema)?;
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                params.push((to_camel_case(name), param_type, is_required));
            }
        }
    }

    Ok(params)
}

/// Convert JSON schema type to TypeScript type annotation
fn json_type_to_typescript(schema: &Value) -> Result<String> {
    let schema_type = schema.get("type").and_then(|v| v.as_str()).unwrap_or("string");

    let type_str = match schema_type {
        "string" => {
            if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
                match format {
                    "uuid" => "string",
                    "date" | "date-time" => "Date",
                    _ => "string",
                }
            } else {
                "string"
            }
        }
        "integer" => "number",
        "number" => "number",
        "boolean" => "boolean",
        "array" => {
            if let Some(items) = schema.get("items") {
                let item_type = json_type_to_typescript(items)?;
                return Ok(format!("{}[]", item_type));
            }
            "any[]"
        }
        "object" => "Record<string, any>",
        _ => "any",
    };

    Ok(type_str.to_string())
}

/// Build parameter schema JSON string
fn build_parameter_schema_json(params: &Value) -> Result<String> {
    // Just serialize the parameters directly
    serde_json::to_string(params).context("Failed to serialize parameter schema")
}

/// Extract file parameters JSON string
fn extract_file_params_json(params: &Value) -> Option<String> {
    if let Some(obj) = params.as_object() {
        if let Some(files) = obj.get("files") {
            return serde_json::to_string(files).ok();
        }
    }
    None
}

/// Sanitize a string to be a valid identifier (lowercase snake_case)
fn sanitize_identifier(s: &str) -> String {
    let mut result = s
        .to_lowercase()
        .replace(|c: char| !c.is_alphanumeric() && c != '_', "_");

    // Collapse multiple consecutive underscores to single underscore
    while result.contains("__") {
        result = result.replace("__", "_");
    }

    result.trim_matches('_').to_string()
}

/// Convert to camelCase
fn to_camel_case(s: &str) -> String {
    let parts: Vec<&str> = s.split(&['_', '-'][..]).collect();
    if parts.is_empty() {
        return String::new();
    }

    let mut result = parts[0].to_lowercase();
    for part in &parts[1..] {
        if !part.is_empty() {
            result.push_str(&capitalize(part));
        }
    }
    result
}

/// Convert to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-'][..])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Make a name unique by adding a suffix if needed
fn make_unique_name(base_name: &str, used_names: &mut HashMap<String, usize>) -> String {
    let count = used_names.entry(base_name.to_string()).or_insert(0);
    *count += 1;

    if *count == 1 {
        base_name.to_string()
    } else {
        format!("{}_{}", base_name, *count - 1)
    }
}
