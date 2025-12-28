//! TypeScript OpenRPC code generation.

use anyhow::Result;
use serde_json::Value;

use crate::codegen::openrpc::spec_parser::{
    OpenRpcSpec, extract_methods, get_method_params_class_name, get_result_class_name,
};

use super::OpenRpcGenerator;

/// TypeScript OpenRPC code generator
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
            code.push_str(&format!("      const result = await {}(params);\n", handler_name));
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

        code.push_str("if (require.main === module) {\n");
        code.push_str("  import('http').then(({ createServer }) => {\n");
        code.push_str("    const server = createServer(async (req, res) => {\n");
        code.push_str("      if (req.method === 'POST' && req.url === '/rpc') {\n");
        code.push_str("        let body = '';\n");
        code.push_str("        req.on('data', (chunk) => { body += chunk; });\n");
        code.push_str("        req.on('end', async () => {\n");
        code.push_str("          try {\n");
        code.push_str("            const request = JSON.parse(body);\n");
        code.push_str("            const response = await handleJsonRpcCall(request);\n");
        code.push_str("            res.writeHead(200, { 'Content-Type': 'application/json' });\n");
        code.push_str("            res.end(JSON.stringify(response));\n");
        code.push_str("          } catch (e) {\n");
        code.push_str("            res.writeHead(400);\n");
        code.push_str("            res.end('Invalid request');\n");
        code.push_str("          }\n");
        code.push_str("        });\n");
        code.push_str("      } else {\n");
        code.push_str("        res.writeHead(404);\n");
        code.push_str("        res.end('Not found');\n");
        code.push_str("      }\n");
        code.push_str("    });\n");
        code.push_str("    server.listen(8000, () => console.log('Server running on port 8000'));\n");
        code.push_str("  });\n");
        code.push_str("}\n");

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
        code.push_str(&format!("const {} = z.object({{\n", schema_name));
        for param in &method.params {
            code.push_str(&format!("  {}: {},\n", param.name, json_schema_to_zod(&param.schema)));
        }
        code.push_str("});\n\n");
    }

    let result_schema_name = format!("{}ResultSchema", pascal_case(&method.name));
    code.push_str(&format!("const {} = z.object({{\n", result_schema_name));
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for (field_name, field_schema) in props {
            code.push_str(&format!("  {}: {},\n", field_name, json_schema_to_zod(field_schema)));
        }
    }
    code.push_str("});\n\n");

    Ok(())
}

fn generate_typescript_types(
    code: &mut String,
    method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod,
) -> Result<()> {
    if !method.params.is_empty() {
        let type_name = get_method_params_class_name(&method.name);
        let schema_name = format!("{}ParamsSchema", pascal_case(&method.name));
        code.push_str(&format!("type {} = z.infer<typeof {}>;\n", type_name, schema_name));
    }

    let result_type_name = get_result_class_name(&method.name);
    let result_schema_name = format!("{}ResultSchema", pascal_case(&method.name));
    code.push_str(&format!(
        "type {} = z.infer<typeof {}>;\n",
        result_type_name, result_schema_name
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
        "async function {}(params: unknown): Promise<{}> {{\n",
        handler_name, result_type_name
    ));
    code.push_str("  /**\n");
    if let Some(summary) = &method.summary {
        code.push_str(&format!("   * {}\n", summary));
    }
    if let Some(desc) = &method.description {
        code.push_str(&format!("   *\n   * {}\n", desc));
    }
    code.push_str("   *\n");
    code.push_str("   * @param params - Method parameters\n");
    code.push_str("   * @returns Result object\n");
    code.push_str("   * @throws JSONRPCError on validation or execution failure\n");
    code.push_str("   */\n");

    if !method.params.is_empty() {
        let schema_name = format!("{}ParamsSchema", pascal_case(&method.name));
        code.push_str(&format!("  const parsedParams = {}.parse(params);\n\n", schema_name));
    }

    // TODO comment
    code.push_str("  // TODO: Implement business logic\n");
    code.push_str("  // This handler receives parsed parameters and should:\n");
    code.push_str("  // 1. Validate inputs\n");
    code.push_str("  // 2. Execute business logic\n");
    code.push_str("  // 3. Return result matching schema\n");
    code.push_str("  // 4. Throw appropriate JSON-RPC errors on failure\n\n");

    code.push_str("  // Example return structure (update with real data):\n");
    code.push_str("  const result: Record<string, unknown> = {};\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            code.push_str(&format!("  result[\"{}\"] = \"TODO\";\n", field_name));
        }
    }
    code.push_str("  return result as any; // TODO: type-safe return\n");
    code.push_str("}\n\n");

    Ok(())
}

fn json_schema_to_zod(schema: &Value) -> String {
    if let Some(type_str) = schema.get("type") {
        match type_str.as_str() {
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
            Some("object") => "z.record(z.unknown())".to_string(),
            _ => "z.unknown()".to_string(),
        }
    } else if schema.get("enum").is_some() {
        "z.string()".to_string()
    } else {
        "z.unknown()".to_string()
    }
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
        .collect::<Vec<_>>()
        .join("")
}
