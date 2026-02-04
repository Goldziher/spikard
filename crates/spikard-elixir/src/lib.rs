//! Elixir bindings for the Spikard HTTP framework.
//!
//! This crate provides NIF (Native Implemented Functions) for Elixir via Rustler,
//! enabling high-performance HTTP server functionality in Elixir applications.

rustler::init!("Elixir.Spikard.Native");
