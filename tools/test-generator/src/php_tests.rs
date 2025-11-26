//! PHP PHPUnit test generator (placeholder).
//!
//! Generates a skipped test suite from fixtures to keep parity while bindings
//! are implemented.

use anyhow::{Context, Result};
use spikard_codegen::openapi::{Fixture, load_fixtures_from_dir};
use std::fs;
use std::path::Path;

pub fn generate_php_tests(fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let tests_dir = output_dir.join("tests");
    if tests_dir.exists() {
        fs::remove_dir_all(&tests_dir).context("Failed to clear existing PHP tests")?;
    }
    fs::create_dir_all(&tests_dir).context("Failed to create PHP tests directory")?;

    let fixtures = load_all_fixtures(fixtures_dir)?;
    let test_code = build_test_file(&fixtures);
    fs::write(tests_dir.join("GeneratedTest.php"), test_code).context("Failed to write GeneratedTest.php")?;

    let bootstrap = r#"<?php
declare(strict_types=1);

require_once __DIR__ . '/../bootstrap.php';
"#;
    fs::write(tests_dir.join("bootstrap.php"), bootstrap).context("Failed to write test bootstrap")?;

    fs::write(output_dir.join("bootstrap.php"), bootstrap_file()?).context("Failed to write bootstrap.php")?;
    fs::write(output_dir.join("phpunit.xml"), phpunit_config()?).context("Failed to write phpunit.xml")?;

    Ok(())
}

fn load_all_fixtures(fixtures_dir: &Path) -> Result<Vec<Fixture>> {
    let mut fixtures = load_fixtures_from_dir(fixtures_dir).context("Failed to load fixtures")?;
    fixtures.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(fixtures)
}

fn build_test_file(fixtures: &[Fixture]) -> String {
    let mut code = String::new();
    code.push_str(
        "<?php\ndeclare(strict_types=1);\n\nuse PHPUnit\\Framework\\TestCase;\n\n/**
 * Generated from testing_data fixtures.
 * These tests are skipped until the PHP bindings are implemented.
 */\nfinal class GeneratedTest extends TestCase\n{\n    protected function setUp(): void\n    {\n        $this->markTestSkipped('PHP bindings not implemented yet.');\n    }\n\n",
    );

    for (index, fixture) in fixtures.iter().enumerate() {
        let method_name = format!("test_fixture_{}", index + 1);
        let metadata = format!(
            "['name' => '{}', 'category' => '{}']",
            fixture.name,
            fixture.category.as_deref().unwrap_or("")
        );
        code.push_str(&format!(
            "    /**\n     * @group fixtures\n     * @var array<string, string> $meta\n     */\n    public function {method}(): void\n    {{\n        $meta = {meta};\n        $this->assertIsArray($meta);\n    }}\n\n",
            method = method_name,
            meta = metadata
        ));
    }

    code.push_str("}\n");
    code
}

fn phpunit_config() -> Result<String> {
    let config = r#"<?xml version="1.0" encoding="UTF-8"?>
<phpunit bootstrap="bootstrap.php" colors="true">
    <testsuites>
        <testsuite name="Spikard PHP E2E">
            <directory>tests</directory>
        </testsuite>
    </testsuites>
</phpunit>
"#;
    Ok(config.to_string())
}

fn bootstrap_file() -> Result<String> {
    let bootstrap = r#"<?php
declare(strict_types=1);

require_once __DIR__ . '/../../packages/php/vendor/autoload.php';
require_once __DIR__ . '/app/main.php';
"#;
    Ok(bootstrap.to_string())
}
