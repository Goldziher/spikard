//! Python OpenRPC code generation.

use anyhow::Result;
use serde_json::Value;

use crate::codegen::openrpc::spec_parser::{OpenRpcMethod, OpenRpcSpec};

use super::OpenRpcGenerator;

/// Python OpenRPC code generator
pub struct PythonOpenRpcGenerator;

impl OpenRpcGenerator for PythonOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        // Header
        code.push_str("#!/usr/bin/env python3\n");
        code.push_str("\"\"\"JSON-RPC 2.0 handlers generated from OpenRPC specification.\n\n");
        code.push_str("Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push_str("\n\"\"\"\n\n");

        // Imports
        code.push_str("from typing import Any, Dict, Optional, Union\n");
        code.push_str("import json\n");
        code.push_str("import msgspec\n\n");

        // Generate DTO classes for each method
        code.push_str("# ============================================================================\n");
        code.push_str("# Data Transfer Objects (DTOs)\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_dtos(&mut code, method)?;
        }

        // Generate handler functions
        code.push_str("# ============================================================================\n");
        code.push_str("# JSON-RPC Method Handlers\n");
        code.push_str("# ============================================================================\n\n");

        for method in &spec.methods {
            generate_python_handler(&mut code, method)?;
        }

        // Generate method router
        code.push_str("# ============================================================================\n");
        code.push_str("# Method Router\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str(
            "async def handle_jsonrpc_call(method_name: str, params: Any, request_id: Any) -> Dict[str, Any]:\n",
        );
        code.push_str("    \"\"\"\n");
        code.push_str("    Route JSON-RPC method calls to appropriate handlers.\n");
        code.push('\n');
        code.push_str("    Args:\n");
        code.push_str("        method_name: The JSON-RPC method name\n");
        code.push_str("        params: The parameters for the method\n");
        code.push_str("        request_id: The JSON-RPC request ID\n");
        code.push('\n');
        code.push_str("    Returns:\n");
        code.push_str("        A JSON-RPC 2.0 response object\n");
        code.push_str("    \"\"\"\n");
        code.push_str("    try:\n");

        for method in &spec.methods {
            let handler_name = format!("handle_{}", method.name.replace('.', "_"));
            code.push_str(&format!("        if method_name == \"{}\":\n", method.name));
            code.push_str(&format!("            result = await {}(params)\n", handler_name));
            code.push_str("            return {\"jsonrpc\": \"2.0\", \"result\": result, \"id\": request_id}\n");
        }

        code.push_str("        else:\n");
        code.push_str("            return {\n");
        code.push_str("                \"jsonrpc\": \"2.0\",\n");
        code.push_str("                \"error\": {\"code\": -32601, \"message\": \"Method not found\"},\n");
        code.push_str("                \"id\": request_id,\n");
        code.push_str("            }\n");
        code.push_str("    except Exception as e:\n");
        code.push_str("        return {\n");
        code.push_str("            \"jsonrpc\": \"2.0\",\n");
        code.push_str(
            "            \"error\": {\"code\": -32603, \"message\": \"Internal error\", \"data\": str(e)},\n",
        );
        code.push_str("            \"id\": request_id,\n");
        code.push_str("        }\n\n");

        // Generate example usage
        code.push_str("# ============================================================================\n");
        code.push_str("# Example Usage\n");
        code.push_str("# ============================================================================\n\n");

        code.push_str("if __name__ == \"__main__\":\n");
        code.push_str("    import asyncio\n");
        code.push_str("    from spikard import Spikard\n\n");

        code.push_str("    app = Spikard()\n\n");

        code.push_str("    @app.post(\"/rpc\")\n");
        code.push_str("    async def rpc_handler(request: Dict[str, Any]) -> Dict[str, Any]:\n");
        code.push_str("        \"\"\"JSON-RPC 2.0 entrypoint.\"\"\"\n");
        code.push_str("        method = request.get(\"method\")\n");
        code.push_str("        params = request.get(\"params\", {})\n");
        code.push_str("        request_id = request.get(\"id\")\n");
        code.push_str("        return await handle_jsonrpc_call(method, params, request_id)\n\n");

        code.push_str("    # app.run(host=\"0.0.0.0\", port=8000)\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "python"
    }
}

fn generate_python_dtos(code: &mut String, method: &OpenRpcMethod) -> Result<()> {
    // Generate params DTO
    if !method.params.is_empty() {
        let params_class = get_python_params_class_name(&method.name);
        code.push_str(&format!("class {}(msgspec.Struct, frozen=True):\n", params_class));
        code.push_str("    \"\"\"\n");
        if let Some(desc) = &method.description {
            code.push_str("    ");
            code.push_str(desc);
            code.push_str(" (Parameters)\n");
        }
        code.push_str("    \"\"\"\n");

        for param in &method.params {
            let py_type = json_schema_to_python_type(&param.schema);
            code.push_str(&format!("    {}: {}\n", param.name, py_type));
        }
        code.push_str("\n\n");
    }

    // Generate result DTO
    let result_class = get_python_result_class_name(&method.name);
    code.push_str(&format!("class {}(msgspec.Struct, frozen=True):\n", result_class));
    code.push_str("    \"\"\"\n");
    if let Some(desc) = &method.result.description {
        code.push_str("    ");
        code.push_str(desc);
        code.push('\n');
    }
    code.push_str("    \"\"\"\n");

    // Extract fields from result schema
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for (field_name, field_schema) in props {
            let py_type = json_schema_to_python_type(field_schema);
            code.push_str(&format!("    {}: {}\n", field_name, py_type));
        }
    }
    code.push_str("\n\n");

    Ok(())
}

fn generate_python_handler(code: &mut String, method: &OpenRpcMethod) -> Result<()> {
    let handler_name = format!("handle_{}", method.name.replace('.', "_"));
    let params_class = get_python_params_class_name(&method.name);

    code.push_str(&format!("async def {}(params: Any) -> Dict[str, Any]:\n", handler_name));
    code.push_str("    \"\"\"\n");
    if let Some(summary) = &method.summary {
        code.push_str("    ");
        code.push_str(summary);
        code.push_str(".\n");
    }
    if let Some(desc) = &method.description {
        code.push('\n');
        code.push_str("    ");
        code.push_str(desc);
        code.push('\n');
    }
    code.push_str("\n    Args:\n");
    code.push_str("        params: Method parameters\n");
    code.push_str("\n    Returns:\n");
    code.push_str("        Result object\n");
    code.push_str("\n    Raises:\n");
    for error in &method.errors {
        code.push_str(&format!(
            "        JSONRPCError: {} (code: {})\n",
            error.message, error.code
        ));
    }
    code.push_str("    \"\"\"\n");

    // Parse params
    if !method.params.is_empty() {
        code.push_str(&format!(
            "    parsed_params = msgspec.convert(params, type={})\n",
            params_class
        ));
    }

    // TODO comment for business logic
    code.push_str("\n    # TODO: Implement business logic\n");
    code.push_str("    # This handler receives parsed parameters and should:\n");
    code.push_str("    # 1. Validate inputs\n");
    code.push_str("    # 2. Execute business logic\n");
    code.push_str("    # 3. Return result as dict matching schema\n");
    code.push_str("    # 4. Raise appropriate JSON-RPC errors on failure\n\n");

    // Placeholder return
    code.push_str("    # Example return structure (update with real data):\n");
    code.push_str("    result_data = {}\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            code.push_str(&format!("    result_data[\"{}\"] = \"TODO\"\n", field_name));
        }
    }
    code.push_str("    return result_data\n\n");

    Ok(())
}

fn json_schema_to_python_type(schema: &Value) -> String {
    if let Some(type_str) = schema.get("type") {
        match type_str.as_str() {
            Some("string") => {
                if let Some(format) = schema.get("format") {
                    match format.as_str() {
                        Some("date-time") => "str  # ISO 8601 datetime".to_string(),
                        Some("uuid") => "str  # UUID".to_string(),
                        Some("email") => "str  # Email".to_string(),
                        Some("date") => "str  # Date".to_string(),
                        _ => "str".to_string(),
                    }
                } else {
                    "str".to_string()
                }
            }
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
            Some("object") => "Dict[str, Any]".to_string(),
            _ => "Any".to_string(),
        }
    } else if schema.get("enum").is_some() {
        "str  # Enum".to_string()
    } else {
        "Any".to_string()
    }
}

fn get_python_params_class_name(method_name: &str) -> String {
    let pascal = method_name
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("");

    format!("{}Params", pascal)
}

fn get_python_result_class_name(method_name: &str) -> String {
    let pascal = method_name
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<Vec<_>>()
        .join("");

    format!("{}Result", pascal)
}
