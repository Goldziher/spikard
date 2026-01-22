//! Response interpretation and handling for language bindings
//!
//! This module provides traits and types for abstracting over language-specific
//! response patterns, enabling consistent handling across all bindings
//! (Python, Node.js, Ruby, PHP, WASM).
//!
//! # Overview
//!
//! Language bindings receive responses from handler functions that can take three forms:
//! 1. **Streaming responses** - Iterables that yield chunks incrementally
//! 2. **Custom responses** - Objects with explicit status codes, headers, and bodies
//! 3. **Plain responses** - Simple JSON values that become 200 OK responses
//!
//! Rather than duplicating response detection logic in each binding, this module
//! provides a shared [`ResponseInterpreter`] trait that bindings implement to translate
//! language-specific values into a unified [`InterpretedResponse`] enum.
//!
//! # Benefits
//!
//! - **Code reuse**: ~150 LOC of response detection logic shared across all bindings
//! - **Consistency**: All bindings interpret responses identically
//! - **Maintainability**: Single source of truth for response patterns
//! - **Extensibility**: Adding new response types requires changes in one place
//!
//! # Architecture
//!
//! ```text
//! Handler Function (Language-specific)
//!     ↓
//! Language Value (Python object, Node object, etc.)
//!     ↓
//! [ResponseInterpreter impl for language] — interprets language value
//!     ↓
//! InterpretedResponse — unified enum (streaming, custom, or plain)
//!     ↓
//! HTTP Response (status, headers, body)
//! ```
//!
//! # Examples
//!
//! ## Implementing `ResponseInterpreter` for a Language Binding
//!
//! ```ignore
//! use spikard_bindings_shared::{ResponseInterpreter, InterpretedResponse, StreamSource};
//! use serde_json::Value;
//! use std::collections::HashMap;
//!
//! struct PythonInterpreter;
//!
//! impl ResponseInterpreter for PythonInterpreter {
//!     type LanguageValue = PyObject;
//!     type Error = PyErr;
//!
//!     fn is_streaming(&self, value: &Self::LanguageValue) -> bool {
//!         // Check if object is iterable/generator
//!         // e.g., hasattr(obj, '__iter__') and not isinstance(obj, dict)
//!         todo!()
//!     }
//!
//!     fn is_custom_response(&self, value: &Self::LanguageValue) -> bool {
//!         // Check if object has 'status_code' or 'headers' attributes
//!         todo!()
//!     }
//!
//!     fn interpret(&self, value: &Self::LanguageValue) -> Result<InterpretedResponse, Self::Error> {
//!         if self.is_streaming(value) {
//!             // Wrap Python iterator in StreamSource
//!             let stream = PythonStreamSource { obj: value.clone() };
//!             Ok(InterpretedResponse::Streaming {
//!                 enumerator: Box::new(stream),
//!                 status: 200,
//!                 headers: HashMap::new(),
//!             })
//!         } else if self.is_custom_response(value) {
//!             // Extract status, headers, body from custom response object
//!             let status = extract_status(value)?;
//!             let headers = extract_headers(value)?;
//!             let body = extract_body(value)?;
//!             Ok(InterpretedResponse::Custom {
//!                 status,
//!                 headers,
//!                 body,
//!                 raw_body: None,
//!             })
//!         } else {
//!             // Treat as plain JSON
//!             let body = python_to_json(value)?;
//!             Ok(InterpretedResponse::Plain { body })
//!         }
//!     }
//! }
//! ```

use serde_json::Value;
use std::collections::HashMap;

/// A trait for language-specific response stream sources
///
/// This trait abstracts over different language iteration mechanisms
/// (Python generators, Node.js async iterables, Ruby enumerables, etc.)
/// to provide a uniform interface for streaming responses.
///
/// # Design Goals
///
/// - **Object-safe**: No generic type parameters, supports dynamic dispatch via `Box<dyn StreamSource>`
/// - **Chunk-oriented**: Returns `Option<Vec<u8>>` for each iteration, `None` on completion
/// - **Memory-efficient**: Yields data incrementally without buffering entire response
/// - **Async-compatible**: `Send + Sync` allows usage with async runtimes
///
/// # Implementation Notes
///
/// - Must be stateful: tracks position in the underlying iterator/generator
/// - Should handle language-specific iteration protocol (e.g., Python's `__next__`)
/// - Must propagate errors from the underlying iterator
/// - Should handle encoding (binary data or UTF-8 conversion as needed)
///
/// # Examples
///
/// ```ignore
/// // Python generator implementation
/// struct PythonStreamSource {
///     generator: PyObject,
///     py: Python<'static>,
/// }
///
/// impl StreamSource for PythonStreamSource {
///     fn next_chunk(&mut self) -> Option<Vec<u8>> {
///         // Call next() on Python generator
///         // Convert chunk to Vec<u8>
///         // Return None when StopIteration raised
///         todo!()
///     }
/// }
/// ```
pub trait StreamSource: Send + Sync {
    /// Get the next chunk of data from the stream
    ///
    /// # Returns
    ///
    /// - `Some(chunk)` - A chunk of data (may be empty `vec![]` for flush signals)
    /// - `None` - End of stream reached
    ///
    /// # Errors
    ///
    /// Errors from the underlying iterator should be handled by converting
    /// to UTF-8 encoded error messages in the chunk stream, or by returning
    /// `None` to signal end of stream and allowing the binding to handle errors
    /// before streaming begins.
    fn next_chunk(&mut self) -> Option<Vec<u8>>;
}

/// Represents an interpreted HTTP response from a handler function
///
/// This enum captures the three possible response patterns:
/// 1. **Streaming** - Iterative data delivery
/// 2. **Custom** - Explicit control over status, headers, and body
/// 3. **Plain** - Simple JSON response
///
/// # Semantics
///
/// - **Streaming**: Used for server-sent events, file downloads, large responses
///   - Status code and headers are sent immediately
///   - Chunks are sent as they become available
///   - HTTP connection remains open until `StreamSource::next_chunk()` returns `None`
///
/// - **Custom**: Used for responses with explicit status codes or headers
///   - `status` is required (default 200 if not specified)
///   - `headers` can be empty for default behavior
///   - `body` is the JSON response body (may be `None` for streaming custom responses)
///   - `raw_body` can contain pre-encoded bytes (skips JSON serialization)
///
/// - **Plain**: Simplest form, always 200 OK
///   - `body` is a `Value` that will be JSON-serialized
///   - No custom headers or status codes
///   - Most common response type (~90% of real-world APIs)
///
/// # Performance Characteristics
///
/// - **Streaming**: Zero-copy streaming, minimal memory overhead
/// - **Custom**: One allocation for headers map
/// - **Plain**: One allocation for `Value`
///
/// # Examples
///
/// ```ignore
/// use spikard_bindings_shared::InterpretedResponse;
/// use serde_json::json;
/// use std::collections::HashMap;
///
/// // Streaming response
/// let response = InterpretedResponse::Streaming {
///     enumerator: Box::new(my_stream),
///     status: 200,
///     headers: Default::default(),
/// };
///
/// // Custom response with status and headers
/// let mut headers = HashMap::new();
/// headers.insert("content-type".to_string(), "application/json".to_string());
/// let response = InterpretedResponse::Custom {
///     status: 201,
///     headers,
///     body: Some(json!({ "id": 42 })),
///     raw_body: None,
/// };
///
/// // Plain JSON response
/// let response = InterpretedResponse::Plain {
///     body: json!({ "success": true }),
/// };
/// ```
pub enum InterpretedResponse {
    /// Streaming response with incremental data delivery
    ///
    /// Used for responses where data is produced over time:
    /// - File downloads
    /// - Server-sent events
    /// - Large data sets
    /// - Real-time data feeds
    ///
    /// # Fields
    ///
    /// - `enumerator`: Boxed `StreamSource` trait object that yields chunks
    /// - `status`: HTTP status code (typically 200)
    /// - `headers`: Response headers (e.g., Content-Type, Cache-Control)
    Streaming {
        /// Stream of data chunks
        enumerator: Box<dyn StreamSource>,
        /// HTTP status code
        status: u16,
        /// Response headers
        headers: HashMap<String, String>,
    },

    /// Custom response with explicit control over HTTP semantics
    ///
    /// Used when the handler wants fine-grained control:
    /// - Custom status codes (201, 202, 204, 3xx, 4xx, 5xx)
    /// - Custom response headers
    /// - Pre-encoded binary body
    /// - Streaming with custom status/headers
    ///
    /// # Fields
    ///
    /// - `status`: HTTP status code
    /// - `headers`: Custom headers
    /// - `body`: JSON body (None for 204 No Content or streaming responses)
    /// - `raw_body`: Pre-encoded bytes (takes precedence over `body` if present)
    Custom {
        /// HTTP status code
        status: u16,
        /// Response headers
        headers: HashMap<String, String>,
        /// JSON response body (may be None)
        body: Option<Value>,
        /// Pre-encoded response body (takes precedence over body)
        raw_body: Option<Vec<u8>>,
    },

    /// Plain JSON response with default HTTP semantics
    ///
    /// Simplest and most common response type. Automatically:
    /// - Sets status to 200 OK
    /// - Sets Content-Type: application/json
    /// - Serializes body as JSON
    ///
    /// # Fields
    ///
    /// - `body`: The JSON response body
    Plain { body: Value },
}

impl std::fmt::Debug for InterpretedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Streaming { status, headers, .. } => f
                .debug_struct("Streaming")
                .field("status", status)
                .field("headers", headers)
                .field("enumerator", &"<StreamSource>")
                .finish(),
            Self::Custom {
                status,
                headers,
                body,
                raw_body,
            } => f
                .debug_struct("Custom")
                .field("status", status)
                .field("headers", headers)
                .field("body", body)
                .field("raw_body", &raw_body.as_ref().map(|_| "<Vec<u8>>"))
                .finish(),
            Self::Plain { body } => f.debug_struct("Plain").field("body", body).finish(),
        }
    }
}

impl InterpretedResponse {
    /// Get the HTTP status code for this response
    ///
    /// # Returns
    ///
    /// The status code (200 for Plain, otherwise as specified)
    #[must_use]
    pub const fn status(&self) -> u16 {
        match self {
            Self::Streaming { status, .. } | Self::Custom { status, .. } => *status,
            Self::Plain { .. } => 200,
        }
    }

    /// Get the response headers, if any
    ///
    /// # Returns
    ///
    /// - `Some(headers)` for Streaming or Custom responses with headers
    /// - `None` for responses without custom headers
    #[must_use]
    #[allow(clippy::missing_const_for_fn)]
    pub fn headers(&self) -> Option<&HashMap<String, String>> {
        match self {
            Self::Streaming { headers, .. } | Self::Custom { headers, .. } => Some(headers),
            Self::Plain { .. } => None,
        }
    }

    /// Check if this is a streaming response
    #[must_use]
    pub const fn is_streaming(&self) -> bool {
        matches!(self, Self::Streaming { .. })
    }

    /// Check if this is a custom response
    #[must_use]
    pub const fn is_custom(&self) -> bool {
        matches!(self, Self::Custom { .. })
    }

    /// Check if this is a plain response
    #[must_use]
    pub const fn is_plain(&self) -> bool {
        matches!(self, Self::Plain { .. })
    }
}

/// A trait for interpreting language-specific response values
///
/// Language bindings implement this trait to translate handler response values
/// into a unified [`InterpretedResponse`] enum. This consolidates response detection
/// logic that was previously duplicated across bindings.
///
/// # Design
///
/// Each binding has its own response patterns:
/// - **Python**: Functions return values, generators for streaming, Response objects for custom
/// - **Node.js**: Functions return values or Promises, `AsyncIterable`s for streaming, Response objects
/// - **Ruby**: Methods return values, Enumerables for streaming, Response objects
/// - **PHP**: Functions return values, Generators for streaming, Response objects
/// - **WASM**: Functions return values or Promises (no streaming in initial implementation)
///
/// This trait allows each binding to detect these patterns independently while
/// sharing the response handling downstream.
///
/// # Associated Types
///
/// - `LanguageValue`: The language's value type (`PyObject`, `JsValue`, `VALUE`, etc.)
/// - `Error`: The language's error type (`PyErr`, `napi::Error`, `magnus::Error`, etc.)
///
/// # Methods
///
/// Bindings should implement detection logic in `is_streaming` and `is_custom_response`,
/// then use those in `interpret` to construct the appropriate [`InterpretedResponse`] variant.
///
/// # Example Implementation
///
/// ```ignore
/// struct PythonInterpreter;
///
/// impl ResponseInterpreter for PythonInterpreter {
///     type LanguageValue = PyObject;
///     type Error = PyErr;
///
///     fn is_streaming(&self, value: &PyObject) -> bool {
///         // Check __iter__ and __next__ methods
///         Python::with_gil(|py| {
///             value.getattr(py, "__iter__").is_ok() &&
///             value.getattr(py, "__next__").is_ok() &&
///             !is_dict(value)
///         })
///     }
///
///     fn is_custom_response(&self, value: &PyObject) -> bool {
///         // Check for status_code or headers attributes
///         Python::with_gil(|py| {
///             value.getattr(py, "status_code").is_ok() ||
///             value.getattr(py, "headers").is_ok()
///         })
///     }
///
///     fn interpret(&self, value: &PyObject) -> Result<InterpretedResponse, PyErr> {
///         if self.is_streaming(value) {
///             // Create StreamSource wrapper
///         } else if self.is_custom_response(value) {
///             // Extract status, headers, body
///         } else {
///             // Treat as plain JSON
///         }
///     }
/// }
/// ```
pub trait ResponseInterpreter {
    /// The language-specific response value type
    ///
    /// Examples: `PyObject` (Python), `JsValue` (Node.js), `VALUE` (Ruby), etc.
    type LanguageValue;

    /// The language-specific error type
    ///
    /// Examples: `PyErr` (Python), `napi::Error` (Node.js), `magnus::Error` (Ruby), etc.
    type Error: std::fmt::Display;

    /// Check if a value is a streaming response
    ///
    /// Streaming responses are iterables that yield chunks incrementally.
    /// This method should detect the language-specific pattern for iteration
    /// without consuming the value.
    ///
    /// # Implementation Notes
    ///
    /// - Should not consume the value (non-mutable, non-destructive check)
    /// - Should return false for dicts/objects (they're custom responses or plain values)
    /// - Should return true for iterators, generators, async iterables, etc.
    ///
    /// # Examples
    ///
    /// - Python: `hasattr(obj, '__iter__') and hasattr(obj, '__next__') and not isinstance(obj, dict)`
    /// - Node.js: `obj && typeof obj[Symbol.asyncIterator] === 'function'`
    /// - Ruby: `obj.respond_to?(:each) and !obj.is_a?(Hash)`
    ///
    /// # Returns
    ///
    /// `true` if the value is a streaming response, `false` otherwise
    fn is_streaming(&self, value: &Self::LanguageValue) -> bool;

    /// Check if a value is a custom response object
    ///
    /// Custom responses have explicit status codes, headers, or body control.
    /// This method should detect the language-specific pattern for response objects
    /// without consuming the value.
    ///
    /// # Implementation Notes
    ///
    /// - Should not consume the value (non-mutable, non-destructive check)
    /// - Should return false for plain JSON types (primitives, arrays, plain dicts)
    /// - Should return true for response wrapper objects
    ///
    /// # Examples
    ///
    /// - Python: `hasattr(obj, 'status_code') or hasattr(obj, 'headers')`
    /// - Node.js: `obj && (obj.statusCode !== undefined or obj.status !== undefined)`
    /// - Ruby: `obj.respond_to?(:status_code) or obj.respond_to?(:headers)`
    ///
    /// # Returns
    ///
    /// `true` if the value is a custom response, `false` otherwise
    fn is_custom_response(&self, value: &Self::LanguageValue) -> bool;

    /// Interpret a language-specific value into a unified response format
    ///
    /// This method performs the full interpretation logic:
    /// 1. Checks if streaming (call `is_streaming()`)
    /// 2. Checks if custom response (call `is_custom_response()`)
    /// 3. Otherwise treats as plain JSON
    ///
    /// Each branch extracts the relevant data and constructs an [`InterpretedResponse`].
    ///
    /// # Arguments
    ///
    /// * `value` - The language-specific value from the handler
    ///
    /// # Returns
    ///
    /// - `Ok(InterpretedResponse)` - Successfully interpreted response
    /// - `Err(Self::Error)` - Error during interpretation (type conversion, missing fields, etc.)
    ///
    /// # Errors
    ///
    /// Returns `Err(Self::Error)` if:
    /// - Streaming source creation failed
    /// - Status code extraction failed
    /// - Header extraction/conversion failed
    /// - Body conversion to JSON failed
    fn interpret(&self, value: &Self::LanguageValue) -> Result<InterpretedResponse, Self::Error>;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Mock `StreamSource` for testing
    struct MockStreamSource {
        chunks: Vec<Vec<u8>>,
        index: usize,
    }

    impl MockStreamSource {
        fn new(chunks: Vec<Vec<u8>>) -> Self {
            Self { chunks, index: 0 }
        }
    }

    impl StreamSource for MockStreamSource {
        fn next_chunk(&mut self) -> Option<Vec<u8>> {
            if self.index < self.chunks.len() {
                let chunk = self.chunks[self.index].clone();
                self.index += 1;
                Some(chunk)
            } else {
                None
            }
        }
    }

    /// Mock interpreter for testing
    enum TestValue {
        Stream,
        Custom,
        Plain,
    }

    #[derive(Debug)]
    struct TestError(String);

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    struct TestInterpreter;

    impl ResponseInterpreter for TestInterpreter {
        type LanguageValue = TestValue;
        type Error = TestError;

        fn is_streaming(&self, value: &Self::LanguageValue) -> bool {
            matches!(value, TestValue::Stream)
        }

        fn is_custom_response(&self, value: &Self::LanguageValue) -> bool {
            matches!(value, TestValue::Custom)
        }

        fn interpret(&self, value: &Self::LanguageValue) -> Result<InterpretedResponse, Self::Error> {
            match value {
                TestValue::Stream => Ok(InterpretedResponse::Streaming {
                    enumerator: Box::new(MockStreamSource::new(vec![b"chunk1".to_vec(), b"chunk2".to_vec()])),
                    status: 200,
                    headers: HashMap::new(),
                }),
                TestValue::Custom => {
                    let mut headers = HashMap::new();
                    headers.insert("x-custom".to_string(), "header".to_string());
                    Ok(InterpretedResponse::Custom {
                        status: 201,
                        headers,
                        body: Some(Value::from("custom body")),
                        raw_body: None,
                    })
                }
                TestValue::Plain => Ok(InterpretedResponse::Plain {
                    body: Value::from("plain body"),
                }),
            }
        }
    }

    #[test]
    fn test_stream_source_trait_object_safety() {
        let mut stream = MockStreamSource::new(vec![b"test".to_vec()]);
        assert_eq!(stream.next_chunk(), Some(b"test".to_vec()));
        assert_eq!(stream.next_chunk(), None);

        // Verify we can box it
        let mut boxed: Box<dyn StreamSource> = Box::new(MockStreamSource::new(vec![b"test".to_vec()]));
        assert!(boxed.next_chunk().is_some());
    }

    #[test]
    fn test_interpreted_response_streaming_construction() {
        let stream = MockStreamSource::new(vec![b"chunk".to_vec()]);
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/octet-stream".to_string());

        let response = InterpretedResponse::Streaming {
            enumerator: Box::new(stream),
            status: 200,
            headers,
        };

        assert!(response.is_streaming());
        assert!(!response.is_custom());
        assert!(!response.is_plain());
        assert_eq!(response.status(), 200);
        assert!(response.headers().is_some());
    }

    #[test]
    fn test_interpreted_response_custom_construction() {
        let mut headers = HashMap::new();
        headers.insert("x-custom-header".to_string(), "value".to_string());

        let response = InterpretedResponse::Custom {
            status: 201,
            headers,
            body: Some(serde_json::json!({ "id": 42 })),
            raw_body: None,
        };

        assert!(!response.is_streaming());
        assert!(response.is_custom());
        assert!(!response.is_plain());
        assert_eq!(response.status(), 201);
        assert!(response.headers().is_some());
        assert_eq!(response.headers().unwrap().len(), 1);
    }

    #[test]
    fn test_interpreted_response_plain_construction() {
        let response = InterpretedResponse::Plain {
            body: serde_json::json!({ "message": "hello" }),
        };

        assert!(!response.is_streaming());
        assert!(!response.is_custom());
        assert!(response.is_plain());
        assert_eq!(response.status(), 200);
        assert_eq!(response.headers(), None);
    }

    #[test]
    fn test_interpreted_response_status_codes() {
        let codes = vec![200u16, 201, 202, 204, 400, 401, 403, 404, 500, 502, 503];

        for code in codes {
            let response = InterpretedResponse::Custom {
                status: code,
                headers: HashMap::new(),
                body: None,
                raw_body: None,
            };
            assert_eq!(response.status(), code);
        }
    }

    #[test]
    fn test_interpreted_response_headers_empty() {
        let response = InterpretedResponse::Custom {
            status: 200,
            headers: HashMap::new(),
            body: None,
            raw_body: None,
        };

        let headers = response.headers().unwrap();
        assert_eq!(headers.len(), 0);
    }

    #[test]
    fn test_interpreted_response_headers_multiple() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("cache-control".to_string(), "no-cache".to_string());
        headers.insert("x-custom".to_string(), "value".to_string());

        let response = InterpretedResponse::Custom {
            status: 200,
            headers,
            body: None,
            raw_body: None,
        };

        let resp_headers = response.headers().unwrap();
        assert_eq!(resp_headers.len(), 3);
        assert_eq!(resp_headers.get("content-type"), Some(&"application/json".to_string()));
        assert_eq!(resp_headers.get("cache-control"), Some(&"no-cache".to_string()));
    }

    #[test]
    fn test_interpreted_response_body_json() {
        let body = serde_json::json!({
            "id": 42,
            "name": "test",
            "nested": {
                "value": true
            }
        });

        let response = InterpretedResponse::Plain { body };

        // Verify response preserves the value
        match response {
            InterpretedResponse::Plain {
                body: ref returned_body,
            } => {
                assert_eq!(returned_body["id"], 42);
                assert_eq!(returned_body["name"], "test");
                assert_eq!(returned_body["nested"]["value"], true);
            }
            _ => panic!("Expected Plain response"),
        }
    }

    #[test]
    fn test_interpreted_response_raw_body_precedence() {
        let response = InterpretedResponse::Custom {
            status: 200,
            headers: HashMap::new(),
            body: Some(serde_json::json!({ "ignored": true })),
            raw_body: Some(b"raw bytes".to_vec()),
        };

        match response {
            InterpretedResponse::Custom {
                body,
                raw_body: Some(raw),
                ..
            } => {
                assert!(body.is_some()); // body is still present
                assert_eq!(raw, b"raw bytes");
            }
            _ => panic!("Expected Custom response with raw_body"),
        }
    }

    #[test]
    fn test_response_interpreter_streaming() {
        let interpreter = TestInterpreter;
        let result = interpreter.interpret(&TestValue::Stream).unwrap();

        assert!(result.is_streaming());
        assert_eq!(result.status(), 200);
    }

    #[test]
    fn test_response_interpreter_custom() {
        let interpreter = TestInterpreter;
        let result = interpreter.interpret(&TestValue::Custom).unwrap();

        assert!(result.is_custom());
        assert_eq!(result.status(), 201);
        assert_eq!(result.headers().unwrap().get("x-custom"), Some(&"header".to_string()));
    }

    #[test]
    fn test_response_interpreter_plain() {
        let interpreter = TestInterpreter;
        let result = interpreter.interpret(&TestValue::Plain).unwrap();

        assert!(result.is_plain());
        assert_eq!(result.status(), 200);
    }

    #[test]
    fn test_stream_source_multiple_chunks() {
        let chunks = vec![b"first".to_vec(), b"second".to_vec(), b"third".to_vec()];
        let mut stream = MockStreamSource::new(chunks);

        assert_eq!(stream.next_chunk(), Some(b"first".to_vec()));
        assert_eq!(stream.next_chunk(), Some(b"second".to_vec()));
        assert_eq!(stream.next_chunk(), Some(b"third".to_vec()));
        assert_eq!(stream.next_chunk(), None);
        assert_eq!(stream.next_chunk(), None); // Idempotent
    }

    #[test]
    fn test_stream_source_empty() {
        let mut stream = MockStreamSource::new(vec![]);
        assert_eq!(stream.next_chunk(), None);
    }

    #[test]
    fn test_stream_source_large_chunks() {
        let large_chunk = vec![0u8; 1024 * 1024]; // 1MB
        let mut stream = MockStreamSource::new(vec![large_chunk]);

        let retrieved = stream.next_chunk().unwrap();
        assert_eq!(retrieved.len(), 1024 * 1024);
        assert_eq!(stream.next_chunk(), None);
    }

    #[test]
    fn test_streaming_response_headers_empty() {
        let response = InterpretedResponse::Streaming {
            enumerator: Box::new(MockStreamSource::new(vec![])),
            status: 200,
            headers: HashMap::new(),
        };

        let headers = response.headers().unwrap();
        assert!(headers.is_empty());
    }

    #[test]
    fn test_streaming_response_headers_with_values() {
        let mut headers = HashMap::new();
        headers.insert("transfer-encoding".to_string(), "chunked".to_string());
        headers.insert("content-type".to_string(), "application/json".to_string());

        let response = InterpretedResponse::Streaming {
            enumerator: Box::new(MockStreamSource::new(vec![])),
            status: 200,
            headers,
        };

        let resp_headers = response.headers().unwrap();
        assert_eq!(resp_headers.len(), 2);
        assert_eq!(resp_headers.get("transfer-encoding"), Some(&"chunked".to_string()));
    }

    #[test]
    fn test_custom_response_no_body() {
        let response = InterpretedResponse::Custom {
            status: 204,
            headers: HashMap::new(),
            body: None,
            raw_body: None,
        };

        match response {
            InterpretedResponse::Custom { body: None, .. } => {
                // Expected: 204 No Content
            }
            _ => panic!("Expected Custom response with no body"),
        }
    }

    #[test]
    fn test_custom_response_json_body() {
        let body = serde_json::json!({
            "success": true,
            "data": [1, 2, 3]
        });

        let response = InterpretedResponse::Custom {
            status: 201,
            headers: HashMap::new(),
            body: Some(body),
            raw_body: None,
        };

        match response {
            InterpretedResponse::Custom {
                status: 201,
                body: Some(b),
                ..
            } => {
                assert_eq!(b["success"], true);
                assert_eq!(b["data"][0], 1);
            }
            _ => panic!("Expected Custom response with JSON body"),
        }
    }

    #[test]
    fn test_plain_response_various_json_types() {
        // Test with object
        let obj = InterpretedResponse::Plain {
            body: serde_json::json!({ "key": "value" }),
        };
        assert!(obj.is_plain());

        // Test with array
        let arr = InterpretedResponse::Plain {
            body: serde_json::json!([1, 2, 3]),
        };
        assert!(arr.is_plain());

        // Test with string
        let str_val = InterpretedResponse::Plain {
            body: serde_json::json!("hello"),
        };
        assert!(str_val.is_plain());

        // Test with number
        let num = InterpretedResponse::Plain {
            body: serde_json::json!(42),
        };
        assert!(num.is_plain());

        // Test with null
        let null_val = InterpretedResponse::Plain {
            body: serde_json::Value::Null,
        };
        assert!(null_val.is_plain());
    }

    #[test]
    fn test_stream_source_send_sync() {
        // Verify that StreamSource trait objects are Send + Sync
        let stream = MockStreamSource::new(vec![b"test".to_vec()]);
        let boxed: Box<dyn StreamSource> = Box::new(stream);

        // This compiles only if Box<dyn StreamSource> implements Send + Sync
        let _: Box<dyn StreamSource + Send + Sync> = boxed;
    }

    #[test]
    fn test_interpreted_response_debug() {
        let plain = InterpretedResponse::Plain {
            body: serde_json::json!({ "test": true }),
        };

        let debug_string = format!("{plain:?}");
        assert!(debug_string.contains("Plain"));
    }

    #[test]
    fn test_custom_response_with_all_fields() {
        let mut headers = HashMap::new();
        headers.insert("content-type".to_string(), "application/json".to_string());
        headers.insert("x-custom".to_string(), "header-value".to_string());

        let response = InterpretedResponse::Custom {
            status: 200,
            headers,
            body: Some(serde_json::json!({ "data": "value" })),
            raw_body: Some(b"fallback".to_vec()),
        };

        match response {
            InterpretedResponse::Custom {
                status: 200,
                headers: h,
                body: Some(_),
                raw_body: Some(_),
            } => {
                assert_eq!(h.len(), 2);
            }
            _ => panic!("Expected Custom response with all fields"),
        }
    }
}
