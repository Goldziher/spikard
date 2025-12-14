use std::fs;
use std::process::Command;

#[test]
fn cli_can_generate_openapi_php() {
    let dir = tempfile::tempdir().expect("tempdir");
    let schema_path = dir.path().join("openapi.json");
    let out_path = dir.path().join("generated.php");

    let schema = serde_json::json!({
        "openapi": "3.0.3",
        "info": { "title": "Test API", "version": "1.0.0" },
        "paths": {
            "/health": {
                "get": {
                    "responses": { "200": { "description": "ok" } }
                }
            }
        }
    });
    fs::write(&schema_path, serde_json::to_vec(&schema).unwrap()).expect("write schema");

    let exe = env!("CARGO_BIN_EXE_spikard");
    let output = Command::new(exe)
        .args([
            "generate",
            "openapi",
            schema_path.to_str().unwrap(),
            "--lang",
            "php",
            "--output",
            out_path.to_str().unwrap(),
        ])
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let generated = fs::read_to_string(&out_path).expect("generated file");
    assert!(generated.contains("declare(strict_types=1);"));
}
