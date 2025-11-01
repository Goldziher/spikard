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

use crate::fixture_analysis::infer_body_schema;

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

    // Generate main app file
    let app_content = generate_app_file(&fixtures_by_category)?;
    fs::write(app_dir.join("main.py"), app_content).context("Failed to write main.py")?;

    // Generate __init__.py
    fs::write(app_dir.join("__init__.py"), "\"\"\"E2E test application.\"\"\"\n")
        .context("Failed to write __init__.py")?;

    println!("  ✓ Generated app/main.py");
    Ok(())
}

/// Generate the main app file with all handlers
fn generate_app_file(fixtures_by_category: &HashMap<String, Vec<Fixture>>) -> Result<String> {
    let mut code = String::new();

    // Imports
    code.push_str("\"\"\"Generated E2E test application.\"\"\"\n\n");
    code.push_str("from dataclasses import asdict, dataclass\n");
    code.push_str("from datetime import date, datetime\n");
    code.push_str("from enum import Enum\n");
    code.push_str("from typing import Any, NamedTuple, TypedDict\n");
    code.push_str("from uuid import UUID\n\n");
    code.push_str("import msgspec\n");
    code.push_str("from pydantic import BaseModel, Field\n");
    code.push_str("from spikard import Spikard, get, post, put, patch, delete, head, options, trace\n\n");

    // Create app instance
    code.push_str("app = Spikard()\n\n");

    // Track handler names to make them unique
    let mut handler_names = HashMap::new();

    // Collect all fixtures across categories
    // INCLUDE validation_errors for schema inference (to identify required fields)
    let mut all_fixtures = Vec::new();
    for (_category, fixtures) in fixtures_by_category.iter() {
        // Include ALL fixtures - validation_errors are crucial for schema inference
        // (they identify required fields via "missing" errors)
        all_fixtures.extend(fixtures.iter().cloned());
    }

    // Group all fixtures by route and method
    let routes = group_fixtures_by_route(&all_fixtures);

    // Sort routes to ensure more specific routes (without path params) come before generic ones
    // This prevents /items/{id} from matching before /items/unicode
    let mut sorted_routes: Vec<_> = routes.into_iter().collect();
    sorted_routes.sort_by(|a, b| {
        let (route_a, method_a) = &a.0;
        let (route_b, method_b) = &b.0;

        // First, sort by presence of path parameters (routes without params first)
        let has_param_a = route_a.contains('{');
        let has_param_b = route_b.contains('{');

        match (has_param_a, has_param_b) {
            (false, true) => std::cmp::Ordering::Less, // a (no params) before b (has params)
            (true, false) => std::cmp::Ordering::Greater, // b (no params) before a (has params)
            _ => {
                // If both have params or both don't, sort by path length (longer first for specificity)
                // Then by route name, then by method
                route_b
                    .len()
                    .cmp(&route_a.len())
                    .then_with(|| route_a.cmp(route_b))
                    .then_with(|| method_a.cmp(method_b))
            }
        }
    });

    // Generate handlers - rotate through all type systems for comprehensive testing
    for (handler_index, ((route, method), route_fixtures)) in sorted_routes.into_iter().enumerate() {
        let body_type = BodyType::for_index(handler_index);
        let handler = generate_handler(&route, &method, &route_fixtures, &mut handler_names, body_type)?;
        if !handler.is_empty() {
            code.push_str(&handler);
            code.push_str("\n\n");
        }
    }

    code.push_str("\nif __name__ == \"__main__\":\n");
    code.push_str("    app.run()\n");

    Ok(code)
}

/// Group fixtures by (route, method)
fn group_fixtures_by_route(fixtures: &[Fixture]) -> HashMap<(String, String), Vec<&Fixture>> {
    let mut grouped: HashMap<(String, String), Vec<&Fixture>> = HashMap::new();

    for fixture in fixtures {
        // Use handler.route if available, otherwise fall back to request.path (without query string)
        let route = if let Some(ref handler) = fixture.handler {
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

        let method = fixture.request.method.to_uppercase();
        grouped.entry((route, method)).or_default().push(fixture);
    }

    grouped
}

/// Generate a Python handler function
fn generate_handler(
    route: &str,
    method: &str,
    fixtures: &[&Fixture],
    handler_names: &mut HashMap<String, usize>,
    body_type: BodyType,
) -> Result<String> {
    // All HTTP methods are now supported
    // (no need to skip any methods)

    // Generate unique handler name
    let base_handler_name = generate_handler_name(route, method);
    let handler_name = make_unique_name(&base_handler_name, handler_names);

    // Try to get handler from first fixture, but handle case where there is no handler
    let first = fixtures[0];
    let handler_opt = first.handler.as_ref();

    // Extract parameters from handler schema if available
    let params = if let Some(handler) = handler_opt {
        if let Some(ref param_schema) = handler.parameters {
            extract_parameters(param_schema)?
        } else {
            vec![]
        }
    } else {
        vec![]
    };

    // Extract body schema - try explicit first, then infer from fixtures
    // Per RFC 9110/5789: No method syntactically REQUIRES a body
    let body_schema = if let Some(handler) = handler_opt {
        if let Some(ref explicit_schema) = handler.body_schema {
            Some(explicit_schema.clone())
        } else {
            // No explicit schema, try inference
            infer_body_schema(fixtures)
        }
    } else {
        // No handler field, try inference
        infer_body_schema(fixtures)
    };

    let (body_model, model_name) = if let Some(ref schema) = body_schema {
        let model_name_base = format!("{}Body", capitalize(&handler_name));
        let model_name = make_unique_name(&model_name_base, handler_names);
        let model_code = extract_body_model(schema, &model_name, body_type)?;
        (Some(model_code), Some(model_name))
    } else {
        (None, None)
    };

    // Determine response based on fixtures
    let response_body = determine_response_body(fixtures);

    // Generate the handler
    let mut code = String::new();

    // Add model definition if we have a body
    if let Some(ref model_code) = body_model {
        code.push_str(model_code);
        code.push_str("\n\n");
    }

    // Add decorator
    // Emit body_schema only when present (RFC 9110: no method requires a body)
    if let Some(ref schema) = body_schema {
        // Serialize the schema as JSON then convert to Python dict syntax
        let schema_json = serde_json::to_string(schema).unwrap_or_else(|_| "{}".to_string());
        let schema_python = json_to_python_dict(&schema_json);
        code.push_str(&format!(
            "@{}(\"{}\", body_schema={})\n",
            method.to_lowercase(),
            route,
            schema_python
        ));
    } else {
        // No body schema - decorator without body_schema parameter
        code.push_str(&format!("@{}(\"{}\")\n", method.to_lowercase(), route));
    }

    // Function signature
    code.push_str(&format!("def {}(\n", handler_name));

    // Add body parameter if present - type annotation depends on BodyType
    if body_schema.is_some() {
        let body_param_type = match body_type {
            BodyType::PlainDict => "dict[str, Any]".to_string(),
            _ => model_name.as_deref().unwrap_or("dict[str, Any]").to_string(),
        };
        code.push_str(&format!("    body: {},\n", body_param_type));
    }

    // Add other parameters - required first, then optional (Python syntax requirement)
    // First pass: required parameters
    for (param_name, param_type, is_required) in &params {
        if *is_required {
            code.push_str(&format!("    {}: {},\n", param_name, param_type));
        }
    }
    // Second pass: optional parameters
    for (param_name, param_type, is_required) in &params {
        if !*is_required {
            code.push_str(&format!("    {}: {} | None = None,\n", param_name, param_type));
        }
    }

    code.push_str(") -> Any:\n");
    code.push_str(&format!("    \"\"\"Handler for {} {}.\"\"\"\n", method, route));

    // Function body - return expected response or echo parameters
    if let Some(body_json) = response_body {
        // Use the expected response from fixtures if available
        code.push_str(&format!("    return {}\n", body_json));
    } else {
        // Echo back the parameters for validation testing
        code.push_str("    # Echo back parameters for testing\n");
        code.push_str("    result = {}\n");

        // Add body parameters to result - conversion depends on BodyType
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
                    // Use ._asdict()
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

        // Add other parameters to result
        for (param_name, _param_type, _is_required) in &params {
            code.push_str(&format!("    if {} is not None:\n", param_name));
            code.push_str(&format!("        result[\"{}\"] = {}\n", param_name, param_name));
        }

        code.push_str("    return result\n");
    }

    Ok(code)
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

            for (prop_name, prop_schema) in properties {
                let prop_type = json_type_to_python(prop_schema)?;
                let is_required = required_fields.contains(prop_name);
                let python_prop_name = to_python_identifier(prop_name);

                if is_required {
                    code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
                } else {
                    code.push_str(&format!("    {}: {} | None\n", python_prop_name, prop_type));
                }
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

            for (prop_name, prop_schema) in properties {
                let prop_type = json_type_to_python(prop_schema)?;
                let is_required = required_fields.contains(prop_name);
                let python_prop_name = to_python_identifier(prop_name);

                if is_required {
                    code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
                } else {
                    code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
                }
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

            for (prop_name, prop_schema) in properties {
                let prop_type = json_type_to_python(prop_schema)?;
                let is_required = required_fields.contains(prop_name);
                let python_prop_name = to_python_identifier(prop_name);

                if is_required {
                    code.push_str(&format!("    {}: {}\n", python_prop_name, prop_type));
                } else {
                    code.push_str(&format!("    {}: {} | None = None\n", python_prop_name, prop_type));
                }
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
fn determine_response_body(fixtures: &[&Fixture]) -> Option<String> {
    // Use the first successful response
    for fixture in fixtures {
        if fixture.expected_response.status_code >= 200 && fixture.expected_response.status_code < 300 {
            if let Some(ref body) = fixture.expected_response.body {
                // Convert JSON to Python dict literal
                return Some(json_to_python(body));
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

/// Generate handler name from route and method
fn generate_handler_name(route: &str, method: &str) -> String {
    let method_part = method.to_lowercase();

    // Remove type annotations from path params like {count:int}
    let route_cleaned = route
        .split('/')
        .map(|segment| {
            if segment.starts_with('{') && segment.contains(':') {
                // Extract just the param name: {count:int} -> count
                segment.trim_start_matches('{').split(':').next().unwrap_or(segment)
            } else {
                segment
            }
        })
        .collect::<Vec<_>>()
        .join("/");

    let mut route_part = route_cleaned
        .replace('/', "_")
        .replace(['{', '}', '-', ':', '.'], "")
        .trim_matches('_')
        .to_string();

    // If route_part starts with a digit, prefix with underscore
    if route_part.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        route_part = format!("_{}", route_part);
    }

    if route_part.is_empty() {
        format!("{}_root", method_part)
    } else {
        format!("{}_{}", method_part, route_part)
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
