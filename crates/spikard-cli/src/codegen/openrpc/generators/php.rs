//! PHP `OpenRPC` code generation.

use anyhow::Result;
use serde_json::Value;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

use super::OpenRpcGenerator;

/// PHP `OpenRPC` code generator
pub struct PhpOpenRpcGenerator;

impl OpenRpcGenerator for PhpOpenRpcGenerator {
    fn generate_handler_app(&self, spec: &OpenRpcSpec) -> Result<String> {
        let mut code = String::new();

        code.push_str("<?php\n");
        code.push_str("/**\n");
        code.push_str(" * JSON-RPC 2.0 handlers generated from OpenRPC specification.\n");
        code.push_str(" *\n");
        code.push_str(" * Generated from: ");
        code.push_str(&spec.info.title);
        code.push_str(" v");
        code.push_str(&spec.info.version);
        code.push('\n');
        code.push_str(" */\n\n");

        code.push_str("declare(strict_types=1);\n\n");

        code.push_str("namespace JsonRpc\\Handlers;\n\n");

        code.push_str("// ============================================================================\n");
        code.push_str("// JSON-RPC Method Handlers\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("interface JsonRpcHandler\n");
        code.push_str("{\n");
        code.push_str("    /**\n");
        code.push_str("     * @param array<array-key, mixed> $params\n");
        code.push_str("     * @return mixed\n");
        code.push_str("     */\n");
        code.push_str("    public function execute(array $params): mixed;\n");
        code.push_str("}\n\n");

        for method in &spec.methods {
            generate_php_handler(&mut code, method)?;
        }

        code.push_str("// ============================================================================\n");
        code.push_str("// Handler Registry\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("final class HandlerRegistry {\n");
        code.push_str("    /** @var array<string, JsonRpcHandler> */\n");
        code.push_str("    private static array $handlers = [];\n\n");

        code.push_str("    public static function register(): void {\n");
        for method in &spec.methods {
            let handler_class = format!("Handle{}", pascal_case(&method.name));
            code.push_str(&format!(
                "        self::$handlers[\"{}\"] = new {}();\n",
                method.name, handler_class
            ));
        }
        code.push_str("    }\n\n");

        code.push_str("    /**\n");
        code.push_str("     * @param array<array-key, mixed> $params\n");
        code.push_str("     * @return array<string, mixed>\n");
        code.push_str("     */\n");
        code.push_str(
            "    public static function handle(?string $methodName, array $params, int|string|null $requestId): array {\n",
        );
        code.push_str("        if ($methodName === null || !isset(self::$handlers[$methodName])) {\n");
        code.push_str("            return self::errorResponse(-32601, \"Method not found\", $requestId);\n");
        code.push_str("        }\n\n");

        code.push_str("        try {\n");
        code.push_str("            $handler = self::$handlers[$methodName];\n");
        code.push_str("            $result = $handler->execute($params);\n");
        code.push_str("            return [\n");
        code.push_str("                'jsonrpc' => '2.0',\n");
        code.push_str("                'result' => $result,\n");
        code.push_str("                'id' => $requestId,\n");
        code.push_str("            ];\n");
        code.push_str("        } catch (\\Throwable $e) {\n");
        code.push_str(
            "            return self::errorResponse(-32603, \"Internal error\", $requestId, $e->getMessage());\n",
        );
        code.push_str("        }\n");
        code.push_str("    }\n\n");

        code.push_str("    /**\n");
        code.push_str("     * @return array<string, mixed>\n");
        code.push_str("     */\n");
        code.push_str("    private static function errorResponse(\n");
        code.push_str("        int $code,\n");
        code.push_str("        string $message,\n");
        code.push_str("        int|string|null $requestId,\n");
        code.push_str("        ?string $data = null\n");
        code.push_str("    ): array {\n");
        code.push_str(
            "        /** @var array{jsonrpc: '2.0', error: array{code: int, message: string, data?: string}, id: int|string|null} $response */\n",
        );
        code.push_str("        $response = [\n");
        code.push_str("            'jsonrpc' => '2.0',\n");
        code.push_str("            'error' => [\n");
        code.push_str("                'code' => $code,\n");
        code.push_str("                'message' => $message,\n");
        code.push_str("            ],\n");
        code.push_str("            'id' => $requestId,\n");
        code.push_str("        ];\n");
        code.push_str("        if ($data !== null) {\n");
        code.push_str("            $response['error']['data'] = $data;\n");
        code.push_str("        }\n");
        code.push_str("        return $response;\n");
        code.push_str("    }\n");
        code.push_str("}\n\n");

        code.push_str("// ============================================================================\n");
        code.push_str("// Example Usage\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("// Register handlers on startup\n");
        code.push_str("HandlerRegistry::register();\n\n");

        code.push_str("// Example HTTP endpoint (using your preferred framework)\n");
        code.push_str("if ($_SERVER['REQUEST_METHOD'] === 'POST' && $_SERVER['REQUEST_URI'] === '/rpc') {\n");
        code.push_str("    header('Content-Type: application/json');\n");
        code.push_str("    $rawRequest = file_get_contents('php://input');\n");
        code.push_str("    $request = json_decode($rawRequest === false ? '{}' : $rawRequest, associative: true);\n");
        code.push_str(
            "    $method = is_array($request) && is_string($request['method'] ?? null) ? $request['method'] : null;\n",
        );
        code.push_str(
            "    $params = is_array($request) && is_array($request['params'] ?? null) ? $request['params'] : [];\n",
        );
        code.push_str(
            "    $requestId = is_array($request) && (is_int($request['id'] ?? null) || is_string($request['id'] ?? null)) ? $request['id'] : null;\n\n",
        );

        code.push_str("    $response = HandlerRegistry::handle($method, $params, $requestId);\n");
        code.push_str("    echo json_encode($response);\n");
        code.push_str("}\n");

        Ok(code)
    }

    fn language_name(&self) -> &'static str {
        "php"
    }
}

fn generate_php_handler(code: &mut String, method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod) -> Result<()> {
    let handler_name = format!("Handle{}", pascal_case(&method.name));
    let params_doc_type = params_doc_type(method);
    let result_doc_type = schema_to_phpdoc_type(&method.result.schema, false);
    let result_native_type = schema_to_php_native_type(&method.result.schema, false);

    code.push_str(&format!("final class {handler_name} implements JsonRpcHandler {{\n"));
    code.push_str("    /**\n");
    if let Some(summary) = &method.summary {
        code.push_str(&format!("     * {summary}\n"));
    }
    if let Some(desc) = &method.description {
        code.push_str(&format!("     *\n     * {desc}\n"));
    }
    code.push_str(&format!("     *\n     * @param {params_doc_type} $params\n"));
    code.push_str(&format!("     * @return {result_doc_type}\n"));
    code.push_str("     */\n");
    code.push_str(&format!(
        "    public function execute(array $params): {result_native_type}\n"
    ));
    code.push_str("    {\n");

    if !method.params.is_empty() {
        code.push_str("        $this->validateParams($params);\n");
    }

    code.push_str("\n        throw new \\RuntimeException('Not implemented');\n");
    code.push_str("    }\n");

    if !method.params.is_empty() {
        code.push_str("\n    /**\n");
        code.push_str("     * @param array<array-key, mixed> $params\n");
        code.push_str("     */\n");
        code.push_str("    private function validateParams(array $params): void {\n");
        for param in &method.params {
            if param.required {
                code.push_str(&format!(
                    "        if (!array_key_exists('{}', $params)) {{\n",
                    param.name
                ));
                code.push_str(&format!(
                    "            throw new \\InvalidArgumentException('Missing required parameter: {}');\n",
                    param.name
                ));
                code.push_str("        }\n");
            }
        }
        code.push_str("    }\n");
    }

    code.push_str("}\n\n");

    Ok(())
}

fn params_doc_type(method: &crate::codegen::openrpc::spec_parser::OpenRpcMethod) -> String {
    if method.params.is_empty() {
        return "array{}".to_string();
    }

    let mut entries = Vec::new();
    for param in &method.params {
        let param_type = schema_to_phpdoc_type(&param.schema, !param.required);
        if param.required {
            entries.push(format!("{}: {param_type}", param.name));
        } else {
            entries.push(format!("{}?: {param_type}", param.name));
        }
    }

    format!("array{{{}}}", entries.join(", "))
}

fn schema_to_php_native_type(schema: &Value, optional: bool) -> String {
    let base_type = match resolve_schema(schema).get("type").and_then(Value::as_str) {
        Some("string") => "string",
        Some("integer") => "int",
        Some("number") => "float",
        Some("boolean") => "bool",
        Some("array") | Some("object") => "array",
        Some("null") => "null",
        _ => "mixed",
    };

    if optional && !matches!(base_type, "mixed" | "null") {
        format!("?{base_type}")
    } else {
        base_type.to_string()
    }
}

fn schema_to_phpdoc_type(schema: &Value, optional: bool) -> String {
    let resolved = resolve_schema(schema);
    let base_type = match resolved.get("type").and_then(Value::as_str) {
        Some("string") => "string".to_string(),
        Some("integer") => "int".to_string(),
        Some("number") => "float".to_string(),
        Some("boolean") => "bool".to_string(),
        Some("array") => {
            let item_type = resolved
                .get("items")
                .map(|item| schema_to_phpdoc_type(item, false))
                .unwrap_or_else(|| "mixed".to_string());
            format!("list<{item_type}>")
        }
        Some("object") => object_shape_doc_type(resolved),
        Some("null") => "null".to_string(),
        _ => "mixed".to_string(),
    };

    if optional {
        format!("{base_type}|null")
    } else {
        base_type
    }
}

fn resolve_schema<'a>(schema: &'a Value) -> &'a Value {
    schema
}

fn object_shape_doc_type(schema: &Value) -> String {
    let Some(properties) = schema.get("properties").and_then(Value::as_object) else {
        return "array<string, mixed>".to_string();
    };

    if properties.is_empty() {
        return "array<string, mixed>".to_string();
    }

    let required: Vec<&str> = schema
        .get("required")
        .and_then(Value::as_array)
        .map(|values| required_names(values))
        .unwrap_or_default();

    let mut entries = Vec::new();
    for (name, property_schema) in properties {
        let is_required = required.iter().any(|required_name| *required_name == name);
        let property_type = schema_to_phpdoc_type(property_schema, !is_required);
        if is_required {
            entries.push(format!("{name}: {property_type}"));
        } else {
            entries.push(format!("{name}?: {property_type}"));
        }
    }

    format!("array{{{}}}", entries.join(", "))
}

fn required_names(values: &[Value]) -> Vec<&str> {
    values.iter().filter_map(Value::as_str).collect()
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
