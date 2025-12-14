use std::path::Path;
use std::process::Command;

fn count_json_files(dir: &Path) -> usize {
    let mut count = 0;
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return 0,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            count += count_json_files(&path);
        } else if path.extension().is_some_and(|ext| ext == "json") {
            count += 1;
        }
    }

    count
}

#[test]
fn cli_can_generate_asyncapi_fixtures_from_examples() {
    let exe = env!("CARGO_BIN_EXE_spikard");
    let dir = tempfile::tempdir().expect("tempdir");
    let output_dir = dir.path().join("testing_data");

    let schema = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/schemas/chat-service.asyncapi.yaml");
    assert!(schema.exists(), "missing example schema at {}", schema.display());

    let output = Command::new(exe)
        .args([
            "testing",
            "asyncapi",
            "fixtures",
            schema.to_str().unwrap(),
            "--output",
            output_dir.to_str().unwrap(),
        ])
        .output()
        .expect("run failed");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        count_json_files(&output_dir) > 0,
        "expected generated fixture json files"
    );
}
