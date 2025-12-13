use std::path::PathBuf;
use std::process::Command;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("workspace root")
        .to_path_buf()
}

#[test]
fn cli_rejects_invalid_python_dto_choice() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = Command::new(exe)
        .current_dir(workspace_root())
        .args([
            "generate",
            "openapi",
            "examples/schemas/todo-api.openapi.yaml",
            "--lang",
            "python",
            "--dto",
            "zod",
        ])
        .output()
        .expect("run failed");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not supported for Python"));
}

#[test]
fn cli_rejects_invalid_typescript_dto_choice() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = Command::new(exe)
        .current_dir(workspace_root())
        .args([
            "generate",
            "openapi",
            "examples/schemas/todo-api.openapi.yaml",
            "--lang",
            "typescript",
            "--dto",
            "msgspec",
        ])
        .output()
        .expect("run failed");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not supported for TypeScript"));
}
