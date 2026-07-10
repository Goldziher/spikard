//! HTTP-domain IR types for spikard's alef extension.
//!
//! These types were formerly part of alef core's `application.rs` before the
//! HTTP-domain IR was extracted into this consumer-side extension.

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// ---------------------------------------------------------------------------

/// A lifecycle hook contract — a named callback slot registered on the service owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LifecycleHookDef {
    /// Canonical hook name, e.g. `"on_request"`.
    pub name: String,
    /// Name of the callback contract (trait/interface) the hook function must satisfy.
    pub callback_contract: String,
    /// Documentation for the generated registration method.
    #[serde(default)]
    pub doc: String,
    /// Whether the hook callback is invoked asynchronously.
    #[serde(default)]
    pub is_async: bool,
}

/// A WebSocket route registration contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketRouteDef {
    /// Name of the concrete Rust wrapper struct for passing callables across the FFI.
    pub handler_wrapper_type: String,
    /// Name of the WebSocket connection type passed to the handler.
    pub socket_type: String,
    /// Documentation for the generated `app.websocket(...)` method.
    #[serde(default)]
    pub doc: String,
}

/// An SSE (Server-Sent Events) route registration contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SseRouteDef {
    /// Name of the concrete Rust wrapper struct for passing the producer callable.
    pub producer_wrapper_type: String,
    /// Name of the SSE event type yielded by the producer.
    pub event_type: String,
    /// Documentation for the generated `app.sse(...)` method.
    #[serde(default)]
    pub doc: String,
}

/// HTTP status code classification for an [`ErrorTypeDef`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum HttpStatus {
    /// Named status variant deserialized from a string like `"not_found"`.
    Named(NamedHttpStatus),
    /// Explicit numeric status code.
    Code(u16),
}

/// Named HTTP status variants (snake\_case strings in TOML).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NamedHttpStatus {
    /// 400 Bad Request.
    BadRequest,
    /// 401 Unauthorized.
    Unauthorized,
    /// 403 Forbidden.
    Forbidden,
    /// 404 Not Found.
    NotFound,
    /// 409 Conflict.
    Conflict,
    /// 422 Unprocessable Entity.
    UnprocessableEntity,
    /// 429 Too Many Requests.
    TooManyRequests,
    /// 500 Internal Server Error.
    InternalServerError,
}

impl HttpStatus {
    /// Returns the numeric HTTP status code.
    #[must_use]
    pub const fn as_u16(self) -> u16 {
        match self {
            Self::Named(n) => match n {
                NamedHttpStatus::BadRequest => 400,
                NamedHttpStatus::Unauthorized => 401,
                NamedHttpStatus::Forbidden => 403,
                NamedHttpStatus::NotFound => 404,
                NamedHttpStatus::Conflict => 409,
                NamedHttpStatus::UnprocessableEntity => 422,
                NamedHttpStatus::TooManyRequests => 429,
                NamedHttpStatus::InternalServerError => 500,
            },
            Self::Code(code) => code,
        }
    }
}

/// A cross-binding error type emitted as a native exception class in every language.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorTypeDef {
    /// `PascalCase` error class name, e.g. `"NotFoundError"`.
    pub name: String,
    /// The HTTP status code this error maps to.
    pub http_status: HttpStatus,
    /// Optional RFC 9457 `ProblemDetails` `type` URI.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub problem_details_type: Option<String>,
    /// Documentation for the generated error class.
    #[serde(default)]
    pub doc: String,
}
