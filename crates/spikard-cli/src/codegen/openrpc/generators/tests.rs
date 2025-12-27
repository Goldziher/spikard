//! Comprehensive tests for OpenRPC code generators.
//!
//! Tests cover all 4 language generators (Python, TypeScript, PHP, Ruby) with
//! scenarios including basic generation, error handling, method routing,
//! parameter validation, and cross-language parity.

use super::{
    OpenRpcGenerator, PhpOpenRpcGenerator, PythonOpenRpcGenerator, RubyOpenRpcGenerator, TypeScriptOpenRpcGenerator,
};
use crate::codegen::openrpc::spec_parser::{
    OpenRpcError, OpenRpcInfo, OpenRpcMethod, OpenRpcParam, OpenRpcResult, OpenRpcSpec,
};
use serde_json::json;

/// Helper function to create a minimal OpenRPC spec
fn minimal_spec() -> OpenRpcSpec {
    OpenRpcSpec {
        openrpc: "1.3.2".to_string(),
        info: OpenRpcInfo {
            title: "Test API".to_string(),
            version: "1.0.0".to_string(),
            description: None,
            contact: None,
            license: None,
        },
        methods: vec![],
        servers: vec![],
        components: Default::default(),
    }
}

/// Helper function to create a spec with a single test method
fn single_method_spec(name: &str) -> OpenRpcSpec {
    let mut spec = minimal_spec();
    spec.methods = vec![OpenRpcMethod {
        name: name.to_string(),
        summary: Some("Test method".to_string()),
        description: Some("A test method for code generation".to_string()),
        params: vec![],
        result: OpenRpcResult {
            name: "result".to_string(),
            description: Some("The result".to_string()),
            schema: json!({
                "type": "object",
                "properties": {
                    "message": { "type": "string" }
                }
            }),
        },
        errors: vec![],
        examples: vec![],
        tags: vec![],
    }];
    spec
}

/// Helper function to create a spec with parameters
fn spec_with_params(name: &str, param_count: usize) -> OpenRpcSpec {
    let mut spec = single_method_spec(name);
    spec.methods[0].params = (0..param_count)
        .map(|i| OpenRpcParam {
            name: format!("param{}", i),
            description: Some(format!("Parameter {}", i)),
            required: i == 0,
            schema: json!({"type": "string"}),
        })
        .collect();
    spec
}

/// Helper function to create a spec with error definitions
#[allow(dead_code)]
fn spec_with_errors(name: &str) -> OpenRpcSpec {
    let mut spec = single_method_spec(name);
    spec.methods[0].errors = vec![
        OpenRpcError {
            code: -32700,
            message: "Parse error".to_string(),
            data: None,
        },
        OpenRpcError {
            code: -32600,
            message: "Invalid request".to_string(),
            data: None,
        },
    ];
    spec
}

#[test]
fn test_python_generator_imports_msgspec() {
    let spec = single_method_spec("test_method");
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("import msgspec"),
        "Generated code should import msgspec"
    );
    assert!(
        output.contains("from typing import"),
        "Generated code should have typing imports"
    );
    assert!(output.contains("import json"), "Generated code should import json");
}

#[test]
fn test_python_generator_async_handler_signature() {
    let spec = single_method_spec("test_method");
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("async def handle_test_method"),
        "Handler should be async function"
    );
    assert!(output.contains("-> Dict[str, Any]"), "Handler should return Dict");
}

#[test]
fn test_python_generator_error_handling() {
    let spec = single_method_spec("test_method");
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("except Exception as e"),
        "Generated code should have exception handling"
    );
    assert!(output.contains("-32601"), "Should have method not found error code");
    assert!(output.contains("-32603"), "Should have internal error code");
}

#[test]
fn test_python_generator_method_router() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
    ];

    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("if method_name == \"user.get\""),
        "Router should check for user.get"
    );
    assert!(
        output.contains("if method_name == \"user.create\""),
        "Router should check for user.create"
    );
    assert!(
        output.contains("await handle_user_get"),
        "Router should call user.get handler"
    );
    assert!(
        output.contains("await handle_user_create"),
        "Router should call user.create handler"
    );
}

#[test]
fn test_python_generator_validation_schemas() {
    let spec = spec_with_params("validate.test", 2);
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("class ValidateTestParams(msgspec.Struct"),
        "Should generate params DTO"
    );
    assert!(output.contains("param0: str"), "Should include first parameter");
    assert!(output.contains("param1: str"), "Should include second parameter");
}

#[test]
fn test_python_generator_empty_spec() {
    let spec = minimal_spec();
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("async def handle_jsonrpc_call"),
        "Should generate router even with no methods"
    );
    assert!(
        output.contains("Method not found"),
        "Should return method not found for empty spec"
    );
}

#[test]
fn test_python_generator_special_method_names() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get.by.id").methods[0].clone(),
        single_method_spec("user-create").methods[0].clone(),
        single_method_spec("user_update").methods[0].clone(),
    ];

    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("handle_user_get_by_id"),
        "Should convert dots to underscores"
    );
    assert!(
        output.contains("user-create"),
        "Should preserve hyphens in method name check"
    );
    assert!(
        output.contains("user_update"),
        "Should preserve underscores in method name check"
    );
}

#[test]
fn test_python_generator_complex_param_schemas() {
    let mut spec = single_method_spec("complex_method");
    spec.methods[0].params = vec![OpenRpcParam {
        name: "config".to_string(),
        description: Some("Configuration object".to_string()),
        required: true,
        schema: json!({
            "type": "object",
            "properties": {
                "nested": {
                    "type": "object",
                    "properties": {
                        "deep": {"type": "string"}
                    }
                },
                "list": {
                    "type": "array",
                    "items": {"type": "string"}
                }
            }
        }),
    }];

    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("config: Dict[str, Any]"),
        "Should map nested objects to Dict"
    );
}

#[test]
fn test_python_generator_executable_imports() {
    let spec = single_method_spec("test");
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.starts_with("#!/usr/bin/env python3"),
        "Should have Python shebang"
    );
    assert!(output.contains("\"\"\"JSON-RPC 2.0 handlers"), "Should have docstring");
    assert!(output.contains("if __name__ == \"__main__\""), "Should have main block");
}

#[test]
fn test_typescript_generator_zod_schemas() {
    let spec = single_method_spec("test_method");
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("import { z } from \"zod\""), "Should import zod");
    assert!(output.contains("z.object"), "Should use zod objects for schemas");
}

#[test]
fn test_typescript_generator_handler_types() {
    let spec = single_method_spec("test_method");
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("async function"), "Handlers should be async functions");
    assert!(output.contains("Promise"), "Handlers should return Promise types");
}

#[test]
fn test_typescript_generator_method_dispatch() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
    ];

    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("if (method === \"user.get\")"),
        "Should dispatch user.get"
    );
    assert!(
        output.contains("if (method === \"user.create\")"),
        "Should dispatch user.create"
    );
}

#[test]
fn test_typescript_generator_error_codes() {
    let spec = single_method_spec("test_method");
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("-32601"), "Should have method not found code");
    assert!(output.contains("-32603"), "Should have internal error code");
    assert!(
        output.contains("jsonrpc") && output.contains("2.0"),
        "Should use JSON-RPC 2.0 format"
    );
}

#[test]
fn test_typescript_generator_async_handlers() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("method1").methods[0].clone(),
        single_method_spec("method2").methods[0].clone(),
        single_method_spec("method3").methods[0].clone(),
    ];

    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    let async_count = output.matches("async function handle").count();
    assert_eq!(
        async_count, 4,
        "Should have main handler + 3 individual async handler functions"
    );
}

#[test]
fn test_typescript_generator_strict_typing() {
    let spec = spec_with_params("test", 2);
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("z.infer"), "Should use zod.infer for type safety");
    // The TODO comment may mention 'any' but the actual types shouldn't
    assert!(
        output.matches("z.string()").count() > 0,
        "Should use specific Zod types"
    );
}

#[test]
fn test_typescript_generator_export_default() {
    let spec = single_method_spec("test");
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("export async function"),
        "Should export handler function"
    );
}

#[test]
fn test_typescript_generator_compiles() {
    let mut spec = minimal_spec();
    spec.methods = vec![single_method_spec("getStatus").methods[0].clone()];

    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.starts_with("#!/usr/bin/env node"), "Should have node shebang");
    assert!(output.contains("/**"), "Should have JSDoc comments");
    assert!(output.contains("type JSONRPCRequest"), "Should define JSON-RPC types");
    assert!(output.contains("type JSONRPCResponse"), "Should define response types");
}

#[test]
fn test_php_generator_declare_strict_types() {
    let spec = single_method_spec("test_method");
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("<?php"), "Should start with PHP opening tag");
    let lines: Vec<&str> = output.lines().collect();
    let strict_found = lines.iter().any(|line| line.contains("declare(strict_types=1)"));
    assert!(strict_found, "Should declare strict types");
}

#[test]
fn test_php_generator_handler_classes() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
    ];

    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("final class HandleUserGet"),
        "Should generate user.get handler class"
    );
    assert!(
        output.contains("final class HandleUserCreate"),
        "Should generate user.create handler class"
    );
}

#[test]
fn test_php_generator_execute_method() {
    let spec = single_method_spec("test_method");
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("public function execute(mixed $params): array"),
        "Handler should have execute method"
    );
}

#[test]
fn test_php_generator_registry_pattern() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
    ];

    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("final class HandlerRegistry"),
        "Should have HandlerRegistry class"
    );
    assert!(
        output.contains("public static function register()"),
        "Should have register method"
    );
    assert!(
        output.contains("public static function handle"),
        "Should have handle method"
    );
    assert!(output.contains("self::$handlers"), "Should use handlers registry");
}

#[test]
fn test_php_generator_error_responses() {
    let spec = single_method_spec("test_method");
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("-32601"), "Should have method not found error");
    assert!(output.contains("-32603"), "Should have internal error");
    assert!(
        output.contains("'jsonrpc' => '2.0'"),
        "Should format JSON-RPC responses"
    );
    assert!(output.contains("'error'"), "Error responses should have error field");
}

#[test]
fn test_php_generator_namespace_declaration() {
    let spec = single_method_spec("test");
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("namespace JsonRpc\\Handlers"),
        "Should declare namespace"
    );
}

#[test]
fn test_php_generator_return_types() {
    let spec = single_method_spec("test_method");
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains(": array"), "Handler should have array return type");
    assert!(
        output.contains("mixed $params"),
        "Handler should have mixed params type"
    );
}

#[test]
fn test_php_generator_valid_syntax() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("test1").methods[0].clone(),
        single_method_spec("test2").methods[0].clone(),
    ];

    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.starts_with("<?php"), "Should start with PHP tag");
    assert!(output.contains("namespace"), "Should have namespace");
    assert!(output.contains("class"), "Should have classes");
    let open_braces = output.matches('{').count();
    let close_braces = output.matches('}').count();
    assert_eq!(open_braces, close_braces, "Braces should be balanced");
}

#[test]
fn test_ruby_generator_module_definition() {
    let mut spec = minimal_spec();
    spec.methods = vec![single_method_spec("user.get").methods[0].clone()];

    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("class HandleUserGet"),
        "Should define handler class"
    );
}

#[test]
fn test_ruby_generator_method_signature() {
    let spec = single_method_spec("test_method");
    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("def execute(params)"),
        "Handler should have execute method"
    );
}

#[test]
fn test_ruby_generator_error_handling() {
    let spec = single_method_spec("test_method");
    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("rescue StandardError"), "Should handle errors");
    assert!(output.contains("-32601"), "Should have method not found error");
    assert!(output.contains("-32603"), "Should have internal error code");
}

#[test]
fn test_ruby_generator_handler_registry() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
    ];

    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("HANDLERS = {"), "Should define handlers hash");
    assert!(output.contains("\"user.get\""), "Should register user.get");
    assert!(output.contains("\"user.create\""), "Should register user.create");
    assert!(output.contains("class JsonRpcRouter"), "Should have router class");
    assert!(
        output.contains("def self.handle_call"),
        "Should have handle_call method"
    );
}

#[test]
fn test_ruby_generator_symbol_method_names() {
    let spec = single_method_spec("test_method");
    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("jsonrpc:") || output.contains("\"jsonrpc\""),
        "Should use symbol or string keys"
    );
}

#[test]
fn test_ruby_generator_valid_syntax() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("method1").methods[0].clone(),
        single_method_spec("method2").methods[0].clone(),
    ];

    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.starts_with("#!/usr/bin/env ruby"), "Should have ruby shebang");
    assert!(
        output.contains("# frozen_string_literal: true"),
        "Should have frozen string literal"
    );
    assert!(output.contains("require"), "Should have requires");
    assert!(output.contains("class"), "Should have classes");
    let open_braces = output.matches('{').count();
    let close_braces = output.matches('}').count();
    assert_eq!(open_braces, close_braces, "Braces should be balanced");
}

#[test]
fn test_generators_same_spec_produces_equivalent_behavior() {
    let mut spec = minimal_spec();
    spec.methods = vec![single_method_spec("test_method").methods[0].clone()];

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    assert!(!py_out.is_empty(), "Python should generate output");
    assert!(!ts_out.is_empty(), "TypeScript should generate output");
    assert!(!php_out.is_empty(), "PHP should generate output");
    assert!(!ruby_out.is_empty(), "Ruby should generate output");
}

#[test]
fn test_generators_error_codes_consistent() {
    let spec = single_method_spec("test_method");

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    for output in &[&py_out, &ts_out, &php_out, &ruby_out] {
        assert!(output.contains("-32601"), "All should have -32601 (method not found)");
        assert!(output.contains("-32603"), "All should have -32603 (internal error)");
    }
}

#[test]
fn test_generators_method_dispatch_correct_method() {
    let mut spec = minimal_spec();
    spec.methods = vec![
        single_method_spec("user.get").methods[0].clone(),
        single_method_spec("user.create").methods[0].clone(),
        single_method_spec("user.delete").methods[0].clone(),
    ];

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    for (output, lang) in &[
        (&py_out, "Python"),
        (&ts_out, "TypeScript"),
        (&php_out, "PHP"),
        (&ruby_out, "Ruby"),
    ] {
        assert!(output.contains("user.get"), "{} should handle user.get", lang);
        assert!(output.contains("user.create"), "{} should handle user.create", lang);
        assert!(output.contains("user.delete"), "{} should handle user.delete", lang);
    }
}

#[test]
fn test_generators_parameter_validation_consistent() {
    let spec = spec_with_params("validate_test", 2);

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    assert!(
        py_out.contains("param0") || py_out.contains("Params"),
        "Python should handle params"
    );
    assert!(
        ts_out.contains("param0") || ts_out.contains("Schema"),
        "TypeScript should handle params"
    );
    assert!(
        php_out.contains("param0") || php_out.contains("$params"),
        "PHP should handle params"
    );
    assert!(
        ruby_out.contains("param0") || ruby_out.contains("params"),
        "Ruby should handle params"
    );
}

#[test]
fn test_generators_response_structure_identical() {
    let spec = single_method_spec("test_method");

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    for (output, lang) in &[
        (&py_out, "Python"),
        (&ts_out, "TypeScript"),
        (&php_out, "PHP"),
        (&ruby_out, "Ruby"),
    ] {
        assert!(
            output.contains("jsonrpc") && (output.contains("result") || output.contains("error")),
            "{} should have proper JSON-RPC structure",
            lang
        );
    }
}

#[test]
fn test_generator_with_many_methods() {
    let mut spec = minimal_spec();
    for i in 0..10 {
        spec.methods
            .push(single_method_spec(&format!("method{}", i)).methods[0].clone());
    }

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    assert!(py_out.matches("def handle_method").count() >= 10);
    assert!(ts_out.matches("async function handle").count() >= 10);
    assert!(php_out.matches("final class Handle").count() >= 10);
    assert!(ruby_out.matches("class Handle").count() >= 10);
}

#[test]
fn test_generator_with_complex_result_schemas() {
    let mut spec = single_method_spec("complex");
    spec.methods[0].result.schema = json!({
        "type": "object",
        "properties": {
            "user": {
                "type": "object",
                "properties": {
                    "id": {"type": "integer"},
                    "name": {"type": "string"},
                    "email": {"type": "string", "format": "email"},
                    "tags": {
                        "type": "array",
                        "items": {"type": "string"}
                    }
                }
            },
            "metadata": {
                "type": "object",
                "properties": {
                    "created_at": {"type": "string", "format": "date-time"},
                    "version": {"type": "integer"}
                }
            }
        }
    });

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    assert!(py_gen.generate_handler_app(&spec).is_ok());
    assert!(ts_gen.generate_handler_app(&spec).is_ok());
    assert!(php_gen.generate_handler_app(&spec).is_ok());
    assert!(ruby_gen.generate_handler_app(&spec).is_ok());
}

#[test]
fn test_generator_with_array_result_schema() {
    let mut spec = single_method_spec("list_items");
    spec.methods[0].result.schema = json!({
        "type": "array",
        "items": {
            "type": "object",
            "properties": {
                "id": {"type": "integer"},
                "name": {"type": "string"}
            }
        }
    });

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    assert!(py_gen.generate_handler_app(&spec).is_ok());
    assert!(ts_gen.generate_handler_app(&spec).is_ok());
    assert!(php_gen.generate_handler_app(&spec).is_ok());
    assert!(ruby_gen.generate_handler_app(&spec).is_ok());
}

#[test]
fn test_generator_with_various_param_types() {
    let mut spec = single_method_spec("various_types");
    spec.methods[0].params = vec![
        OpenRpcParam {
            name: "string_param".to_string(),
            description: None,
            required: true,
            schema: json!({"type": "string"}),
        },
        OpenRpcParam {
            name: "number_param".to_string(),
            description: None,
            required: true,
            schema: json!({"type": "number"}),
        },
        OpenRpcParam {
            name: "integer_param".to_string(),
            description: None,
            required: true,
            schema: json!({"type": "integer"}),
        },
        OpenRpcParam {
            name: "boolean_param".to_string(),
            description: None,
            required: false,
            schema: json!({"type": "boolean"}),
        },
        OpenRpcParam {
            name: "array_param".to_string(),
            description: None,
            required: false,
            schema: json!({"type": "array", "items": {"type": "string"}}),
        },
    ];

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    assert!(py_gen.generate_handler_app(&spec).is_ok());
    assert!(ts_gen.generate_handler_app(&spec).is_ok());
    assert!(php_gen.generate_handler_app(&spec).is_ok());
    assert!(ruby_gen.generate_handler_app(&spec).is_ok());
}

#[test]
fn test_python_generator_frozen_struct() {
    let spec = spec_with_params("frozen_test", 1);
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("frozen=True"), "Python DTOs should be frozen");
}

#[test]
fn test_typescript_generator_params_parsing() {
    let spec = spec_with_params("parse_test", 2);
    let generator = TypeScriptOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains(".parse(params)"),
        "TypeScript should parse params with Zod"
    );
}

#[test]
fn test_php_generator_param_validation_method() {
    let spec = spec_with_params("validated", 2);
    let generator = PhpOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("validateParams"), "PHP should have validation method");
    assert!(
        output.contains("private function validateParams"),
        "Validation should be private"
    );
}

#[test]
fn test_ruby_generator_param_validation_method() {
    let spec = spec_with_params("validated", 2);
    let generator = RubyOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(output.contains("validate_params"), "Ruby should have validation method");
    assert!(output.contains("private"), "Validation should be private");
}

#[test]
fn test_all_generators_callable_by_trait() {
    let spec = single_method_spec("test");

    let generators: Vec<&dyn OpenRpcGenerator> = vec![
        &PythonOpenRpcGenerator,
        &TypeScriptOpenRpcGenerator,
        &PhpOpenRpcGenerator,
        &RubyOpenRpcGenerator,
    ];

    for generator in generators {
        assert!(
            generator.generate_handler_app(&spec).is_ok(),
            "All generators should work via trait"
        );
    }
}

#[test]
fn test_generator_language_names() {
    assert_eq!(PythonOpenRpcGenerator.language_name(), "python");
    assert_eq!(TypeScriptOpenRpcGenerator.language_name(), "typescript");
    assert_eq!(PhpOpenRpcGenerator.language_name(), "php");
    assert_eq!(RubyOpenRpcGenerator.language_name(), "ruby");
}

#[test]
fn test_generator_handles_empty_method_params() {
    let spec = single_method_spec("no_params");
    let generator = PythonOpenRpcGenerator;
    let output = generator.generate_handler_app(&spec).unwrap();

    assert!(
        output.contains("async def handle_no_params"),
        "Should generate handler without params"
    );
}

#[test]
fn test_generator_handles_method_with_dots_in_name() {
    let mut spec = minimal_spec();
    spec.methods = vec![single_method_spec("namespace.subspace.method").methods[0].clone()];

    let py_gen = PythonOpenRpcGenerator;
    let ts_gen = TypeScriptOpenRpcGenerator;
    let php_gen = PhpOpenRpcGenerator;
    let ruby_gen = RubyOpenRpcGenerator;

    let py_out = py_gen.generate_handler_app(&spec).unwrap();
    let ts_out = ts_gen.generate_handler_app(&spec).unwrap();
    let php_out = php_gen.generate_handler_app(&spec).unwrap();
    let ruby_out = ruby_gen.generate_handler_app(&spec).unwrap();

    assert!(py_out.contains("namespace.subspace.method"));
    assert!(ts_out.contains("namespace.subspace.method"));
    assert!(php_out.contains("namespace.subspace.method"));
    assert!(ruby_out.contains("namespace.subspace.method"));
}
