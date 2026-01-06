//! Project initialization scaffolding for Spikard
//!
//! This module provides the foundation for the `spikard init` command, enabling users
//! to bootstrap new Spikard projects with language-specific structure, configuration,
//! and example handlers.
//!
//! # Architecture
//!
//! The initialization system follows a layered design:
//!
//! - **`ProjectScaffolder`**: Language-agnostic trait defining what files and structure
//!   are needed for a new project.
//! - **`InitEngine`**: Orchestrates the scaffolding process, validates inputs, and
//!   manages file creation.
//! - **`ScaffoldedFile`**: Represents a file to be written with path and content.
//!
//! # Example
//!
//! ```ignore
//! use spikard_cli::init::{InitRequest, InitEngine};
//! use spikard_cli::codegen::TargetLanguage;
//! use std::path::PathBuf;
//!
//! let request = InitRequest {
//!     project_name: "my_api".to_string(),
//!     language: TargetLanguage::Python,
//!     project_dir: PathBuf::from("."),
//!     schema_path: None,
//! };
//!
//! let response = InitEngine::execute(request)?;
//! println!("Created {} files", response.files_created.len());
//! for step in response.next_steps {
//!     println!("  - {}", step);
//! }
//! # Ok::<(), anyhow::Error>(())
//! ```

pub mod engine;
pub mod php;
pub mod python;
pub mod ruby;
pub mod rust_lang;
pub mod scaffolder;
pub mod typescript;

pub use engine::{InitEngine, InitError, InitRequest, InitResponse};
pub use php::PhpScaffolder;
pub use python::PythonScaffolder;
pub use ruby::RubyScaffolder;
pub use rust_lang::RustScaffolder;
pub use scaffolder::{ProjectScaffolder, ScaffoldedFile};
pub use typescript::TypeScriptScaffolder;
