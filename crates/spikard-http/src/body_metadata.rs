//! Shared metadata stored on `http::Response` extensions.

/// Extension type storing the original, uncompressed response body size in bytes.
///
/// Middleware (like compression) can inspect this to make deterministic decisions
/// even when the final body length would otherwise require buffering.
#[derive(Debug, Clone, Copy)]
pub struct ResponseBodySize(pub usize);
