//! Elixir dependency injection implementation.
//!
//! This module provides support for dependency injection in Elixir handlers.
//! Dependencies are resolved at the Elixir level and passed to handlers through
//! the RequestData dependencies field.

#![allow(dead_code)]
#![deny(clippy::unwrap_used)]

use rustler::{Encoder, Env, NifResult, Term};
use tracing::debug;

use crate::atoms;

/// Placeholder NIF for future factory dependency support.
/// For now, all DI is handled at the Elixir level.
#[rustler::nif]
pub fn deliver_factory_response<'a>(
    env: Env<'a>,
    _request_id: u64,
    _result: Term<'a>,
) -> NifResult<Term<'a>> {
    debug!("deliver_factory_response called (placeholder implementation)");
    // TODO: Implement factory dependency resolution when needed
    Ok(atoms::ok().encode(env))
}
