//! JSON-RPC 2.0 Protocol Types
//!
//! This module provides type definitions for the JSON-RPC 2.0 specification.
//! See [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification)
//!
//! # Overview
//!
//! JSON-RPC is a stateless, light-weight remote procedure call (RPC) protocol.
//! This module implements the complete specification including:
//!
//! - Request/Response messages
//! - Standard error codes
//! - Helper constructors for building valid messages
//! - Full serialization support via serde

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// JSON-RPC 2.0 Request
///
/// Represents a JSON-RPC request method invocation with optional parameters and identifier.
///
/// # Fields
///
/// * `jsonrpc` - A String specifying the JSON-RPC version. MUST be exactly "2.0"
/// * `method` - A String containing the name of the method to be invoked
/// * `params` - Optional structured data that serves as arguments to the method.
///   The order of the objects in the Array is significant to the method.
/// * `id` - A value which is used to match the response with the request that it is replying to.
///   Can be a string, number, or NULL. Notifications MUST NOT include an "id".
///
/// # Example
///
/// ```ignore
/// use serde_json::json;
/// use spikard_http::jsonrpc::JsonRpcRequest;
///
/// // Request with parameters and ID
/// let req = JsonRpcRequest::new("add", Some(json!([1, 2])), Some(json!(1)));
/// assert!(!req.is_notification());
///
/// // Notification (no ID)
/// let notif = JsonRpcRequest::new("notify", Some(json!({})), None);
/// assert!(notif.is_notification());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcRequest {
    /// JSON-RPC version, must be "2.0"
    pub jsonrpc: String,

    /// The name of the method to invoke
    pub method: String,

    /// Optional parameters for the method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,

    /// Optional request identifier. When absent, this is a notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Value>,
}

impl JsonRpcRequest {
    /// Creates a new JSON-RPC 2.0 request
    ///
    /// # Arguments
    ///
    /// * `method` - The method name to invoke
    /// * `params` - Optional parameters (can be array, object, or null)
    /// * `id` - Optional request identifier (string, number, or null)
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::jsonrpc::JsonRpcRequest;
    ///
    /// let req = JsonRpcRequest::new("subtract", Some(json!({"a": 5, "b": 3})), Some(json!(2)));
    /// assert_eq!(req.method, "subtract");
    /// assert!(!req.is_notification());
    /// ```
    pub fn new(method: impl Into<String>, params: Option<Value>, id: Option<Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
            id,
        }
    }

    /// Checks if this request is a notification
    ///
    /// A notification is a JSON-RPC request without an "id" field.
    /// The server MUST NOT reply to a notification.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use spikard_http::jsonrpc::JsonRpcRequest;
    ///
    /// let req = JsonRpcRequest::new("method", None, Some(serde_json::json!(1)));
    /// assert!(!req.is_notification());
    ///
    /// let notif = JsonRpcRequest::new("notify", None, None);
    /// assert!(notif.is_notification());
    /// ```
    pub fn is_notification(&self) -> bool {
        self.id.is_none()
    }
}

/// JSON-RPC 2.0 Success Response
///
/// Represents a successful JSON-RPC response containing the result of the method invocation.
///
/// # Fields
///
/// * `jsonrpc` - A String specifying the JSON-RPC version. MUST be exactly "2.0"
/// * `result` - The result of the method invocation. This MUST be null in case of an error.
/// * `id` - This MUST be the same id as the request it is responding to
///
/// # Example
///
/// ```ignore
/// use serde_json::json;
/// use spikard_http::jsonrpc::JsonRpcResponse;
///
/// let response = JsonRpcResponse::success(json!(19), json!(1));
/// assert_eq!(response.jsonrpc, "2.0");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcResponse {
    /// JSON-RPC version, must be "2.0"
    pub jsonrpc: String,

    /// The result of the method invocation
    pub result: Value,

    /// The request identifier this response corresponds to
    pub id: Value,
}

impl JsonRpcResponse {
    /// Creates a new JSON-RPC 2.0 success response
    ///
    /// # Arguments
    ///
    /// * `result` - The result value from the method invocation
    /// * `id` - The request identifier from the original request
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::jsonrpc::JsonRpcResponse;
    ///
    /// let response = JsonRpcResponse::success(json!({"sum": 7}), json!("abc"));
    /// assert_eq!(response.jsonrpc, "2.0");
    /// ```
    pub fn success(result: Value, id: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            result,
            id,
        }
    }
}

/// JSON-RPC 2.0 Error Object
///
/// Represents a JSON-RPC error that occurred during method invocation.
///
/// # Fields
///
/// * `code` - A Number that indicates the error type that occurred
/// * `message` - A String providing a short description of the error
/// * `data` - Optional additional error information
///
/// # Standard Error Codes
///
/// - `-32700`: Parse error
/// - `-32600`: Invalid Request
/// - `-32601`: Method not found
/// - `-32602`: Invalid params
/// - `-32603`: Internal error
/// - `-32000 to -32099`: Server error (reserved)
///
/// # Example
///
/// ```ignore
/// use spikard_http::jsonrpc::{JsonRpcError, error_codes};
///
/// let err = JsonRpcError {
///     code: error_codes::INVALID_PARAMS,
///     message: "Invalid method parameters".to_string(),
///     data: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcError {
    /// Numeric error code
    pub code: i32,

    /// Human-readable error description
    pub message: String,

    /// Optional additional error context
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
}

/// JSON-RPC 2.0 Error Response
///
/// Represents a JSON-RPC response containing an error result.
///
/// # Fields
///
/// * `jsonrpc` - A String specifying the JSON-RPC version. MUST be exactly "2.0"
/// * `error` - An Error Object with error information
/// * `id` - This MUST be the same id as the request it is responding to
///
/// # Example
///
/// ```ignore
/// use serde_json::json;
/// use spikard_http::jsonrpc::{JsonRpcErrorResponse, error_codes};
///
/// let err_response = JsonRpcErrorResponse::error(
///     error_codes::METHOD_NOT_FOUND,
///     "Method not found",
///     json!(1)
/// );
/// assert_eq!(err_response.jsonrpc, "2.0");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcErrorResponse {
    /// JSON-RPC version, must be "2.0"
    pub jsonrpc: String,

    /// Error object containing error information
    pub error: JsonRpcError,

    /// The request identifier this response corresponds to
    pub id: Value,
}

impl JsonRpcErrorResponse {
    /// Creates a new JSON-RPC 2.0 error response
    ///
    /// # Arguments
    ///
    /// * `code` - The numeric error code
    /// * `message` - The error message
    /// * `id` - The request identifier from the original request
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::jsonrpc::{JsonRpcErrorResponse, error_codes};
    ///
    /// let response = JsonRpcErrorResponse::error(
    ///     error_codes::METHOD_NOT_FOUND,
    ///     "Unknown method",
    ///     json!(null)
    /// );
    /// assert_eq!(response.jsonrpc, "2.0");
    /// ```
    pub fn error(code: i32, message: impl Into<String>, id: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            error: JsonRpcError {
                code,
                message: message.into(),
                data: None,
            },
            id,
        }
    }

    /// Creates a new JSON-RPC 2.0 error response with additional error data
    ///
    /// # Arguments
    ///
    /// * `code` - The numeric error code
    /// * `message` - The error message
    /// * `data` - Additional context about the error
    /// * `id` - The request identifier from the original request
    ///
    /// # Example
    ///
    /// ```ignore
    /// use serde_json::json;
    /// use spikard_http::jsonrpc::{JsonRpcErrorResponse, error_codes};
    ///
    /// let response = JsonRpcErrorResponse::error_with_data(
    ///     error_codes::INVALID_PARAMS,
    ///     "Invalid method parameters",
    ///     json!({"reason": "Missing required field 'name'"}),
    ///     json!(1)
    /// );
    /// assert_eq!(response.jsonrpc, "2.0");
    /// assert!(response.error.data.is_some());
    /// ```
    pub fn error_with_data(code: i32, message: impl Into<String>, data: Value, id: Value) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            error: JsonRpcError {
                code,
                message: message.into(),
                data: Some(data),
            },
            id,
        }
    }
}

/// JSON-RPC 2.0 Response Type
///
/// An enum that represents either a successful response or an error response.
/// This is useful for untagged deserialization and handling both response types uniformly.
///
/// # Variants
///
/// * `Success(JsonRpcResponse)` - A successful response with a result
/// * `Error(JsonRpcErrorResponse)` - An error response with error details
///
/// # Example
///
/// ```ignore
/// use serde_json::json;
/// use spikard_http::jsonrpc::{JsonRpcResponseType, JsonRpcResponse, JsonRpcErrorResponse, error_codes};
///
/// let success = JsonRpcResponseType::Success(
///     JsonRpcResponse::success(json!(42), json!(1))
/// );
///
/// let error = JsonRpcResponseType::Error(
///     JsonRpcErrorResponse::error(error_codes::INVALID_PARAMS, "Bad params", json!(1))
/// );
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JsonRpcResponseType {
    /// Successful response containing a result
    Success(JsonRpcResponse),

    /// Error response containing error details
    Error(JsonRpcErrorResponse),
}

/// JSON-RPC 2.0 Standard Error Codes
///
/// This module contains the standard error codes defined by the JSON-RPC 2.0 specification.
/// The error codes from -32768 to -32000 are reserved for JSON-RPC specification use.
pub mod error_codes {
    /// Parse error
    ///
    /// Invalid JSON was received by the server.
    /// An error occurred on the server while parsing the JSON text.
    pub const PARSE_ERROR: i32 = -32700;

    /// Invalid Request
    ///
    /// The JSON sent is not a valid Request object.
    pub const INVALID_REQUEST: i32 = -32600;

    /// Method not found
    ///
    /// The method does not exist / is not available.
    pub const METHOD_NOT_FOUND: i32 = -32601;

    /// Invalid params
    ///
    /// Invalid method parameter(s).
    pub const INVALID_PARAMS: i32 = -32602;

    /// Internal error
    ///
    /// Internal JSON-RPC error.
    pub const INTERNAL_ERROR: i32 = -32603;

    /// Server error (base)
    ///
    /// Server errors are reserved for implementation-defined server-errors.
    /// The error codes from -32099 to -32000 are reserved for server error codes.
    pub const SERVER_ERROR_BASE: i32 = -32000;

    /// Server error (end of reserved range)
    pub const SERVER_ERROR_END: i32 = -32099;

    /// Helper function to check if a code is a reserved server error code
    pub fn is_server_error(code: i32) -> bool {
        code >= SERVER_ERROR_END && code <= SERVER_ERROR_BASE
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_jsonrpc_request_creation() {
        let req = JsonRpcRequest::new("method", Some(json!({"key": "value"})), Some(json!(1)));
        assert_eq!(req.jsonrpc, "2.0");
        assert_eq!(req.method, "method");
        assert!(!req.is_notification());
    }

    #[test]
    fn test_jsonrpc_notification() {
        let notif = JsonRpcRequest::new("notify", None, None);
        assert!(notif.is_notification());
    }

    #[test]
    fn test_jsonrpc_response_success() {
        let response = JsonRpcResponse::success(json!(42), json!(1));
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.result, json!(42));
        assert_eq!(response.id, json!(1));
    }

    #[test]
    fn test_jsonrpc_error_response() {
        let err = JsonRpcErrorResponse::error(error_codes::METHOD_NOT_FOUND, "Method not found", json!(1));
        assert_eq!(err.jsonrpc, "2.0");
        assert_eq!(err.error.code, error_codes::METHOD_NOT_FOUND);
        assert_eq!(err.error.message, "Method not found");
        assert!(err.error.data.is_none());
    }

    #[test]
    fn test_jsonrpc_error_response_with_data() {
        let data = json!({"reason": "Missing parameter"});
        let err = JsonRpcErrorResponse::error_with_data(
            error_codes::INVALID_PARAMS,
            "Invalid parameters",
            data.clone(),
            json!(null),
        );
        assert_eq!(err.error.code, error_codes::INVALID_PARAMS);
        assert_eq!(err.error.data, Some(data));
    }

    #[test]
    fn test_error_codes_constants() {
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);
    }

    #[test]
    fn test_is_server_error() {
        assert!(error_codes::is_server_error(-32000));
        assert!(error_codes::is_server_error(-32050));
        assert!(error_codes::is_server_error(-32099));
        assert!(!error_codes::is_server_error(-32700));
        assert!(!error_codes::is_server_error(0));
    }

    #[test]
    fn test_request_serialization() {
        let req = JsonRpcRequest::new("test", Some(json!([1, 2])), Some(json!(1)));
        let json = serde_json::to_value(&req).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["method"], "test");
        assert!(json["params"].is_array());
        assert_eq!(json["id"], 1);
    }

    #[test]
    fn test_notification_serialization() {
        let notif = JsonRpcRequest::new("notify", Some(json!({})), None);
        let json = serde_json::to_value(&notif).unwrap();
        assert!(!json.get("id").is_some() || json["id"].is_null());
    }

    #[test]
    fn test_response_serialization() {
        let resp = JsonRpcResponse::success(json!({"result": 100}), json!("string-id"));
        let json = serde_json::to_value(&resp).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["id"], "string-id");
    }

    #[test]
    fn test_error_response_serialization() {
        let err = JsonRpcErrorResponse::error(error_codes::PARSE_ERROR, "Parse error", json!(null));
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["jsonrpc"], "2.0");
        assert_eq!(json["error"]["code"], -32700);
        assert_eq!(json["error"]["message"], "Parse error");
    }

    #[test]
    fn test_response_type_enum() {
        let success_resp = JsonRpcResponseType::Success(JsonRpcResponse::success(json!(1), json!(1)));
        let error_resp = JsonRpcResponseType::Error(JsonRpcErrorResponse::error(
            error_codes::INVALID_REQUEST,
            "Invalid",
            json!(1),
        ));

        // Verify we can serialize both variants
        let _success_json = serde_json::to_value(&success_resp).unwrap();
        let _error_json = serde_json::to_value(&error_resp).unwrap();
    }
}
