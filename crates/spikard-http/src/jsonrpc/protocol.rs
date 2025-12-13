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

/// Maximum allowed length for a JSON-RPC method name
const MAX_METHOD_NAME_LENGTH: usize = 255;

/// Validates a JSON-RPC method name for security and correctness
///
/// This function prevents DoS attacks through method name validation by enforcing:
/// - Maximum length of 255 characters to prevent resource exhaustion
/// - Only allowed characters: alphanumeric (a-z, A-Z, 0-9), dot (.), underscore (_), hyphen (-)
/// - No control characters (0x00-0x1F, 0x7F) to prevent injection attacks
/// - No leading or trailing whitespace to ensure proper formatting
///
/// # Arguments
///
/// * `method_name` - The method name to validate
///
/// # Returns
///
/// * `Ok(())` - If the method name is valid
/// * `Err(String)` - If the method name is invalid with a descriptive error message
///
/// # Example
///
/// ```ignore
/// use spikard_http::jsonrpc::validate_method_name;
///
/// // Valid method names
/// assert!(validate_method_name("user.getById").is_ok());
/// assert!(validate_method_name("calculate_sum").is_ok());
/// assert!(validate_method_name("api-v1-handler").is_ok());
/// assert!(validate_method_name("rpc1").is_ok());
///
/// // Invalid method names
/// assert!(validate_method_name("").is_err());  // Empty
/// assert!(validate_method_name(" method").is_err());  // Leading whitespace
/// assert!(validate_method_name("method ").is_err());  // Trailing whitespace
/// assert!(validate_method_name("method\x00name").is_err());  // Control character
/// assert!(validate_method_name("a".repeat(256)).is_err());  // Too long
/// ```
pub fn validate_method_name(method_name: &str) -> Result<(), String> {
    if method_name.is_empty() {
        return Err("Method name cannot be empty".to_string());
    }

    if method_name.starts_with(char::is_whitespace) || method_name.ends_with(char::is_whitespace) {
        return Err("Method name cannot have leading or trailing whitespace".to_string());
    }

    if method_name.len() > MAX_METHOD_NAME_LENGTH {
        return Err(format!(
            "Method name exceeds maximum length of {} characters (got {})",
            MAX_METHOD_NAME_LENGTH,
            method_name.len()
        ));
    }

    for ch in method_name.chars() {
        match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' => {}
            '.' => {}
            '_' => {}
            '-' => {}
            c if (c as u32) < 0x20 || (c as u32) == 0x7F => {
                return Err(format!(
                    "Method name contains invalid control character: 0x{:02X}",
                    c as u32
                ));
            }
            c => {
                return Err(format!(
                    "Method name contains invalid character: '{}' (0x{:02X}). \
                     Only alphanumeric, dot (.), underscore (_), and hyphen (-) are allowed",
                    c, c as u32
                ));
            }
        }
    }

    Ok(())
}

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
        (SERVER_ERROR_END..=SERVER_ERROR_BASE).contains(&code)
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

        let _success_json = serde_json::to_value(&success_resp).unwrap();
        let _error_json = serde_json::to_value(&error_resp).unwrap();
    }

    #[test]
    fn test_validate_method_name_valid_simple() {
        assert!(validate_method_name("test").is_ok());
        assert!(validate_method_name("method").is_ok());
        assert!(validate_method_name("rpc").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_with_dot() {
        assert!(validate_method_name("user.get").is_ok());
        assert!(validate_method_name("api.v1.endpoint").is_ok());
        assert!(validate_method_name("service.method.action").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_with_underscore() {
        assert!(validate_method_name("get_user").is_ok());
        assert!(validate_method_name("_private_method").is_ok());
        assert!(validate_method_name("method_v1").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_with_hyphen() {
        assert!(validate_method_name("get-user").is_ok());
        assert!(validate_method_name("api-v1").is_ok());
        assert!(validate_method_name("my-method-name").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_with_numbers() {
        assert!(validate_method_name("method1").is_ok());
        assert!(validate_method_name("v2.endpoint").is_ok());
        assert!(validate_method_name("rpc123abc").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_mixed() {
        assert!(validate_method_name("user.get_by_id").is_ok());
        assert!(validate_method_name("api-v1.service_name").is_ok());
        assert!(validate_method_name("Service_v1_2_3").is_ok());
    }

    #[test]
    fn test_validate_method_name_valid_max_length() {
        let max_name = "a".repeat(255);
        assert!(validate_method_name(&max_name).is_ok());
    }

    #[test]
    fn test_validate_method_name_empty() {
        let result = validate_method_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_method_name_leading_space() {
        let result = validate_method_name(" method");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("leading or trailing whitespace"));
    }

    #[test]
    fn test_validate_method_name_trailing_space() {
        let result = validate_method_name("method ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("leading or trailing whitespace"));
    }

    #[test]
    fn test_validate_method_name_leading_and_trailing_space() {
        let result = validate_method_name(" method ");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("leading or trailing whitespace"));
    }

    #[test]
    fn test_validate_method_name_internal_space() {
        let result = validate_method_name("method name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_too_long() {
        let too_long_name = "a".repeat(256);
        let result = validate_method_name(&too_long_name);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_method_name_null_byte() {
        let result = validate_method_name("method\x00name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_newline() {
        let result = validate_method_name("method\nname");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_carriage_return() {
        let result = validate_method_name("method\rname");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_tab() {
        let result = validate_method_name("method\tname");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_delete_char() {
        let result = validate_method_name("method\x7fname");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_special_char_at_sign() {
        let result = validate_method_name("method@name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_special_char_hash() {
        let result = validate_method_name("method#name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_special_char_percent() {
        let result = validate_method_name("method%name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_special_char_slash() {
        let result = validate_method_name("method/name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_special_char_backslash() {
        let result = validate_method_name("method\\name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_special_char_quote() {
        let result = validate_method_name("method\"name");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("invalid character"));
    }

    #[test]
    fn test_validate_method_name_dos_attack_very_long() {
        let very_long = "a".repeat(10000);
        let result = validate_method_name(&very_long);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("exceeds maximum length"));
    }

    #[test]
    fn test_validate_method_name_dos_attack_control_chars() {
        let result = validate_method_name("method\x00\x00\x00\x00");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("control character"));
    }

    #[test]
    fn test_validate_method_name_edge_case_single_char() {
        assert!(validate_method_name("a").is_ok());
        assert!(validate_method_name("_").is_ok());
        assert!(validate_method_name("-").is_ok());
        assert!(validate_method_name(".").is_ok());
    }

    #[test]
    fn test_request_with_null_id_is_notification() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "notify",
            "params": []
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.is_notification());
        assert_eq!(request.method, "notify");
    }

    #[test]
    fn test_request_with_string_id_preserved() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "add",
            "params": [1, 2],
            "id": "abc-123"
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(!request.is_notification());
        assert_eq!(request.id, Some(json!("abc-123")));
    }

    #[test]
    fn test_request_with_zero_id_valid() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "test",
            "id": 0
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(!request.is_notification());
        assert_eq!(request.id, Some(json!(0)));
    }

    #[test]
    fn test_request_with_negative_id_valid() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "test",
            "id": -999
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(!request.is_notification());
        assert_eq!(request.id, Some(json!(-999)));
    }

    #[test]
    fn test_request_without_jsonrpc_field_invalid() {
        let json = json!({
            "method": "test",
            "id": 1
        });

        let result: serde_json::Result<JsonRpcRequest> = serde_json::from_value(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_response_preserves_id_type_numeric() {
        let response = JsonRpcResponse::success(json!(42), json!(999));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["id"], 999);
        assert!(serialized["id"].is_number());
    }

    #[test]
    fn test_error_response_never_has_result_field() {
        let err = JsonRpcErrorResponse::error(error_codes::INVALID_PARAMS, "Bad params", json!(1));
        let serialized = serde_json::to_value(&err).unwrap();

        assert!(serialized.get("result").is_none());
        assert!(serialized.get("error").is_some());
    }

    #[test]
    fn test_success_response_never_has_error_field() {
        let success = JsonRpcResponse::success(json!({"data": 123}), json!(1));
        let serialized = serde_json::to_value(&success).unwrap();

        assert!(serialized.get("error").is_none());
        assert!(serialized.get("result").is_some());
    }

    #[test]
    fn test_params_array_type() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "sum",
            "params": [1, 2, 3, 4, 5],
            "id": 1
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.params.is_some());
        let params = request.params.unwrap();
        assert!(params.is_array());
        let arr = params.as_array().unwrap();
        assert_eq!(arr.len(), 5);
        assert_eq!(arr[0], json!(1));
    }

    #[test]
    fn test_params_object_type() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "subtract",
            "params": {"a": 5, "b": 3},
            "id": 2
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.params.is_some());
        let params = request.params.unwrap();
        assert!(params.is_object());
        let obj = params.as_object().unwrap();
        assert_eq!(obj.get("a"), Some(&json!(5)));
        assert_eq!(obj.get("b"), Some(&json!(3)));
    }

    #[test]
    fn test_params_null_type() {
        let json_no_params = json!({
            "jsonrpc": "2.0",
            "method": "test",
            "id": 3
        });

        let request: JsonRpcRequest = serde_json::from_value(json_no_params).unwrap();
        assert!(request.params.is_none());
    }

    #[test]
    fn test_params_primitive_string() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "echo",
            "params": "hello world",
            "id": 4
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.params.is_some());
        assert_eq!(request.params.unwrap(), json!("hello world"));
    }

    #[test]
    fn test_params_primitive_number() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "increment",
            "params": 42,
            "id": 5
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.params.is_some());
        assert_eq!(request.params.unwrap(), json!(42));
    }

    #[test]
    fn test_params_deeply_nested() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "process",
            "params": {
                "level1": {
                    "level2": {
                        "level3": {
                            "level4": {
                                "level5": {
                                    "level6": {
                                        "level7": {
                                            "level8": {
                                                "level9": {
                                                    "level10": "deep value"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "id": 6
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert!(request.params.is_some());
        let deep = request.params.unwrap();
        assert!(
            deep["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"]["level10"]
                .is_string()
        );
        assert_eq!(
            deep["level1"]["level2"]["level3"]["level4"]["level5"]["level6"]["level7"]["level8"]["level9"]["level10"],
            "deep value"
        );
    }

    #[test]
    fn test_params_unicode_strings() {
        let json = json!({
            "jsonrpc": "2.0",
            "method": "translate",
            "params": {
                "emoji": "Hello 👋 World 🌍",
                "rtl": "שלום עולם",
                "cjk": "你好世界",
                "special": "café ñ ü"
            },
            "id": 7
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        let params = request.params.unwrap();
        assert_eq!(params["emoji"], "Hello 👋 World 🌍");
        assert_eq!(params["rtl"], "שלום עולם");
        assert_eq!(params["cjk"], "你好世界");
        assert_eq!(params["special"], "café ñ ü");
    }

    #[test]
    fn test_response_result_with_null_valid() {
        let response = JsonRpcResponse::success(json!(null), json!(1));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["id"], 1);
        assert_eq!(serialized["result"], json!(null));
        assert!(serialized.get("error").is_none());
    }

    #[test]
    fn test_response_result_with_false_valid() {
        let response = JsonRpcResponse::success(json!(false), json!(2));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["id"], 2);
        assert_eq!(serialized["result"], json!(false));
        assert!(serialized.get("error").is_none());
    }

    #[test]
    fn test_response_result_with_zero_valid() {
        let response = JsonRpcResponse::success(json!(0), json!(3));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["id"], 3);
        assert_eq!(serialized["result"], json!(0));
        assert!(serialized.get("error").is_none());
    }

    #[test]
    fn test_response_result_with_empty_object_valid() {
        let response = JsonRpcResponse::success(json!({}), json!(4));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["id"], 4);
        assert_eq!(serialized["result"], json!({}));
        assert!(serialized.get("error").is_none());
    }

    #[test]
    fn test_response_result_with_empty_array_valid() {
        let response = JsonRpcResponse::success(json!([]), json!(5));
        let serialized = serde_json::to_value(&response).unwrap();

        assert_eq!(serialized["jsonrpc"], "2.0");
        assert_eq!(serialized["id"], 5);
        assert_eq!(serialized["result"], json!([]));
        assert!(serialized.get("error").is_none());
    }

    #[test]
    fn test_error_code_parse_error() {
        let err = JsonRpcErrorResponse::error(error_codes::PARSE_ERROR, "Parse error", json!(null));
        let serialized = serde_json::to_value(&err).unwrap();

        assert_eq!(serialized["error"]["code"], -32700);
        assert!(serialized.get("result").is_none());
    }

    #[test]
    fn test_error_code_roundtrip() {
        let codes = vec![
            error_codes::PARSE_ERROR,
            error_codes::INVALID_REQUEST,
            error_codes::METHOD_NOT_FOUND,
            error_codes::INVALID_PARAMS,
            error_codes::INTERNAL_ERROR,
        ];

        for code in codes {
            let err = JsonRpcErrorResponse::error(code, "Test error", json!(1));
            let serialized = serde_json::to_value(&err).unwrap();
            let deserialized: JsonRpcErrorResponse = serde_json::from_value(serialized).unwrap();

            assert_eq!(deserialized.error.code, code);
        }
    }

    #[test]
    fn test_notification_has_no_id_field() {
        let notif = JsonRpcRequest::new("notify", None, None);
        let serialized = serde_json::to_value(&notif).unwrap();

        assert!(serialized.get("id").is_none());
    }

    #[test]
    fn test_id_preservation_in_batch() {
        let json_batch = json!([
            {
                "jsonrpc": "2.0",
                "method": "method1",
                "id": "string-id"
            },
            {
                "jsonrpc": "2.0",
                "method": "method2",
                "id": 42
            },
            {
                "jsonrpc": "2.0",
                "method": "method3"
            }
        ]);

        let batch: Vec<JsonRpcRequest> = serde_json::from_value(json_batch).unwrap();

        assert_eq!(batch.len(), 3);
        assert_eq!(batch[0].id, Some(json!("string-id")));
        assert_eq!(batch[1].id, Some(json!(42)));
        assert_eq!(batch[2].id, None);
    }

    #[test]
    fn test_mixed_id_types_in_batch() {
        let responses = vec![
            JsonRpcResponse::success(json!(100), json!("id1")),
            JsonRpcResponse::success(json!(200), json!(2)),
            JsonRpcResponse::success(json!(300), json!(null)),
        ];

        for resp in responses {
            let serialized = serde_json::to_value(&resp).unwrap();
            let deserialized: JsonRpcResponse = serde_json::from_value(serialized).unwrap();
            assert_eq!(deserialized.jsonrpc, "2.0");
        }
    }

    #[test]
    fn test_large_numeric_id() {
        let large_id = i64::MAX;
        let json = json!({
            "jsonrpc": "2.0",
            "method": "test",
            "id": large_id
        });

        let request: JsonRpcRequest = serde_json::from_value(json).unwrap();

        assert_eq!(request.id, Some(json!(large_id)));
    }

    #[test]
    fn test_error_always_has_code() {
        let err = JsonRpcErrorResponse::error(error_codes::METHOD_NOT_FOUND, "Not found", json!(1));
        let serialized = serde_json::to_value(&err).unwrap();

        assert!(serialized["error"].get("code").is_some());
        assert_eq!(serialized["error"]["code"], -32601);
    }

    #[test]
    fn test_error_always_has_message() {
        let err = JsonRpcErrorResponse::error(error_codes::INVALID_PARAMS, "Invalid parameters", json!(2));
        let serialized = serde_json::to_value(&err).unwrap();

        assert!(serialized["error"].get("message").is_some());
        assert_eq!(serialized["error"]["message"], "Invalid parameters");
    }

    #[test]
    fn test_error_data_optional() {
        let err_without_data = JsonRpcErrorResponse::error(error_codes::INTERNAL_ERROR, "Internal error", json!(3));
        let serialized_without = serde_json::to_value(&err_without_data).unwrap();

        assert!(serialized_without["error"].get("data").is_none());

        let err_with_data = JsonRpcErrorResponse::error_with_data(
            error_codes::INTERNAL_ERROR,
            "Internal error",
            json!({"details": "something went wrong"}),
            json!(4),
        );
        let serialized_with = serde_json::to_value(&err_with_data).unwrap();

        assert!(serialized_with["error"].get("data").is_some());
    }
}
