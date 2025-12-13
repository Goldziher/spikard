//! PHP OpenRPC code generation.

use anyhow::Result;

use crate::codegen::openrpc::spec_parser::OpenRpcSpec;

use super::OpenRpcGenerator;

/// PHP OpenRPC code generator
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

        for method in &spec.methods {
            generate_php_handler(&mut code, method)?;
        }

        code.push_str("// ============================================================================\n");
        code.push_str("// Handler Registry\n");
        code.push_str("// ============================================================================\n\n");

        code.push_str("final class HandlerRegistry {\n");
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

        code.push_str(
            "    public static function handle(string $methodName, mixed $params, mixed $requestId): array {\n",
        );
        code.push_str("        if (!isset(self::$handlers[$methodName])) {\n");
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

        code.push_str("    private static function errorResponse(\n");
        code.push_str("        int $code,\n");
        code.push_str("        string $message,\n");
        code.push_str("        mixed $requestId,\n");
        code.push_str("        string $data = null\n");
        code.push_str("    ): array {\n");
        code.push_str("        $response = [\n");
        code.push_str("            'jsonrpc' => '2.0',\n");
        code.push_str("            'error' => [\n");
        code.push_str("                'code' => $code,\n");
        code.push_str("                'message' => $message,\n");
        code.push_str("            ],\n");
        code.push_str("            'id' => $requestId,\n");
        code.push_str("        ];\n");
        code.push_str("        if ($data) {\n");
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
        code.push_str("    $request = json_decode(file_get_contents('php://input'), associative: true);\n");
        code.push_str("    $method = $request['method'] ?? null;\n");
        code.push_str("    $params = $request['params'] ?? [];\n");
        code.push_str("    $requestId = $request['id'] ?? null;\n\n");

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

    code.push_str(&format!("final class {} {{\n", handler_name));
    code.push_str("    /**\n");
    if let Some(summary) = &method.summary {
        code.push_str(&format!("     * {}\n", summary));
    }
    if let Some(desc) = &method.description {
        code.push_str(&format!("     *\n     * {}\n", desc));
    }
    code.push_str("     */\n");
    code.push_str("    public function execute(mixed $params): array {\n");

    if !method.params.is_empty() {
        code.push_str("        $this->validateParams($params);\n");
    }

    // TODO comment
    code.push_str("\n        // TODO: Implement business logic\n");
    code.push_str("        // This handler receives parameters and should:\n");
    code.push_str("        // 1. Validate inputs\n");
    code.push_str("        // 2. Execute business logic\n");
    code.push_str("        // 3. Return result as array matching schema\n");
    code.push_str("        // 4. Throw appropriate exceptions on failure\n\n");

    code.push_str("        // Example return structure (update with real data):\n");
    code.push_str("        $result = [];\n");
    if let Some(properties) = method.result.schema.get("properties")
        && let Some(props) = properties.as_object()
    {
        for field_name in props.keys().take(3) {
            code.push_str(&format!("        $result['{}'] = 'TODO';\n", field_name));
        }
    }
    code.push_str("        return $result;\n");
    code.push_str("    }\n");

    if !method.params.is_empty() {
        code.push_str("\n    private function validateParams(mixed $params): void {\n");
        for param in &method.params {
            if param.required {
                code.push_str(&format!(
                    "        if (!is_array($params) || !isset($params['{}'])) {{\n",
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
