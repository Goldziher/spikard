//! Comprehensive Lifecycle Hooks Example
//!
//! This example demonstrates all five lifecycle hook points:
//! - onRequest: Early request logging and request ID generation
//! - preValidation: Rate limiting before validation
//! - preHandler: Authentication and authorization
//! - onResponse: Security headers and response timing
//! - onError: Error logging and formatting
//!
//! Run with: cargo run -p rust-lifecycle-hooks-example
//! Test with: curl http://localhost:3000/

use axum::{
    Json, Router,
    body::Body,
    extract::Extension,
    http::{HeaderValue, Response, StatusCode},
    routing::get,
};
use serde_json::{Value, json};
use spikard_http::{HookResult, LifecycleHooks, request_hook, response_hook};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Instant;


/// Request context passed through hooks and into handlers
#[derive(Clone)]
struct RequestContext {
    request_id: String,
    #[allow(dead_code)]
    start_time: Instant,
    user: Option<User>,
}

/// User information after authentication
#[derive(Clone, Debug)]
struct User {
    id: u64,
    name: String,
    role: String,
}

/// Simple in-memory rate limiter
struct RateLimiter {
    requests: Arc<Mutex<HashMap<String, Vec<Instant>>>>,
    limit: usize,
    window_secs: u64,
}

impl RateLimiter {
    fn new(limit: usize, window_secs: u64) -> Self {
        Self {
            requests: Arc::new(Mutex::new(HashMap::new())),
            limit,
            window_secs,
        }
    }

    fn check(&self, key: &str) -> bool {
        let mut requests = self.requests.lock().unwrap();
        let now = Instant::now();

        let window = std::time::Duration::from_secs(self.window_secs);
        requests
            .entry(key.to_string())
            .or_default()
            .retain(|ts| now.duration_since(*ts) < window);

        let count = requests.get(key).map(|v| v.len()).unwrap_or(0);
        if count >= self.limit {
            return false;
        }

        requests.entry(key.to_string()).or_default().push(now);
        true
    }
}


fn build_lifecycle_hooks() -> LifecycleHooks {
    let rate_limiter = Arc::new(RateLimiter::new(10, 60));

    LifecycleHooks::builder()
        .on_request(request_hook("request_logger", |req| async move {
            println!(
                "[{}] {} {}",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                req.method(),
                req.uri().path()
            );
            Ok(HookResult::Continue(req))
        }))
        .on_request(request_hook("request_id", |mut req| async move {
            let request_id = uuid::Uuid::new_v4().to_string();
            req.headers_mut().insert(
                "X-Request-ID",
                HeaderValue::from_str(&request_id).unwrap(),
            );

            req.extensions_mut().insert(RequestContext {
                request_id,
                start_time: Instant::now(),
                user: None,
            });

            Ok(HookResult::Continue(req))
        }))
        .pre_validation(request_hook("rate_limit", move |req| {
            let rate_limiter = rate_limiter.clone();
            async move {
                let ip = req
                    .headers()
                    .get("X-Forwarded-For")
                    .and_then(|h| h.to_str().ok())
                    .unwrap_or("unknown");

                if !rate_limiter.check(ip) {
                    println!("  ⚠️  Rate limit exceeded for {}", ip);
                    let response = Response::builder()
                        .status(StatusCode::TOO_MANY_REQUESTS)
                        .header("Retry-After", "60")
                        .body(Body::from(
                            json!({
                                "error": "Rate limit exceeded",
                                "message": "Too many requests, please try again later"
                            })
                            .to_string(),
                        ))
                        .unwrap();
                    return Ok(HookResult::ShortCircuit(response));
                }

                Ok(HookResult::Continue(req))
            }
        }))
        .pre_handler(request_hook("authentication", |mut req| async move {
            if req.uri().path().starts_with("/public") {
                return Ok(HookResult::Continue(req));
            }

            let auth_header = req.headers().get("Authorization");

            match auth_header {
                Some(header) if header.to_str().ok().map(|h| h.starts_with("Bearer ")).unwrap_or(false) => {
                    let token = header.to_str().unwrap().trim_start_matches("Bearer ");

                    let user = validate_token(token).await;

                    match user {
                        Ok(user) => {
                            println!("  ✅ Authenticated user: {} ({})", user.name, user.role);

                            if let Some(ctx) = req.extensions_mut().get_mut::<RequestContext>() {
                                ctx.user = Some(user);
                            }

                            Ok(HookResult::Continue(req))
                        }
                        Err(err) => {
                            println!("  ❌ Authentication failed: {}", err);
                            let response = Response::builder()
                                .status(StatusCode::UNAUTHORIZED)
                                .body(Body::from(
                                    json!({
                                        "error": "Invalid token",
                                        "message": err
                                    })
                                    .to_string(),
                                ))
                                .unwrap();
                            Ok(HookResult::ShortCircuit(response))
                        }
                    }
                }
                _ => {
                    println!("  ❌ Missing authorization header");
                    let response = Response::builder()
                        .status(StatusCode::UNAUTHORIZED)
                        .body(Body::from(
                            json!({
                                "error": "Unauthorized",
                                "message": "Missing or invalid Authorization header"
                            })
                            .to_string(),
                        ))
                        .unwrap();
                    Ok(HookResult::ShortCircuit(response))
                }
            }
        }))
        .pre_handler(request_hook("authorization", |req| async move {
            if req.uri().path().starts_with("/admin") {
                if let Some(ctx) = req.extensions().get::<RequestContext>() {
                    if let Some(user) = &ctx.user {
                        if user.role != "admin" {
                            println!("  ❌ Insufficient permissions: {} requires admin role", user.role);
                            let response = Response::builder()
                                .status(StatusCode::FORBIDDEN)
                                .body(Body::from(
                                    json!({
                                        "error": "Forbidden",
                                        "message": "Admin access required"
                                    })
                                    .to_string(),
                                ))
                                .unwrap();
                            return Ok(HookResult::ShortCircuit(response));
                        }
                    }
                }
            }

            Ok(HookResult::Continue(req))
        }))
        .on_response(response_hook("security_headers", |mut resp| async move {
            let headers = resp.headers_mut();
            headers.insert(
                "X-Content-Type-Options",
                HeaderValue::from_static("nosniff"),
            );
            headers.insert("X-Frame-Options", HeaderValue::from_static("DENY"));
            headers.insert(
                "X-XSS-Protection",
                HeaderValue::from_static("1; mode=block"),
            );
            headers.insert(
                "Strict-Transport-Security",
                HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            );

            Ok(HookResult::Continue(resp))
        }))
        .on_response(response_hook("response_timing", |mut resp| async move {
            resp.headers_mut().insert(
                "X-Response-Time",
                HeaderValue::from_static("0ms"),
            );

            println!("  📤 Response sent with status: {}", resp.status());
            Ok(HookResult::Continue(resp))
        }))
        .on_error(response_hook("error_handler", |mut resp| async move {
            let status = resp.status();

            if status.is_server_error() {
                eprintln!("  💥 Server error: {} - {}", status, status.canonical_reason().unwrap_or("Unknown"));
            } else if status.is_client_error() {
                println!("  ⚠️  Client error: {} - {}", status, status.canonical_reason().unwrap_or("Unknown"));
            }

            resp.headers_mut().insert(
                "Content-Type",
                HeaderValue::from_static("application/json"),
            );

            Ok(HookResult::Continue(resp))
        }))
        .build()
}


async fn validate_token(token: &str) -> Result<User, String> {
    match token {
        "admin-token" => Ok(User {
            id: 1,
            name: "Admin User".to_string(),
            role: "admin".to_string(),
        }),
        "user-token" => Ok(User {
            id: 2,
            name: "Regular User".to_string(),
            role: "user".to_string(),
        }),
        _ => Err("Invalid token".to_string()),
    }
}


async fn public_handler() -> Json<Value> {
    Json(json!({
        "message": "This is a public endpoint",
        "requires_auth": false
    }))
}

async fn protected_handler(Extension(ctx): Extension<RequestContext>) -> Json<Value> {
    let user = ctx.user.as_ref().unwrap();
    Json(json!({
        "message": "Welcome to the protected area",
        "user": {
            "id": user.id,
            "name": user.name,
            "role": user.role
        },
        "request_id": ctx.request_id
    }))
}

async fn admin_handler(Extension(ctx): Extension<RequestContext>) -> Json<Value> {
    let user = ctx.user.as_ref().unwrap();
    Json(json!({
        "message": "Admin panel access granted",
        "user": {
            "id": user.id,
            "name": user.name,
            "role": user.role
        },
        "admin_features": ["user_management", "system_settings", "analytics"]
    }))
}


#[tokio::main]
async fn main() {
    println!("🚀 Starting Spikard Lifecycle Hooks Example Server");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();
    println!("📋 Available Endpoints:");
    println!("  GET  /public/hello     - Public endpoint (no auth required)");
    println!("  GET  /api/profile      - Protected endpoint (requires auth)");
    println!("  GET  /admin/dashboard  - Admin endpoint (requires admin role)");
    println!();
    println!("🔑 Test Commands:");
    println!("  Public:  curl http://localhost:3000/public/hello");
    println!("  User:    curl -H 'Authorization: Bearer user-token' http://localhost:3000/api/profile");
    println!("  Admin:   curl -H 'Authorization: Bearer admin-token' http://localhost:3000/admin/dashboard");
    println!("  Invalid: curl -H 'Authorization: Bearer bad-token' http://localhost:3000/api/profile");
    println!();
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!();

    let _hooks = build_lifecycle_hooks();

    let _app = Router::<()>::new()
        .route("/public/hello", get(public_handler))
        .route("/api/profile", get(protected_handler))
        .route("/admin/dashboard", get(admin_handler));



    println!("✅ Lifecycle hooks configured:");
    println!("  • onRequest hooks (2): request_logger, request_id");
    println!("  • preValidation hooks (1): rate_limit");
    println!("  • preHandler hooks (2): authentication, authorization");
    println!("  • onResponse hooks (2): security_headers, response_timing");
    println!("  • onError hooks (1): error_handler");
    println!();
    println!("📡 Server would start on http://0.0.0.0:3000");
    println!("   (This is a demonstration example showing the API)");
}
