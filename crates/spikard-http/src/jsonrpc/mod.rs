//! JSON-RPC protocol support for Spikard HTTP
//!
//! This module provides JSON-RPC 2.0 protocol support including method registration,
//! handler lookup, and metadata management.

pub mod http_handler;
pub mod method_registry;
pub mod protocol;
pub mod router;

use serde::{Deserialize, Serialize};

pub use http_handler::{JsonRpcState, handle_jsonrpc};
pub use method_registry::{JsonRpcMethodRegistry, MethodExample, MethodMetadata};
pub use protocol::{
    JsonRpcErrorResponse, JsonRpcRequest, JsonRpcResponse, JsonRpcResponseType, error_codes, validate_method_name,
};
pub use router::{JsonRpcRequestOrBatch, JsonRpcRouter};

/// JSON-RPC server configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonRpcConfig {
    /// Enable JSON-RPC endpoint
    #[serde(default)]
    pub enabled: bool,
    /// HTTP endpoint path for JSON-RPC requests (default: "/rpc")
    #[serde(default = "default_endpoint_path")]
    pub endpoint_path: String,
    /// Enable batch request processing (default: true)
    #[serde(default = "default_true")]
    pub enable_batch: bool,
    /// Maximum number of requests in a batch (default: 100)
    #[serde(default = "default_max_batch_size")]
    pub max_batch_size: usize,
}

fn default_endpoint_path() -> String {
    "/rpc".to_string()
}

fn default_true() -> bool {
    true
}

fn default_max_batch_size() -> usize {
    100
}

impl Default for JsonRpcConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            endpoint_path: default_endpoint_path(),
            enable_batch: default_true(),
            max_batch_size: default_max_batch_size(),
        }
    }
}
