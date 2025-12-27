//! Spikard Code Generation
//!
//! This crate provides configuration parsing and code generation capabilities for Spikard.
//! It allows defining entire servers declaratively via YAML/JSON and generating idiomatic
//! code for Python, TypeScript, and Rust.
//!
//! # Features
//!
//! - **Multi-Protocol**: HTTP, gRPC, queues, CloudEvents
//! - **Type-Safe**: Generated code is fully typed
//! - **Ecosystem Integration**: Works with OpenAPI and Protobuf
//! - **Cross-Language**: Generate code for Python, TypeScript, and Rust
//!
//! # Example
//!
//! ```rust
//! use spikard_codegen::{Generator, Target};
//! use std::path::Path;
//!
//! # fn example() -> anyhow::Result<()> {
//! // Load configuration from YAML
//! let generator = Generator::from_file(Path::new("spikard.yaml"))?;
//!
//! // Validate configuration
//! generator.validate()?;
//!
//! // Generate Python code
//! generator.generate(Target::Python, Path::new("src/generated"))?;
//!
//! // Generate OpenAPI spec
//! let openapi = generator.generate_openapi()?;
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod generators;
pub mod graphql;
pub mod ir;
pub mod openapi;
pub mod parser;

pub use error::{CodegenError, Result};
pub use generators::{Generator, Target};
pub use graphql::{GraphQLFixture, load_graphql_fixtures, load_graphql_fixtures_by_category};
pub use parser::Config;

/// Protocol types supported by Spikard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    /// HTTP/REST APIs
    Http,
    /// gRPC services
    Grpc,
    /// Queue consumers and producers
    Queue,
    /// CloudEvents handlers
    CloudEvents,
}

/// Code generation options
#[derive(Debug, Clone)]
pub struct GenerateOptions {
    /// Target language
    pub target: Target,
    /// Output directory
    pub output_dir: std::path::PathBuf,
    /// Protocols to generate (None = all)
    pub protocols: Option<Vec<Protocol>>,
    /// Whether to overwrite existing files
    pub overwrite: bool,
    /// Whether to format generated code
    pub format: bool,
}

impl Default for GenerateOptions {
    fn default() -> Self {
        Self {
            target: Target::Python,
            output_dir: std::path::PathBuf::from("generated"),
            protocols: None,
            overwrite: true,
            format: true,
        }
    }
}
