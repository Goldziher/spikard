//! Shared streaming-virtual-fields module for e2e test codegen.
//!
//! Streaming fixtures assert on "virtual" fields that don't exist on the
//! stream result type itself. These fields require the `streaming` assertion
//! recipe unless the call explicitly maps the asserted root as a real result
//! field.
//!
//! [`StreamingFieldResolver`] provides two entry points:
//! - [`StreamingFieldResolver::accessor`] — the language-specific expression
//!   for a virtual field given a local variable that holds the collected list.
//! - [`StreamingFieldResolver::collect_snippet`] — the language-specific
//!   code snippet that drains a stream variable into the collected list.
//!
//! ## Convention
//!
//! The `chunks_var` parameter is the local variable name that holds the
//! collected list (default: `"chunks"`).  The `stream_var` parameter is the
//! result variable produced by the stream call (default: `"result"`).
//!
//! The neutral streaming-virtual field names handled by this module:
//! - `stream.items`        → the collected list itself
//! - `stream.items.length` → length/count of the collected list
//!
//! Legacy fixture fields still handled for explicitly streaming calls:
//! - `chunks`              → the collected list itself
//! - `chunks.length`       → length/count of the collected list
//! - `stream_content`      → concatenation of all delta content strings
//! - `stream_complete`     → boolean — last chunk has a non-null `finish_reason`
//! - `no_chunks_after_done` → structural invariant (true by construction for
//!   channel/iterator-based APIs once the channel is closed; emitted as
//!   `assert!(true)` / `assertTrue` for languages without post-DONE chunk plumbing)
//! - `tool_calls`          → flat list of `tool_calls` from all chunk deltas
//! - `finish_reason`       → `finish_reason` string from the last chunk

mod accessors;
mod model;
mod renderers;
mod snippets;

pub use model::{STREAMING_VIRTUAL_FIELDS, StreamingFieldResolver, is_streaming_virtual_field, resolve_is_streaming};

#[cfg(test)]
use renderers::{TailSeg, parse_tail};

#[cfg(test)]
mod tests;
