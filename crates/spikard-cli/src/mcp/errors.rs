//! MCP error conversion helpers.

use anyhow::Error;
use rmcp::ErrorData;

/// Convert application-layer errors into MCP internal errors.
#[doc(hidden)]
pub fn map_app_error_to_mcp(error: Error) -> ErrorData {
    ErrorData::internal_error(error.to_string(), None)
}
