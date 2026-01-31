//! Fast-path HashMap router for static responses.
//!
//! Routes registered with `StaticResponse` and without path parameters are
//! placed into a two-level `AHashMap` keyed by `Method` then `path`. An axum
//! middleware layer checks this map first — on a hit the pre-built response is
//! returned immediately, avoiding the full Axum routing + middleware pipeline.

use ahash::AHashMap;
use axum::body::Body;
use axum::http::Method;

use crate::handler_trait::StaticResponse;

/// HashMap-based router for static-response routes without path parameters.
///
/// Uses a two-level map (`Method` → `path` → `StaticResponse`) so that
/// lookups only require a `&str` borrow — no heap allocation per request.
///
/// Inserted as the outermost middleware layer so that matching requests
/// never reach the Axum router at all.
#[derive(Clone)]
pub struct FastRouter {
    routes: AHashMap<Method, AHashMap<String, StaticResponse>>,
}

impl FastRouter {
    /// Create an empty fast router.
    pub fn new() -> Self {
        Self {
            routes: AHashMap::new(),
        }
    }

    /// Register a static response for an exact method + path pair.
    pub fn insert(&mut self, method: Method, path: &str, resp: &StaticResponse) {
        self.routes
            .entry(method)
            .or_default()
            .insert(path.to_owned(), resp.clone());
    }

    /// Returns `true` when at least one route has been registered.
    pub fn has_routes(&self) -> bool {
        !self.routes.is_empty()
    }

    /// Try to serve a request from the fast router.
    /// Returns `None` if the method + path pair is not registered.
    ///
    /// `Bytes::clone()` inside `to_response()` is reference-counted (not a
    /// deep copy), so this is cheap even for large response bodies.
    pub fn lookup(&self, method: &Method, path: &str) -> Option<axum::response::Response<Body>> {
        let by_path = self.routes.get(method)?;
        let resp = by_path.get(path)?;
        Some(resp.to_response())
    }
}

impl Default for FastRouter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::{HeaderValue, StatusCode};
    use bytes::Bytes;
    use http_body_util::BodyExt;

    fn make_static_response(status: u16, body: &str) -> StaticResponse {
        StaticResponse {
            status,
            headers: vec![],
            body: Bytes::from(body.to_owned()),
            content_type: HeaderValue::from_static("text/plain"),
        }
    }

    #[test]
    fn test_fast_router_miss_returns_none() {
        let router = FastRouter::new();
        assert!(router.lookup(&Method::GET, "/health").is_none());
    }

    #[test]
    fn test_fast_router_hit_returns_response() {
        let mut router = FastRouter::new();
        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));

        let resp = router.lookup(&Method::GET, "/health");
        assert!(resp.is_some());
        let resp = resp.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[test]
    fn test_fast_router_method_mismatch() {
        let mut router = FastRouter::new();
        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));

        assert!(router.lookup(&Method::POST, "/health").is_none());
    }

    #[test]
    fn test_fast_router_path_mismatch() {
        let mut router = FastRouter::new();
        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));

        assert!(router.lookup(&Method::GET, "/ready").is_none());
    }

    #[test]
    fn test_fast_router_has_routes() {
        let mut router = FastRouter::new();
        assert!(!router.has_routes());

        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));
        assert!(router.has_routes());
    }

    #[test]
    fn test_fast_router_multiple_routes() {
        let mut router = FastRouter::new();
        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));
        router.insert(Method::GET, "/ready", &make_static_response(200, "ready"));
        router.insert(Method::POST, "/health", &make_static_response(201, "created"));

        assert!(router.lookup(&Method::GET, "/health").is_some());
        assert!(router.lookup(&Method::GET, "/ready").is_some());
        assert!(router.lookup(&Method::POST, "/health").is_some());
        assert!(router.lookup(&Method::DELETE, "/health").is_none());
    }

    #[test]
    fn test_fast_router_custom_headers() {
        use axum::http::header::HeaderName;

        let resp = StaticResponse {
            status: 200,
            headers: vec![(HeaderName::from_static("x-custom"), HeaderValue::from_static("value"))],
            body: Bytes::from_static(b"OK"),
            content_type: HeaderValue::from_static("application/json"),
        };

        let mut router = FastRouter::new();
        router.insert(Method::GET, "/test", &resp);

        let response = router.lookup(&Method::GET, "/test").unwrap();
        assert_eq!(response.headers().get("x-custom").unwrap(), "value");
        assert_eq!(response.headers().get("content-type").unwrap(), "application/json");
    }

    #[tokio::test]
    async fn test_fast_router_response_body_content() {
        let mut router = FastRouter::new();
        router.insert(Method::GET, "/health", &make_static_response(200, "OK"));
        router.insert(Method::GET, "/ready", &make_static_response(200, "ready"));

        let resp = router.lookup(&Method::GET, "/health").unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"OK");

        let resp = router.lookup(&Method::GET, "/ready").unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"ready");
    }

    #[tokio::test]
    async fn test_fast_router_custom_status_code() {
        let mut router = FastRouter::new();
        router.insert(Method::POST, "/items", &make_static_response(201, "created"));

        let resp = router.lookup(&Method::POST, "/items").unwrap();
        assert_eq!(resp.status(), StatusCode::CREATED);
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(body.as_ref(), b"created");
    }

    #[test]
    fn test_fast_router_default() {
        let router = FastRouter::default();
        assert!(!router.has_routes());
    }
}
