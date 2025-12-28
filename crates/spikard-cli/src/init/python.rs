//! Python project scaffolder for Spikard applications.
//!
//! This module provides a complete Python project scaffold with:
//! - `pyproject.toml` with uv configuration
//! - Spikard dependency
//! - Example application with health endpoint
//! - Test suite with pytest
//! - `.gitignore` for Python projects
//! - README with setup instructions

use super::scaffolder::{ProjectScaffolder, ScaffoldedFile};
use anyhow::Result;
use std::path::Path;
use std::path::PathBuf;

/// Python project scaffolder
pub struct PythonScaffolder;

impl PythonScaffolder {
    /// Convert a project name to a valid Python package name (snake_case)
    fn to_package_name(project_name: &str) -> String {
        // Replace hyphens with underscores and convert to lowercase
        project_name.replace('-', "_").to_lowercase()
    }

    /// Generate pyproject.toml content
    fn generate_pyproject_toml(project_name: &str, _package_name: &str) -> String {
        format!(
            r#"[project]
name = "{}"
version = "0.1.0"
description = "A Spikard application"
requires-python = ">=3.10"
dependencies = [
    "spikard>=0.6.0",
]

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv]
dev-dependencies = [
    "pytest>=8.0.0",
    "mypy>=1.8.0",
    "ruff>=0.3.0",
]

[tool.uv.sources]
spikard = {{ path = ".", editable = true }}
"#,
            project_name
        )
    }

    /// Generate the main application module
    fn generate_app_module(package_name: &str) -> String {
        format!(
            r#""""Main application module."""
from spikard import App

app = App()

@app.get("/health")
async def health() -> dict[str, str]:
    """Health check endpoint."""
    return {{"status": "ok"}}


if __name__ == "__main__":
    # Run the application
    # For local development: uvicorn {}.app:app --reload
    pass
"#,
            package_name
        )
    }

    /// Generate the __init__.py file for the package
    fn generate_init_py() -> String {
        r#""""Spikard application package.""""#.to_string()
    }

    /// Generate test file
    fn generate_test_app(package_name: &str) -> String {
        format!(
            r#""""Tests for the application."""
import pytest
from {}.app import app


@pytest.fixture
def client():
    """Create a test client."""
    from spikard.testing import TestClient
    return TestClient(app)


def test_health(client):
    """Test health endpoint."""
    response = client.get("/health")
    assert response.status_code == 200
    assert response.json() == {{"status": "ok"}}
"#,
            package_name
        )
    }

    /// Generate .gitignore file
    fn generate_gitignore() -> String {
        r#"__pycache__/
*.py[cod]
*$py.class
*.so
.Python
build/
develop-eggs/
dist/
downloads/
eggs/
.eggs/
lib/
lib64/
parts/
sdist/
var/
wheels/
*.egg-info/
.installed.cfg
*.egg
.pytest_cache/
.mypy_cache/
.ruff_cache/
.venv/
venv/
ENV/
env/
.vscode/
.idea/
*.swp
*.swo
*~
.DS_Store
uv.lock
"#
        .to_string()
    }

    /// Generate README.md file
    fn generate_readme(project_name: &str, package_name: &str) -> String {
        format!(
            r#"# {}

A Spikard application.

## Setup

Install dependencies with uv:

```bash
uv sync
```

## Run

Start the development server:

```bash
uv run python -m {}.app
```

## Test

Run the test suite:

```bash
uv run pytest
```

## Type checking

Run mypy to check types:

```bash
uv run mypy {}/
```

## Linting

Run ruff to lint code:

```bash
uv run ruff check {}/
uv run ruff format {}/
```

## Documentation

For more information about Spikard, visit:
- [Spikard Documentation](https://spikard.dev)
- [Spikard GitHub](https://github.com/Goldziher/spikard)
"#,
            project_name, package_name, package_name, package_name, package_name
        )
    }
}

impl ProjectScaffolder for PythonScaffolder {
    fn scaffold(&self, _project_dir: &Path, project_name: &str) -> Result<Vec<ScaffoldedFile>> {
        let package_name = Self::to_package_name(project_name);

        let mut files = Vec::new();

        // pyproject.toml
        files.push(ScaffoldedFile::new(
            PathBuf::from("pyproject.toml"),
            Self::generate_pyproject_toml(project_name, &package_name),
        ));

        // src/{package_name}/__init__.py
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("src/{}/__init__.py", package_name)),
            Self::generate_init_py(),
        ));

        // src/{package_name}/app.py
        files.push(ScaffoldedFile::new(
            PathBuf::from(format!("src/{}/app.py", package_name)),
            Self::generate_app_module(&package_name),
        ));

        // tests/test_app.py
        files.push(ScaffoldedFile::new(
            PathBuf::from("tests/test_app.py"),
            Self::generate_test_app(&package_name),
        ));

        // .gitignore
        files.push(ScaffoldedFile::new(
            PathBuf::from(".gitignore"),
            Self::generate_gitignore(),
        ));

        // README.md
        files.push(ScaffoldedFile::new(
            PathBuf::from("README.md"),
            Self::generate_readme(project_name, &package_name),
        ));

        Ok(files)
    }

    fn next_steps(&self, project_name: &str) -> Vec<String> {
        let package_name = Self::to_package_name(project_name);
        vec![
            format!("cd {}", project_name),
            "uv sync".to_string(),
            format!("uv run python -m {}.app", package_name),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_package_name() {
        assert_eq!(PythonScaffolder::to_package_name("my-app"), "my_app");
        assert_eq!(PythonScaffolder::to_package_name("MyApp"), "myapp");
        assert_eq!(PythonScaffolder::to_package_name("my_app"), "my_app");
        assert_eq!(PythonScaffolder::to_package_name("MY_APP"), "my_app");
    }

    #[test]
    #[allow(clippy::cmp_owned)]
    fn test_scaffold_creates_files() {
        let files = PythonScaffolder.scaffold(Path::new("."), "test_app").unwrap();

        // Check that we have the expected files
        assert!(files.iter().any(|f| f.path == PathBuf::from("pyproject.toml")));
        assert!(files.iter().any(|f| f.path == PathBuf::from("tests/test_app.py")));
        assert!(files.iter().any(|f| f.path == PathBuf::from(".gitignore")));
        assert!(files.iter().any(|f| f.path == PathBuf::from("README.md")));
    }

    #[test]
    fn test_next_steps() {
        let steps = PythonScaffolder.next_steps("my-app");
        assert_eq!(steps.len(), 3);
        assert!(steps[0].contains("my-app"));
        assert_eq!(steps[1], "uv sync");
        assert!(steps[2].contains("my_app.app"));
    }

    #[test]
    fn test_pyproject_contains_spikard() {
        let content = PythonScaffolder::generate_pyproject_toml("test-app", "test_app");
        assert!(content.contains("spikard>=0.6.0"));
        assert!(content.contains("pytest"));
        assert!(content.contains("mypy"));
    }

    #[test]
    fn test_app_module_contains_health_endpoint() {
        let content = PythonScaffolder::generate_app_module("test_app");
        assert!(content.contains("@app.get(\"/health\")"));
        assert!(content.contains("status"));
        assert!(content.contains("ok"));
    }
}
