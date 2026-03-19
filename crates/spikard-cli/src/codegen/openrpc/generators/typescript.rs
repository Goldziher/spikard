//! TypeScript `OpenRPC` code generation.

use anyhow::Result;
use serde_json::Value;

use crate::codegen::openrpc::spec_parser::{
    OpenRpcSpec, extract_methods, get_method_params_class_name, get_result_class_name,
};

use super::OpenRpcGenerator;

/// TypeScript `OpenRPC` code generator
pub struct TypeScriptOpenRpcGenerator;

impl OpenRpcGenerator for TypeScriptOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        code.push_str("#!/usr/bin/env node\n");
        code.push_str("/**\n");
        code.push_str(" * JSON-RPC 2.0 handlers generated from OpenRPC specification.\n");
        code.push_str(" *\n");
        code.push_str(" * Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push('\n');
        code.push_str(" */\n\n");

        code.push_str("import { z } from \"zod\";\n\n");

        code.push_str("// ============================================================================\n");
        code.push_str("// Zod Validation Schemas\n");
        code.push_str("// ============================================================================\n\n");

        for method in extract_methods(spec) {
            generate_typescript_schemas(&mut code, method)?;
        }

        code.push_str("// ============================================================================\n");
        code.push_str("// Handler Types\n");
        code.push_str("// ============================================================================\n\n");

        for method in extract_methods(spec) {
            generate_typescript_types(&mut code, method)?;
        }

        code.push_str("// ============================================================================\n");
        code.push_str("// JSON-RPC Method Handlers\n");
        code.push_str("// ============================================================================\n\n");

        for method in extract_methods(spec) {
            generate_typescript_handler(&mut code, method)?;
        }

        code.push_str("// ============================================================================\n");
        code.push_str("// Method Router\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("type JSONRPCRequest = {\n");
        code.push_str("  jsonrpc: \"2.0\";\n");
        code.push_str("  method: string;\n");
        code.push_str("  params?: unknown;\n");
        code.push_str("  id?: string | number | null;\n");
        code.push_str("};\n\n");

        code.push_str("type JSONRPCResponse = {\n");
        code.push_str("  jsonrpc: \"2.0\";\n");
        code.push_str("  result?: unknown;\n");
        code.push_str("  error?: { code: number; message: string; data?: unknown };\n");
        code.push_str("  id?: string | number | null;\n");
        code.push_str("};\n\n");

        code.push_str("export async function handleJsonRpcCall(request: JSONRPCRequest): Promise<JSONRPCResponse> {\n");
        code.push_str("  const { method, params, id } = request;\n");
        code.push_str("  try {\n");

        for method in extract_methods(spec) {
            let handler_name = format!("handle{}", pascal_case(&method.name));
            code.push_str(&format!("    if (method === \"{}\") {{\n", method.name));
            code.push_str(&format!("      const result = await {handler_name}(params);\n"));
            code.push_str("      return { jsonrpc: \"2.0\", result, id };\n");
            code.push_str("    }\n");
        }

        code.push_str("    return {\n");
        code.push_str("      jsonrpc: \"2.0\",\n");
        code.push_str("      error: { code: -32601, message: \"Method not found\" },\n");
        code.push_str("      id,\n");
        code.push_str("    };\n");
        code.push_str("  } catch (error) {\n");
        code.push_str("    return {\n");
        code.push_str("      jsonrpc: \"2.0\",\n");
        code.push_str("      error: {\n");
        code.push_str("        code: -32603,\n");
        code.push_str("        message: \"Internal error\",\n");
        code.push_str("        data: error instanceof Error ? error.message : String(error),\n");
        code.push_str("      },\n");
        code.push_str("      id,\n");
        code.push_str("    };\n");
        code.push_str("  }\n");
        code.push_str("}\n\n");

        code.push_str("// ============================================================================\n");
        code.push_str("// Example Usage\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("// Wire `handleJsonRpcCall` into your runtime or framework of choice.\n");
        code.push_str("// Example:\n");
        code.push_str("//   const response = await handleJsonRpcCall({\n");
        code.push_str("//     jsonrpc: \"2.0\",\n");
        code.push_str("//     method: \"example.method\",\n");
        code.push_str("//     params: {},\n");
        code.push_str("//     id: 1,\n");
        code.push_str("//   });\n");
        code.push_str("//   console.log(JSON.stringify(response));\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "typescript"
    }
}

fn generate_typescript_schemas(
    code: &mut String,
    method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod,
) -> Result<()> {
    if !method.params.is_empty() {
        let schema_name = format!("{}ParamsSchema", pascal_case(&method.name));
        code.push_str(&format!(
            "const {schema_name} = {};\n\n",
            method_params_to_zod(&method.params)
        ));
    }

    let result_schema_name = format!("{}ResultSchema", pascal_case(&method.name));
    code.push_str(&format!(
        "const {result_schema_name} = {};\n\n",
        json_schema_to_zod(&method.result.schema)
    ));

    Ok(())
}

fn generate_typescript_types(
    code: &mut String,
    method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod,
) -> Result<()> {
    if !method.params.is_empty() {
        let type_name = get_method_params_class_name(&method.name);
        let schema_name = format!("{}ParamsSchema", pascal_case(&method.name));
        code.push_str(&format!("type {type_name} = z.infer<typeof {schema_name}>;\n"));
    }

    let result_type_name = get_result_class_name(&method.name);
    let result_schema_name = format!("{}ResultSchema", pascal_case(&method.name));
    code.push_str(&format!(
        "type {result_type_name} = z.infer<typeof {result_schema_name}>;\n"
    ));

    code.push('\n');

    Ok(())
}

fn generate_typescript_handler(
    code: &mut String,
    method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod,
) -> Result<()> {
    let handler_name = format!("handle{}", pascal_case(&method.name));
    let result_type_name = get_result_class_name(&method.name);

    code.push_str(&format!(
        "async function {handler_name}(params: unknown): Promise<{result_type_name}> {{\n"
    ));
    code.push_str("  /**\n");
    if let Some(summary) = &method.summary {
        code.push_str(&format!("   * {summary}\n"));
    }
    if let Some(desc) = &method.description {
        code.push_str(&format!("   *\n   * {desc}\n"));
    }
    code.push_str("   *\n");
    code.push_str("   * @param params - Method parameters\n");
    code.push_str("   * @returns Result object\n");
    code.push_str("   * @throws JSONRPCError on validation or execution failure\n");
    code.push_str("   */\n");

    if !method.params.is_empty() {
        let schema_name = format!("{}ParamsSchema", pascal_case(&method.name));
        code.push_str(&format!("  const parsedParams = {schema_name}.parse(params);\n\n"));
        code.push_str("  void parsedParams;\n\n");
    }

    // TODO comment
    code.push_str("  // TODO: Implement business logic\n");
    code.push_str("  // This handler receives parsed parameters and should:\n");
    code.push_str("  // 1. Validate inputs\n");
    code.push_str("  // 2. Execute business logic\n");
    code.push_str("  // 3. Return result matching schema\n");
    code.push_str("  // 4. Throw appropriate JSON-RPC errors on failure\n\n");
    code.push_str("  throw new Error(\"TODO: Implement JSON-RPC method logic\");\n");
    code.push_str("}\n\n");

    Ok(())
}

fn json_schema_to_zod(schema: &Value) -> String {
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let ref_name = reference.split('/').next_back().unwrap_or(reference);
        return format!("{}Schema", pascal_case(ref_name));
    }

    if let Some(enum_values) = schema.get("enum").and_then(Value::as_array) {
        let literals = enum_values
            .iter()
            .filter_map(Value::as_str)
            .map(|value| format!("{value:?}"))
            .collect::<Vec<_>>();
        if !literals.is_empty() {
            return format!("z.enum([{}])", literals.join(", "));
        }
    }

    if let Some(type_str) = schema.get("type") {
        let mut zod_type = match type_str.as_str() {
            Some("string") => {
                if let Some(format) = schema.get("format") {
                    match format.as_str() {
                        Some("uuid") => "z.string().uuid()".to_string(),
                        Some("email") => "z.string().email()".to_string(),
                        Some("date-time") => "z.string().datetime()".to_string(),
                        Some("date") => "z.string()".to_string(),
                        _ => "z.string()".to_string(),
                    }
                } else {
                    "z.string()".to_string()
                }
            }
            Some("number") => "z.number()".to_string(),
            Some("integer") => "z.number().int()".to_string(),
            Some("boolean") => "z.boolean()".to_string(),
            Some("array") => {
                if let Some(items) = schema.get("items") {
                    format!("z.array({})", json_schema_to_zod(items))
                } else {
                    "z.array(z.unknown())".to_string()
                }
            }
            Some("object") => object_schema_to_zod(schema),
            _ => "z.unknown()".to_string(),
        };

        if schema.get("nullable").and_then(Value::as_bool).unwrap_or(false) {
            zod_type.push_str(".nullable()");
        }

        zod_type
    } else {
        "z.unknown()".to_string()
    }
}

fn method_params_to_zod(params: &[crate::codegen::openrpc::spec_parser::OpenRpcParam]) -> String {
    let mut lines = String::from("z.object({\n");

    for param in params {
        let mut param_zod = json_schema_to_zod(&param.schema);
        if !param.required {
            param_zod.push_str(".optional()");
        }
        lines.push_str(&format!("  {}: {param_zod},\n", param.name));
    }

    lines.push_str("})");
    lines
}

fn object_schema_to_zod(schema: &Value) -> String {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return "z.record(z.unknown())".to_string();
    };

    let required_fields = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|items| items.iter().filter_map(Value::as_str).collect::<Vec<_>>())
        .unwrap_or_default();

    let mut lines = String::from("z.object({\n");

    for (field_name, field_schema) in properties {
        let mut field_zod = json_schema_to_zod(field_schema);
        if !required_fields.iter().any(|required| required == field_name) {
            field_zod.push_str(".optional()");
        }
        lines.push_str(&format!("  {}: {field_zod},\n", field_name));
    }

    lines.push_str("})");
    lines
}

fn pascal_case(input: &str) -> String {
    input
        .split('.')
        .map(|part| {
            let mut chars = part.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            }
        })
        .collect::<String>()
}
