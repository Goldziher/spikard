//! Background task support for Spikard handlers.
//!
//! This module provides stub NIF functions for background task scheduling.
//! The actual implementation is in Elixir using Task.start/1.

// This module is currently a stub - all background task logic is implemented
// in Elixir (packages/elixir/lib/spikard/background.ex) using Task.start/1
// to spawn processes that run after the response is sent to the client.
