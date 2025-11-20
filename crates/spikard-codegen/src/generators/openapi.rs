//! OpenAPI specification generator

use crate::error::Result;
use crate::ir::IntermediateRepresentation;
use serde_json::json;

/// Generate OpenAPI 3.1 specification
pub fn generate(_ir: &IntermediateRepresentation) -> Result<serde_json::Value> {
    // TODO: Implement OpenAPI generation

    Ok(json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Generated API",
            "version": "1.0.0"
        },
        "paths": {}
    }))
}
