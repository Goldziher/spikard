```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use tower::ServiceExt;

    async fn handler() -> &'static str {
        "OK"
    }

    #[tokio::test]
    async fn test_auth_guard_valid_token() {
        let app = Router::new()
            .route("/", get(handler))
            .layer(axum::middleware::from_fn(auth_guard));

        let valid_token = create_test_jwt("user-123", vec!["admin".into()]);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", format!("Bearer {}", valid_token))
                    .body(Body::empty())
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn test_auth_guard_missing_token() {
        let app = Router::new()
            .route("/", get(handler))
            .layer(axum::middleware::from_fn(auth_guard));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .body(Body::empty())
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn test_auth_guard_invalid_token() {
        let app = Router::new()
            .route("/", get(handler))
            .layer(axum::middleware::from_fn(auth_guard));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("Authorization", "Bearer invalid-token")
                    .body(Body::empty())
                    .expect("failed to build request"),
            )
            .await
            .expect("request failed");

        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    fn create_test_jwt(sub: &str, roles: Vec<String>) -> String {
        use jsonwebtoken::{encode, EncodingKey, Header};

        let claims = Claims {
            sub: sub.to_string(),
            roles,
            exp: (chrono::Utc::now() + chrono::Duration::hours(1)).timestamp() as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(b"your-secret-key"),
        )
        .expect("failed to encode JWT")
    }
}
```
