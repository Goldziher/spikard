### Request/Response Handling

```rust
use spikard_core::{Request, Response};
use std::collections::HashMap;

// Create a request
let mut request = Request::new(
    "GET".to_string(),
    "/api/users".to_string(),
);

// Add headers
request.headers_mut().insert(
    "Authorization".to_string(),
    "Bearer token123".to_string(),
);

// Add query parameters
let mut query = HashMap::new();
query.insert("filter".to_string(), "active".to_string());
request.set_query_params(query);

// Create a response
let mut response = Response::new(200);
response.set_body(r#"{"users": []}"#.as_bytes().to_vec());
```

### Schema Validation

```rust
use spikard_core::validation::ValidateBody;
use serde_json::json;

let schema = json!({
    "type": "object",
    "properties": {
        "name": { "type": "string" },
        "email": { "type": "string", "format": "email" }
    },
    "required": ["name", "email"]
});

let body = json!({
    "name": "Alice",
    "email": "alice@example.com"
});

// Validate body against schema
validate_body(&body, &schema)?;
```
