//! Quality validation framework for generated code
//!
//! This module provides language-specific code quality validation, including syntax checking,
//! type checking, and linting for all supported target languages (Python, TypeScript, Rust,
//! Ruby, PHP).
//!
//! # Overview
//!
//! The [`QualityValidator`] struct orchestrates validation across multiple quality gates:
//!
//! - **Syntax validation**: Ensures code parses correctly
//! - **Type validation**: Verifies type correctness where applicable
//! - **Lint validation**: Enforces coding standards and best practices
//!
//! # Language Support
//!
//! Each language uses appropriate native tools:
//!
//! - **Python**: `python3 -m py_compile`, `mypy --strict`, `ruff check`
//! - **TypeScript**: `tsc --noEmit`, `biome check`
//! - **Ruby**: `ruby -c`, `steep check`
//! - **PHP**: `php -l`, `phpstan --level=max`
//! - **Rust**: `cargo check`, `cargo clippy -- -D warnings`
//!
//! # Example
//!
//! ```ignore
//! use spikard_cli::codegen::{TargetLanguage, quality::QualityValidator};
//!
//! let validator = QualityValidator::new(TargetLanguage::Python);
//! let report = validator.validate_all("print('hello')")?;
//!
//! if !report.is_valid() {
//!     eprintln!("Validation failed: {:?}", report.errors);
//! }
//! ```

pub mod validator;

pub use validator::{QualityError, QualityValidator, ValidationReport};
