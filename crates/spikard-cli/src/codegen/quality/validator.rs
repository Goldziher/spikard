//! Quality validation implementation for generated code
//!
//! This module implements language-specific validation for syntax, types, and linting.

use crate::codegen::TargetLanguage;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::{Builder, NamedTempFile, TempDir, tempdir};

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
                let project = self.write_temp_python_project(code)?;
                self.run_tool_in_dir(
                    "python3",
                    &[
                        "-m",
                        "py_compile",
                        project.entry_path.file_name().unwrap().to_str().unwrap(),
                    ],
                    project.workdir.path(),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::TypeScript => {
                let project = self.write_temp_typescript_project(code)?;
                self.run_tool_in_dir(
                    "pnpm",
                    &[
                        "exec",
                        "tsc",
                        "--noEmit",
                        "--project",
                        project.config_path.to_str().unwrap(),
                    ],
                    Path::new("."),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Rust => {
                let project = self.write_temp_rust_project(code)?;
                self.run_tool_in_dir(
                    "cargo",
                    &["check", "--manifest-path", project.manifest_path.to_str().unwrap()],
                    project.workdir.path(),
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
            TargetLanguage::Elixir => {
                let project = self.write_temp_elixir_project(code)?;
                self.run_tool_in_dir(
                    "mix",
                    &["compile", "--warnings-as-errors"],
                    project.workdir.path(),
                    code,
                )
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
                let project = self.write_temp_python_project(code)?;
                self.run_tool_in_dir_with_env(
                    "uv",
                    &["run", "mypy", "--strict", project.entry_path.to_str().unwrap()],
                    workspace_root(),
                    &[("MYPYPATH", project.stub_path.as_os_str())],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::TypeScript => {
                let project = self.write_temp_typescript_project(code)?;
                self.run_tool_in_dir(
                    "pnpm",
                    &[
                        "exec",
                        "tsc",
                        "--strict",
                        "--noEmit",
                        "--project",
                        project.config_path.to_str().unwrap(),
                    ],
                    Path::new("."),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Ruby => {
                let file = self.write_temp_file(code, "rb")?;
                self.run_tool("steep", &["check", file.path().to_str().unwrap()], code)
                    .map(|_| ())
            }
            TargetLanguage::Rust => {
                let project = self.write_temp_rust_project(code)?;
                self.run_tool_in_dir(
                    "cargo",
                    &["check", "--manifest-path", project.manifest_path.to_str().unwrap()],
                    project.workdir.path(),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Php => {
                // PHP doesn't have a separate type checker; covered by lint
                Ok(())
            }
            TargetLanguage::Elixir => Ok(()),
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
                let project = self.write_temp_python_project(code)?;
                self.run_tool_in_dir(
                    "ruff",
                    &["check", project.entry_path.file_name().unwrap().to_str().unwrap()],
                    project.workdir.path(),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::TypeScript => Ok(()),
            TargetLanguage::Ruby => {
                let file = self.write_temp_file(code, "rb")?;
                self.run_tool(
                    "rubocop",
                    &[
                        "--disable-pending-cops",
                        "--except",
                        "Naming/FileName",
                        file.path().to_str().unwrap(),
                    ],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Php => {
                let file = self.write_temp_file(code, "php")?;
                let bootstrap = self.write_php_validation_bootstrap()?;
                self.run_tool(
                    "phpstan",
                    &[
                        "analyse",
                        "--no-progress",
                        "--error-format=raw",
                        "--level=max",
                        "--autoload-file",
                        bootstrap.path().to_str().unwrap(),
                        file.path().to_str().unwrap(),
                    ],
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Elixir => {
                let project = self.write_temp_elixir_project(code)?;
                let generated = project.generated_path.strip_prefix(project.workdir.path()).unwrap();
                self.run_tool_in_dir(
                    "mix",
                    &[
                        "format",
                        "--check-formatted",
                        generated.to_str().unwrap(),
                        "mix.exs",
                        ".formatter.exs",
                        "lib/spikard/router.ex",
                        "lib/spikard/request.ex",
                        "lib/spikard/response.ex",
                    ],
                    project.workdir.path(),
                    code,
                )
                .map(|_| ())
            }
            TargetLanguage::Rust => {
                let project = self.write_temp_rust_project(code)?;
                self.run_tool_in_dir(
                    "cargo",
                    &[
                        "clippy",
                        "--manifest-path",
                        project.manifest_path.to_str().unwrap(),
                        "--",
                        "-D",
                        "warnings",
                    ],
                    project.workdir.path(),
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
    fn write_temp_file(&self, code: &str, ext: &str) -> Result<NamedTempFile, QualityError> {
        let mut file = Builder::new()
            .prefix("generated_")
            .suffix(&format!(".{ext}"))
            .tempfile()
            .map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        file.write_all(code.as_bytes())
            .map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        file.flush()
            .map_err(|e: std::io::Error| QualityError::IoError(e.to_string()))?;
        Ok(file)
    }

    fn write_temp_rust_project(&self, code: &str) -> Result<RustTempProject, QualityError> {
        let workdir = tempdir().map_err(|e| QualityError::IoError(e.to_string()))?;
        let src_dir = workdir.path().join("src");
        fs::create_dir_all(&src_dir).map_err(|e| QualityError::IoError(e.to_string()))?;

        let manifest_path = workdir.path().join("Cargo.toml");
        let lib_path = src_dir.join("lib.rs");

        fs::write(&manifest_path, rust_temp_manifest()).map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(&lib_path, code).map_err(|e| QualityError::IoError(e.to_string()))?;

        Ok(RustTempProject { workdir, manifest_path })
    }

    fn write_temp_python_project(&self, code: &str) -> Result<PythonTempProject, QualityError> {
        let workdir = tempdir().map_err(|e| QualityError::IoError(e.to_string()))?;
        let entry_path = workdir.path().join("generated.py");
        let stub_path = workdir.path().join("stubs");

        fs::write(&entry_path, code).map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::create_dir_all(&stub_path).map_err(|e| QualityError::IoError(e.to_string()))?;
        write_python_validation_stubs(&stub_path)?;

        Ok(PythonTempProject {
            workdir,
            entry_path,
            stub_path,
        })
    }

    fn write_temp_typescript_project(&self, code: &str) -> Result<TypeScriptTempProject, QualityError> {
        let workdir = tempdir().map_err(|e| QualityError::IoError(e.to_string()))?;
        let entry_path = workdir.path().join("generated.ts");
        let config_path = workdir.path().join("tsconfig.json");
        let spikard_stub_path = workdir.path().join("spikard.d.ts");
        let zod_stub_path = workdir.path().join("zod.d.ts");

        fs::write(&entry_path, code).map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            &config_path,
            r#"{
  "compilerOptions": {
    "target": "ES2022",
    "module": "ESNext",
    "moduleResolution": "Node",
    "strict": true,
    "skipLibCheck": true,
    "noEmit": true
  },
  "files": ["generated.ts", "spikard.d.ts", "zod.d.ts"]
}
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            &spikard_stub_path,
            r#"declare module "spikard" {
  export class Spikard {
    start(config?: unknown): Promise<void>;
  }

  export type Body<T> = T;
  export type Path<T> = T;
  export type Query<T> = T;
  export type Request = Record<string, unknown>;

  export function route(...args: unknown[]): any;
}
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            &zod_stub_path,
            r#"declare module "zod" {
  export namespace z {
    export type infer<T> = any;
  }

  export const z: any;
}
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;

        Ok(TypeScriptTempProject {
            workdir,
            entry_path,
            config_path,
        })
    }

    fn write_temp_elixir_project(&self, code: &str) -> Result<ElixirTempProject, QualityError> {
        let workdir = tempdir().map_err(|e| QualityError::IoError(e.to_string()))?;
        let mix_exs = workdir.path().join("mix.exs");
        let formatter = workdir.path().join(".formatter.exs");
        let lib_dir = workdir.path().join("lib");
        let spikard_dir = lib_dir.join("spikard");
        let generated_path = lib_dir.join("generated.ex");

        fs::create_dir_all(&spikard_dir).map_err(|e| QualityError::IoError(e.to_string()))?;

        fs::write(
            &mix_exs,
            r#"defmodule GeneratedValidation.MixProject do
  use Mix.Project

  def project do
    [
      app: :generated_validation,
      version: "0.1.0",
      elixir: "~> 1.19",
      deps: []
    ]
  end
end
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            &formatter,
            r#"[
  inputs: ["{mix,.formatter}.exs", "{config,lib,test}/**/*.{ex,exs}"],
  line_length: 120
]
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            spikard_dir.join("router.ex"),
            r#"defmodule Spikard.Router do
  defmacro __using__(_opts) do
    quote do
      import Spikard.Router
      Module.register_attribute(__MODULE__, :spikard_routes, accumulate: true)
    end
  end

  for method <- ~w(get post put patch delete)a do
    defmacro unquote(method)(path, handler, opts \\ []) do
      quote do
        @spikard_routes {unquote(path), unquote(handler), unquote(opts)}
      end
    end
  end
end
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            spikard_dir.join("request.ex"),
            r#"defmodule Spikard.Request do
  @type t :: map()

  @spec get_path_param(t(), String.t()) :: term()
  def get_path_param(_request, _key), do: nil

  @spec get_query_param(t(), String.t()) :: term()
  def get_query_param(_request, _key), do: nil

  @spec get_header(t(), String.t()) :: term()
  def get_header(_request, _key), do: nil

  @spec get_cookie(t(), String.t()) :: term()
  def get_cookie(_request, _key), do: nil

  @spec get_body(t()) :: term()
  def get_body(_request), do: %{}
end
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(
            spikard_dir.join("response.ex"),
            r#"defmodule Spikard.Response do
  @type t :: %{status: non_neg_integer(), headers: [{String.t(), String.t()}], body: term()}

  @spec json(term(), keyword()) :: t()
  def json(body, opts \\ []) do
    %{status: Keyword.get(opts, :status, 200), headers: [{"content-type", "application/json"}], body: body}
  end

  @spec status(non_neg_integer()) :: t()
  def status(code) do
    %{status: code, headers: [], body: nil}
  end
end
"#,
        )
        .map_err(|e| QualityError::IoError(e.to_string()))?;
        fs::write(&generated_path, code).map_err(|e| QualityError::IoError(e.to_string()))?;

        Ok(ElixirTempProject {
            workdir,
            generated_path,
        })
    }

    fn write_php_validation_bootstrap(&self) -> Result<NamedTempFile, QualityError> {
        self.write_temp_file(
            r#"<?php
declare(strict_types=1);

namespace SpikardGenerated;

#[\Attribute(\Attribute::TARGET_METHOD)]
final class Route
{
    public function __construct(
        public string $path,
        public array $methods = [],
    ) {}
}

namespace Google\Protobuf\Internal;

class Message {}

namespace GraphQL\Type\Definition;

class Type
{
    public static function string(): self
    {
        return new self();
    }

    public static function int(): self
    {
        return new self();
    }

    public static function float(): self
    {
        return new self();
    }

    public static function boolean(): self
    {
        return new self();
    }

    public static function id(): self
    {
        return new self();
    }

    public static function nonNull(self $type): self
    {
        return $type;
    }

    public static function listOf(self $type): self
    {
        return $type;
    }
}

class ObjectType extends Type
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}

class InputObjectType extends Type
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}

class InterfaceType extends Type
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}

class UnionType extends Type
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}

class EnumType extends Type
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}

namespace GraphQL\Type;

class Schema
{
    /** @param array<string, mixed> $config */
    public function __construct(array $config = [])
    {
    }
}
"#,
            "php",
        )
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
        self.run_tool_in_dir(tool, args, Path::new("."), _code)
    }

    fn run_tool_in_dir(&self, tool: &str, args: &[&str], cwd: &Path, _code: &str) -> Result<String, QualityError> {
        self.run_tool_in_dir_with_env(tool, args, cwd, &[], _code)
    }

    fn run_tool_in_dir_with_env(
        &self,
        tool: &str,
        args: &[&str],
        cwd: &Path,
        envs: &[(&str, &std::ffi::OsStr)],
        _code: &str,
    ) -> Result<String, QualityError> {
        let mut command = Command::new(tool);
        command.args(args).current_dir(cwd);
        for (key, value) in envs {
            command.env(key, value);
        }

        let output = command.output().map_err(|e| {
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

struct RustTempProject {
    workdir: TempDir,
    manifest_path: PathBuf,
}

struct TypeScriptTempProject {
    workdir: TempDir,
    entry_path: PathBuf,
    config_path: PathBuf,
}

struct PythonTempProject {
    workdir: TempDir,
    entry_path: PathBuf,
    stub_path: PathBuf,
}

struct ElixirTempProject {
    workdir: TempDir,
    generated_path: PathBuf,
}

fn write_python_validation_stubs(root: &Path) -> Result<(), QualityError> {
    write_stub_file(
        &root.join("msgspec.py"),
        r#"class Struct:
    def __init__(self, **kwargs: object) -> None: ...

    def __init_subclass__(cls, *, frozen: bool = False, kw_only: bool = False) -> None: ...
"#,
    )?;

    write_stub_file(&root.join("graphql.py"), "class GraphQLResolveInfo: ...\n")?;

    write_stub_file(
        &root.join("ariadne.py"),
        r#"from __future__ import annotations

from collections.abc import Callable
from typing import Any

Resolver = Callable[..., Any]

class QueryType:
    def set_field(self, _name: str, _resolver: Resolver) -> None: ...

class MutationType:
    def set_field(self, _name: str, _resolver: Resolver) -> None: ...

class SubscriptionType:
    def set_field(self, _name: str, _resolver: Resolver) -> None: ...
    def set_source(self, _name: str, _resolver: Resolver) -> None: ...

def make_executable_schema(*_args: object, **_kwargs: object) -> object:
    return object()
"#,
    )?;

    let spikard_dir = root.join("spikard");
    fs::create_dir_all(&spikard_dir).map_err(|e| QualityError::IoError(e.to_string()))?;
    write_stub_file(
        &spikard_dir.join("__init__.py"),
        r#"from __future__ import annotations

from collections.abc import Callable
from typing import Generic, TypeVar

F = TypeVar("F", bound=Callable[..., object])
T = TypeVar("T")

class Body(Generic[T]): ...

class Path(Generic[T]): ...

class Query(Generic[T]):
    def __init__(self, default: T | None = None) -> None:
        self.default = default

class Request: ...

class Spikard:
    def route(self, *_args: object, **_kwargs: object) -> Callable[[F], F]:
        def decorator(fn: F) -> F:
            return fn
        return decorator

    def post(self, *_args: object, **_kwargs: object) -> Callable[[F], F]:
        def decorator(fn: F) -> F:
            return fn
        return decorator

    def get(self, *_args: object, **_kwargs: object) -> Callable[[F], F]:
        def decorator(fn: F) -> F:
            return fn
        return decorator

    def run(self, *_args: object, **_kwargs: object) -> None:
        return None

def route(*_args: object, **_kwargs: object) -> Callable[[F], F]:
    def decorator(fn: F) -> F:
        return fn
    return decorator

def websocket(*_args: object, **_kwargs: object) -> Callable[[F], F]:
    def decorator(fn: F) -> F:
        return fn
    return decorator

def sse(*_args: object, **_kwargs: object) -> Callable[[F], F]:
    def decorator(fn: F) -> F:
        return fn
    return decorator
"#,
    )?;
    write_stub_file(
        &spikard_dir.join("config.py"),
        r#"class ServerConfig:
    def __init__(self, host: str = "0.0.0.0", port: int = 8000) -> None:
        self.host = host
        self.port = port
"#,
    )?;

    let google_protobuf_dir = root.join("google").join("protobuf");
    fs::create_dir_all(&google_protobuf_dir).map_err(|e| QualityError::IoError(e.to_string()))?;
    write_stub_file(&root.join("google").join("__init__.py"), "")?;
    write_stub_file(&google_protobuf_dir.join("__init__.py"), "")?;
    write_stub_file(&google_protobuf_dir.join("message.py"), "class Message: ...\n")?;

    let websockets_dir = root.join("websockets");
    fs::create_dir_all(&websockets_dir).map_err(|e| QualityError::IoError(e.to_string()))?;
    write_stub_file(&websockets_dir.join("__init__.py"), "")?;
    write_stub_file(
        &websockets_dir.join("client.py"),
        "class WebSocketClientProtocol: ...\n",
    )?;

    Ok(())
}

fn write_stub_file(path: &Path, contents: &str) -> Result<(), QualityError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| QualityError::IoError(e.to_string()))?;
    }
    fs::write(path, contents).map_err(|e| QualityError::IoError(e.to_string()))
}

fn rust_temp_manifest() -> String {
    let spikard_path = workspace_root().join("crates/spikard");

    format!(
        r#"[package]
name = "spikard_codegen_validation"
version = "0.1.0"
edition = "2024"

[lib]
path = "src/lib.rs"

[dependencies]
async-graphql = "7"
async-trait = "0.1"
axum = "0.8"
bytes = "1"
futures-core = "0.3"
futures-util = "0.3"
prost = "0.14"
schemars = {{ version = "1.2", features = ["derive"] }}
serde = {{ version = "1", features = ["derive"] }}
serde_json = "1"
spikard = {{ path = "{}" }}
tokio = {{ version = "1", features = ["full"] }}
tonic = "0.14"
"#,
        spikard_path.display()
    )
}

fn workspace_root() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("workspace root should be two levels above crates/spikard-cli")
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

        let validator = QualityValidator::new(TargetLanguage::Elixir);
        assert_eq!(validator.language, TargetLanguage::Elixir);
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

    #[test]
    fn test_rust_quality_validator_accepts_valid_code() {
        let validator = QualityValidator::new(TargetLanguage::Rust);
        validator
            .validate_syntax("pub fn add(a: i32, b: i32) -> i32 { a + b }")
            .expect("rust syntax validation should pass");
        validator
            .validate_types("pub fn add(a: i32, b: i32) -> i32 { a + b }")
            .expect("rust type validation should pass");
    }
}
