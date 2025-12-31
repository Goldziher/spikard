```rust
use serde_json::json;

#[derive(Debug)]
struct ValidationErrorResponse {
    error: String,
    message: String,
    details: Vec<ValidationDetail>,
}

#[derive(Debug)]
struct ValidationDetail {
    field: String,
    message: String,
}

impl From<ValidationErrorResponse> for Response {
    fn from(err: ValidationErrorResponse) -> Response {
        Response::builder()
            .status(422)
            .json(json!({
                "error": err.error,
                "message": err.message,
                "details": err.details
            }))
    }
}

app.route(
    post("/users").request_body::<CreateUserRequest>(),
    |ctx: Context| async move {
        let user: CreateUserRequest = ctx.json()
            .map_err(|e| ValidationErrorResponse {
                error: "validation_failed".to_string(),
                message: "Request validation failed".to_string(),
                details: vec![ValidationDetail {
                    field: "body".to_string(),
                    message: e.to_string(),
                }]
            })?;

        Ok(Json(json!({"id": "usr_123", "email": user.email})))
    }
)?;
```
