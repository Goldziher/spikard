```rust
use axum::{
    extract::Request,
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tracing::{info, span, Instrument, Level};
use uuid::Uuid;

pub async fn observability_middleware(
    request: Request,
    next: Next,
) -> Response {
    // Generate or propagate request ID
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|h| h.to_str().ok())
        .map(String::from)
        .unwrap_or_else(|| Uuid::new_v4().to_string());

    // Create span for distributed tracing
    let span = span!(
        Level::INFO,
        "http_request",
        request_id = %request_id,
        method = %request.method(),
        path = %request.uri().path(),
        user_agent = request
            .headers()
            .get("user-agent")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("unknown"),
    );

    // Log request start
    info!(parent: &span, "request_started");

    let start = std::time::Instant::now();

    // Execute request within span
    let mut response = next.run(request).instrument(span.clone()).await;

    let duration_ms = start.elapsed().as_millis();

    // Log request completion
    info!(
        parent: &span,
        status = %response.status().as_u16(),
        duration_ms = %duration_ms,
        "request_completed"
    );

    // Propagate request ID in response headers
    response.headers_mut().insert(
        "x-request-id",
        HeaderValue::from_str(&request_id).unwrap_or_else(|_| HeaderValue::from_static("")),
    );

    response
}

// Usage with App:
// app.layer(axum::middleware::from_fn(observability_middleware));
```
