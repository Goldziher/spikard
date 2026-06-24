//! Per-language HTTP e2e test generators.
//!
//! Each submodule holds the HTTP-producing codegen for one language target,
//! staged verbatim from alef core. Generic (non-HTTP) scaffolding stays in alef.

pub mod elixir;
pub mod node;
pub mod php;
pub mod python;
pub mod ruby;
