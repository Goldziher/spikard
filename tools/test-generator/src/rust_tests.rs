//! Rust test generation

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
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

fn discover_fixture_categories(fixtures_dir: &Path) -> Result<HashMap<String, Vec<Fixture>>> {
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

            let fixtures =
                load_fixtures_from_dir(&path).with_context(|| format!("Failed to load fixtures from {}", category))?;

            if !fixtures.is_empty() {
                categories.insert(category, fixtures);
            }
        }
    }

    Ok(categories)
}

fn generate_category_test_file(category: &str, fixtures: &[Fixture]) -> Result<String> {
    let test_name = category.replace('-', "_");

    let mut test_cases = Vec::new();

    for fixture in fixtures {
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

        let test_case = format!(
            r#"
#[tokio::test]
#[ignore = "Test not yet implemented"]
async fn test_{category}_{case_name}() {{
    // Fixture: {fixture_name}
    // Description: {description}

    // TODO: Load fixture and execute test
    // Expected status: {status_code}

    todo!("Implement test for fixture: {fixture_name}");
}}
"#,
            category = test_name,
            case_name = case_name,
            fixture_name = fixture.name,
            description = fixture.description,
            status_code = fixture.expected_response.status_code,
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
