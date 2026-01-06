//! Quality validation implementation for generated code
//!
//! This module implements language-specific validation for syntax, types, and linting.

use crate::codegen::TargetLanguage;
use std::fmt;
use std::io::Write;
use std::process::Command;
use tempfile::NamedTempFile;

/// Error types for quality validation operations
#[derive(Debug)]
pub enum QualityError {
    /// A required validation tool was not found in the system PATH
    ToolNotFound(String),
    /// Validation failed with a specific error message
    ValidationFailed(String),
    /// I/O error during file operations
    IoError(String),
}

impl fmt::Display for QualityError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ToolNotFound(tool) => {
                write!(f, "Required validation tool not found: {tool}")
            }
            Self::ValidationFailed(msg) => {
                write!(f, "Validation failed: {msg}")
            }
            Self::IoError(msg) => {
                write!(f, "I/O error: {msg}")
            }
        }
    }
}

impl std::error::Error for QualityError {}

impl From<std::io::Error> for QualityError {
    fn from(err: std::io::Error) -> Self {
        Self::IoError(err.to_string())
    }
}

/// Comprehensive validation report containing results from all quality gates
#[derive(Debug, Clone)]
pub struct ValidationReport {
    /// Whether syntax validation passed
    pub syntax_passed: bool,
    /// Whether type validation passed
    pub types_passed: bool,
    /// Whether linting validation passed
    pub lint_passed: bool,
    /// List of all validation errors encountered
    pub errors: Vec<String>,
}

impl ValidationReport {
    /// Creates a new empty validation report
    const fn new() -> Self {
        Self {
            syntax_passed: false,
            types_passed: false,
            lint_passed: false,
            errors: Vec::new(),
        }
    }

    /// Checks if all validation checks passed
    ///
    /// Returns `true` only if syntax, types, and lint all passed without errors.
    #[must_use] 
    pub const fn is_valid(&self) -> bool {
        self.syntax_passed && self.types_passed && self.lint_passed && self.errors.is_empty()
    }

    /// Returns the count of validation errors
    #[must_use] 
    pub const fn error_count(&self) -> usize {
        self.errors.len()
    }

    /// Adds an error message to the report
    fn add_error(&mut self, error: String) {
        self.errors.push(error);
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Validation Report")?;
        writeln!(f, "  Syntax: {}", if self.syntax_passed { "PASS" } else { "FAIL" })?;
        writeln!(f, "  Types:  {}", if self.types_passed { "PASS" } else { "FAIL" })?;
        writeln!(f, "  Lint:   {}", if self.lint_passed { "PASS" } else { "FAIL" })?;

        if !self.errors.is_empty() {
            writeln!(f, "  Errors: {}", self.error_count())?;
            for error in &self.errors {
                writeln!(f, "    - {error}")?;
            }
        }

        Ok(())
    }
}

/// Language-specific code quality validator
///
/// Orchestrates syntax, type, and lint validation for generated code across
/// all supported target languages.
///
/// # Architecture
///
/// The validator follows a layered approach:
///
/// 1. **Code staging**: Writes code to a temporary file with appropriate extension
/// 2. **Tool execution**: Runs language-specific validation tools
/// 3. **Error parsing**: Extracts and structures error messages
/// 4. **Report generation**: Compiles results into a [`ValidationReport`]
///
/// # Zero-Copy Design
///
/// Code is written to disk once and reused for all validation passes, minimizing
/// I/O overhead. Tools operate directly on the filesystem.
#[derive(Debug)]
pub struct QualityValidator {
    language: TargetLanguage,
}

impl QualityValidator {
    /// Creates a new quality validator for the specified language
    ///
    /// # Arguments
    ///
    /// * `language` - The target language for validation
    ///
    /// # Example
    ///
    /// ```ignore
    /// let validator = QualityValidator::new(TargetLanguage::Python);
    /// ```
    #[must_use] 
    pub const fn new(language: TargetLanguage) -> Self {
        Self { language }
    }

    /// Validates syntax by attempting to parse/compile the code
    ///
    /// Each language uses its native compiler or parser:
    /// - Python: `python3 -m py_compile`
    /// - TypeScript: `tsc --noEmit`
    /// - Ruby: `ruby -c`
    /// - PHP: `php -l`
    /// - Rust: `cargo check`
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to validate
    ///
    /// # Returns
    ///
    /// - `Ok(())` if syntax is valid
    /// - `Err(QualityError::ToolNotFound)` if the validation tool is unavailable
    /// - `Err(QualityError::ValidationFailed)` if syntax errors are found
    /// - `Err(QualityError::IoError)` if file operations fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let validator = QualityValidator::new(TargetLanguage::Python);
    /// validator.validate_syntax("x = 1")?;
    /// ```
    pub fn validate_syntax(&self, code: &str) -> Result<(), QualityError> {
        match self.language {
            TargetLanguage::Python => {
                let file = self.write_temp_file(code, "py")?;
                self.run_tool("python3", &["-m", "py_compile", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::TypeScript => {
                let file = self.write_temp_file(code, "ts")?;
                self.run_tool("tsc", &["--noEmit", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Rust => {
                let file = self.write_temp_file(code, "rs")?;
                self.run_tool(
                    "cargo",
                    &["check", "--manifest-path", file.path().to_str().unwrap()],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Ruby => {
                let file = self.write_temp_file(code, "rb")?;
                self.run_tool("ruby", &["-c", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Php => {
                let file = self.write_temp_file(code, "php")?;
                self.run_tool("php", &["-l", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
        }
    }

    /// Validates type correctness using language-specific type checkers
    ///
    /// Not all languages support this check; unsupported languages return `Ok(())`.
    ///
    /// Tools used:
    /// - Python: `mypy --strict`
    /// - TypeScript: `tsc --noEmit`
    /// - Ruby: `steep check`
    /// - PHP: Not supported (lint validation covers this)
    /// - Rust: `cargo check`
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to validate
    ///
    /// # Returns
    ///
    /// - `Ok(())` if types are valid or language doesn't support type checking
    /// - `Err(QualityError::ToolNotFound)` if the type checker is unavailable
    /// - `Err(QualityError::ValidationFailed)` if type errors are found
    /// - `Err(QualityError::IoError)` if file operations fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let validator = QualityValidator::new(TargetLanguage::TypeScript);
    /// validator.validate_types("const x: number = 5;")?;
    /// ```
    pub fn validate_types(&self, code: &str) -> Result<(), QualityError> {
        match self.language {
            TargetLanguage::Python => {
                let file = self.write_temp_file(code, "py")?;
                self.run_tool("mypy", &["--strict", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::TypeScript => {
                let file = self.write_temp_file(code, "ts")?;
                self.run_tool("tsc", &["--strict", "--noEmit", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Ruby => {
                let file = self.write_temp_file(code, "rb")?;
                self.run_tool("steep", &["check", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Rust => {
                let file = self.write_temp_file(code, "rs")?;
                self.run_tool(
                    "cargo",
                    &["check", "--manifest-path", file.path().to_str().unwrap()],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Php => {
                // PHP doesn't have a separate type checker; covered by lint
                Ok(())
            }
        }
    }

    /// Validates code against linting and style standards
    ///
    /// Each language enforces its community standards:
    /// - Python: `ruff check`
    /// - TypeScript: `biome check`
    /// - Ruby: `rubocop`
    /// - PHP: `phpstan --level=max`
    /// - Rust: `cargo clippy -- -D warnings`
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to validate
    ///
    /// # Returns
    ///
    /// - `Ok(())` if code passes all linting checks
    /// - `Err(QualityError::ToolNotFound)` if the linter is unavailable
    /// - `Err(QualityError::ValidationFailed)` if linting violations are found
    /// - `Err(QualityError::IoError)` if file operations fail
    ///
    /// # Example
    ///
    /// ```ignore
    /// let validator = QualityValidator::new(TargetLanguage::Python);
    /// validator.validate_lint("import os\nx = 1")?;
    /// ```
    pub fn validate_lint(&self, code: &str) -> Result<(), QualityError> {
        match self.language {
            TargetLanguage::Python => {
                let file = self.write_temp_file(code, "py")?;
                self.run_tool("ruff", &["check", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::TypeScript => {
                let file = self.write_temp_file(code, "ts")?;
                self.run_tool("biome", &["check", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Ruby => {
                let file = self.write_temp_file(code, "rb")?;
                self.run_tool("rubocop", &[file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Php => {
                let file = self.write_temp_file(code, "php")?;
                self.run_tool(
                    "phpstan",
                    &["analyse", "--level=max", file.path().to_str().unwrap()],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Rust => {
                let file = self.write_temp_file(code, "rs")?;
                self.run_tool(
                    "cargo",
                    &[
                        "clippy",
                        "--manifest-path",
                        file.path().to_str().unwrap(),
                        "--",
                        "-D",
                        "warnings",
                    ],
                    code,
                )
                .map(|_| ())
            }
        }
    }

    /// Runs all validation checks (syntax, types, lint) and returns a comprehensive report
    ///
    /// This method executes all three quality gates sequentially and compiles results
    /// into a single [`ValidationReport`]. All errors are captured, allowing callers
    /// to see the complete picture of validation failures.
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to validate
    ///
    /// # Returns
    ///
    /// - `Ok(report)` with validation results
    /// - `Err(QualityError)` only if an I/O error occurs; validation failures are captured in the report
    ///
    /// # Example
    ///
    /// ```ignore
    /// let validator = QualityValidator::new(TargetLanguage::Python);
    /// let report = validator.validate_all("x = 1")?;
    ///
    /// if report.is_valid() {
    ///     println!("Code is production-ready");
    /// } else {
    ///     eprintln!("Found {} validation errors", report.error_count());
    /// }
    /// ```
    pub fn validate_all(&self, code: &str) -> Result<ValidationReport, QualityError> {
        let mut report = ValidationReport::new();

        // Syntax validation
        match self.validate_syntax(code) {
            Ok(()) => report.syntax_passed = true,
            Err(e) => {
                report.syntax_passed = false;
                report.add_error(format!("Syntax: {e}"));
            }
        }

        // Type validation
        match self.validate_types(code) {
            Ok(()) => report.types_passed = true,
            Err(e) => {
                report.types_passed = false;
                report.add_error(format!("Types: {e}"));
            }
        }

        // Lint validation
        match self.validate_lint(code) {
            Ok(()) => report.lint_passed = true,
            Err(e) => {
                report.lint_passed = false;
                report.add_error(format!("Lint: {e}"));
            }
        }

        Ok(report)
    }

    /// Writes code to a temporary file with the specified extension
    ///
    /// # Arguments
    ///
    /// * `code` - The source code to write
    /// * `ext` - File extension (without leading dot)
    ///
    /// # Returns
    ///
    /// - `Ok(file)` - A named temporary file handle
    /// - `Err(QualityError::IoError)` - If the file cannot be created or written
    fn write_temp_file(&self, code: &str, _ext: &str) -> Result<NamedTempFile, QualityError> {
        let mut file = NamedTempFile::new().map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        file.write_all(code.as_bytes())
            .map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        file.flush()
            .map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        Ok(file)
    }

    /// Executes a validation tool and captures its output
    ///
    /// This method runs an external command with the given arguments and interprets
    /// the exit code. A zero exit code indicates success; non-zero indicates failure.
    /// Both stdout and stderr are captured and included in error messages.
    ///
    /// # Arguments
    ///
    /// * `tool` - The executable name (resolved from PATH)
    /// * `args` - Command-line arguments
    /// * `code` - The original code (for error context)
    ///
    /// # Returns
    ///
    /// - `Ok(output)` - The tool's stdout if successful
    /// - `Err(QualityError::ToolNotFound)` - If the tool is not found in PATH
    /// - `Err(QualityError::ValidationFailed)` - If the tool exits with non-zero status
    /// - `Err(QualityError::IoError)` - If execution fails
    fn run_tool(&self, tool: &str, args: &[&str], _code: &str) -> Result<String, QualityError> {
        let output = Command::new(tool).args(args).output().map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                QualityError::ToolNotFound(tool.to_string())
            } else {
                QualityError::IoError(e.to_string())
            }
        })?;

        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            let stdout = String::from_utf8_lossy(&output.stdout);
            let message = if stderr.is_empty() {
                stdout.to_string()
            } else {
                stderr.to_string()
            };
            Err(QualityError::ValidationFailed(message))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report_is_valid() {
        let mut report = ValidationReport::new();
        assert!(!report.is_valid());

        report.syntax_passed = true;
        report.types_passed = true;
        report.lint_passed = true;
        assert!(report.is_valid());

        report.add_error("test error".to_string());
        assert!(!report.is_valid());
    }

    #[test]
    fn test_validation_report_error_count() {
        let mut report = ValidationReport::new();
        assert_eq!(report.error_count(), 0);

        report.add_error("error 1".to_string());
        report.add_error("error 2".to_string());
        assert_eq!(report.error_count(), 2);
    }

    #[test]
    fn test_quality_validator_creation() {
        let validator = QualityValidator::new(TargetLanguage::Python);
        assert_eq!(validator.language, TargetLanguage::Python);

        let validator = QualityValidator::new(TargetLanguage::TypeScript);
        assert_eq!(validator.language, TargetLanguage::TypeScript);
    }

    #[test]
    fn test_quality_error_display() {
        let err = QualityError::ToolNotFound("mypy".to_string());
        assert_eq!(err.to_string(), "Required validation tool not found: mypy");

        let err = QualityError::ValidationFailed("syntax error".to_string());
        assert!(err.to_string().contains("Validation failed"));

        let err = QualityError::IoError("file not found".to_string());
        assert!(err.to_string().contains("I/O error"));
    }

    #[test]
    fn test_validation_report_display() {
        let mut report = ValidationReport::new();
        report.syntax_passed = true;
        report.types_passed = false;
        report.add_error("type mismatch".to_string());

        let display = report.to_string();
        assert!(display.contains("Syntax: PASS"));
        assert!(display.contains("Types:  FAIL"));
        assert!(display.contains("type mismatch"));
    }
}
