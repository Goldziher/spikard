//! GraphQL SDL (Schema Definition Language) utilities.
//!
//! This module provides language-agnostic tools for working with GraphQL SDL:
//!
//! - **builder**: Consolidates SDL reconstruction logic from all language generators
//! - **type_mapper**: Utilities for mapping between GraphQL and target language types

pub mod builder;
pub mod type_mapper;

pub use builder::SdlBuilder;
pub use type_mapper::{TargetLanguage, TypeMapper};
