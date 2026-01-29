//! E2E test WASM component for wasmtime.
//!
//! This component implements test fixture routes as a WASIp3 HTTP handler.
//! It is built and served by the vitest global setup.

wasip3::http::proxy::export!(TestComponent);

struct TestComponent;

impl wasip3::exports::http::handler::Guest for TestComponent {
    async fn handle(
        request: wasip3::http::types::IncomingRequest,
    ) -> Result<wasip3::http::types::Response, wasip3::http::types::ErrorCode> {
        let (parts, _body) = wasip3::http::compat::into_request(request)
            .map_err(|_| wasip3::http::types::ErrorCode::InternalError(Some("conversion failed".into())))?
            .into_parts();

        let path = parts.uri.path();

        match path {
            "/health" => json_ok(serde_json::json!({"status": "ok"})),
            _ => {
                let response = http::Response::builder()
                    .status(404)
                    .header("content-type", "application/json")
                    .body(
                        serde_json::to_vec(&serde_json::json!({"error": "not found", "code": 404}))
                            .unwrap_or_default(),
                    )
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))?;
                wasip3::http::compat::from_response(response)
                    .map_err(|_| wasip3::http::types::ErrorCode::InternalError(None))
            }
        }
    }
}

fn json_ok(
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
