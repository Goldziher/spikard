//! Shared utilities for language bindings
//!
//! This crate provides common functionality used across all language bindings
//! (Python, Node.js, Ruby, PHP, WASM) to eliminate code duplication and ensure
//! consistent behavior.

pub mod config_extractor;
pub mod conversion_traits;
pub mod di_traits;
pub mod error_response;
pub mod handler_base;
pub mod lifecycle_base;
pub mod lifecycle_executor;
pub mod response_builder;
pub mod test_client_base;
pub mod validation_helpers;

// Re-export commonly used types
pub use config_extractor::{ConfigExtractor, ConfigSource};
pub use di_traits::{FactoryDependencyAdapter, ValueDependencyAdapter};
pub use error_response::ErrorResponseBuilder;
pub use handler_base::{HandlerError, HandlerExecutor, LanguageHandler};
pub use lifecycle_executor::{
    HookResultData, LanguageLifecycleHook, LifecycleExecutor, RequestModifications, extract_body,
};
