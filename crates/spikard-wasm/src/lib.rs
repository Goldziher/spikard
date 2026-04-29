//! Wasm-safe surface of spikard.
//!
//! Spikard's server runtime depends on `axum`/`tokio`/`mio` and cannot compile
//! to `wasm32-unknown-unknown`. This crate exposes a deliberately narrow,
//! transport-free subset — JSON Schema validation and RFC 7807 problem-details
//! construction — that runs in browsers and edge runtimes (Cloudflare Workers,
//! Deno, etc.) via `wasm-bindgen`.
//!
//! All exports return JSON strings rather than complex object handles so the
//! ABI stays stable across `wasm-bindgen` versions and integrates cleanly with
//! JavaScript / TypeScript callers.

#![allow(clippy::missing_errors_doc, clippy::module_name_repetitions)]

use spikard_core::{ProblemDetails, SchemaValidator};
use wasm_bindgen::prelude::*;

/// Validate `data_json` against `schema_json` (both JSON-encoded).
///
/// Returns `null` on success, or a JSON-encoded `ValidationError` on failure.
/// Returns a JS error if either argument is not valid JSON or the schema is malformed.
#[wasm_bindgen(js_name = validateJsonSchema)]
pub fn validate_json_schema(schema_json: &str, data_json: &str) -> Result<JsValue, JsError> {
    let schema: serde_json::Value =
        serde_json::from_str(schema_json).map_err(|e| JsError::new(&format!("invalid schema JSON: {e}")))?;
    let data: serde_json::Value =
        serde_json::from_str(data_json).map_err(|e| JsError::new(&format!("invalid data JSON: {e}")))?;
    let validator = SchemaValidator::new(schema).map_err(|e| JsError::new(&format!("schema build failed: {e}")))?;
    match validator.validate(&data) {
        Ok(()) => Ok(JsValue::NULL),
        Err(err) => {
            let payload =
                serde_json::to_string(&err).map_err(|e| JsError::new(&format!("error serialisation failed: {e}")))?;
            Ok(JsValue::from_str(&payload))
        }
    }
}

/// RFC 7807 `application/problem+json` builder. Returns the serialized JSON.
#[wasm_bindgen(js_name = problemDetails)]
pub fn problem_details(type_uri: &str, title: &str, status: u16, detail: Option<String>) -> Result<String, JsError> {
    let status = http::StatusCode::from_u16(status).map_err(|e| JsError::new(&format!("invalid status code: {e}")))?;
    let mut p = ProblemDetails::new(type_uri.to_owned(), title.to_owned(), status);
    if let Some(d) = detail {
        p = p.with_detail(d);
    }
    p.to_json()
        .map_err(|e| JsError::new(&format!("problem details serialisation failed: {e}")))
}

/// Convenience: 404 Not Found problem details with the given detail string.
#[wasm_bindgen(js_name = problemNotFound)]
pub fn problem_not_found(detail: &str) -> Result<String, JsError> {
    ProblemDetails::not_found(detail.to_owned())
        .to_json()
        .map_err(|e| JsError::new(&format!("serialisation failed: {e}")))
}

/// Convenience: 400 Bad Request problem details.
#[wasm_bindgen(js_name = problemBadRequest)]
pub fn problem_bad_request(detail: &str) -> Result<String, JsError> {
    ProblemDetails::bad_request(detail.to_owned())
        .to_json()
        .map_err(|e| JsError::new(&format!("serialisation failed: {e}")))
}

/// Convenience: 500 Internal Server Error problem details.
#[wasm_bindgen(js_name = problemInternalServerError)]
pub fn problem_internal_server_error(detail: &str) -> Result<String, JsError> {
    ProblemDetails::internal_server_error(detail.to_owned())
        .to_json()
        .map_err(|e| JsError::new(&format!("serialisation failed: {e}")))
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use spikard_core::SchemaValidator;

    #[test]
    fn schema_validator_runs_without_di_feature() {
        let schema: serde_json::Value = serde_json::json!({
            "type": "object",
            "properties": { "id": { "type": "integer" } },
            "required": ["id"],
        });
        let validator = SchemaValidator::new(schema).unwrap();
        assert!(validator.validate(&serde_json::json!({ "id": 1 })).is_ok());
        assert!(validator.validate(&serde_json::json!({})).is_err());
    }

    #[test]
    fn problem_details_serialises_to_json() {
        let p = spikard_core::ProblemDetails::not_found("missing".to_owned());
        let json = p.to_json().unwrap();
        assert!(json.contains("not-found") || json.contains("Not Found"));
    }
}
