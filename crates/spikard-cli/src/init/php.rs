//! PHP Project Scaffolder
//!
//! Generates a minimal PHP project structure with Spikard integration.
//! Follows PSR-4 autoloading conventions and modern PHP 8.2+ standards.

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use std::path::{Path, PathBuf};

/// PHP project scaffolder
pub struct PhpScaffolder;

impl ProjectScaffolder for PhpScaffolder {
    #[allow(clippy::vec_init_then_push)]
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let mut files = Vec::new();

        // Create composer.json
        files.push(ScaffoldedFile::new(
            PathBuf::from("composer.json"),
            self.generate_composer_json(project_name),
        ));

        // Create phpstan.neon
        files.push(ScaffoldedFile::new(
            PathBuf::from("phpstan.neon"),
            self.generate_phpstan_neon(),
        ));

        // Create phpunit.xml
        files.push(ScaffoldedFile::new(
            PathBuf::from("phpunit.xml"),
            self.generate_phpunit_xml(),
        ));

        // Create src/AppController.php
        files.push(ScaffoldedFile::new(
            PathBuf::from("src/AppController.php"),
            self.generate_app_php(),
        ));

        // Create bin/server.php
        files.push(ScaffoldedFile::new(
            PathBuf::from("bin/server.php"),
            self.generate_server_php(),
        ));

        // Create tests/AppTest.php
        files.push(ScaffoldedFile::new(
            PathBuf::from("tests/AppTest.php"),
            self.generate_app_test_php(),
        ));

        // Create .gitignore
        files.push(ScaffoldedFile::new(
            PathBuf::from(".gitignore"),
            self.generate_gitignore(),
        ));

        // Create README.md
        files.push(ScaffoldedFile::new(
            PathBuf::from("README.md"),
            self.generate_readme(project_name),
        ));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        vec![
            format!("cd {}", project_name),
            "composer install".to_string(),
            "php bin/server.php".to_string(),
        ]
    }
}

impl PhpScaffolder {
    fn generate_composer_json(&self, project_name: &str) -> String {
        let version = env!("CARGO_PKG_VERSION");
        let package_name = project_name.replace('_', "-").to_lowercase();
        format!(
            r#"{{
  "name": "your-vendor/{package_name}",
  "description": "Spikard PHP application",
  "type": "project",
  "require": {{
    "php": "^8.2",
    "spikard/spikard": "^{version}"
  }},
  "require-dev": {{
    "phpunit/phpunit": "^11.0",
    "phpstan/phpstan": "^1.10"
  }},
  "autoload": {{
    "psr-4": {{
      "App\\": "src/"
    }}
  }},
  "autoload-dev": {{
    "psr-4": {{
      "App\\Tests\\": "tests/"
    }}
  }},
  "authors": [
    {{
      "name": "Your Name",
      "email": "you@example.com"
    }}
  ],
  "license": "MIT",
  "scripts": {{
    "serve": "php bin/server.php",
    "test": "vendor/bin/phpunit --configuration phpunit.xml",
    "phpstan": "vendor/bin/phpstan analyse --configuration phpstan.neon"
  }}
}}
"#
        )
    }

    fn generate_phpstan_neon(&self) -> String {
        r"parameters:
  level: max
  paths:
    - src
    - tests
  excludePaths:
    - */vendor/*
  treatPhpDocTypesAsCertain: false
  checkMissingIterableValueType: false
"
        .to_string()
    }

    fn generate_phpunit_xml(&self) -> String {
        r#"<?xml version="1.0" encoding="UTF-8"?>
<phpunit xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:noNamespaceSchemaLocation="https://schema.phpunit.de/11.0/phpunit.xsd"
         bootstrap="vendor/autoload.php"
         cacheDirectory=".phpunit.cache"
         colors="true"
         verbose="true">
  <testsuites>
    <testsuite name="Unit Tests">
      <directory>tests</directory>
    </testsuite>
  </testsuites>

  <coverage processUncoveredFiles="true">
    <include>
      <directory suffix=".php">src</directory>
    </include>
    <exclude>
      <directory>tests</directory>
    </exclude>
  </coverage>
</phpunit>
"#
        .to_string()
    }

    fn generate_app_php(&self) -> String {
        r"<?php

declare(strict_types=1);

namespace App;

use Spikard\Attributes\Get;
use Spikard\Http\Response;

/**
 * Main application controller
 *
 * Demonstrates a simple Spikard application with a health check endpoint.
 */
final class AppController
{
    #[Get('/health')]
    public function health(): Response
    {
        return Response::json(['status' => 'healthy', 'message' => 'Server is running']);
    }

    #[Get('/')]
    public function index(): Response
    {
        return Response::text('Welcome to Spikard PHP');
    }
}
"
        .to_string()
    }

    fn generate_server_php(&self) -> String {
        r#"<?php

declare(strict_types=1);

require_once __DIR__ . '/../vendor/autoload.php';

use App\AppController;
use Spikard\App;
use Spikard\Config\ServerConfig;

$config = new ServerConfig(port: 8000);
$app = (new App($config))->registerController(new AppController());

echo "Starting server on http://127.0.0.1:8000\n";
echo "Press Ctrl+C to stop\n\n";

$app->run();
"#
        .to_string()
    }

    fn generate_app_test_php(&self) -> String {
        r"<?php

declare(strict_types=1);

namespace App\Tests;

use App\AppController;
use PHPUnit\Framework\TestCase;
use Spikard\Http\Response;

/**
 * Tests for the main application
 */
final class AppTest extends TestCase
{
    public function testControllerCreatesResponses(): void
    {
        $controller = new AppController();

        $this->assertInstanceOf(Response::class, $controller->health());
        $this->assertInstanceOf(Response::class, $controller->index());
    }
}
"
        .to_string()
    }

    fn generate_gitignore(&self) -> String {
        r"# Dependencies
/vendor/

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# PHP
.php-version

# Testing
.phpunit.cache/
coverage/

# Environment
.env
.env.local
.env.*.local

# OS
.DS_Store
Thumbs.db
"
        .to_string()
    }

    fn generate_readme(&self, project_name: &str) -> String {
        format!(
            r"# {project_name}

A Spikard PHP application.

## Requirements

- PHP 8.2+
- Composer

## Installation

```bash
composer install
```

## Running the Application

```bash
php bin/server.php
```

The server will start on `http://127.0.0.1:8000`.

## Testing

```bash
composer test
```

## Static Analysis

```bash
composer phpstan
```

## Next Steps

1. Install dependencies: `composer install`
2. Run the server: `php bin/server.php`
3. Make requests to `http://localhost:8000/health` to verify

## Documentation

- [Spikard Documentation](https://spikard.dev)
- [PHP PSR Standards](https://www.php-fig.org/)
"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_php_scaffolder_generates_composer_json() {
        let scaffolder = PhpScaffolder;
        let content = scaffolder.generate_composer_json("test-app");

        assert!(content.contains("\"your-vendor/test-app\""));
        assert!(content.contains("\"php\": \"^8.2\""));
        assert!(content.contains("\"spikard/spikard\": \"^"));
        assert!(content.contains("\"psr-4\""));
    }

    #[test]
    fn test_php_scaffolder_generates_phpstan_config() {
        let scaffolder = PhpScaffolder;
        let content = scaffolder.generate_phpstan_neon();

        assert!(content.contains("level: max"));
        assert!(content.contains("- src"));
        assert!(content.contains("- tests"));
    }

    #[test]
    fn test_php_scaffolder_generates_php_files_with_strict_types() {
        let scaffolder = PhpScaffolder;
        let app_content = scaffolder.generate_app_php();

        assert!(app_content.starts_with("<?php"));
        assert!(app_content.contains("declare(strict_types=1);"));
        assert!(app_content.contains("namespace App;"));

        let test_content = scaffolder.generate_app_test_php();
        assert!(test_content.starts_with("<?php"));
        assert!(test_content.contains("declare(strict_types=1);"));
        assert!(test_content.contains("namespace App\\Tests;"));
    }

    #[test]
    fn test_php_scaffolder_next_steps() {
        let scaffolder = PhpScaffolder;
        let steps = scaffolder.next_steps("my-project");

        assert_eq!(steps.len(), 3);
        assert!(steps[0].contains("cd my-project"));
        assert_eq!(steps[1], "composer install");
        assert_eq!(steps[2], "php bin/server.php");
    }
}
