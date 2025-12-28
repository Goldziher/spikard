//! Formatters for language-specific code generation output
//!
//! This module provides a trait-based system for formatting generated code in a way
//! that respects language-specific conventions and tooling requirements. Each language
//! binding (Python, TypeScript, Ruby, PHP) implements the `Formatter` trait to handle
//! header formatting, import organization, docstring styling, and section merging.
//!
//! # Design
//!
//! The `Formatter` trait abstracts away language-specific formatting rules while ensuring
//! consistent code generation across the entire spikard toolkit. Implementers handle:
//!
//! - **Headers**: Shebangs, auto-generation notices, module docstrings
//! - **Imports**: Dependency declarations, type imports, organization
//! - **Docstrings**: Language-specific documentation formatting (NumPy, JSDoc, etc.)
//! - **Merging**: Combining sections with proper spacing and deduplication
//!
//! # Example
//!
//! ```no_run
//! use spikard_cli::codegen::formatters::{Formatter, Import, HeaderMetadata, PythonFormatter, PhpFormatter, RustFormatter};
//!
//! let formatter = PythonFormatter::new();
//! let metadata = HeaderMetadata {
//!     auto_generated: true,
//!     schema_file: Some("schema.graphql".to_string()),
//!     generator_version: Some("0.6.2".to_string()),
//! };
//!
//! let header = formatter.format_header(&metadata);
//! println!("{}", header);
//! ```

mod php;
mod python;
mod ruby;
mod rust_lang;
mod typescript;

pub use php::PhpFormatter;
pub use python::PythonFormatter;
pub use ruby::RubyFormatter;
pub use rust_lang::RustFormatter;
pub use typescript::TypeScriptFormatter;

/// Metadata about a generated file used when formatting headers
#[derive(Debug, Clone)]
pub struct HeaderMetadata {
    /// Whether this file is auto-generated and should not be edited manually
    pub auto_generated: bool,
    /// Optional path to the source schema file (GraphQL, OpenAPI, etc.)
    pub schema_file: Option<String>,
    /// Optional version of the generator tool that created this file
    pub generator_version: Option<String>,
}

/// Represents an import/require/use statement in any language
#[derive(Debug, Clone)]
pub struct Import {
    /// Module or package name (e.g., "typing", "graphql", "@apollo/client")
    pub module: String,
    /// Specific items to import (e.g., ["List", "Dict"] for Python)
    /// If empty, import the entire module
    pub items: Vec<String>,
    /// For TypeScript: whether this is a type-only import (import type { ... } from ...)
    pub is_type_only: bool,
}

impl Import {
    /// Create a new import with no specific items (full module import)
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_cli::codegen::formatters::Import;
    /// let import = Import::new("typing");
    /// assert_eq!(import.module, "typing");
    /// assert!(import.items.is_empty());
    /// ```
    pub fn new(module: impl Into<String>) -> Self {
        Self {
            module: module.into(),
            items: Vec::new(),
            is_type_only: false,
        }
    }

    /// Create an import with specific items
    ///
    /// # Example
    ///
    /// ```
    /// use spikard_cli::codegen::formatters::Import;
    /// let import = Import::with_items("typing", vec!["List", "Dict"]);
    /// assert_eq!(import.items.len(), 2);
    /// ```
    pub fn with_items(module: impl Into<String>, items: Vec<&str>) -> Self {
        Self {
            module: module.into(),
            items: items.iter().map(|s| s.to_string()).collect(),
            is_type_only: false,
        }
    }

    /// Mark this import as type-only (TypeScript only)
    pub fn with_type_only(mut self, type_only: bool) -> Self {
        self.is_type_only = type_only;
        self
    }
}

/// Represents a section of generated code to be merged
#[derive(Debug, Clone)]
pub enum Section {
    /// File header (shebang, auto-gen notice, module docstring)
    Header(String),
    /// Import statements and require declarations
    Imports(String),
    /// Main code body
    Body(String),
}

/// Core formatter trait for language-specific code generation output
///
/// Implement this trait to support a new target language. Each method should produce
/// formatted code that adheres to the language's conventions and integrates with its
/// standard tooling (linters, formatters, type checkers).
///
/// # Safety
///
/// Implementations must:
/// - Never panic (return errors via `Result` types where applicable)
/// - Escape special characters appropriately for the language
/// - Handle both empty and non-empty inputs gracefully
/// - Preserve code semantics when reformatting
pub trait Formatter: Send + Sync {
    /// Format a file header with metadata about auto-generation
    ///
    /// This method should produce language-specific output including:
    /// - Shebang line (if applicable)
    /// - Tool directives (ruff: noqa, eslint-disable, etc.)
    /// - Auto-generation notices
    /// - Module/file docstrings
    ///
    /// The output should NOT include trailing newlines; those are added during merge.
    fn format_header(&self, metadata: &HeaderMetadata) -> String;

    /// Format import/require/use statements
    ///
    /// This method should:
    /// - Group imports by category (stdlib, third-party, local, type-only)
    /// - Sort imports alphabetically within each group
    /// - Handle language-specific syntax (from/import, require, use, etc.)
    /// - Preserve any special import semantics (type imports in TypeScript, etc.)
    ///
    /// Input imports may be in any order; output should be normalized.
    /// The output should NOT include trailing newlines.
    fn format_imports(&self, imports: &[Import]) -> String;

    /// Format a docstring with proper escaping and indentation
    ///
    /// This method should:
    /// - Use the language's standard docstring format (triple quotes, JSDoc, etc.)
    /// - Escape any special characters (e.g., triple quotes within the content)
    /// - Apply proper indentation for nested context
    /// - Preserve line breaks and formatting in the original content
    ///
    /// The output should NOT include trailing newlines.
    fn format_docstring(&self, content: &str) -> String;

    /// Merge multiple code sections into final output
    ///
    /// This method combines header, imports, and body sections with proper spacing:
    /// - Exactly 2 blank lines between top-level definitions (PEP 8, similar standards)
    /// - No duplicate headers (if both are present, deduplicate)
    /// - Ensure trailing newline on final output
    /// - Handle sections in any order (normalize to header → imports → body)
    ///
    /// # Example
    ///
    /// The output might look like:
    /// ```text
    /// #!/usr/bin/env python3
    /// # Auto-generated by spikard
    ///
    /// from typing import List
    /// import msgspec
    ///
    /// class MyType(msgspec.Struct):
    ///     fields: List[str]
    /// ```
    fn merge_sections(&self, sections: &[Section]) -> String;
}
