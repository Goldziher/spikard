use anyhow::Result;
use std::path::PathBuf;
use tempfile::TempDir;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .expect("CARGO_MANIFEST_DIR should be crates/spikard-cli")
        .to_path_buf()
}

#[test]
fn cli_features_command_runs() -> Result<()> {
    spikard_cli::cli::run_from(["spikard", "features"])?;
    Ok(())
}

#[test]
fn cli_validate_asyncapi_command_runs() -> Result<()> {
    let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
    spikard_cli::cli::run_from(["spikard", "validate-asyncapi", schema.to_string_lossy().as_ref()])?;
    Ok(())
}

#[test]
fn cli_testing_asyncapi_fixtures_generates_output() -> Result<()> {
    let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
    let tmp = TempDir::new()?;
    spikard_cli::cli::run_from([
        "spikard",
        "testing",
        "asyncapi",
        "fixtures",
        schema.to_string_lossy().as_ref(),
        "-o",
        tmp.path().to_string_lossy().as_ref(),
    ])?;

    fn count_json_files(root: &std::path::Path) -> std::io::Result<usize> {
        let mut count = 0usize;
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                count += count_json_files(&path)?;
            } else if path.extension().and_then(|s| s.to_str()) == Some("json") {
                count += 1;
            }
        }
        Ok(count)
    }

    let json_files = count_json_files(tmp.path())?;

    assert!(json_files > 0, "expected fixture JSON files to be generated");
    Ok(())
}

#[test]
fn cli_generate_asyncapi_handlers_writes_file() -> Result<()> {
    let schema = repo_root().join("examples/schemas/chat-service.asyncapi.yaml");
    let tmp = TempDir::new()?;
    let output = tmp.path().join("handlers.py");

    spikard_cli::cli::run_from([
        "spikard",
        "generate",
        "asyncapi",
        schema.to_string_lossy().as_ref(),
        "-l",
        "python",
        "-o",
        output.to_string_lossy().as_ref(),
    ])?;

    assert!(output.exists());
    assert!(output.metadata()?.len() > 0);
    Ok(())
}

#[test]
fn cli_generate_openapi_writes_file() -> Result<()> {
    let schema = repo_root().join("examples/schemas/todo-api.openapi.yaml");
    let tmp = TempDir::new()?;
    let output = tmp.path().join("handlers.py");

    spikard_cli::cli::run_from([
        "spikard",
        "generate",
        "openapi",
        schema.to_string_lossy().as_ref(),
        "-l",
        "python",
        "-o",
        output.to_string_lossy().as_ref(),
    ])?;

    assert!(output.exists());
    assert!(output.metadata()?.len() > 0);
    Ok(())
}

#[test]
fn cli_generate_openapi_multi_language_outputs_non_empty_files() -> Result<()> {
    let schema = repo_root().join("examples/schemas/todo-api.openapi.yaml");
    let tmp = TempDir::new()?;

    let cases = [
        ("ruby", "handlers.rb"),
        ("typescript", "handlers.ts"),
        ("rust", "handlers.rs"),
        ("php", "handlers.php"),
    ];

    for (lang, filename) in cases {
        let output = tmp.path().join(filename);
        spikard_cli::cli::run_from([
            "spikard",
            "generate",
            "openapi",
            schema.to_string_lossy().as_ref(),
            "-l",
            lang,
            "-o",
            output.to_string_lossy().as_ref(),
        ])?;
        assert!(output.exists(), "expected {filename} to exist");
        assert!(output.metadata()?.len() > 0, "expected {filename} to be non-empty");
    }

    Ok(())
}

#[test]
fn cli_generate_jsonrpc_writes_file() -> Result<()> {
    let schema = repo_root().join("examples/schemas/user-api.openrpc.json");
    let tmp = TempDir::new()?;
    let output = tmp.path().join("handlers.py");

    spikard_cli::cli::run_from([
        "spikard",
        "generate",
        "jsonrpc",
        schema.to_string_lossy().as_ref(),
        "-l",
        "python",
        "-o",
        output.to_string_lossy().as_ref(),
    ])?;

    assert!(output.exists());
    assert!(output.metadata()?.len() > 0);
    Ok(())
}

#[test]
fn cli_generate_php_dto_writes_files() -> Result<()> {
    let tmp = TempDir::new()?;
    spikard_cli::cli::run_from([
        "spikard",
        "generate",
        "php-dto",
        "-o",
        tmp.path().to_string_lossy().as_ref(),
    ])?;

    let request = tmp.path().join("Request.php");
    let response = tmp.path().join("Response.php");
    assert!(request.exists());
    assert!(response.exists());
    Ok(())
}
