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

/// The set of field names treated as streaming-virtual fields.
pub const STREAMING_VIRTUAL_FIELDS: &[&str] = &[
    "stream.items",
    "stream.items.length",
    "chunks",
    "chunks.length",
    "stream_content",
    "stream_complete",
    "no_chunks_after_done",
    "tool_calls",
    "finish_reason",
    // Event-stream variant predicates: resolve against the collected `chunks`
    // list where each item is a tagged union with `page` / `error` / `complete`
    // variants. `event_count_min` is a synonym for the chunk
    // count, used with `greater_than_or_equal` to assert "at least N events".
    "stream.has_page_event",
    "stream.has_error_event",
    "stream.has_complete_event",
    "stream.event_count_min",
];

/// The set of streaming-virtual root names that may have deep-path continuations.
///
/// A field like `tool_calls[0].function.name` starts with `tool_calls` and has
/// a continuation `[0].function.name`. These are handled by
/// [`StreamingFieldResolver::accessor`] via the deep-path logic.
///
/// `usage` is a stream-level root: `usage.total_tokens` resolves against the
/// last chunk that carried a usage payload (accessed via the collected chunks
/// list). Python accessor: `(chunks[-1].usage if chunks else None)`.
pub(super) const STREAMING_VIRTUAL_ROOTS: &[&str] = &["tool_calls", "finish_reason"];

/// Returns `true` when `field` is a streaming-virtual field name, including
/// deep-nested paths that start with a known streaming-virtual root.
///
/// Examples that return `true`:
/// - `"tool_calls"` (exact root)
/// - `"tool_calls[0].function.name"` (deep path)
/// - `"tool_calls[0].id"` (deep path)
#[must_use]
pub fn is_streaming_virtual_field(field: &str) -> bool {
    if STREAMING_VIRTUAL_FIELDS.contains(&field) {
        return true;
    }
    // Check deep-path prefixes: `tool_calls[…` or `tool_calls.`
    for root in STREAMING_VIRTUAL_ROOTS {
        if field.len() > root.len() && field.starts_with(root) {
            let rest = &field[root.len()..];
            if rest.starts_with('[') || rest.starts_with('.') {
                return true;
            }
        }
    }
    false
}

/// Split a field path into `(root, tail)` when it starts with a streaming-virtual
/// root and has a continuation.
///
/// Returns `None` when the field is an exact root match (no tail) or is not a
/// streaming-virtual root at all.
pub(super) fn split_streaming_deep_path(field: &str) -> Option<(&str, &str)> {
    for root in STREAMING_VIRTUAL_ROOTS {
        if field.len() > root.len() && field.starts_with(root) {
            let rest = &field[root.len()..];
            if rest.starts_with('[') || rest.starts_with('.') {
                return Some((root, rest));
            }
        }
    }
    None
}

/// Field names that unambiguously imply a streaming test (no overlap with
/// non-streaming response shapes). `usage`, `tool_calls`, and `finish_reason`
/// are intentionally excluded — they exist on non-streaming responses too
/// (`usage.total_tokens` on `ChatCompletionResponse`, `choices[0].finish_reason`,
/// etc.) and would otherwise drag non-streaming fixtures into streaming
/// codegen.
const STREAMING_ONLY_AUTO_DETECT_FIELDS: &[&str] = &[
    "stream.items",
    "stream.items.length",
    "stream_content",
    "stream_complete",
    "no_chunks_after_done",
    "stream.has_page_event",
    "stream.has_error_event",
    "stream.has_complete_event",
    "stream.event_count_min",
];

/// Resolve whether a fixture should be treated as streaming, honoring the
/// call-level three-valued opt-in/out (`CallConfig::streaming`):
///
/// - `Some(true)` → forced streaming.
/// - `Some(false)` → forced non-streaming (skip the auto-detect even when an
///   assertion references a streaming-virtual-field name like `chunks`).
/// - `None` → auto-detect: streaming iff the fixture has a streaming mock
///   (`mock_response.stream_chunks`) or any assertion references one of the
///   unambiguous streaming-only field names.
///
/// All backends should use this helper so the opt-out is respected uniformly.
#[must_use]
pub fn resolve_is_streaming(fixture: &alef::e2e::fixture::Fixture, call_streaming: Option<bool>) -> bool {
    if let Some(forced) = call_streaming {
        return forced;
    }
    fixture.is_streaming_mock()
        || fixture.assertions.iter().any(|a| {
            a.field
                .as_deref()
                .is_some_and(|f| !f.is_empty() && STREAMING_ONLY_AUTO_DETECT_FIELDS.contains(&f))
        })
}

/// Shared streaming-virtual-fields resolver for e2e test codegen.
pub struct StreamingFieldResolver;
