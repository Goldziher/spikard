//! PHP test app generator (placeholder).
//!
//!
//!

use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

pub fn generate_php_app(_fixtures_dir: &Path, output_dir: &Path) -> Result<()> {
    let app_dir = output_dir.join("app");
    if app_dir.exists() {
        fs::remove_dir_all(&app_dir).context("Failed to clear existing PHP app directory")?;
    }
    fs::create_dir_all(&app_dir).context("Failed to create PHP app directory")?;

    let main_php = r#"<?php
declare(strict_types=1);

use Spikard\App;

/**
 * Generated placeholder app factory.
 * Replace with real per-fixture routing when PHP bindings are ready.
 */
function create_app(): App
{
    return new App();
}
"#;

    fs::write(app_dir.join("main.php"), main_php).context("Failed to write PHP app main.php")?;
    Ok(())
}
