use spikard_cli::cli::run_from;
use std::path::PathBuf;

#[test]
fn cli_can_generate_jsonrpc_in_process() {
    let schema = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/user-api.openrpc.json");
    let dir = tempfile::tempdir().expect("tempdir");
    let output_path = dir.path().join("jsonrpc.py");

    run_from([
        "spikard".to_string(),
        "generate".to_string(),
        "jsonrpc".to_string(),
        schema.display().to_string(),
        "--lang".to_string(),
        "python".to_string(),
        "--output".to_string(),
        output_path.display().to_string(),
    ])
    .expect("cli run");

    assert!(output_path.exists());
}

#[test]
fn cli_can_generate_php_dtos_in_process() {
    let dir = tempfile::tempdir().expect("tempdir");

    run_from([
        "spikard".to_string(),
        "generate".to_string(),
        "php-dto".to_string(),
        "--output".to_string(),
        dir.path().display().to_string(),
    ])
    .expect("cli run");

    let entries = std::fs::read_dir(dir.path()).expect("read dir").count();
    assert!(entries > 0);
}
