//! Rust test generation

use anyhow::{Context, Result};
use spikard_codegen::openapi::Fixture;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn generate_rust_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    println!("Loading fixtures from {}...", fixtures_dir.display());

    // Load fixtures from all subdirectories
    let categories = discover_fixture_categories(fixtures_dir)?;

    println!("Found {} fixture categories", categories.len());

    let tests_dir = output_dir.join("tests");
    fs::create_dir_all(&tests_dir).context("Failed to create tests directory")?;

    // Generate test file for each category
    for (category, fixtures) in categories {
        let test_file = format!("{}_tests.rs", category);
        let content = generate_category_test_file(&category, &fixtures)?;

        fs::write(tests_dir.join(&test_file), content)
            .with_context(|| format!("Failed to write test file: {}", test_file))?;

        println!("  ✓ Generated {}", test_file);
    }

    // Generate common module
    let common_content = generate_common_module();
    let common_dir = tests_dir.join("common");
    fs::create_dir_all(&common_dir).context("Failed to create common directory")?;
    fs::write(common_dir.join("mod.rs"), common_content).context("Failed to write common/mod.rs")?;

    println!("  ✓ Generated common/mod.rs");

    Ok(())
}

fn discover_fixture_categories(fixtures_dir: &Path) -> Result<HashMap<String, Vec<(Fixture, String)>>> {
    let mut categories = HashMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|n| n.to_str())
                .context("Invalid directory name")?
                .to_string();

            let mut fixtures_with_files = Vec::new();

            // Load fixtures manually to track filenames
            for file_entry in fs::read_dir(&path).context("Failed to read category directory")? {
                let file_entry = file_entry.context("Failed to read file entry")?;
                let file_path = file_entry.path();

                if file_path.extension().is_some_and(|e| e == "json") {
                    let filename = file_path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .context("Invalid filename")?
                        .to_string();

                    if filename.starts_with("00-") || filename == "schema.json" {
                        continue;
                    }

                    let content = fs::read_to_string(&file_path)?;
                    match serde_json::from_str::<Fixture>(&content) {
                        Ok(fixture) => fixtures_with_files.push((fixture, filename)),
                        Err(e) => {
                            eprintln!("Warning: Skipping {}: {}", file_path.display(), e);
                        }
                    }
                }
            }

            if !fixtures_with_files.is_empty() {
                categories.insert(category, fixtures_with_files);
            }
        }
    }

    Ok(categories)
}

fn generate_category_test_file(category: &str, fixtures: &[(Fixture, String)]) -> Result<String> {
    let test_name = category.replace('-', "_");

    let mut test_cases = Vec::new();

    for (fixture, filename) in fixtures {
        let mut case_name = fixture
            .name
            .replace(['-', ' ', '/'], "_")
            .replace(['(', ')'], "")
            .replace(':', "_")
            .replace('+', "_plus_")
            .replace('=', "_eq_")
            .replace(['\'', '"'], "")
            .replace(['.', ','], "_")
            .to_lowercase();

        // Replace multiple consecutive underscores with single underscore
        while case_name.contains("__") {
            case_name = case_name.replace("__", "_");
        }

        let fixture_path = format!("../../testing_data/{}/{}", category, filename);
        let method = &fixture.request.method;
        let path = &fixture.request.path;
        let expected_status = fixture.expected_response.status_code;

        let test_case = format!(
            r#"
#[tokio::test]
async fn test_{category}_{case_name}() {{
    // Fixture: {fixture_name}
    // Description: {description}
    // Expected status: {expected_status}

    use axum::body::Body;
    use axum::http::{{Request, StatusCode}};
    use tower::ServiceExt;
    use serde_json::Value;

    // Load fixture
    let fixture_json = std::fs::read_to_string("{fixture_path}")
        .expect("Failed to read fixture file");
    let fixture: Value = serde_json::from_str(&fixture_json)
        .expect("Failed to parse fixture JSON");

    // Create app
    let app = spikard_e2e_app::create_app();

    // Build request
    let mut uri = "{path}".to_string();

    if let Some(query_params) = fixture["request"]["query_params"].as_object() {{
        let query_string = query_params
            .iter()
            .map(|(k, v)| format!("{{}}={{}}", k, v.as_str().unwrap_or("")))
            .collect::<Vec<_>>()
            .join("&");
        if !query_string.is_empty() {{
            uri.push_str("?");
            uri.push_str(&query_string);
        }}
    }}

    let request = Request::builder()
        .method("{method}")
        .uri(uri)
        .body(Body::empty())
        .unwrap();

    // Send request
    let response = app.oneshot(request).await.unwrap();

    // Assert status code
    assert_eq!(
        response.status(),
        StatusCode::from_u16({expected_status}).unwrap(),
        "Expected status {expected_status}, got {{:?}}",
        response.status()
    );
}}
"#,
            category = test_name,
            case_name = case_name,
            fixture_name = fixture.name,
            description = fixture.description,
            fixture_path = fixture_path,
            method = method,
            path = path,
            expected_status = expected_status,
        );

        test_cases.push(test_case);
    }

    Ok(format!(
        r#"//! Tests for {category} fixtures
//! Generated from: testing_data/{category}

#[cfg(test)]
mod {test_name} {{
{test_cases}
}}
"#,
        category = category,
        test_name = test_name,
        test_cases = test_cases.join("\n"),
    ))
}

fn generate_common_module() -> String {
    r#"//! Common test utilities

pub mod client {
    // TODO: Add HTTP client helpers
}

pub mod fixtures {
    // TODO: Add fixture loading helpers
}
"#
    .to_string()
}
