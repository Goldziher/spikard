```rust
use axum::{
    extract::Request,
    http::{header, StatusCode},
    middleware::Next,
    response::Response,
    Extension,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub roles: Vec<String>,
    pub exp: usize,
}

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: String,
    pub roles: Vec<String>,
}

pub async fn auth_guard(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // Strip "Bearer "

    // Verify and decode JWT
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(b"your-secret-key"),
        &Validation::default(),
    )
    .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Enrich context with authenticated user
    let auth_ctx = AuthContext {
        user_id: token_data.claims.sub,
        roles: token_data.claims.roles,
    };

    request.extensions_mut().insert(auth_ctx);

    Ok(next.run(request).await)
}

// Usage with App:
// app.layer(axum::middleware::from_fn(auth_guard));
```
