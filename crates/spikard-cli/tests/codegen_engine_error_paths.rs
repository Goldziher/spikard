use spikard_cli::codegen::{CodegenEngine, CodegenRequest, CodegenTargetKind, SchemaKind, TargetLanguage};
use std::path::PathBuf;
use tempfile::tempdir;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn codegen_engine_rejects_unsupported_schema_target_combinations() {
    let tmp = tempdir().unwrap();
    let schema_path = tmp.path().join("schema.json");
    std::fs::write(&schema_path, "{}").unwrap();

    let request = CodegenRequest {
        schema_path,
        schema_kind: SchemaKind::OpenRpc,
        target: CodegenTargetKind::AsyncFixtures {
            output: tmp.path().to_path_buf(),
        },
        dto: None,
    };

    let err = CodegenEngine::execute(request).unwrap_err().to_string();
    assert!(err.contains("Unsupported schema/target combination"));
}

#[test]
fn codegen_engine_reports_asyncapi_unsupported_languages() {
    let output_dir = tempdir().unwrap();
    let out_file = output_dir.path().join("out.txt");

    let request = CodegenRequest {
        schema_path: workspace_root().join("examples/schemas/chat-service.asyncapi.yaml"),
        schema_kind: SchemaKind::AsyncApi,
        target: CodegenTargetKind::AsyncTestApp {
            language: TargetLanguage::Php,
            output: out_file,
        },
        dto: None,
    };

    let err = CodegenEngine::execute(request).unwrap_err().to_string();
    assert!(err.contains("AsyncAPI test apps"), "{err}");
}
