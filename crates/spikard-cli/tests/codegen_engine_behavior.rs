#![allow(
    clippy::doc_markdown,
    clippy::redundant_clone,
    reason = "Test file with code generation"
)]

use spikard_cli::codegen::{
    CodegenEngine, CodegenOutcome, CodegenRequest, CodegenTargetKind, SchemaKind, TargetLanguage,
};
use std::path::PathBuf;

fn root_schema(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/schemas")
        .join(path)
}

#[test]
fn codegen_engine_openapi_in_memory() {
    let request = CodegenRequest {
        schema_path: root_schema("auth-service.openapi.yaml"),
        schema_kind: SchemaKind::OpenApi,
        target: CodegenTargetKind::Server {
            language: TargetLanguage::TypeScript,
            output: None,
        },
        dto: None,
    };

    let outcome = CodegenEngine::execute(request).expect("openapi codegen should succeed");
    match outcome {
        CodegenOutcome::InMemory(code) => assert!(!code.trim().is_empty()),
        CodegenOutcome::Files(_) => panic!("expected in-memory outcome"),
    }
}

#[test]
fn codegen_engine_openrpc_writes_output_file() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let out_path = temp_dir.path().join("handlers.ts");

    let request = CodegenRequest {
        schema_path: root_schema("user-api.openrpc.json"),
        schema_kind: SchemaKind::OpenRpc,
        target: CodegenTargetKind::JsonRpcHandlers {
            language: TargetLanguage::TypeScript,
            output: out_path.clone(),
        },
        dto: None,
    };

    let outcome = CodegenEngine::execute(request).expect("openrpc handler generation should succeed");
    match outcome {
        CodegenOutcome::Files(files) => {
            assert_eq!(files.len(), 1);
            assert_eq!(files[0].path, out_path);
            assert!(files[0].description.contains("JSON-RPC"));
        }
        CodegenOutcome::InMemory(_) => panic!("expected file outcome"),
    }

    let generated = std::fs::read_to_string(&out_path).expect("handler output should be written");
    assert!(generated.contains("handleJsonRpcCall"));
}

#[test]
fn codegen_engine_asyncapi_bundle_creates_assets() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let output_dir = temp_dir.path().join("out");

    let request = CodegenRequest {
        schema_path: root_schema("chat-service.asyncapi.yaml"),
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncAll {
            output: output_dir.clone(),
        },
        dto: None,
    };

    let outcome = CodegenEngine::execute(request).expect("asyncapi bundle generation should succeed");
    let CodegenOutcome::Files(assets) = outcome else {
        panic!("expected file outcome");
    };

    assert!(!assets.is_empty(), "expected generated assets");
    for asset in &assets {
        assert!(asset.path.exists(), "missing generated file: {}", asset.path.display());
        assert!(!asset.description.trim().is_empty());
    }
}

#[test]
fn unsupported_schema_target_pair_is_rejected() {
    let temp_dir = tempfile::tempdir().expect("temp dir");
    let request = CodegenRequest {
        schema_path: root_schema("auth-service.openapi.yaml"),
        schema_kind: SchemaKind::OpenApi,
        target: CodegenTargetKind::JsonRpcHandlers {
            language: TargetLanguage::TypeScript,
            output: temp_dir.path().join("out.ts"),
        },
        dto: None,
    };

    let err = CodegenEngine::execute(request).expect_err("unsupported combo should be rejected");
    assert!(err.to_string().contains("Unsupported schema/target combination"));
}
