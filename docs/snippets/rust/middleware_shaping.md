```rust
use axum::{
    body::Body,
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
    Extension,
};
use governor::{Quota, RateLimiter};
use nonzero_ext::nonzero;
use std::{net::IpAddr, sync::Arc};

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub tenant: String,
    pub features: Vec<String>,
}

type SharedRateLimiter = Arc<RateLimiter<IpAddr, governor::state::keyed::DefaultKeyedStateStore<IpAddr>, governor::clock::DefaultClock>>;

pub fn create_rate_limiter() -> SharedRateLimiter {
    // 100 requests per minute per IP
    Arc::new(RateLimiter::keyed(Quota::per_minute(nonzero!(100u32))))
}

pub async fn request_shaper(
    Extension(limiter): Extension<SharedRateLimiter>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Rate limiting
    let client_ip: IpAddr = request
        .headers()
        .get("x-forwarded-for")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.split(',').next())
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or([127, 0, 0, 1].into());

    if limiter.check_key(&client_ip).is_err() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // 2. Extract tenant from subdomain
    let host = request
        .headers()
        .get(header::HOST)
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");

    let tenant = host
        .split('.')
        .next()
        .filter(|s| !s.is_empty())
        .unwrap_or("default")
        .to_string();

    // 3. Parse feature flags from query params
    let features: Vec<String> = request
        .uri()
        .query()
        .and_then(|q| {
            q.split('&')
                .find(|p| p.starts_with("features="))
                .map(|p| p.trim_start_matches("features="))
        })
        .map(|f| f.split(',').map(String::from).collect())
        .unwrap_or_default();

    // Inject context for handlers
    let ctx = RequestContext { tenant, features };
    request.extensions_mut().insert(ctx);

    Ok(next.run(request).await)
}

// Usage with App:
// let limiter = create_rate_limiter();
// app.layer(Extension(limiter));
// app.layer(axum::middleware::from_fn(request_shaper));
```
