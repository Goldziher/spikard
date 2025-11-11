# Rust Lifecycle Hooks Example

This example demonstrates the complete lifecycle hooks API in pure Rust, showcasing all five hook points with practical use cases.

## Features Demonstrated

### 🔄 All Five Hook Points

1. **onRequest**: Early request processing
   - Request logging with timestamps
   - Request ID generation
   - Request context initialization

2. **preValidation**: Pre-validation processing
   - Rate limiting (10 requests/minute per IP)
   - Early short-circuit if rate limit exceeded

3. **preHandler**: Authentication and authorization
   - JWT-style token validation
   - User authentication
   - Role-based authorization
   - Request context enrichment

4. **onResponse**: Response post-processing
   - Security headers (X-Frame-Options, CSP, etc.)
   - Response timing
   - Response logging

5. **onError**: Error handling
   - Server error logging
   - Client error tracking
   - Consistent error formatting

## Running the Example

```bash
# From the example directory
cargo run

# Or from the repository root
cargo run -p rust-lifecycle-hooks-example
```

## Testing the Endpoints

### Public Endpoint (No Auth Required)
```bash
curl http://localhost:3000/public/hello
```

Expected: 200 OK with JSON response

### Protected Endpoint (Requires Auth)
```bash
# With valid user token
curl -H 'Authorization: Bearer user-token' http://localhost:3000/api/profile

# With invalid token (should fail)
curl -H 'Authorization: Bearer bad-token' http://localhost:3000/api/profile

# Without token (should fail)
curl http://localhost:3000/api/profile
```

### Admin Endpoint (Requires Admin Role)
```bash
# With admin token (should succeed)
curl -H 'Authorization: Bearer admin-token' http://localhost:3000/admin/dashboard

# With regular user token (should fail with 403)
curl -H 'Authorization: Bearer user-token' http://localhost:3000/admin/dashboard
```

## Valid Test Tokens

- `admin-token` - Admin user (role: admin)
- `user-token` - Regular user (role: user)
- Any other token - Invalid (will be rejected)

## Key Concepts

### Builder Pattern
```rust
let hooks = LifecycleHooks::builder()
    .on_request(request_hook("logger", |req| async move { ... }))
    .pre_handler(request_hook("auth", |req| async move { ... }))
    .on_response(response_hook("security", |resp| async move { ... }))
    .build();
```

### Request Context with Axum Extensions
```rust
// In hook: Store context
req.extensions_mut().insert(RequestContext {
    request_id: uuid::Uuid::new_v4().to_string(),
    user: Some(user),
    ...
});

// In handler: Extract context
async fn handler(Extension(ctx): Extension<RequestContext>) -> Json<Value> {
    let user = ctx.user.unwrap();
    ...
}
```

### Short-Circuit Pattern
```rust
request_hook("auth", |req| async move {
    if !authorized(&req) {
        // Stop processing and return error immediately
        let response = Response::builder()
            .status(StatusCode::UNAUTHORIZED)
            .body(Body::from(r#"{"error":"Unauthorized"}"#))
            .unwrap();
        return Ok(HookResult::ShortCircuit(response));
    }

    // Continue to next hook/handler
    Ok(HookResult::Continue(req))
})
```

### Hook Ordering

Hooks execute in registration order. Multiple hooks of the same type run sequentially:

```rust
.on_request(request_hook("first", ...))   // Runs first
.on_request(request_hook("second", ...))  // Runs second
.on_request(request_hook("third", ...))   // Runs third (unless previous hook short-circuits)
```

## Implementation Notes

This is a **demonstration** of the lifecycle hooks API. In a production application:

1. **Token Validation**: Use proper JWT libraries with signature verification, expiry checks, etc.
2. **Rate Limiting**: Use Redis or similar for distributed rate limiting
3. **Error Handling**: Integrate with monitoring services (Sentry, etc.)
4. **State Management**: Consider using proper database connections
5. **Server Integration**: Integrate hooks with the actual HTTP server middleware

## Code Structure

- `build_lifecycle_hooks()` - Constructs all hooks using the builder pattern
- `RequestContext` - Shared context passed through extensions
- `RateLimiter` - Simple in-memory rate limiter
- `validate_token()` - Token validation logic
- HTTP handlers - Example endpoints demonstrating auth flows

## Performance

- **No hooks**: ~0.5ns overhead (null check)
- **With hooks**: O(n) where n = number of hooks
- **Short-circuit**: Immediately stops further processing

## Next Steps

To integrate this with a real server:

1. Convert `LifecycleHooks` into Axum middleware/layer
2. Add hooks to the request processing pipeline
3. Hook into error handling middleware
4. Add response transformation middleware

See the [design documentation](../../docs/design/lifecycle-hooks-api-design.md) for the complete API specification.
