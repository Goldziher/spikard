//! Shared utilities for language bindings
//!
//! This crate provides common functionality used across all language bindings
//! (Python, Node.js, Ruby, PHP, WASM) to eliminate code duplication and ensure
//! consistent behavior.

pub mod config_extractor;
pub mod conversion_traits;
pub mod di_traits;
pub mod error_response;
pub mod grpc_metadata;
pub mod handler_base;
pub mod json_conversion;
pub mod lazy_cache;
pub mod lifecycle_base;
pub mod lifecycle_executor;
pub mod response_builder;
pub mod response_interpreter;
pub mod test_client_base;
pub mod validation_helpers;

pub use config_extractor::{ConfigExtractor, ConfigSource};
pub use di_traits::{FactoryDependencyAdapter, ValueDependencyAdapter};
pub use error_response::ErrorResponseBuilder;
pub use grpc_metadata::{extract_metadata_to_hashmap, hashmap_to_metadata};
pub use handler_base::{HandlerError, HandlerExecutor, LanguageHandler};
pub use json_conversion::{JsonConversionError, JsonConversionHelper, JsonConverter, JsonPrimitive};
pub use lazy_cache::LazyCache;
pub use lifecycle_executor::{
    HookResultData, LanguageLifecycleHook, LifecycleExecutor, RequestModifications, extract_body,
};
pub use response_builder::{build_optimized_response, build_optimized_response_bytes};
pub use response_interpreter::{InterpretedResponse, ResponseInterpreter, StreamSource};
