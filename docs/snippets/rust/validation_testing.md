```rust
#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_user_creation_validation() {
        let server = TestServer::new(app).unwrap();

        // Valid request succeeds
        let response = server
            .post("/users")
            .json(&serde_json::json!({
                "email": "test@example.com",
                "age": 25,
                "username": "testuser"
            }))
            .await;
        assert_eq!(response.status_code(), 200);

        // Invalid email rejected
        let response = server
            .post("/users")
            .json(&serde_json::json!({
                "email": "not-an-email",
                "age": 25,
                "username": "testuser"
            }))
            .await;
        assert_eq!(response.status_code(), 422);

        // Age below minimum rejected
        let response = server
            .post("/users")
            .json(&serde_json::json!({
                "email": "test@example.com",
                "age": 16,
                "username": "testuser"
            }))
            .await;
        assert_eq!(response.status_code(), 422);

        // Missing required field rejected
        let response = server
            .post("/users")
            .json(&serde_json::json!({
                "email": "test@example.com",
                "age": 25
            }))
            .await;
        assert_eq!(response.status_code(), 422);
    }
}
```
