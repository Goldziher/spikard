//! Python `OpenRPC` code generation.

use anyhow::Result;
use heck::ToSnakeCase;
use serde_json::Value;

use crate::codegen::openrpc::spec_parser::{
    OpenRpcMethod, OpenRpcSpec, get_method_params_class_name, get_result_class_name,
};

use super::OpenRpcGenerator;

/// Python `OpenRPC` code generator
pub struct PythonOpenRpcGenerator;

impl OpenRpcGenerator for PythonOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        code.push_str("# ruff: noqa: INP001\n");
        code.push_str("\"\"\"JSON-RPC 2.0 handlers generated from OpenRPC specification.\n\n");
        code.push_str("Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\"\"\"\n\n");

        code.push_str("from typing import Any\n\n");
        code.push_str("import msgspec\n\n");

        code.push_str("# ============================================================================\n");
        code.push_str("# Data Transfer Objects (DTOs)\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_dtos(&mut code, method)?;
        }

        code.push_str("# ============================================================================\n");
        code.push_str("# JSON-RPC Method Handlers\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_handler(&mut code, method)?;
        }

        code.push_str("# ============================================================================\n");
        code.push_str("# Method Router\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str(
            "async def handle_jsonrpc_call(method_name: str, params: Any, request_id: Any) -> dict[str, Any]:\n",
        );
        code.push_str("    \"\"\"Route JSON-RPC method calls to appropriate handlers.\"\"\"\n");
        code.push_str("    try:\n");

        for method in &spec.methods {
            let handler_name = handler_name_for_method(&method.name);
            code.push_str(&format!("        if method_name == \"{}\":\n", method.name));
            code.push_str(&format!("            result = await {handler_name}(params)\n"));
            code.push_str("            return {\"jsonrpc\": \"2.0\", \"result\": result, \"id\": request_id}\n");
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
        code.push_str("    async def rpc_handler(request: dict[str, Any]) -> dict[str, Any]:\n");
        code.push_str("        \"\"\"JSON-RPC 2.0 entrypoint.\"\"\"\n");
        code.push_str("        method = request.get(\"method\")\n");
        code.push_str("        if not isinstance(method, str):\n");
        code.push_str("            return {\n");
        code.push_str("                \"jsonrpc\": \"2.0\",\n");
        code.push_str("                \"error\": {\"code\": -32600, \"message\": \"Invalid request\"},\n");
        code.push_str("                \"id\": request.get(\"id\"),\n");
        code.push_str("            }\n");
        code.push_str("        params = request.get(\"params\", {})\n");
        code.push_str("        request_id = request.get(\"id\")\n");
        code.push_str("        return await handle_jsonrpc_call(method, params, request_id)\n\n");

        code.push_str("    # Call `app.run(...)` to start the JSON-RPC server.\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "python"
    }
}

fn generate_python_dtos(code: &mut String, method: &OpenRpcMethod) -> Result<()> {
    if !method.params.is_empty() {
        let params_class = get_method_params_class_name(&method.name);
        code.push_str(&format!("class {params_class}(msgspec.Struct, frozen=True):\n"));
        code.push_str(&format!(
            "    \"\"\"{}.\"\"\"\n",
            dto_docstring(
                method.description.as_deref(),
                &format!("{params_class} parameters"),
                Some("Parameters"),
            )
        ));

        for param in &method.params {
            let py_type = json_schema_to_python_type(&param.schema);
            let field_name = python_identifier(&param.name);
            code.push_str("    ");
            code.push_str(&render_field_definition(&field_name, &py_type, &param.name));
            code.push('\n');
        }
        code.push('\n');
    }

    let result_class = get_result_class_name(&method.name);
    code.push_str(&format!("class {result_class}(msgspec.Struct, frozen=True):\n"));
    code.push_str(&format!(
        "    \"\"\"{}.\"\"\"\n",
        dto_docstring(
            method.result.description.as_deref(),
            &format!("{result_class} result"),
            None
        )
    ));

    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for (field_name, field_schema) in props {
            let py_type = json_schema_to_python_type(field_schema);
            let python_name = python_identifier(field_name);
            code.push_str("    ");
            code.push_str(&render_field_definition(&python_name, &py_type, field_name));
            code.push('\n');
        }
    }
    code.push('\n');

    Ok(())
}

fn generate_python_handler(code: &mut String, method: &OpenRpcMethod) -> Result<()> {
    let handler_name = handler_name_for_method(&method.name);
    let params_class = get_method_params_class_name(&method.name);

    code.push_str(&format!("async def {handler_name}(params: Any) -> dict[str, Any]:\n"));
    code.push_str(&format!(
        "    \"\"\"{}.\"\"\"\n",
        handler_docstring(method.summary.as_deref(), method.description.as_deref(), &method.name)
    ));

    if !method.params.is_empty() {
        code.push_str(&format!(
            "    _parsed_params = msgspec.convert(params, type={params_class})\n"
        ));
    }

    code.push_str("\n    # TODO: Implement business logic.\n");
    code.push_str("    # This handler receives validated parameters and should return a JSON-RPC result object.\n\n");

    code.push_str("    result_data: dict[str, Any] = {}\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            code.push_str(&format!("    result_data[\"{field_name}\"] = \"TODO\"\n"));
        }
    }
    code.push_str("    return result_data\n\n");

    Ok(())
}

fn handler_name_for_method(method_name: &str) -> String {
    format!(
        "handle_{}",
        python_identifier(&method_name.replace('.', "_").replace('-', "_"))
    )
}

fn python_identifier(name: &str) -> String {
    if name.contains(['-', '.']) || name.chars().any(char::is_uppercase) {
        name.to_snake_case()
    } else {
        name.to_string()
    }
}

fn render_field_definition(field_name: &str, field_type: &str, wire_name: &str) -> String {
    if field_name == wire_name {
        format!("{field_name}: {field_type}")
    } else {
        format!("{field_name}: {field_type} = msgspec.field(name=\"{wire_name}\")")
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

fn json_schema_to_python_type(schema: &Value) -> String {
    if let Some(type_str) = schema.get("type") {
        match type_str.as_str() {
            Some("string") => "str".to_string(),
            Some("number") => "float".to_string(),
            Some("integer") => "int".to_string(),
            Some("boolean") => "bool".to_string(),
            Some("array") => {
                if let Some(items) = schema.get("items") {
                    format!("list[{}]", json_schema_to_python_type(items))
                } else {
                    "list[Any]".to_string()
                }
            }
            Some("object") => "dict[str, Any]".to_string(),
            _ => "Any".to_string(),
        }
    } else if schema.get("enum").is_some() {
        "str".to_string()
    } else {
        "Any".to_string()
    }
}
