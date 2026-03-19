//! Python `OpenRPC` code generation.

use anyhow::Result;
use heck::{ToPascalCase, ToSnakeCase};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

use crate::codegen::openrpc::spec_parser::{
    OpenRpcMethod, OpenRpcSpec, get_method_params_class_name, get_result_class_name, resolve_schema, schema_ref_name,
};

use super::OpenRpcGenerator;

/// Python `OpenRPC` code generator
pub struct PythonOpenRpcGenerator;

struct PythonGenerationState {
    emitted_classes: BTreeSet<String>,
}

impl PythonGenerationState {
    fn new() -> Self {
        Self {
            emitted_classes: BTreeSet::new(),
        }
    }
}

impl OpenRpcGenerator for PythonOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();
        let mut state = PythonGenerationState::new();

        code.push_str("# ruff: noqa: INP001, F401\n");
        code.push_str("\"\"\"JSON-RPC 2.0 handlers generated from OpenRPC specification.\n\n");
        code.push_str("Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\"\"\"\n\n");
        code.push_str("from __future__ import annotations\n\n");

        code.push_str("from datetime import date, datetime\n");
        code.push_str("from typing import Literal\n");
        code.push_str("from uuid import UUID\n\n");
        code.push_str("import msgspec\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# Shared Component DTOs\n");
        code.push_str("# ============================================================================\n\n");

        generate_component_dtos(&mut code, spec, &mut state)?;

        code.push_str("# ============================================================================\n");
        code.push_str("# Data Transfer Objects (DTOs)\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_dtos(&mut code, spec, method, &mut state)?;
        }

        code.push_str("# ============================================================================\n");
        code.push_str("# JSON-RPC Method Handlers\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_handler(&mut code, spec, method)?;
        }

        code.push_str("# ============================================================================\n");
        code.push_str("# Method Router\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str(
            "async def handle_jsonrpc_call(method_name: str, params: object, request_id: object) -> dict[str, object]:\n",
        );
        code.push_str("    \"\"\"Route JSON-RPC method calls to appropriate handlers.\"\"\"\n");
        code.push_str("    try:\n");
        code.push_str("        _ = params\n");

        for method in &spec.methods {
            let handler_name = handler_name_for_method(&method.name);
            code.push_str(&format!("        if method_name == \"{}\":\n", method.name));
            if method.params.is_empty() {
                code.push_str(&format!(
                    "            return {{\"jsonrpc\": \"2.0\", \"result\": msgspec.to_builtins(await {handler_name}()), \"id\": request_id}}\n"
                ));
            } else {
                let params_class = get_method_params_class_name(&method.name);
                code.push_str(&format!(
                    "            return {{\"jsonrpc\": \"2.0\", \"result\": msgspec.to_builtins(await {handler_name}(msgspec.convert(params if params is not None else {{}}, type={params_class}))), \"id\": request_id}}\n"
                ));
            }
        }

        code.push_str("        return {\n");
        code.push_str("            \"jsonrpc\": \"2.0\",\n");
        code.push_str("            \"error\": {\"code\": -32601, \"message\": \"Method not found\"},\n");
        code.push_str("            \"id\": request_id,\n");
        code.push_str("        }\n");
        code.push_str("    except Exception as e:\n");
        code.push_str("        return {\n");
        code.push_str("            \"jsonrpc\": \"2.0\",\n");
        code.push_str(
            "            \"error\": {\"code\": -32603, \"message\": \"Internal error\", \"data\": str(e)},\n",
        );
        code.push_str("            \"id\": request_id,\n");
        code.push_str("        }\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# Example Usage\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("if __name__ == \"__main__\":\n");
        code.push_str("    from spikard import Spikard\n\n");

        code.push_str("    app = Spikard()\n\n");

        code.push_str("    @app.post(\"/rpc\")\n");
        code.push_str("    async def rpc_handler(request: dict[str, object]) -> dict[str, object]:\n");
        code.push_str("        \"\"\"JSON-RPC 2.0 entrypoint.\"\"\"\n");
        code.push_str("        method = request.get(\"method\")\n");
        code.push_str("        if not isinstance(method, str):\n");
        code.push_str("            return {\n");
        code.push_str("                \"jsonrpc\": \"2.0\",\n");
        code.push_str("                \"error\": {\"code\": -32600, \"message\": \"Invalid request\"},\n");
        code.push_str("                \"id\": request.get(\"id\"),\n");
        code.push_str("            }\n");
        code.push_str("        params = request.get(\"params\")\n");
        code.push_str("        request_id = request.get(\"id\")\n");
        code.push_str("        return await handle_jsonrpc_call(method, params, request_id)\n\n");

        code.push_str("    # Call `app.run(...)` to start the JSON-RPC server.\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "python"
    }
}

fn generate_component_dtos(code: &mut String, spec: &OpenRpcSpec, state: &mut PythonGenerationState) -> Result<()> {
    let components = spec
        .components
        .schemas
        .iter()
        .map(|(name, schema)| (name.clone(), schema))
        .collect::<BTreeMap<_, _>>();

    for (name, schema) in components {
        generate_struct_family(code, spec, &component_class_name(&name), schema, Some(&name), state)?;
    }

    Ok(())
}

fn generate_python_dtos(
    code: &mut String,
    spec: &OpenRpcSpec,
    method: &OpenRpcMethod,
    state: &mut PythonGenerationState,
) -> Result<()> {
    if !method.params.is_empty() {
        let params_class = get_method_params_class_name(&method.name);
        let params_schema = params_object_schema(method);
        generate_struct_family(
            code,
            spec,
            &params_class,
            &params_schema,
            method.description.as_deref(),
            state,
        )?;
    }

    if schema_ref_name(&method.result.schema).is_none() {
        let result_class = get_result_class_name(&method.name);
        generate_struct_family(
            code,
            spec,
            &result_class,
            &method.result.schema,
            method.result.description.as_deref(),
            state,
        )?;
    }

    Ok(())
}

fn params_object_schema(method: &OpenRpcMethod) -> Value {
    let mut properties = serde_json::Map::new();
    let mut required = Vec::new();

    for param in &method.params {
        properties.insert(param.name.clone(), param.schema.clone());
        if param.required {
            required.push(Value::String(param.name.clone()));
        }
    }

    Value::Object(serde_json::Map::from_iter([
        ("type".to_string(), Value::String("object".to_string())),
        ("properties".to_string(), Value::Object(properties)),
        ("required".to_string(), Value::Array(required)),
    ]))
}

fn generate_struct_family(
    code: &mut String,
    spec: &OpenRpcSpec,
    class_name: &str,
    schema: &Value,
    description: Option<&str>,
    state: &mut PythonGenerationState,
) -> Result<()> {
    if !state.emitted_classes.insert(class_name.to_string()) {
        return Ok(());
    }

    generate_nested_structs(code, spec, class_name, schema, state)?;
    generate_struct_class(code, spec, class_name, schema, description)?;
    Ok(())
}

fn generate_nested_structs(
    code: &mut String,
    spec: &OpenRpcSpec,
    parent_class_name: &str,
    schema: &Value,
    state: &mut PythonGenerationState,
) -> Result<()> {
    let resolved = resolve_schema(spec, schema);
    let Some(properties) = resolved.get("properties").and_then(Value::as_object) else {
        return Ok(());
    };

    for (field_name, field_schema) in properties {
        if let Some(class_name) = inline_object_class_name(spec, parent_class_name, field_name, field_schema) {
            generate_struct_family(
                code,
                spec,
                &class_name,
                field_schema,
                schema_description(field_schema),
                state,
            )?;
        }

        if let Some(class_name) = inline_array_item_class_name(spec, parent_class_name, field_name, field_schema)
            && let Some(item_schema) = resolve_schema(spec, field_schema).get("items")
        {
            generate_struct_family(
                code,
                spec,
                &class_name,
                item_schema,
                schema_description(item_schema),
                state,
            )?;
        }
    }

    Ok(())
}

fn generate_struct_class(
    code: &mut String,
    spec: &OpenRpcSpec,
    class_name: &str,
    schema: &Value,
    description: Option<&str>,
) -> Result<()> {
    let resolved = resolve_schema(spec, schema);

    code.push_str(&format!("class {class_name}(msgspec.Struct, frozen=True):\n"));
    code.push_str(&format!(
        "    \"\"\"{}.\"\"\"\n",
        dto_docstring(description, &format!("{class_name} object"), None)
    ));

    let Some(props) = resolved.get("properties").and_then(Value::as_object) else {
        let wrapper_type = json_schema_to_python_type(spec, schema, None, None, None);
        code.push_str("    ");
        code.push_str(&render_field_definition("value", &wrapper_type, "value", true));
        code.push_str("\n\n");
        return Ok(());
    };

    if props.is_empty() {
        code.push_str("    pass\n\n");
        return Ok(());
    }

    let required_fields = required_field_names(resolved);
    for (field_name, field_schema) in props {
        let py_type = json_schema_to_python_type(spec, field_schema, Some(class_name), Some(field_name), None);
        let python_name = python_identifier(field_name);
        code.push_str("    ");
        code.push_str(&render_field_definition(
            &python_name,
            &py_type,
            field_name,
            required_fields.contains(field_name),
        ));
        code.push('\n');
    }
    code.push('\n');

    Ok(())
}

fn generate_python_handler(code: &mut String, spec: &OpenRpcSpec, method: &OpenRpcMethod) -> Result<()> {
    let handler_name = handler_name_for_method(&method.name);
    let result_type = result_type_for_method(spec, method);

    if method.params.is_empty() {
        code.push_str(&format!("async def {handler_name}() -> {result_type}:\n"));
    } else {
        let params_class = get_method_params_class_name(&method.name);
        code.push_str(&format!(
            "async def {handler_name}(params: {params_class}) -> {result_type}:\n"
        ));
    }
    code.push_str(&format!(
        "    \"\"\"{}.\"\"\"\n",
        handler_docstring(method.summary.as_deref(), method.description.as_deref(), &method.name)
    ));
    if !method.params.is_empty() {
        code.push_str("    _ = params\n");
    }
    code.push_str("    # TODO: Implement business logic.\n");
    code.push_str(
        "    # This handler receives validated parameters and should return a typed JSON-RPC result object.\n\n",
    );
    code.push_str("    return ");
    code.push_str(&render_python_placeholder(
        spec,
        &method.result.schema,
        Some(&result_type),
        1,
    ));
    code.push_str("\n\n");

    Ok(())
}

fn handler_name_for_method(method_name: &str) -> String {
    format!(
        "handle_{}",
        python_identifier(&method_name.replace('.', "_").replace('-', "_"))
    )
}

fn component_class_name(name: &str) -> String {
    name.to_pascal_case()
}

fn schema_description<'a>(schema: &'a Value) -> Option<&'a str> {
    resolve_inline_schema(schema)
        .get("description")
        .and_then(Value::as_str)
        .filter(|value| !value.trim().is_empty())
}

fn resolve_inline_schema(schema: &Value) -> &Value {
    schema
}

fn result_type_for_method(spec: &OpenRpcSpec, method: &OpenRpcMethod) -> String {
    schema_ref_name(&method.result.schema)
        .map(component_class_name)
        .unwrap_or_else(|| get_result_class_name(&method.name))
}

fn python_identifier(name: &str) -> String {
    if name.contains(['-', '.']) || name.chars().any(char::is_uppercase) {
        name.to_snake_case()
    } else {
        name.to_string()
    }
}

fn render_field_definition(field_name: &str, field_type: &str, wire_name: &str, required: bool) -> String {
    let annotated_type = if required || field_type.ends_with(" | None") {
        field_type.to_string()
    } else {
        format!("{field_type} | None")
    };

    match (field_name == wire_name, required) {
        (true, true) => format!("{field_name}: {annotated_type}"),
        (false, true) => format!("{field_name}: {annotated_type} = msgspec.field(name=\"{wire_name}\")"),
        (true, false) => format!("{field_name}: {annotated_type} = None"),
        (false, false) => {
            format!("{field_name}: {annotated_type} = msgspec.field(default=None, name=\"{wire_name}\")")
        }
    }
}

fn dto_docstring(description: Option<&str>, fallback: &str, suffix: Option<&str>) -> String {
    let base = description.unwrap_or(fallback).trim();
    let decorated = match suffix {
        Some(value) => format!("{base} ({value})"),
        None => base.to_string(),
    };
    ensure_sentence(&decorated)
}

fn handler_docstring(summary: Option<&str>, description: Option<&str>, method_name: &str) -> String {
    let base = summary
        .or(description)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or(method_name);
    ensure_sentence(base)
}

fn ensure_sentence(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.ends_with(['.', '!', '?']) {
        trimmed.to_string()
    } else {
        format!("{trimmed}.")
    }
}

fn required_field_names(schema: &Value) -> BTreeSet<String> {
    schema
        .get("required")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .map(ToOwned::to_owned)
        .collect()
}

fn json_schema_to_python_type(
    spec: &OpenRpcSpec,
    schema: &Value,
    parent_class_name: Option<&str>,
    field_name: Option<&str>,
    inline_name: Option<&str>,
) -> String {
    if let Some(reference_name) = schema_ref_name(schema) {
        return component_class_name(reference_name);
    }

    let resolved = resolve_schema(spec, schema);
    if let Some(enum_values) = json_enum_to_python_literal(resolved) {
        return enum_values;
    }

    if let Some(type_str) = resolved.get("type") {
        match type_str.as_str() {
            Some("string") => string_schema_to_python_type(resolved),
            Some("number") => "float".to_string(),
            Some("integer") => "int".to_string(),
            Some("boolean") => "bool".to_string(),
            Some("array") => {
                if let Some(items) = resolved.get("items") {
                    let nested_name = parent_class_name
                        .zip(field_name)
                        .and_then(|(parent, field)| inline_array_item_class_name(spec, parent, field, schema));
                    format!(
                        "list[{}]",
                        json_schema_to_python_type(spec, items, None, None, nested_name.as_deref())
                    )
                } else {
                    "list[object]".to_string()
                }
            }
            Some("object") => {
                if resolved
                    .get("properties")
                    .and_then(Value::as_object)
                    .is_some_and(|props| !props.is_empty())
                {
                    inline_name
                        .map(ToOwned::to_owned)
                        .or_else(|| {
                            parent_class_name
                                .zip(field_name)
                                .and_then(|(parent, field)| inline_object_class_name(spec, parent, field, schema))
                        })
                        .unwrap_or_else(|| "dict[str, object]".to_string())
                } else {
                    "dict[str, object]".to_string()
                }
            }
            _ => "object".to_string(),
        }
    } else {
        "object".to_string()
    }
}

fn string_schema_to_python_type(schema: &Value) -> String {
    match schema.get("format").and_then(Value::as_str) {
        Some("date") => "date".to_string(),
        Some("date-time") => "datetime".to_string(),
        Some("uuid") => "UUID".to_string(),
        _ => "str".to_string(),
    }
}

fn json_enum_to_python_literal(schema: &Value) -> Option<String> {
    let values = schema.get("enum")?.as_array()?;
    let mut rendered = Vec::new();
    for value in values {
        match value {
            Value::String(string) => rendered.push(format!("{string:?}")),
            Value::Number(number) => rendered.push(number.to_string()),
            Value::Bool(boolean) => rendered.push(boolean.to_string()),
            _ => return None,
        }
    }

    (!rendered.is_empty()).then(|| format!("Literal[{}]", rendered.join(", ")))
}

fn inline_object_class_name(
    spec: &OpenRpcSpec,
    parent_class_name: &str,
    field_name: &str,
    schema: &Value,
) -> Option<String> {
    if schema_ref_name(schema).is_some() {
        return None;
    }

    let resolved = resolve_schema(spec, schema);
    resolved
        .get("type")
        .and_then(Value::as_str)
        .filter(|type_name| *type_name == "object")
        .and_then(|_| resolved.get("properties").and_then(Value::as_object))
        .filter(|properties| !properties.is_empty())
        .map(|_| format!("{parent_class_name}{}", field_name.to_pascal_case()))
}

fn inline_array_item_class_name(
    spec: &OpenRpcSpec,
    parent_class_name: &str,
    field_name: &str,
    schema: &Value,
) -> Option<String> {
    let resolved = resolve_schema(spec, schema);
    let items = resolved.get("items")?;
    inline_object_class_name(spec, parent_class_name, &format!("{field_name} item"), items)
        .map(|_| format!("{parent_class_name}{}Item", field_name.to_pascal_case()))
}

fn render_python_placeholder(spec: &OpenRpcSpec, schema: &Value, expected_type: Option<&str>, indent: usize) -> String {
    let resolved = resolve_schema(spec, schema);
    let expected_type = expected_type.map(strip_optional_type);

    if let Some(class_name) = expected_type.filter(|name| is_generated_python_struct_type(name)) {
        let has_named_object_fields = resolved
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|type_name| type_name == "object")
            && resolved
                .get("properties")
                .and_then(Value::as_object)
                .is_some_and(|properties| !properties.is_empty());

        if !has_named_object_fields {
            let child_indent = "    ".repeat(indent + 1);
            let base_indent = "    ".repeat(indent);
            let inner = render_python_placeholder(spec, schema, None, indent + 1);
            return format!("{class_name}(\n{child_indent}value={inner}\n{base_indent})");
        }
    }

    if let Some(literal) = schema
        .get("enum")
        .and_then(Value::as_array)
        .and_then(|values| values.first())
    {
        return match literal {
            Value::String(string) => format!("{string:?}"),
            Value::Number(number) => number.to_string(),
            Value::Bool(boolean) => boolean.to_string(),
            _ => "None".to_string(),
        };
    }

    match resolved.get("type").and_then(Value::as_str) {
        Some("string") => render_string_placeholder(resolved, expected_type),
        Some("integer") => "0".to_string(),
        Some("number") => "0.0".to_string(),
        Some("boolean") => "False".to_string(),
        Some("array") => {
            let item_schema = resolved.get("items");
            let item_value = item_schema
                .map(|items| {
                    render_python_placeholder(
                        spec,
                        items,
                        Some(&json_schema_to_python_type(spec, items, None, None, None)),
                        indent,
                    )
                })
                .unwrap_or_else(|| "{}".to_string());
            format!("[{item_value}]")
        }
        Some("object") => render_python_object_placeholder(spec, resolved, expected_type, indent),
        _ => "None".to_string(),
    }
}

fn render_string_placeholder(schema: &Value, expected_type: Option<&str>) -> String {
    match expected_type {
        Some("UUID") => "UUID(\"00000000-0000-0000-0000-000000000000\")".to_string(),
        Some("date") => "date.fromisoformat(\"1970-01-01\")".to_string(),
        Some("datetime") => "datetime.fromisoformat(\"1970-01-01T00:00:00+00:00\")".to_string(),
        Some(type_name) if type_name.starts_with("Literal[") => schema
            .get("enum")
            .and_then(Value::as_array)
            .and_then(|values| values.first())
            .and_then(Value::as_str)
            .map(|value| format!("{value:?}"))
            .unwrap_or_else(|| "\"TODO\"".to_string()),
        _ => "\"TODO\"".to_string(),
    }
}

fn render_python_object_placeholder(
    spec: &OpenRpcSpec,
    schema: &Value,
    expected_type: Option<&str>,
    indent: usize,
) -> String {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return "{}".to_string();
    };

    if properties.is_empty() {
        return "{}".to_string();
    }

    let base_indent = "    ".repeat(indent);
    let child_indent = "    ".repeat(indent + 1);
    let use_struct = expected_type.is_some_and(is_generated_python_struct_type);
    let parent_class_name = expected_type.filter(|name| is_generated_python_struct_type(name));

    let rendered = properties
        .iter()
        .map(|(name, value)| {
            let field_type = json_schema_to_python_type(spec, value, parent_class_name, Some(name), None);
            let placeholder = render_python_placeholder(spec, value, Some(&field_type), indent + 1);
            if use_struct {
                format!("{child_indent}{}={placeholder}", python_identifier(name))
            } else {
                format!("{child_indent}\"{name}\": {placeholder}")
            }
        })
        .collect::<Vec<_>>()
        .join(",\n");

    if let Some(class_name) = expected_type.filter(|name| is_generated_python_struct_type(name)) {
        format!("{class_name}(\n{rendered}\n{base_indent})")
    } else {
        format!("{{\n{rendered}\n{base_indent}}}")
    }
}

fn strip_optional_type(type_name: &str) -> &str {
    type_name.split(" | ").next().unwrap_or(type_name).trim()
}

fn is_generated_python_struct_type(type_name: &str) -> bool {
    matches!(type_name.chars().next(), Some(ch) if ch.is_ascii_uppercase())
        && !matches!(type_name, "UUID" | "date" | "datetime")
        && !type_name.starts_with("Literal[")
}
