//! Python test app generator
//!
//! Generates a Spikard Python application from fixtures for e2e testing.
//!
//! Rotates through all Python type systems to ensure validation works with:
//! - Plain dict (fastest, no conversion)
//! - TypedDict (typed hints, no runtime conversion)
//! - dataclass (stdlib, mutable)
//! - NamedTuple (stdlib, immutable)
//! - msgspec.Struct (fastest typed)
//! - Pydantic BaseModel (popular, slower)

use anyhow::{Context, Result};
use serde_json::Value;
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Convert JSON string to Python dict syntax
/// Replaces JSON literals (true, false, null) with Python equivalents (True, False, None)
fn json_to_python_dict(json_str: &str) -> String {
    json_str
        .replace(":true", ":True")
        .replace(":false", ":False")
        .replace(":null", ":None")
        .replace("[true", "[True")
        .replace("[false", "[False")
        .replace("[null", "[None")
        .replace(",true", ",True")
        .replace(",false", ",False")
        .replace(",null", ",None")
}

/// Type system to use for request body parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BodyType {
    /// Plain dict[str, Any] - no conversion needed (fastest)
    PlainDict,
    /// TypedDict - type hints only, no runtime conversion (fastest)
    TypedDict,
    /// @dataclass - stdlib mutable typed object
    Dataclass,
    /// NamedTuple - stdlib immutable typed tuple
    NamedTuple,
    /// msgspec.Struct - fastest typed conversion
    MsgspecStruct,
    /// Pydantic BaseModel - popular but slower
    Pydantic,
}

impl BodyType {
    /// Rotate through all type systems to ensure comprehensive testing
    fn for_index(index: usize) -> Self {
        match index % 6 {
            0 => BodyType::PlainDict,
            1 => BodyType::TypedDict,
            2 => BodyType::Dataclass,
            3 => BodyType::NamedTuple,
            4 => BodyType::MsgspecStruct,
            5 => BodyType::Pydantic,
            _ => unreachable!(),
        }
    }
}

/// Generate Python test application from fixtures
pub fn generate_python_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Generating Python test app...");

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
    fs::write(app_dir.join("main.py"), app_content).context("Failed to write main.py")?;

    // Generate __init__.py
    fs::write(app_dir.join("__init__.py"), "\"\"\"E2E test application.\"\"\"\n")
        .context("Failed to write __init__.py")?;

    println!("  ✓ Generated app/main.py");
    Ok(())
}

/// Generate app file with per-fixture app factory functions (matches Rust pattern)
fn generate_app_file_per_fixture(fixtures_by_category: &HashMap<String, Vec<Fixture>>) -> Result<String> {
    let mut code = String::new();

    // Imports
    code.push_str("\"\"\"Generated E2E test application with per-fixture app factories.\"\"\"\n");
    code.push_str("# ruff: noqa: ARG001, A002\n"); // Suppress unused argument and builtin shadowing warnings
    code.push_str("# mypy: ignore-errors\n"); // Generated code - skip type checking
    code.push('\n');
    code.push_str("from dataclasses import asdict, dataclass\n");
    code.push_str("from datetime import date, datetime\n");
    code.push_str("from enum import Enum\n");
    code.push_str("from typing import Any, NamedTuple, TypedDict\n");
    code.push_str("from uuid import UUID\n\n");
    code.push_str("import msgspec\n");
    code.push_str("from pydantic import BaseModel\n\n");
    code.push_str("from spikard import Response, Spikard, delete, get, head, options, patch, post, put\n\n");

    // Track handler names for uniqueness
    let mut handler_names = HashMap::new();

    // Collect all fixtures and generate per-fixture functions
    let mut all_app_factories = Vec::new();

    for (category, fixtures) in fixtures_by_category.iter() {
        for (index, fixture) in fixtures.iter().enumerate() {
            // Generate unique identifier for this fixture
            let fixture_id = sanitize_identifier(&format!("{}_{}", category, &fixture.name));
            let handler_name = make_unique_name(&fixture_id, &mut handler_names);

            // Rotate through body types for comprehensive testing
            let body_type = BodyType::for_index(index);

            // Generate handler and app factory for this fixture
            let (handler_code, app_factory_code) =
                generate_fixture_handler_and_app_python(fixture, &handler_name, body_type, &mut handler_names)?;

            code.push_str(&handler_code);
            code.push_str("\n\n");
            code.push_str(&app_factory_code);
            code.push_str("\n\n");

            all_app_factories.push((
                category.clone(),
                fixture.name.clone(),
                format!("create_app_{}", handler_name),
            ));
        }
    }

    // Add a comment listing all app factories
    code.push_str("# App factory functions:\n");
    for (category, fixture_name, factory_fn) in all_app_factories {
        code.push_str(&format!("# - {}() for {} / {}\n", factory_fn, category, fixture_name));
    }

    Ok(code)
}

/// Generate handler and app factory for a single fixture (Python version)
fn generate_fixture_handler_and_app_python(
    fixture: &Fixture,
    handler_name: &str,
    body_type: BodyType,
    handler_names: &mut HashMap<String, usize>,
) -> Result<(String, String)> {
    // Get route from handler or request
    let route = if let Some(handler) = &fixture.handler {
        handler.route.clone()
    } else {
        fixture.request.path.clone()
    };

    // Strip query string from route
    let route_path = route.split('?').next().unwrap_or(&route);
    let method = fixture.request.method.as_str();

    // Generate models at module level and get the model name
    let (models_code, model_name) =
        generate_models_for_fixture_with_name(fixture, handler_name, body_type, handler_names)?;

    // Generate handler function at module level (without decorator)
    let handler_func = generate_handler_function_for_fixture(
        fixture,
        route_path,
        method,
        handler_name,
        body_type,
        model_name.as_deref(),
    )?;

    // Combine models and handler
    let mut handler_code = String::new();
    if !models_code.is_empty() {
        handler_code.push_str(&models_code);
        handler_code.push_str("\n\n");
    }
    handler_code.push_str(&handler_func);

    // Generate app factory function that registers the handler
    let app_factory_name = format!("create_app_{}", handler_name);

    // Extract body_schema for registration
    let body_schema_str = if let Some(handler) = &fixture.handler {
        if let Some(schema) = &handler.body_schema {
            let schema_json = serde_json::to_string(schema)?;
            json_to_python_dict(&schema_json)
        } else {
            "None".to_string()
        }
    } else {
        "None".to_string()
    };

    let app_factory_code = format!(
        r#"def {}() -> Spikard:
    """App factory for fixture: {}"""
    app = Spikard()
    # Register handler with this app instance
    app.register_route("{}", "{}", body_schema={})({})
    return app"#,
        app_factory_name,
        fixture.name,
        method.to_uppercase(),
        route_path,
        body_schema_str,
        handler_name
    );

    Ok((handler_code, app_factory_code))
}

/// Generate just the models for a fixture (module-level) and return the model name
fn generate_models_for_fixture_with_name(
    fixture: &Fixture,
    handler_name: &str,
    body_type: BodyType,
    handler_names: &mut HashMap<String, usize>,
) -> Result<(String, Option<String>)> {
    // Extract body schema if present
    let body_schema = if let Some(handler) = &fixture.handler {
        handler.body_schema.as_ref()
    } else {
        None
    };

    if let Some(schema) = body_schema {
        let model_name_base = format!("{}Body", to_pascal_case(handler_name));
        let model_name = make_unique_name(&model_name_base, handler_names);
        let model_code = extract_body_model(schema, &model_name, body_type)?;
        Ok((model_code, Some(model_name)))
    } else {
        Ok((String::new(), None))
    }
}

/// Generate handler function (without decorator, for manual registration)
fn generate_handler_function_for_fixture(
    fixture: &Fixture,
    route: &str,
    method: &str,
    handler_name: &str,
    body_type: BodyType,
    model_name: Option<&str>,
) -> Result<String> {
    // Extract handler info from fixture
    let handler_opt = fixture.handler.as_ref();

    // Extract parameters
    let params = if let Some(handler) = handler_opt {
        if let Some(ref param_schema) = handler.parameters {
            extract_parameters(param_schema)?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Extract body schema
    let body_schema = if let Some(handler) = handler_opt {
        handler.body_schema.as_ref()
    } else {
        None
    };

    // Determine response body and status code
    let response_info = determine_response_body(&[fixture]);

    // Generate handler function
    let mut code = String::new();

    // Function signature
    code.push_str(&format!("def {}(\n", handler_name));

    // Add body parameter if present
    // IMPORTANT: All parameters must use their original names (no underscore prefix)
    // because Rust FFI passes them by name in kwargs.
    if body_schema.is_some() {
        let body_param_type = match body_type {
            BodyType::PlainDict => "dict[str, Any]".to_string(),
            _ => model_name.unwrap_or("dict[str, Any]").to_string(),
        };
        code.push_str(&format!("    body: {},\n", body_param_type));
    }

    // Add other parameters - required first, then optional
    for (param_name, param_type, is_required) in &params {
        if *is_required {
            code.push_str(&format!("    {}: {},\n", param_name, param_type));
        }
    }
    for (param_name, param_type, is_required) in &params {
        if !*is_required {
            code.push_str(&format!("    {}: {} | None = None,\n", param_name, param_type));
        }
    }

    code.push_str(") -> Any:\n");
    code.push_str(&format!(
        "    \"\"\"Handler for {} {}.\"\"\"\n",
        method.to_uppercase(),
        route
    ));

    // Function body
    if let Some((body_json, status_code)) = response_info {
        if status_code == 200 {
            // Default status code - return plain JSON
            code.push_str(&format!("    return {}\n", body_json));
        } else {
            // Custom status code - wrap in Response()
            code.push_str(&format!(
                "    return Response(content={}, status_code={})\n",
                body_json, status_code
            ));
        }
    } else {
        code.push_str("    # Echo back parameters for testing\n");
        code.push_str("    result: dict[str, Any] = {}\n");

        if body_schema.is_some() {
            code.push_str("    if body:\n");
            match body_type {
                BodyType::PlainDict | BodyType::TypedDict => {
                    // Already a dict at runtime
                    code.push_str("        result.update(body)\n");
                }
                BodyType::Dataclass => {
                    // Use dataclasses.asdict()
                    code.push_str("        result.update(asdict(body))\n");
                }
                BodyType::NamedTuple => {
                    // Use ._asdict() method
                    code.push_str("        result.update(body._asdict())\n");
                }
                BodyType::MsgspecStruct => {
                    // Use msgspec.to_builtins()
                    code.push_str("        result.update(msgspec.to_builtins(body))\n");
                }
                BodyType::Pydantic => {
                    // Use .model_dump()
                    code.push_str("        result.update(body.model_dump())\n");
                }
            }
        }

        for (param_name, _, _) in &params {
            code.push_str(&format!("    if {} is not None:\n", param_name));
            code.push_str(&format!("        result[\"{}\"] = {}\n", param_name, param_name));
        }

        code.push_str("    return result\n");
    }

    Ok(code)
}

/// Sanitize a string to be a valid Python identifier (lowercase snake_case)
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

/// Generate CORS preflight handler
#[allow(dead_code)]
fn generate_cors_preflight_handler(handler_name: &str, route: &str, _cors_config: &Value) -> Result<String> {
    let mut code = String::new();

    code.push_str(&format!("@options(\"{}\")\n", route));
    code.push_str(&format!("def {}() -> dict[str, Any]:\n", handler_name));
    code.push_str("    \"\"\"CORS preflight handler.\"\"\"\n");
    code.push_str("    # CORS is handled by Spikard middleware\n");
    code.push_str("    return {}\n");

    Ok(code)
}

/// Extract parameters from parameter schema
fn extract_parameters(schema: &Value) -> Result<Vec<(String, String, bool)>> {
    let mut params = Vec::new();

    if let Some(obj) = schema.as_object() {
        // Extract path parameters
        if let Some(path_params) = obj.get("path").and_then(|v| v.as_object()) {
            for (name, param_schema) in path_params {
                let param_type = json_type_to_python(param_schema)?;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, true));
            }
        }

        // Extract query parameters
        if let Some(query_params) = obj.get("query").and_then(|v| v.as_object()) {
            for (name, param_schema) in query_params {
                let param_type = json_type_to_python(param_schema)?;
                // Parameters are required by default unless marked optional: true
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }

        // Extract header parameters
        if let Some(headers) = obj.get("headers").and_then(|v| v.as_object()) {
            for (name, param_schema) in headers {
                let param_type = json_type_to_python(param_schema)?;
                // Parameters are required by default unless marked optional: true
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }

        // Extract cookie parameters
        if let Some(cookies) = obj.get("cookies").and_then(|v| v.as_object()) {
            for (name, param_schema) in cookies {
                let param_type = json_type_to_python(param_schema)?;
                // Parameters are required by default unless marked optional: true
                let is_optional = param_schema.get("optional").and_then(|v| v.as_bool()).unwrap_or(false);
                let is_required = !is_optional;
                let python_name = to_python_identifier(name);
                params.push((python_name, param_type, is_required));
            }
        }
    }

    Ok(params)
}

/// Convert a name to a valid Python identifier
///
/// NOTE: We do NOT add underscore suffixes for Python builtins because:
/// - FFI passes parameters by their exact schema name (e.g., "filter", "id", "type")
/// - Python allows shadowing builtins in function parameters (only shadows within scope)
/// - Adding suffixes would cause FFI mismatch (Rust passes "filter", Python expects "filter_")
fn to_python_identifier(name: &str) -> String {
    name.replace(['-', '.'], "_").to_lowercase()
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

/// Extract body model definition - generates code for different type systems
fn extract_body_model(schema: &Value, model_name: &str, body_type: BodyType) -> Result<String> {
    match body_type {
        BodyType::PlainDict => {
            // No model needed - handler will use dict[str, Any]
            Ok(String::new())
        }
        BodyType::TypedDict => generate_typed_dict(schema, model_name),
        BodyType::Dataclass => generate_dataclass(schema, model_name),
        BodyType::NamedTuple => generate_namedtuple(schema, model_name),
        BodyType::MsgspecStruct => generate_msgspec_struct(schema, model_name),
        BodyType::Pydantic => generate_pydantic_model(schema, model_name),
    }
}

/// Generate TypedDict definition
fn generate_typed_dict(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(TypedDict):\n", model_name));
    code.push_str("    \"\"\"Request body type (TypedDict - runtime is dict).\"\"\"\n\n");

    if let Some(obj) = schema.as_object() {
        if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
            let required_fields: Vec<String> = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            // Separate required and optional fields for consistent ordering
            let mut required_props: Vec<(&String, &Value)> = Vec::new();
            let mut optional_props: Vec<(&String, &Value)> = Vec::new();

            for (prop_name, prop_schema) in properties {
                if required_fields.contains(prop_name) {
                    required_props.push((prop_name, prop_schema));
                } else {
                    optional_props.push((prop_name, prop_schema));
                }
            }

            // Output required fields first
            for (prop_name, prop_schema) in required_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
            }

            // Then output optional fields
            for (prop_name, prop_schema) in optional_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {} | None\n", python_prop_name, prop_type));
            }
        }
    }

    Ok(code)
}

/// Generate dataclass definition
fn generate_dataclass(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str("@dataclass\n");
    code.push_str(&format!("class {}:\n", model_name));
    code.push_str("    \"\"\"Request body dataclass.\"\"\"\n\n");

    if let Some(obj) = schema.as_object() {
        if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
            let required_fields: Vec<String> = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            // Separate required and optional fields
            // dataclass requires all fields without defaults to come before fields with defaults
            let mut required_props: Vec<(&String, &Value)> = Vec::new();
            let mut optional_props: Vec<(&String, &Value)> = Vec::new();

            for (prop_name, prop_schema) in properties {
                if required_fields.contains(prop_name) {
                    required_props.push((prop_name, prop_schema));
                } else {
                    optional_props.push((prop_name, prop_schema));
                }
            }

            // Output required fields first
            for (prop_name, prop_schema) in required_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
            }

            // Then output optional fields
            for (prop_name, prop_schema) in optional_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
            }
        }
    }

    Ok(code)
}

/// Generate NamedTuple definition
fn generate_namedtuple(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(NamedTuple):\n", model_name));
    code.push_str("    \"\"\"Request body NamedTuple (immutable).\"\"\"\n\n");

    if let Some(obj) = schema.as_object() {
        if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
            let required_fields: Vec<String> = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            // Separate required and optional fields
            // NamedTuple requires all fields without defaults to come before fields with defaults
            let mut required_props: Vec<(&String, &Value)> = Vec::new();
            let mut optional_props: Vec<(&String, &Value)> = Vec::new();

            for (prop_name, prop_schema) in properties {
                if required_fields.contains(prop_name) {
                    required_props.push((prop_name, prop_schema));
                } else {
                    optional_props.push((prop_name, prop_schema));
                }
            }

            // Output required fields first
            for (prop_name, prop_schema) in required_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
            }

            // Then output optional fields
            for (prop_name, prop_schema) in optional_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
            }
        }
    }

    Ok(code)
}

/// Generate msgspec.Struct definition
fn generate_msgspec_struct(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(msgspec.Struct):\n", model_name));
    code.push_str("    \"\"\"Request body msgspec.Struct (fast typed).\"\"\"\n\n");

    if let Some(obj) = schema.as_object() {
        if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
            let required_fields: Vec<String> = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            // Separate required and optional fields for consistent ordering
            let mut required_props: Vec<(&String, &Value)> = Vec::new();
            let mut optional_props: Vec<(&String, &Value)> = Vec::new();

            for (prop_name, prop_schema) in properties {
                if required_fields.contains(prop_name) {
                    required_props.push((prop_name, prop_schema));
                } else {
                    optional_props.push((prop_name, prop_schema));
                }
            }

            // Output required fields first
            for (prop_name, prop_schema) in required_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
            }

            // Then output optional fields
            for (prop_name, prop_schema) in optional_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
            }
        }
    }

    Ok(code)
}

/// Generate Pydantic BaseModel definition
fn generate_pydantic_model(schema: &Value, model_name: &str) -> Result<String> {
    let mut code = String::new();
    code.push_str(&format!("class {}(BaseModel):\n", model_name));
    code.push_str("    \"\"\"Request body Pydantic model.\"\"\"\n\n");

    if let Some(obj) = schema.as_object() {
        if let Some(properties) = obj.get("properties").and_then(|v| v.as_object()) {
            let required_fields: Vec<String> = obj
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                .unwrap_or_default();

            // Separate required and optional fields for consistent ordering
            let mut required_props: Vec<(&String, &Value)> = Vec::new();
            let mut optional_props: Vec<(&String, &Value)> = Vec::new();

            for (prop_name, prop_schema) in properties {
                if required_fields.contains(prop_name) {
                    required_props.push((prop_name, prop_schema));
                } else {
                    optional_props.push((prop_name, prop_schema));
                }
            }

            // Output required fields first
            for (prop_name, prop_schema) in required_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
            }

            // Then output optional fields
            for (prop_name, prop_schema) in optional_props {
                let prop_type = json_type_to_python(prop_schema)?;
                let python_prop_name = to_python_identifier(prop_name);
                code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
            }
        }
    }

    Ok(code)
}

/// Convert JSON schema type to Python type annotation
fn json_type_to_python(schema: &Value) -> Result<String> {
    let schema_type = schema.get("type").and_then(|v| v.as_str()).unwrap_or("string");

    let type_str = match schema_type {
        "string" => {
            if let Some(format) = schema.get("format").and_then(|v| v.as_str()) {
                match format {
                    "uuid" => "UUID",
                    "date" => "date",
                    "date-time" => "datetime",
                    _ => "str",
                }
            } else {
                "str" // handles both enum and other string types
            }
        }
        "integer" => "int",
        "number" => "float",
        "boolean" => "bool",
        "array" => {
            if let Some(items) = schema.get("items") {
                let item_type = json_type_to_python(items)?;
                return Ok(format!("list[{}]", item_type));
            }
            "list[Any]"
        }
        "object" => "dict[str, Any]",
        _ => "Any",
    };

    Ok(type_str.to_string())
}

/// Determine the response body from fixtures
fn determine_response_body(fixtures: &[&Fixture]) -> Option<(String, u16)> {
    // Use the first successful response, returning (body, status_code)
    for fixture in fixtures {
        if fixture.expected_response.status_code >= 200 && fixture.expected_response.status_code < 300 {
            if let Some(ref body) = fixture.expected_response.body {
                // Convert JSON to Python dict literal
                return Some((json_to_python(body), fixture.expected_response.status_code));
            }
        }
    }

    None
}

/// Convert JSON value to Python dict literal
fn json_to_python(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(b) => if *b { "True" } else { "False" }.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            // Escape special characters
            let escaped = s
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\n")
                .replace('\r', "\\r")
                .replace('\t', "\\t")
                .replace('\0', "\\0");
            format!("\"{}\"", escaped)
        }
        Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(json_to_python).collect();
            format!("[{}]", items.join(", "))
        }
        Value::Object(obj) => {
            let items: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("\"{}\": {}", k, json_to_python(v)))
                .collect();
            format!("{{{}}}", items.join(", "))
        }
    }
}

/// Capitalize first letter
fn capitalize(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}

/// Convert snake_case or kebab-case to PascalCase
/// E.g., "post_cookies_samesite_strict" -> "PostCookiesSamesiteStrict"
fn to_pascal_case(s: &str) -> String {
    s.split(&['_', '-'][..])
        .filter(|part| !part.is_empty())
        .map(capitalize)
        .collect()
}
