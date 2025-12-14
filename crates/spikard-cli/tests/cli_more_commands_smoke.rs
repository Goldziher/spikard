use std::path::PathBuf;
use std::process::Command;

#[test]
fn cli_can_validate_asyncapi_specs() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/chat-service.asyncapi.yaml");

    let output = Command::new(exe)
        .arg("validate-asyncapi")
        .arg(schema)
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn cli_can_generate_jsonrpc_scaffolding_from_openrpc() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/user-api.openrpc.json");
    let dir = tempfile::tempdir().expect("tempdir");
    let output_path = dir.path().join("jsonrpc.py");

    let output = Command::new(exe)
        .arg("generate")
        .arg("jsonrpc")
        .arg(schema)
        .arg("--lang")
        .arg("python")
        .arg("--output")
        .arg(&output_path)
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output_path.exists());
}

#[test]
fn cli_can_generate_php_dtos() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let dir = tempfile::tempdir().expect("tempdir");

    let output = Command::new(exe)
        .arg("generate")
        .arg("php-dto")
        .arg("--output")
        .arg(dir.path())
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let entries = std::fs::read_dir(dir.path()).expect("read dir").count();
    assert!(entries > 0);
}
