//! PHP test app generator (stub).
//!
//! Generates a namespaced AppFactory with per-category creators. Route wiring
//! will be filled in once the PHP bindings are implemented; for now the
//! factories return empty `Spikard\App` instances to keep generated tests
//! structured for TDD.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

pub fn generate_php_app(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("app");
    if app_dir.exists() {
        fs::remove_dir_all(&app_dir).context("Failed to clear existing PHP app directory")?;
    }
    fs::create_dir_all(&app_dir).context("Failed to create PHP app directory")?;

    let fixtures_by_category = load_fixtures_grouped(fixtures_dir)?;
    let code = build_app_factory(&fixtures_by_category);
    fs::write(app_dir.join("main.php"), code).context("Failed to write PHP app main.php")?;
    Ok(())
}

fn load_fixtures_grouped(fixtures_dir: &Path) -> Result<BTreeMap<String, Vec<Fixture>>> {
    let mut grouped: BTreeMap<String, Vec<Fixture>> = BTreeMap::new();

    for entry in fs::read_dir(fixtures_dir).context("Failed to read fixtures directory")? {
        let entry = entry.context("Failed to read fixture directory entry")?;
        let path = entry.path();
        if path.is_dir() {
            let category = path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("fixtures")
                .to_string();
            let mut fixtures = load_fixtures_from_dir(&path)
                .with_context(|| format!("Failed to load fixtures from {}", path.display()))?;
            fixtures.sort_by(|a, b| a.name.cmp(&b.name));
            grouped.insert(category, fixtures);
        }
    }

    Ok(grouped)
}

fn build_app_factory(fixtures_by_category: &BTreeMap<String, Vec<Fixture>>) -> String {
    let mut code = String::new();
    code.push_str(
        "<?php\n\ndeclare(strict_types=1);\n\nnamespace E2E\\Php;\n\nuse Spikard\\App;\n\n/**\n * Generated App factory for PHP e2e tests.\n * Routes will be registered once the PHP bindings are wired.\n */\nfinal class AppFactory\n{\n",
    );

    if fixtures_by_category.is_empty() {
        code.push_str("    public static function create(): App\n    {\n        return new App();\n    }\n}\n");
        return code;
    }

    for (category, fixtures) in fixtures_by_category {
        let method_name = format!("create_{}", sanitize_identifier(category));
        code.push_str(&format!(
            "    public static function {method}(): App\n    {{\n        // TODO: register {count} routes for category '{category}'\n        return new App();\n    }}\n\n",
            method = method_name,
            count = fixtures.len(),
            category = category
        ));
    }

    code.push_str("}\n");
    code
}

fn sanitize_identifier(input: &str) -> String {
    let mut s = input
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    while s.contains("__") {
        s = s.replace("__", "_");
    }
    s.trim_matches('_').to_ascii_lowercase()
}
