//! Orchestration engine for project initialization.
//!
//! This module provides the `InitEngine` which manages the end-to-end
//! initialization workflow: request validation, scaffolder selection,
//! file creation, and user guidance generation.

use crate::codegen::TargetLanguage;
use anyhow::{Context, Result, bail};
use std::path::PathBuf;
use thiserror::Error;

use super::scaffolder::ScaffoldedFile;

/// Errors that can occur during project initialization.
///
/// # Variants
///
/// - `InvalidProjectName`: The project name does not conform to naming rules
/// - `DirectoryAlreadyExists`: The target directory already exists
/// - `SchemaPathNotFound`: A schema path was specified but does not exist
/// - `LanguageNotSupported`: The target language is not yet supported for initialization
/// - `ScaffoldingFailed`: An error occurred during file generation or writing
#[derive(Debug, Error)]
pub enum InitError {
    /// Project name does not conform to language-specific naming conventions
    #[error("Invalid project name '{name}': {reason}")]
    InvalidProjectName { name: String, reason: String },

    /// The target directory already exists and init should not overwrite it
    #[error("Directory '{path}' already exists; initialize in a new directory")]
    DirectoryAlreadyExists { path: PathBuf },

    /// The provided schema path does not exist or cannot be read
    #[error("Schema file not found: {path}")]
    SchemaPathNotFound { path: PathBuf },

    /// Project initialization is not yet supported for this language
    #[error("Project initialization is not yet supported for {language:?}")]
    LanguageNotSupported { language: TargetLanguage },

    /// An error occurred during scaffolding or file creation
    #[error("Scaffolding failed: {reason}")]
    ScaffoldingFailed { reason: String },
}

/// Request to initialize a new Spikard project.
///
/// # Fields
///
/// - `project_name`: The name of the project (used for packages, modules, etc.)
/// - `language`: Target implementation language
/// - `project_dir`: Root directory where the project will be created
/// - `schema_path`: Optional path to an existing API schema to include in setup
///
/// # Example
///
/// ```ignore
/// use spikard_cli::init::InitRequest;
/// use spikard_cli::codegen::TargetLanguage;
/// use std::path::PathBuf;
///
/// let request = InitRequest {
///     project_name: "my_api".to_string(),
///     language: TargetLanguage::Python,
///     project_dir: PathBuf::from("."),
///     schema_path: Some(PathBuf::from("openapi.json")),
/// };
/// ```
#[derive(Debug, Clone)]
pub struct InitRequest {
    /// The name of the project to be created
    pub project_name: String,
    /// Target programming language for the project
    pub language: TargetLanguage,
    /// Directory where the project will be initialized
    pub project_dir: PathBuf,
    /// Optional path to an existing schema to include in setup
    pub schema_path: Option<PathBuf>,
}

/// Response from a successful project initialization.
///
/// # Fields
///
/// - `files_created`: Paths to all files that were created
/// - `next_steps`: User-friendly instructions for what to do next
///
/// # Example
///
/// ```ignore
/// let response = InitEngine::execute(request)?;
/// println!("Created {} files", response.files_created.len());
/// for step in &response.next_steps {
///     println!("  → {}", step);
/// }
/// ```
#[derive(Debug)]
pub struct InitResponse {
    /// Absolute paths to all files that were created
    pub files_created: Vec<PathBuf>,
    /// Next steps to guide the user (e.g., "cd my_api", "pip install", etc.)
    pub next_steps: Vec<String>,
}

/// Orchestrates the project initialization workflow.
///
/// # Overview
///
/// `InitEngine` is the main entry point for the `spikard init` command.
/// It handles:
///
/// 1. **Validation**: Ensures project name and paths are valid
/// 2. **Scaffolder Selection**: Routes to the correct language scaffolder
/// 3. **File Creation**: Writes scaffolded files to disk
/// 4. **Guidance**: Returns user-friendly next steps
///
/// # Validation Rules
///
/// - **Project Name**: Must be a valid identifier in the target language
/// - **Directory**: The project directory must not already exist
/// - **Schema Path**: If provided, must exist and be readable
///
/// # Architecture
///
/// The engine does not generate code itself; instead, it delegates to
/// language-specific `ProjectScaffolder` implementations. This keeps
/// the engine lightweight and allows independent evolution of language support.
///
/// # Example
///
/// ```ignore
/// use spikard_cli::init::{InitEngine, InitRequest};
/// use spikard_cli::codegen::TargetLanguage;
/// use std::path::PathBuf;
///
/// let request = InitRequest {
///     project_name: "my_api".to_string(),
///     language: TargetLanguage::Python,
///     project_dir: PathBuf::from("."),
///     schema_path: None,
/// };
///
/// match InitEngine::execute(request) {
///     Ok(response) => {
///         println!("Successfully created {} files", response.files_created.len());
///         for step in response.next_steps {
///             println!("  → {}", step);
///         }
///     }
///     Err(e) => eprintln!("Initialization failed: {}", e),
/// }
/// ```
pub struct InitEngine;

impl InitEngine {
    /// Execute the project initialization workflow.
    ///
    /// This method is the primary entry point for initializing a new Spikard project.
    /// It validates the request, selects the appropriate scaffolder, generates files,
    /// writes them to disk, and returns guidance for next steps.
    ///
    /// # Arguments
    ///
    /// - `request`: An `InitRequest` specifying project name, language, and location
    ///
    /// # Returns
    ///
    /// On success, returns an `InitResponse` with created file paths and next steps.
    /// On failure, returns an error detailing what went wrong.
    ///
    /// # Errors
    ///
    /// - `InvalidProjectName`: If the project name is not valid for the target language
    /// - `DirectoryAlreadyExists`: If the project directory already exists
    /// - `SchemaPathNotFound`: If a schema path was provided but doesn't exist
    /// - `LanguageNotSupported`: If the target language is not yet supported
    /// - `ScaffoldingFailed`: If file creation or writing fails
    ///
    /// # Side Effects
    ///
    /// This method creates the project directory and all scaffolded files on disk.
    /// If any error occurs after directory creation, the directory is left as-is
    /// for the user to clean up (to avoid accidental data loss).
    ///
    /// # Example
    ///
    /// ```ignore
    /// let request = InitRequest {
    ///     project_name: "my_api".to_string(),
    ///     language: TargetLanguage::Python,
    ///     project_dir: PathBuf::from("."),
    ///     schema_path: None,
    /// };
    ///
    /// let response = InitEngine::execute(request)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn execute(request: InitRequest) -> Result<InitResponse> {
        // Validate request inputs
        Self::validate_request(&request).context("Project initialization request validation failed")?;

        // Get the appropriate scaffolder for the language
        let scaffolder = Self::get_scaffolder(request.language)?;

        // Generate files via scaffolder
        let files = scaffolder
            .scaffold(
                &request.project_dir,
                &request.project_name,
            )
            .context("Failed to scaffold project files")?;

        // Create project directory
        std::fs::create_dir_all(&request.project_dir)
            .context(format!("Failed to create project directory: {}", request.project_dir.display()))?;

        // Write files to disk and collect paths
        let mut files_created = Vec::new();
        for file in files {
            let full_path = request.project_dir.join(&file.path);

            // Create parent directories if needed
            if let Some(parent) = full_path.parent() {
                std::fs::create_dir_all(parent)
                    .context(format!("Failed to create directory: {}", parent.display()))?;
            }

            // Write file content
            std::fs::write(&full_path, &file.content)
                .context(format!("Failed to write file: {}", full_path.display()))?;

            files_created.push(full_path);
        }

        // Get next steps from scaffolder
        let next_steps = scaffolder.next_steps(&request.project_name);

        Ok(InitResponse {
            files_created,
            next_steps,
        })
    }

    /// Get the appropriate scaffolder for a language
    fn get_scaffolder(language: TargetLanguage) -> Result<Box<dyn super::scaffolder::ProjectScaffolder>> {
        match language {
            TargetLanguage::Python => Ok(Box::new(super::python::PythonScaffolder)),
            TargetLanguage::TypeScript => Ok(Box::new(super::typescript::TypeScriptScaffolder)),
            TargetLanguage::Rust => Ok(Box::new(super::rust_lang::RustScaffolder)),
            TargetLanguage::Ruby => Err(anyhow::anyhow!(InitError::LanguageNotSupported {
                language: TargetLanguage::Ruby,
            })),
            TargetLanguage::Php => Ok(Box::new(super::php::PhpScaffolder)),
        }
    }

    /// Validate the initialization request.
    ///
    /// This method checks:
    ///
    /// - Project name is valid for the target language
    /// - Project directory doesn't already exist
    /// - Schema path (if provided) exists and is accessible
    ///
    /// # Arguments
    ///
    /// - `request`: The `InitRequest` to validate
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if all validations pass, otherwise returns an appropriate error.
    ///
    /// # Errors
    ///
    /// Returns validation errors with context about what failed.
    fn validate_request(request: &InitRequest) -> Result<()> {
        // Validate project name format
        Self::validate_project_name(&request.project_name, request.language)
            .context("Project name validation failed")?;

        // Validate project directory doesn't already exist
        if request.project_dir.exists() {
            bail!(InitError::DirectoryAlreadyExists {
                path: request.project_dir.clone(),
            });
        }

        // Validate schema path if provided
        if let Some(schema_path) = &request.schema_path {
            if !schema_path.exists() {
                bail!(InitError::SchemaPathNotFound {
                    path: schema_path.clone(),
                });
            }
        }

        Ok(())
    }

    /// Validate that a project name is appropriate for the target language.
    ///
    /// Naming rules vary by language:
    ///
    /// - **Python**: Lowercase, alphanumeric + underscore, no leading digit
    /// - **TypeScript**: Must be valid npm package name (lowercase, hyphen OK)
    /// - **Ruby**: Snake_case, no leading digit
    /// - **Rust**: Snake_case, alphanumeric + underscore, no leading digit
    /// - **PHP**: Alphanumeric + underscore, no leading digit
    ///
    /// # Arguments
    ///
    /// - `project_name`: The name to validate
    /// - `language`: The target language whose rules apply
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the name is valid, otherwise returns a descriptive error.
    pub fn validate_project_name(project_name: &str, language: TargetLanguage) -> Result<()> {
        if project_name.is_empty() {
            bail!(InitError::InvalidProjectName {
                name: project_name.to_string(),
                reason: "Project name cannot be empty".to_string(),
            });
        }

        match language {
            TargetLanguage::Python => {
                // Python: lowercase letters, digits, underscores; no leading digit
                if !project_name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Python project names must contain only lowercase letters, digits, and underscores"
                            .to_string(),
                    });
                }
                if project_name.starts_with(|c: char| c.is_ascii_digit()) {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Python project names cannot start with a digit".to_string(),
                    });
                }
            }
            TargetLanguage::TypeScript => {
                // npm package name rules (simplified): lowercase, alphanumeric, hyphens
                if !project_name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
                {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "TypeScript project names must contain only lowercase letters, digits, and hyphens"
                            .to_string(),
                    });
                }
            }
            TargetLanguage::Rust => {
                // Rust: snake_case, alphanumeric + underscores, no leading digit
                if !project_name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Rust project names must contain only lowercase letters, digits, and underscores"
                            .to_string(),
                    });
                }
                if project_name.starts_with(|c: char| c.is_ascii_digit()) {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Rust project names cannot start with a digit".to_string(),
                    });
                }
            }
            TargetLanguage::Ruby => {
                // Ruby: snake_case, alphanumeric + underscores, no leading digit
                if !project_name
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
                {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Ruby project names must contain only lowercase letters, digits, and underscores"
                            .to_string(),
                    });
                }
                if project_name.starts_with(|c: char| c.is_ascii_digit()) {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "Ruby project names cannot start with a digit".to_string(),
                    });
                }
            }
            TargetLanguage::Php => {
                // PHP: alphanumeric + underscores, no leading digit
                if !project_name
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '_')
                {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "PHP project names must contain only alphanumeric characters and underscores"
                            .to_string(),
                    });
                }
                if project_name.starts_with(|c: char| c.is_ascii_digit()) {
                    bail!(InitError::InvalidProjectName {
                        name: project_name.to_string(),
                        reason: "PHP project names cannot start with a digit".to_string(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Create project directory and write all scaffolded files to disk.
    ///
    /// This is an internal helper method that will be used once language-specific
    /// scaffolders are implemented.
    ///
    /// # Arguments
    ///
    /// - `project_dir`: The root directory to create
    /// - `files`: The scaffolded files to write
    ///
    /// # Returns
    ///
    /// Returns a vector of absolute paths to the created files on success.
    ///
    /// # Errors
    ///
    /// Returns an error if directory creation or file writing fails.
    #[allow(dead_code)]
    fn write_files(project_dir: &std::path::Path, files: Vec<ScaffoldedFile>) -> Result<Vec<PathBuf>> {
        std::fs::create_dir_all(project_dir)
            .context("Failed to create project directory")?;

        let mut created_files = Vec::new();

        for file in files {
            let full_path = project_dir.join(&file.path);

            // Create parent directories if needed
            if let Some(parent) = full_path.parent() {
                if !parent.exists() {
                    std::fs::create_dir_all(parent)
                        .context(format!("Failed to create directory: {}", parent.display()))?;
                }
            }

            // Write file
            std::fs::write(&full_path, &file.content)
                .context(format!("Failed to write file: {}", full_path.display()))?;

            created_files.push(full_path);
        }

        Ok(created_files)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_python_project_name_valid() {
        assert!(InitEngine::validate_project_name("my_api", TargetLanguage::Python).is_ok());
        assert!(InitEngine::validate_project_name("api_v2", TargetLanguage::Python).is_ok());
        assert!(InitEngine::validate_project_name("a", TargetLanguage::Python).is_ok());
    }

    #[test]
    fn test_validate_python_project_name_invalid() {
        // Uppercase not allowed
        assert!(InitEngine::validate_project_name("MyApi", TargetLanguage::Python).is_err());
        // Cannot start with digit
        assert!(InitEngine::validate_project_name("2api", TargetLanguage::Python).is_err());
        // Hyphens not allowed in Python
        assert!(InitEngine::validate_project_name("my-api", TargetLanguage::Python).is_err());
        // Empty name
        assert!(InitEngine::validate_project_name("", TargetLanguage::Python).is_err());
    }

    #[test]
    fn test_validate_typescript_project_name_valid() {
        assert!(InitEngine::validate_project_name("my-api", TargetLanguage::TypeScript).is_ok());
        assert!(InitEngine::validate_project_name("api", TargetLanguage::TypeScript).is_ok());
    }

    #[test]
    fn test_validate_typescript_project_name_invalid() {
        // Uppercase not allowed
        assert!(InitEngine::validate_project_name("MyApi", TargetLanguage::TypeScript).is_err());
        // Underscores not allowed in npm package names
        assert!(InitEngine::validate_project_name("my_api", TargetLanguage::TypeScript).is_err());
    }

    #[test]
    fn test_validate_rust_project_name_valid() {
        assert!(InitEngine::validate_project_name("my_api", TargetLanguage::Rust).is_ok());
        assert!(InitEngine::validate_project_name("api", TargetLanguage::Rust).is_ok());
    }

    #[test]
    fn test_validate_rust_project_name_invalid() {
        // Uppercase not allowed
        assert!(InitEngine::validate_project_name("MyApi", TargetLanguage::Rust).is_err());
        // Cannot start with digit
        assert!(InitEngine::validate_project_name("2api", TargetLanguage::Rust).is_err());
        // Hyphens not allowed in Rust
        assert!(InitEngine::validate_project_name("my-api", TargetLanguage::Rust).is_err());
    }

    #[test]
    fn test_validate_ruby_project_name_valid() {
        assert!(InitEngine::validate_project_name("my_api", TargetLanguage::Ruby).is_ok());
    }

    #[test]
    fn test_validate_ruby_project_name_invalid() {
        assert!(InitEngine::validate_project_name("2api", TargetLanguage::Ruby).is_err());
    }

    #[test]
    fn test_validate_php_project_name_valid() {
        assert!(InitEngine::validate_project_name("my_api", TargetLanguage::Php).is_ok());
        assert!(InitEngine::validate_project_name("MyApi", TargetLanguage::Php).is_ok());
    }

    #[test]
    fn test_validate_php_project_name_invalid() {
        assert!(InitEngine::validate_project_name("2api", TargetLanguage::Php).is_err());
    }
}
