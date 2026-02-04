//! Elixir bindings for the Spikard HTTP framework.
//!
//! This crate provides NIF (Native Implemented Functions) for Elixir via Rustler,
//! enabling high-performance HTTP server functionality in Elixir applications.
//!
//! The bindings expose Rust HTTP handling capabilities through Elixir, allowing
//! Elixir code to define request handlers that are executed with Rust performance.
//!
//! ## Modules
//!
//! - `atoms`: Elixir atom definitions for NIF communication
//! - `conversion`: Elixir ↔ Rust type conversions
//! - `error`: Error types and translation
//! - `handler`: ElixirHandler trait implementation
//! - `server`: HTTP server setup and lifecycle management

#![allow(dead_code)]
#![allow(deprecated)]
#![deny(clippy::unwrap_used)]
#![allow(clippy::similar_names)] // Common in FFI code
#![allow(clippy::missing_errors_doc)] // Many FFI functions return Result
#![allow(clippy::doc_markdown)] // FFI types don't need backticks
#![allow(clippy::missing_const_for_fn)] // FFI functions can't be const
#![allow(clippy::too_many_arguments)] // FFI bridge functions often need many parameters
#![allow(clippy::too_many_lines)] // FFI wrappers accumulate code
#![allow(clippy::unused_self)] // Rustler methods may not use self
#![allow(clippy::unnecessary_wraps)] // Rustler patterns require Result wrappers
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
#![allow(clippy::only_used_in_recursion)] // Recursive conversion functions
#![allow(clippy::implicit_hasher)] // FFI functions use concrete HashMap types
#![allow(clippy::use_self)] // FFI enum conversions
#![allow(clippy::elidable_lifetime_names)] // Explicit lifetimes in FFI signatures
#![allow(clippy::return_self_not_must_use)] // Builder methods in FFI
#![allow(clippy::collapsible_if)] // FFI validation patterns

pub mod atoms;
pub mod conversion;
pub mod error;
pub mod handler;
pub mod server;

rustler::init!("Elixir.Spikard.Native");
