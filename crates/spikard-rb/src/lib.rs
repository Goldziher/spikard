#![allow(deprecated)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::similar_names)] // Common in FFI code
#![allow(clippy::missing_errors_doc)] // Many FFI functions return Result
#![allow(clippy::doc_markdown)] // FFI types don't need backticks
#![allow(clippy::missing_const_for_fn)] // FFI functions can't be const
#![allow(clippy::too_many_arguments)] // FFI bridge functions often need many parameters
#![allow(clippy::too_many_lines)] // FFI wrappers accumulate code
#![allow(clippy::unused_self)] // Magnus methods may not use self
#![allow(clippy::unnecessary_wraps)] // Magnus patterns require Result wrappers
#![allow(clippy::must_use_candidate)] // FFI constructors follow Rust patterns
#![allow(clippy::struct_excessive_bools)] // FFI configs use multiple bools
#![allow(clippy::fn_params_excessive_bools)] // FFI builders pass multiple bools
#![allow(clippy::items_after_statements)] // Common in Rust code
#![allow(clippy::if_not_else)] // FFI code style preference
#![allow(clippy::redundant_clone)] // May be necessary in FFI boundary
#![allow(clippy::uninlined_format_args)] // FFI error messages
#![allow(clippy::cognitive_complexity)] // FFI handlers have complex logic
#![allow(clippy::cast_lossless)] // Type conversions in FFI
#![allow(clippy::option_if_let_else)] // FFI error handling patterns
#![allow(clippy::missing_panics_doc)] // Runtime server panics acceptable in server context
#![allow(clippy::unused_async)] // Async trait methods may not await
#![allow(clippy::non_std_lazy_statics)] // using_once_cell pattern
#![allow(clippy::ptr_as_ptr)] // Raw pointer casts in FFI code
#![allow(clippy::ptr_cast_constness)] // Cast constness for FFI interop
#![allow(clippy::significant_drop_tightening)] // Drop timing in FFI bridges
#![allow(clippy::trivially_copy_pass_by_ref)] // FFI compatibility
#![allow(clippy::cast_possible_wrap)] // Cast wrapping in FFI
#![allow(clippy::cast_possible_truncation)] // Type size differences in FFI
#![allow(clippy::used_underscore_binding)] // Internal FFI code
#![allow(clippy::redundant_closure)] // FFI closure patterns
#![allow(clippy::explicit_iter_loop)] // FFI iteration style
#![allow(clippy::cast_sign_loss)] // Unsigned/signed casts in FFI
#![allow(clippy::map_unwrap_or)] // Idiomatic Option/Result handling
#![allow(clippy::implicit_clone)] // String conversions in FFI
#![allow(clippy::ref_option_ref)] // Reference patterns in FFI
#![allow(clippy::should_implement_trait)] // FFI trait implementation
#![allow(clippy::match_like_matches_macro)] // FFI match patterns
#![allow(clippy::match_bool)] // Boolean matching in FFI
#![allow(clippy::format_push_string)] // String formatting in FFI
#![allow(clippy::option_option)] // Option nesting in FFI
#![allow(clippy::enum_variant_names)] // FFI variant naming
#![allow(clippy::identity_op)] // FFI operations
#![allow(clippy::filter_next)] // Filter operations in FFI
#![allow(clippy::manual_let_else)] // Let-else patterns in FFI
#![allow(clippy::if_then_some_else_none)] // If-then-some patterns
#![allow(clippy::clone_on_copy)] // Clone on copy types in FFI
#![allow(clippy::unit_arg)] // Unit argument handling
#![allow(clippy::impl_trait_in_params)] // Trait parameters in FFI
#![allow(clippy::match_same_arms)] // Identical match arms
#![allow(clippy::needless_pass_by_value)] // FFI argument passing style
#![allow(clippy::ref_as_ptr)] // Explicit pointer casts in FFI
#![allow(clippy::while_let_on_iterator)] // Iterator patterns in FFI
#![allow(clippy::redundant_closure_for_method_calls)] // Closure patterns in FFI
#![allow(clippy::as_ptr_cast_mut)] // Raw pointer casting in FFI
#![allow(clippy::match_wildcard_for_single_variants)] // Wildcard patterns in FFI
#![allow(clippy::ignored_unit_patterns)] // Unit pattern handling in FFI
#![allow(clippy::option_as_ref_deref)] // Option reference patterns
#![allow(clippy::semicolon_if_nothing_returned)] // Return statement consistency
#![allow(clippy::map_identity)] // Identity mapping patterns

//! Spikard Ruby bindings using Magnus FFI.
//!
//! This crate provides Ruby bindings for the Spikard HTTP toolkit, allowing
//! Ruby developers to build and test HTTP services with Rust performance.
//!
//! ## Modules
//!
//! - `testing`: Testing utilities (client, SSE, WebSocket)
//! - `handler`: RubyHandler trait implementation
//! - `di`: Dependency injection bridge for Ruby types
//! - `config`: ServerConfig extraction from Ruby objects
//! - `conversion`: Ruby ↔ Rust type conversions
//! - `app`: HTTP server setup, lifecycle, and application logic
//! - `background`: Background task management
//! - `lifecycle`: Lifecycle hook implementations
//! - `sse`: Server-Sent Events support
//! - `websocket`: WebSocket support
//! - `grpc`: gRPC handler support

mod app;
mod background;
mod config;
mod conversion;
mod di;
mod grpc;
mod gvl;
mod handler;
mod integration;
mod lifecycle;
mod metadata;
mod request;
mod runtime;
mod server;
mod sse;
mod testing;
mod websocket;

use magnus::{Error, Ruby, function, gc::Marker, method};
use magnus::prelude::*;

use crate::app::{
    NativeBuiltResponse, NativeTestClient, NativeDependencyRegistry, NativeLifecycleRegistry,
    RubyHandler, build_response, build_streaming_response, mark, version,
};
use crate::metadata::build_route_metadata;
use crate::request::NativeRequest;
use crate::runtime::{run_server, normalize_route_metadata};

#[magnus::init]
pub fn init(ruby: &Ruby) -> Result<(), Error> {
    let spikard = ruby.define_module("Spikard")?;
    spikard.define_singleton_method("version", function!(version, 0))?;
    let native = match spikard.const_get("Native") {
        Ok(module) => module,
        Err(_) => spikard.define_module("Native")?,
    };

    native.define_singleton_method("run_server", function!(run_server, 8))?;
    native.define_singleton_method("normalize_route_metadata", function!(normalize_route_metadata, 1))?;
    native.define_singleton_method("background_run", function!(background::background_run, 1))?;
    native.define_singleton_method(
        "__shutdown_websocket_workers__",
        function!(websocket::shutdown_websocket_workers, 0),
    )?;
    native.define_singleton_method("build_route_metadata", function!(build_route_metadata, 12))?;
    native.define_singleton_method("build_response", function!(build_response, 3))?;
    native.define_singleton_method("build_streaming_response", function!(build_streaming_response, 3))?;

    let class = native.define_class("TestClient", ruby.class_object())?;
    class.define_alloc_func::<NativeTestClient>();
    class.define_method("initialize", method!(NativeTestClient::initialize, 6))?;
    class.define_method("request", method!(NativeTestClient::request, 3))?;
    class.define_method("websocket", method!(NativeTestClient::websocket, 1))?;
    class.define_method("sse", method!(NativeTestClient::sse, 1))?;
    class.define_method("graphql", method!(NativeTestClient::graphql, 3))?;
    class.define_method("graphql_with_status", method!(NativeTestClient::graphql_with_status, 3))?;
    class.define_method("close", method!(NativeTestClient::close, 0))?;

    let built_response_class = native.define_class("BuiltResponse", ruby.class_object())?;
    built_response_class.define_method("status_code", method!(NativeBuiltResponse::status_code, 0))?;
    built_response_class.define_method("headers", method!(NativeBuiltResponse::headers, 0))?;

    let request_class = native.define_class("Request", ruby.class_object())?;
    request_class.define_method("method", method!(NativeRequest::method, 0))?;
    request_class.define_method("path", method!(NativeRequest::path, 0))?;
    request_class.define_method("path_params", method!(NativeRequest::path_params, 0))?;
    request_class.define_method("query", method!(NativeRequest::query, 0))?;
    request_class.define_method("raw_query", method!(NativeRequest::raw_query, 0))?;
    request_class.define_method("headers", method!(NativeRequest::headers, 0))?;
    request_class.define_method("cookies", method!(NativeRequest::cookies, 0))?;
    request_class.define_method("body", method!(NativeRequest::body, 0))?;
    request_class.define_method("raw_body", method!(NativeRequest::raw_body, 0))?;
    request_class.define_method("params", method!(NativeRequest::params, 0))?;
    request_class.define_method("to_h", method!(NativeRequest::to_h, 0))?;
    request_class.define_method("[]", method!(NativeRequest::index, 1))?;

    let lifecycle_registry_class = native.define_class("LifecycleRegistry", ruby.class_object())?;
    lifecycle_registry_class.define_alloc_func::<NativeLifecycleRegistry>();
    lifecycle_registry_class.define_method("add_on_request", method!(NativeLifecycleRegistry::add_on_request, 1))?;
    lifecycle_registry_class.define_method(
        "add_pre_validation",
        method!(NativeLifecycleRegistry::add_pre_validation, 1),
    )?;
    lifecycle_registry_class.define_method(
        "pre_validation",
        method!(NativeLifecycleRegistry::add_pre_validation, 1),
    )?;
    lifecycle_registry_class.define_method("add_pre_handler", method!(NativeLifecycleRegistry::add_pre_handler, 1))?;
    lifecycle_registry_class.define_method("pre_handler", method!(NativeLifecycleRegistry::add_pre_handler, 1))?;
    lifecycle_registry_class.define_method("add_on_response", method!(NativeLifecycleRegistry::add_on_response, 1))?;
    lifecycle_registry_class.define_method("on_response", method!(NativeLifecycleRegistry::add_on_response, 1))?;
    lifecycle_registry_class.define_method("add_on_error", method!(NativeLifecycleRegistry::add_on_error, 1))?;
    lifecycle_registry_class.define_method("on_error", method!(NativeLifecycleRegistry::add_on_error, 1))?;

    let dependency_registry_class = native.define_class("DependencyRegistry", ruby.class_object())?;
    dependency_registry_class.define_alloc_func::<NativeDependencyRegistry>();
    dependency_registry_class.define_method("register_value", method!(NativeDependencyRegistry::register_value, 2))?;
    dependency_registry_class.define_method(
        "register_factory",
        method!(NativeDependencyRegistry::register_factory, 5),
    )?;
    dependency_registry_class.define_method("keys", method!(NativeDependencyRegistry::keys, 0))?;
    dependency_registry_class.define_method("resolve", method!(NativeDependencyRegistry::resolve, 1))?;

    let spikard_module = ruby.define_module("Spikard")?;
    testing::websocket::init(ruby, &spikard_module)?;
    testing::sse::init(ruby, &spikard_module)?;
    grpc::handler::init(ruby, &spikard_module)?;

    let _ = NativeBuiltResponse::mark as fn(&NativeBuiltResponse, &Marker);
    let _ = NativeLifecycleRegistry::mark as fn(&NativeLifecycleRegistry, &Marker);
    let _ = NativeDependencyRegistry::mark as fn(&NativeDependencyRegistry, &Marker);
    let _ = NativeRequest::mark as fn(&NativeRequest, &Marker);
    let _ = RubyHandler::mark as fn(&RubyHandler, &Marker);
    let _ = mark as fn(&NativeTestClient, &Marker);

    Ok(())
}
