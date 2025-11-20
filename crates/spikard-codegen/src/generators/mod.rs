//! Code generators for different target languages

pub mod openapi;
pub mod python;
pub mod rust;
pub mod typescript;

use crate::error::Result;
use crate::ir::IntermediateRepresentation;
use crate::parser::{load_config, validate_config};
use std::path::Path;

/// Target language for code generation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    Python,
    TypeScript,
    Rust,
}

/// Code generator
pub struct Generator {
    ir: IntermediateRepresentation,
}

impl Generator {
    /// Create generator from configuration file
    pub fn from_file(path: &Path) -> Result<Self> {
        let config = load_config(path)?;
        validate_config(&config)?;

        let ir = IntermediateRepresentation::from_config(config)?;

        Ok(Self { ir })
    }

    /// Validate the configuration
    pub fn validate(&self) -> Result<()> {
        // TODO: Validate schema references, handler paths, etc.
        Ok(())
    }

    /// Generate code for a target language
    pub fn generate(&self, target: Target, output_dir: &Path) -> Result<()> {
        match target {
            Target::Python => python::generate(&self.ir, output_dir),
            Target::TypeScript => typescript::generate(&self.ir, output_dir),
            Target::Rust => rust::generate(&self.ir, output_dir),
        }
    }

    /// Generate OpenAPI specification
    pub fn generate_openapi(&self) -> Result<serde_json::Value> {
        openapi::generate(&self.ir)
    }
}
