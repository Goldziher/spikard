//! Comprehensive integration tests for the `spikard init` command.
//!
//! These tests verify that the project initialization workflow correctly scaffolds
//! projects in all supported languages (Python, TypeScript, Rust, Ruby, PHP) with
//! proper file structure, content validation, and error handling.

use spikard_cli::codegen::TargetLanguage;
use spikard_cli::init::{InitEngine, InitRequest};
use tempfile::TempDir;

// ============================================================================
// PYTHON LANGUAGE SCAFFOLDER TESTS
// ============================================================================

#[test]
fn test_init_python_creates_all_files() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("my_api");

    let request = InitRequest {
        project_name: "my_api".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify response structure
    assert!(!response.files_created.is_empty(), "Should create files");
    assert!(!response.next_steps.is_empty(), "Should have next steps");

    // Verify expected files exist
    assert!(project_dir.join("pyproject.toml").exists());
    assert!(project_dir.join("README.md").exists());
    assert!(project_dir.join(".gitignore").exists());
    assert!(project_dir.join("src/my_api/__init__.py").exists());
    assert!(project_dir.join("src/my_api/app.py").exists());
    assert!(project_dir.join("tests/test_app.py").exists());

    Ok(())
}

#[test]
fn test_init_python_pyproject_toml_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("test_proj");

    let request = InitRequest {
        project_name: "test_proj".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let pyproject_path = project_dir.join("pyproject.toml");
    let content = std::fs::read_to_string(&pyproject_path)?;

    // Verify pyproject.toml contains required fields
    assert!(content.contains("[project]"));
    assert!(content.contains("name = \"test_proj\""));
    assert!(content.contains("version = \"0.1.0\""));
    assert!(content.contains("requires-python = \">=3.10\""));
    assert!(content.contains("spikard"));
    assert!(content.contains("pytest"));
    assert!(content.contains("mypy"));
    assert!(content.contains("ruff"));

    Ok(())
}

#[test]
fn test_init_python_with_underscores_in_name() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("api_service");

    let request = InitRequest {
        project_name: "api_service".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    assert!(!response.files_created.is_empty());
    assert!(project_dir.join("src/api_service/__init__.py").exists());
    assert!(project_dir.join("src/api_service/app.py").exists());

    // Verify pyproject.toml has the correct project name
    let pyproject_content = std::fs::read_to_string(project_dir.join("pyproject.toml"))?;
    assert!(pyproject_content.contains("name = \"api_service\""));

    Ok(())
}

#[test]
fn test_init_python_app_module_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("health_api");

    let request = InitRequest {
        project_name: "health_api".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let app_path = project_dir.join("src/health_api/app.py");
    let content = std::fs::read_to_string(&app_path)?;

    // Verify app module contains required elements
    assert!(content.contains("from spikard import App"));
    assert!(content.contains("app = App()"));
    assert!(content.contains("@app.get(\"/health\")"));
    assert!(content.contains("async def health"));

    Ok(())
}

#[test]
fn test_init_python_next_steps() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("next_step_test");

    let request = InitRequest {
        project_name: "next_step_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify next steps contain expected guidance
    assert!(response.next_steps.len() >= 2);
    assert!(response.next_steps[0].contains("cd next_step_test"));

    Ok(())
}

// ============================================================================
// TYPESCRIPT LANGUAGE SCAFFOLDER TESTS
// ============================================================================

#[test]
fn test_init_typescript_creates_all_files() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("my-api");

    let request = InitRequest {
        project_name: "my-api".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify files created
    assert!(response.files_created.len() >= 8);
    assert!(project_dir.join("package.json").exists());
    assert!(project_dir.join("tsconfig.json").exists());
    assert!(project_dir.join("vitest.config.ts").exists());
    assert!(project_dir.join("pnpm-lock.yaml").exists());
    assert!(project_dir.join(".gitignore").exists());
    assert!(project_dir.join("README.md").exists());
    assert!(project_dir.join("src/app.ts").exists());
    assert!(project_dir.join("tests/app.spec.ts").exists());

    Ok(())
}

#[test]
fn test_init_typescript_package_json_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("test-service");

    let request = InitRequest {
        project_name: "test-service".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let package_json_path = project_dir.join("package.json");
    let content = std::fs::read_to_string(&package_json_path)?;

    // Verify package.json content
    assert!(content.contains("\"name\"") && content.contains("test-service"));
    assert!(content.contains("\"version\": \"0.0.1\"") || content.contains("\"version\""));
    assert!(content.contains("\"type\": \"module\""));
    assert!(content.contains("dev") && content.contains("tsx"));
    assert!(content.contains("build") && content.contains("tsc"));
    assert!(content.contains("test") && content.contains("vitest"));

    Ok(())
}

#[test]
fn test_init_typescript_tsconfig_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("ts-app");

    let request = InitRequest {
        project_name: "ts-app".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let tsconfig_path = project_dir.join("tsconfig.json");
    let content = std::fs::read_to_string(&tsconfig_path)?;

    // Verify tsconfig.json has expected configuration
    assert!(content.contains("strict"));
    assert!(content.contains("moduleResolution"));
    assert!(content.contains("target"));

    Ok(())
}

#[test]
fn test_init_typescript_app_module_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("ts-server");

    let request = InitRequest {
        project_name: "ts-server".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let app_path = project_dir.join("src/app.ts");
    let content = std::fs::read_to_string(&app_path)?;

    // Verify app module structure
    assert!(content.contains("import"));
    assert!(content.contains("app"));

    Ok(())
}

#[test]
fn test_init_typescript_next_steps() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("ts-steps");

    let request = InitRequest {
        project_name: "ts-steps".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify next steps
    assert!(response.next_steps.len() >= 2);
    assert!(response.next_steps[0].contains("cd ts-steps"));
    assert!(response.next_steps.iter().any(|s| s.contains("pnpm")));

    Ok(())
}

#[test]
fn test_init_typescript_with_valid_kebab_case() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("my-api");

    let request = InitRequest {
        project_name: "my-api".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // TypeScript requires kebab-case format
    let package_json = std::fs::read_to_string(project_dir.join("package.json"))?;
    assert!(package_json.contains("my-api"));

    // Next steps should reference kebab-case
    assert!(response.next_steps[0].contains("my-api"));

    Ok(())
}

// ============================================================================
// RUST LANGUAGE SCAFFOLDER TESTS
// ============================================================================

#[test]
fn test_init_rust_creates_all_files() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("my_app");

    let request = InitRequest {
        project_name: "my_app".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify file count
    assert!(response.files_created.len() >= 7);

    // Verify expected files exist
    assert!(project_dir.join("Cargo.toml").exists());
    assert!(project_dir.join("Cargo.lock").exists());
    assert!(project_dir.join("src/main.rs").exists());
    assert!(project_dir.join("src/lib.rs").exists());
    assert!(project_dir.join("tests/integration_test.rs").exists());
    assert!(project_dir.join(".gitignore").exists());
    assert!(project_dir.join("README.md").exists());

    Ok(())
}

#[test]
fn test_init_rust_cargo_toml_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("rust_service");

    let request = InitRequest {
        project_name: "rust_service".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let cargo_toml_path = project_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&cargo_toml_path)?;

    // Verify Cargo.toml contains required fields
    assert!(content.contains("[package]"));
    assert!(content.contains("name = \"rust-service\"") || content.contains("name = \"rust_service\""));
    assert!(content.contains("version = \"0.1.0\""));
    assert!(content.contains("edition = \"2024\""));
    assert!(content.contains("[dependencies]"));
    assert!(content.contains("spikard"));

    Ok(())
}

#[test]
fn test_init_rust_main_rs_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("rust_app");

    let request = InitRequest {
        project_name: "rust_app".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let main_rs_path = project_dir.join("src/main.rs");
    let content = std::fs::read_to_string(&main_rs_path)?;

    // Verify main.rs has main function
    assert!(content.contains("fn main"));
    assert!(content.contains("#[tokio::main]") || content.contains("async fn main"));

    Ok(())
}

#[test]
fn test_init_rust_lib_rs_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("rust_lib");

    let request = InitRequest {
        project_name: "rust_lib".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let lib_rs_path = project_dir.join("src/lib.rs");
    let content = std::fs::read_to_string(&lib_rs_path)?;

    // Verify lib.rs exists and has module documentation
    assert!(!content.is_empty());

    Ok(())
}

#[test]
fn test_init_rust_next_steps() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("rust_steps");

    let request = InitRequest {
        project_name: "rust_steps".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify next steps
    assert!(response.next_steps.len() >= 3);
    assert!(response.next_steps[0].contains("cd rust_steps"));
    assert!(response.next_steps.iter().any(|s| s.contains("cargo build")));
    assert!(response.next_steps.iter().any(|s| s.contains("cargo run")));

    Ok(())
}

// ============================================================================
// PHP LANGUAGE SCAFFOLDER TESTS
// ============================================================================

#[test]
fn test_init_php_creates_all_files() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("my_api");

    let request = InitRequest {
        project_name: "my_api".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify file count
    assert!(response.files_created.len() >= 8);

    // Verify expected files exist
    assert!(project_dir.join("composer.json").exists());
    assert!(project_dir.join("composer.lock").exists());
    assert!(project_dir.join("phpstan.neon").exists());
    assert!(project_dir.join("phpunit.xml").exists());
    assert!(project_dir.join("src/App.php").exists());
    assert!(project_dir.join("tests/AppTest.php").exists());
    assert!(project_dir.join(".gitignore").exists());
    assert!(project_dir.join("README.md").exists());

    Ok(())
}

#[test]
fn test_init_php_composer_json_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("php_service");

    let request = InitRequest {
        project_name: "php_service".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let composer_json_path = project_dir.join("composer.json");
    let content = std::fs::read_to_string(&composer_json_path)?;

    // Verify composer.json structure
    assert!(content.contains("\"name\""));
    assert!(content.contains("\"description\""));
    assert!(content.contains("\"require\""));
    assert!(content.contains("\"php\""));
    assert!(content.contains("spikard"));
    assert!(content.contains("\"autoload\""));
    assert!(content.contains("\"psr-4\""));

    Ok(())
}

#[test]
fn test_init_php_app_class_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("php_app");

    let request = InitRequest {
        project_name: "php_app".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let app_php_path = project_dir.join("src/App.php");
    let content = std::fs::read_to_string(&app_php_path)?;

    // Verify PHP file structure
    assert!(content.contains("<?php"));
    assert!(content.contains("declare(strict_types=1)"));
    assert!(content.contains("class App"));

    Ok(())
}

#[test]
fn test_init_php_next_steps() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("php_steps");

    let request = InitRequest {
        project_name: "php_steps".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify next steps
    assert!(response.next_steps.len() >= 2);
    assert!(response.next_steps[0].contains("cd php_steps"));
    assert!(response.next_steps.iter().any(|s| s.contains("composer")));

    Ok(())
}

#[test]
fn test_init_php_with_uppercase_and_underscore() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("MyPhpApp");

    let request = InitRequest {
        project_name: "MyPhpApp".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    assert!(!response.files_created.is_empty());
    assert!(project_dir.join("composer.json").exists());

    Ok(())
}

// ============================================================================
// ERROR HANDLING TESTS
// ============================================================================

#[test]
fn test_init_invalid_python_project_name_with_hyphen() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("my-api");

    let request = InitRequest {
        project_name: "my-api".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
    // Directory should not be created when validation fails
    assert!(!project_dir.exists());
}

#[test]
fn test_init_invalid_python_project_name_starting_with_digit() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("2api");

    let request = InitRequest {
        project_name: "2api".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
    assert!(!project_dir.exists());
}

#[test]
fn test_init_invalid_typescript_project_name_with_underscore() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("my_api");

    let request = InitRequest {
        project_name: "my_api".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    // TypeScript validation rejects underscores
    let result = InitEngine::execute(request);
    assert!(result.is_err());
    assert!(!project_dir.exists());
}

#[test]
fn test_init_invalid_rust_project_name_starting_with_digit() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("4rust");

    let request = InitRequest {
        project_name: "4rust".to_string(),
        language: TargetLanguage::Rust,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
    assert!(!project_dir.exists());
}

#[test]
fn test_init_invalid_php_project_name_starting_with_digit() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("5app");

    let request = InitRequest {
        project_name: "5app".to_string(),
        language: TargetLanguage::Php,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
    assert!(!project_dir.exists());
}

#[test]
fn test_init_empty_project_name() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("empty");

    let request = InitRequest {
        project_name: String::new(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
}

#[test]
fn test_init_directory_already_exists() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("existing");

    // Create the directory first
    std::fs::create_dir_all(&project_dir)?;

    let request = InitRequest {
        project_name: "existing".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());

    // Verify the error is about directory already existing
    let err_msg = format!("{:?}", result);
    assert!(err_msg.contains("already exists") || err_msg.contains("DirectoryAlreadyExists"));

    Ok(())
}

#[test]
fn test_init_with_nonexistent_schema_path() {
    let temp_dir = TempDir::new().unwrap();
    let project_dir = temp_dir.path().join("schema_test");
    let nonexistent_schema = temp_dir.path().join("nonexistent.json");

    let request = InitRequest {
        project_name: "schema_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: Some(nonexistent_schema),
    };

    let result = InitEngine::execute(request);
    assert!(result.is_err());
    assert!(!project_dir.exists());
}

// ============================================================================
// INTEGRATION AND EDGE CASE TESTS
// ============================================================================

#[test]
fn test_init_creates_nested_directory_structure() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("nested/path/project");

    let request = InitRequest {
        project_name: "project".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    assert!(!response.files_created.is_empty());
    assert!(project_dir.exists());
    assert!(project_dir.join("pyproject.toml").exists());

    Ok(())
}

#[test]
fn test_init_all_languages_minimal_file_count() -> anyhow::Result<()> {
    let temp_base = TempDir::new()?;

    // Test Python
    let py_dir = temp_base.path().join("py_test");
    let py_request = InitRequest {
        project_name: "py_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: py_dir,
        schema_path: None,
    };
    let py_response = InitEngine::execute(py_request)?;
    assert!(py_response.files_created.len() >= 6);

    // Test TypeScript (use kebab-case)
    let ts_dir = temp_base.path().join("ts-test");
    let ts_request = InitRequest {
        project_name: "ts-test".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: ts_dir,
        schema_path: None,
    };
    let ts_response = InitEngine::execute(ts_request)?;
    assert!(ts_response.files_created.len() >= 8);

    // Test Rust
    let rs_dir = temp_base.path().join("rs_test");
    let rs_request = InitRequest {
        project_name: "rs_test".to_string(),
        language: TargetLanguage::Rust,
        project_dir: rs_dir,
        schema_path: None,
    };
    let rs_response = InitEngine::execute(rs_request)?;
    assert!(rs_response.files_created.len() >= 7);

    // Test PHP
    let php_dir = temp_base.path().join("php_test");
    let php_request = InitRequest {
        project_name: "php_test".to_string(),
        language: TargetLanguage::Php,
        project_dir: php_dir,
        schema_path: None,
    };
    let php_response = InitEngine::execute(php_request)?;
    assert!(php_response.files_created.len() >= 8);

    Ok(())
}

#[test]
fn test_init_response_paths_are_absolute() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("absolute_test");

    let request = InitRequest {
        project_name: "absolute_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    // Verify all returned paths are absolute
    for path in &response.files_created {
        assert!(path.is_absolute(), "Path should be absolute: {:?}", path);
    }

    Ok(())
}

#[test]
fn test_init_python_single_letter_project_name() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("a");

    let request = InitRequest {
        project_name: "a".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    let response = InitEngine::execute(request)?;

    assert!(!response.files_created.is_empty());
    assert!(project_dir.join("src/a/__init__.py").exists());

    Ok(())
}

#[test]
fn test_init_multiple_consecutive_operations() -> anyhow::Result<()> {
    let temp_base = TempDir::new()?;

    // Initialize first project
    let proj1_dir = temp_base.path().join("project1");
    let request1 = InitRequest {
        project_name: "project1".to_string(),
        language: TargetLanguage::Python,
        project_dir: proj1_dir.clone(),
        schema_path: None,
    };
    let response1 = InitEngine::execute(request1)?;

    // Initialize second project in same temp directory
    let proj2_dir = temp_base.path().join("project2");
    let request2 = InitRequest {
        project_name: "project2".to_string(),
        language: TargetLanguage::TypeScript,
        project_dir: proj2_dir.clone(),
        schema_path: None,
    };
    let response2 = InitEngine::execute(request2)?;

    // Verify both projects were created correctly
    assert!(!response1.files_created.is_empty());
    assert!(!response2.files_created.is_empty());
    assert!(proj1_dir.exists());
    assert!(proj2_dir.exists());
    assert!(proj1_dir.join("pyproject.toml").exists());
    assert!(proj2_dir.join("package.json").exists());

    Ok(())
}

#[test]
fn test_init_gitignore_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("gitignore_test");

    let request = InitRequest {
        project_name: "gitignore_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let gitignore_path = project_dir.join(".gitignore");
    assert!(gitignore_path.exists());

    let content = std::fs::read_to_string(&gitignore_path)?;
    assert!(!content.is_empty());

    Ok(())
}

#[test]
fn test_init_readme_content() -> anyhow::Result<()> {
    let temp_dir = TempDir::new()?;
    let project_dir = temp_dir.path().join("readme_test");

    let request = InitRequest {
        project_name: "readme_test".to_string(),
        language: TargetLanguage::Python,
        project_dir: project_dir.clone(),
        schema_path: None,
    };

    InitEngine::execute(request)?;

    let readme_path = project_dir.join("README.md");
    assert!(readme_path.exists());

    let content = std::fs::read_to_string(&readme_path)?;
    assert!(!content.is_empty());
    assert!(content.contains("#"));

    Ok(())
}
