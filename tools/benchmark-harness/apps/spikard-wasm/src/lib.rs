//! Spikard WASM benchmark component.
//!
//! Implements standard benchmark endpoints as a WASIp3 HTTP component.

wasip3::http::proxy::export!(BenchComponent);

struct BenchComponent;

impl wasip3::exports::http::handler::Guest for BenchComponent {
    async fn handle(
        request: wasip3::http::types::IncomingRequest,
    ) -> Result<wasip3::http::types::Response, wasip3::http::types::ErrorCode> {
        let (parts, body) = wasip3::http::compat::into_request(request)
            .map_err(|_| wasip3::http::types::ErrorCode::InternalError(Some("conversion failed".into())))?
            .into_parts();

        let path = parts.uri.path();
        let method = parts.method.as_str();

        match (method, path) {
            ("GET", "/json/small") => json_response(serde_json::json!({"message": "hello"})),
            ("GET", "/json/medium") => {
                let items: Vec<_> = (0..100)
                    .map(|i| serde_json::json!({"id": i, "name": format!("item_{i}"), "active": i % 2 == 0}))
                    .collect();
                json_response(serde_json::json!({"items": items, "total": 100}))
            }
            ("GET", "/json/large") => {
                let items: Vec<_> = (0..1000)
                    .map(|i| serde_json::json!({"id": i, "name": format!("item_{i}"), "value": i * 42}))
                    .collect();
                json_response(serde_json::json!({"items": items, "total": 1000}))
            }
            ("POST", "/json/echo") => {
                let body_bytes = wasip3::http::compat::collect_body(body)
                    .await
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(Some("body read failed".into())))?;
                let value: serde_json::Value = serde_json::from_slice(&body_bytes)
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(Some("invalid json".into())))?;
                json_response(value)
            }
            ("GET", p) if p.starts_with("/path-params/") => {
                let segments: Vec<&str> = p.trim_start_matches("/path-params/").split('/').collect();
                json_response(serde_json::json!({"segments": segments}))
            }
            ("GET", "/query-params") => {
                let query = parts.uri.query().unwrap_or("");
                json_response(serde_json::json!({"query": query}))
            }
            ("GET", "/health") => json_response(serde_json::json!({"status": "ok"})),
            _ => {
                let response = http::Response::builder()
                    .status(404)
                    .header("content-type", "application/json")
                    .body(serde_json::to_vec(&serde_json::json!({"error": "not found"})).unwrap_or_default())
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))?;
                wasip3::http::compat::from_response(response)
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))
            }
        }
    }
}

fn json_response(
    value: serde_json::Value,
) -> Result<wasip3::http::types::Response, wasip3::http::types::ErrorCode> {
    let body = serde_json::to_vec(&value).unwrap_or_default();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(body)
        .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))?;
    wasip3::http::compat::from_response(response)
        .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))
}
