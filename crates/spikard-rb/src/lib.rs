//! Ruby bindings for Spikard.
//!
//! This crate intentionally keeps the surface area minimal. The Ruby layer loads the
//! compiled extension and exposes a small public API. Heavy lifting remains in the
//! Rust engine (`spikard-*` crates).

use magnus::{Ruby, function, prelude::*};

/// Return the current crate version so Ruby can introspect the extension.
fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Initialise the Ruby extension.
///
/// Ruby expects an `Init_*` symbol in cdylib crates. The `magnus::init` macro wires this up.
#[magnus::init]
fn init(ruby: &Ruby) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Spikard")?;
    module.define_singleton_method("version", function!(version, 0))?;
    Ok(())
}
