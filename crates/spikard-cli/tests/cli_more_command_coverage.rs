#![allow(
    clippy::doc_markdown,
    clippy::items_after_statements,
    reason = "Test file for CLI commands"
)]

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
fn cli_init_command_creates_python_project_by_default() -> Result<()> {
    let tmp = TempDir::new()?;
    let project_name = "cli_init_demo";

    spikard_cli::cli::run_from([
        "spikard",
        "init",
        project_name,
        "--dir",
        tmp.path().to_string_lossy().as_ref(),
    ])?;

    let project_dir = tmp.path().join(project_name);
    assert!(project_dir.exists());
    assert!(project_dir.join("pyproject.toml").exists());
    assert!(project_dir.join("src").join(project_name).join("__init__.py").exists());
    assert!(project_dir.join("src").join(project_name).join("app.py").exists());
    assert!(project_dir.join("tests").join("test_app.py").exists());
    Ok(())
}

#[test]
fn cli_init_command_creates_expected_project_structures_for_each_binding() -> Result<()> {
    let cases = [
        (
            "python",
            "py_demo",
            vec![
                "pyproject.toml",
                "README.md",
                ".gitignore",
                "src/py_demo/__init__.py",
                "src/py_demo/app.py",
                "tests/test_app.py",
            ],
        ),
        (
            "typescript",
            "ts-demo",
            vec![
                "package.json",
                "tsconfig.json",
                "vitest.config.ts",
                "pnpm-lock.yaml",
                ".gitignore",
                "README.md",
                "src/app.ts",
                "tests/app.spec.ts",
            ],
        ),
        (
            "rust",
            "rust_demo",
            vec![
                "Cargo.toml",
                "Cargo.lock",
                "README.md",
                ".gitignore",
                "src/main.rs",
                "src/lib.rs",
                "tests/integration_test.rs",
            ],
        ),
        (
            "ruby",
            "ruby_demo",
            vec![
                "Gemfile",
                "Gemfile.lock",
                ".ruby-version",
                ".gitignore",
                "README.md",
                "lib/ruby_demo.rb",
                "sig/ruby_demo.rbs",
                "spec/ruby_demo_spec.rb",
                ".rspec",
                "Rakefile",
            ],
        ),
        (
            "php",
            "php_demo",
            vec![
                "composer.json",
                "composer.lock",
                "phpstan.neon",
                "phpunit.xml",
                ".gitignore",
                "README.md",
                "src/App.php",
                "tests/AppTest.php",
            ],
        ),
        (
            "elixir",
            "elixir_demo",
            vec![
                "mix.exs",
                ".formatter.exs",
                ".gitignore",
                "lib/elixir_demo.ex",
                "test/elixir_demo_test.exs",
                "test/test_helper.exs",
            ],
        ),
    ];

    for (lang, project_name, expected_paths) in cases {
        let tmp = TempDir::new()?;

        spikard_cli::cli::run_from([
            "spikard",
            "init",
            project_name,
            "--lang",
            lang,
            "--dir",
            tmp.path().to_string_lossy().as_ref(),
        ])?;

        let project_dir = tmp.path().join(project_name);
        assert!(project_dir.exists(), "expected {} project root", lang);

        for expected in expected_paths {
            assert!(
                project_dir.join(expected).exists(),
                "expected {} to create {}",
                lang,
                expected
            );
        }
    }

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

#[test]
fn cli_generate_protobuf_resolves_imports_from_include_paths() -> Result<()> {
    let tmp = TempDir::new()?;
    let shared_dir = tmp.path().join("common");
    std::fs::create_dir_all(&shared_dir)?;

    let shared_proto = shared_dir.join("types.proto");
    std::fs::write(
        &shared_proto,
        r#"syntax = "proto3";

package common;

message SharedType {
  string id = 1;
}
"#,
    )?;

    let root_proto = tmp.path().join("service.proto");
    std::fs::write(
        &root_proto,
        r#"syntax = "proto3";

import "common/types.proto";

package example;

message UsesShared {
  SharedType shared = 1;
}
"#,
    )?;

    let output = tmp.path().join("generated.py");

    spikard_cli::cli::run_from([
        "spikard",
        "generate",
        "protobuf",
        root_proto.to_string_lossy().as_ref(),
        "-l",
        "python",
        "-o",
        output.to_string_lossy().as_ref(),
        "--include",
        tmp.path().to_string_lossy().as_ref(),
    ])?;

    let generated = std::fs::read_to_string(&output)?;
    assert!(generated.contains("class SharedType"));
    assert!(generated.contains("class UsesShared"));
    Ok(())
}
