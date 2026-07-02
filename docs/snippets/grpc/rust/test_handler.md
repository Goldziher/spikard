# Rust gRPC Handler Tests

Comprehensive test examples for gRPC handlers using Tokio test.

```rust
// tests/user_handler_test.rs
use bytes::Bytes;
use spikard_http::grpc::{GrpcHandler, GrpcRequestData, GrpcResponseData};
use std::sync::Arc;
use tonic::metadata::MetadataMap;

mod userservice {
    include!("../src/userservice.rs");
}

use crate::user_handler::UserServiceHandler;

// Mock repository for testing
struct MockUserRepository {
    users: std::sync::RwLock<Vec<userservice::User>>,
}

impl MockUserRepository {
    fn new() -> Self {
        let users = vec![
            userservice::User {
                id: 1,
                name: "Alice".to_string(),
                email: "alice@example.com".to_string(),
                created_at: "2024-01-01T00:00:00Z".to_string(),
            },
            userservice::User {
                id: 2,
                name: "Bob".to_string(),
                email: "bob@example.com".to_string(),
                created_at: "2024-01-02T00:00:00Z".to_string(),
            },
        ];
        Self {
            users: std::sync::RwLock::new(users),
        }
    }
}

#[async_trait::async_trait]
impl UserRepository for MockUserRepository {
    async fn find_by_id(&self, id: i32) -> Result<Option<userservice::User>, String> {
        let users = self.users.read().expect("lock poisoned");
        Ok(users.iter().find(|u| u.id == id).cloned())
    }

    async fn create(&self, name: &str, email: &str) -> Result<userservice::User, String> {
        let mut users = self.users.write().expect("lock poisoned");
        let new_id = users.len() as i32 + 1;
        let user = userservice::User {
            id: new_id,
            name: name.to_string(),
            email: email.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };
        users.push(user.clone());
        Ok(user)
    }
}

fn create_handler() -> UserServiceHandler {
    let repo = Arc::new(MockUserRepository::new());
    UserServiceHandler::new(repo)
}

#[tokio::test]
async fn test_get_user_success() {
    use prost::Message;

    let handler = create_handler();

    // Create request
    let req = userservice::GetUserRequest { id: 1 };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    };

    // Call handler
    let response = handler.call(grpc_request).await.expect("handler call failed");

    // Deserialize response
    let user_response = userservice::User::decode(response.payload).expect("failed to decode response payload");

    // Assertions
    assert_eq!(user_response.id, 1);
    assert_eq!(user_response.name, "Alice");
    assert_eq!(user_response.email, "alice@example.com");
    assert_eq!(
        response.metadata.get("x-user-found").expect("x-user-found header missing").to_str().expect("invalid metadata value"),
        "true"
    );
}

#[tokio::test]
async fn test_get_user_not_found() {
    use prost::Message;

    let handler = create_handler();

    // Create request for non-existent user
    let req = userservice::GetUserRequest { id: 999 };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    };

    // Call handler - should return error
    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::NotFound);
    assert!(status.message().contains("not found"));
}

#[tokio::test]
async fn test_get_user_invalid_id() {
    use prost::Message;

    let handler = create_handler();

    // Create request with invalid ID
    let req = userservice::GetUserRequest { id: 0 };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    };

    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert!(status.message().contains("must be positive"));
}

#[tokio::test]
async fn test_create_user_success() {
    use prost::Message;

    let handler = create_handler();

    // Create request
    let req = userservice::CreateUserRequest {
        name: "Charlie".to_string(),
        email: "charlie@example.com".to_string(),
    };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    // Add authorization metadata
    let mut metadata = MetadataMap::new();
    metadata.insert("authorization", "Bearer valid-token".parse().expect("failed to parse authorization header"));

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "CreateUser".to_string(),
        payload: Bytes::from(buf),
        metadata,
    };

    // Call handler
    let response = handler.call(grpc_request).await.expect("handler call failed");

    // Deserialize response
    let user_response = userservice::User::decode(response.payload).expect("failed to decode response payload");

    // Assertions
    assert_eq!(user_response.id, 3); // Next available ID
    assert_eq!(user_response.name, "Charlie");
    assert_eq!(user_response.email, "charlie@example.com");
    assert_eq!(
        response.metadata.get("x-user-id").expect("x-user-id header missing").to_str().expect("invalid metadata value"),
        "3"
    );
    assert_eq!(
        response.metadata.get("x-created").expect("x-created header missing").to_str().expect("invalid metadata value"),
        "true"
    );
}

#[tokio::test]
async fn test_create_user_validation_error() {
    use prost::Message;

    let handler = create_handler();

    // Create request with missing email
    let req = userservice::CreateUserRequest {
        name: "Test User".to_string(),
        email: "".to_string(),
    };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let mut metadata = MetadataMap::new();
    metadata.insert("authorization", "Bearer token".parse().expect("failed to parse authorization header"));

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "CreateUser".to_string(),
        payload: Bytes::from(buf),
        metadata,
    };

    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
    assert!(status.message().contains("required"));
}

#[tokio::test]
async fn test_create_user_requires_authentication() {
    use prost::Message;

    let handler = create_handler();

    let req = userservice::CreateUserRequest {
        name: "Test".to_string(),
        email: "test@example.com".to_string(),
    };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    // Request without authorization header
    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "CreateUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    };

    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unauthenticated);
    assert!(status.message().contains("Authentication required"));
}

#[tokio::test]
async fn test_unknown_method() {
    let handler = create_handler();

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "DeleteUser".to_string(),
        payload: Bytes::new(),
        metadata: MetadataMap::new(),
    };

    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::Unimplemented);
    assert!(status.message().contains("Unknown method"));
}
```

## Test Patterns

### Using Test Fixtures

```rust
use once_cell::sync::Lazy;

static TEST_HANDLER: Lazy<UserServiceHandler> = Lazy::new(|| {
    let repo = Arc::new(MockUserRepository::new());
    UserServiceHandler::new(repo)
});

#[tokio::test]
async fn test_with_shared_handler() {
    use prost::Message;

    let req = userservice::GetUserRequest { id: 1 };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    };

    let response = TEST_HANDLER.call(grpc_request).await.expect("handler call failed");
    let user = userservice::User::decode(response.payload).expect("failed to decode response payload");

    assert_eq!(user.name, "Alice");
}
```

### Testing Error Cases

```rust
#[tokio::test]
async fn test_handles_malformed_payload() {
    let handler = create_handler();

    let grpc_request = GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from("invalid protobuf data"),
        metadata: MetadataMap::new(),
    };

    let result = handler.call(grpc_request).await;

    assert!(result.is_err());
    let status = result.unwrap_err();
    assert_eq!(status.code(), tonic::Code::InvalidArgument);
}
```

### Helper Functions

```rust
fn create_get_user_request(user_id: i32) -> GrpcRequestData {
    use prost::Message;

    let req = userservice::GetUserRequest { id: user_id };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "GetUser".to_string(),
        payload: Bytes::from(buf),
        metadata: MetadataMap::new(),
    }
}

fn create_create_user_request(name: &str, email: &str, auth_token: Option<&str>) -> GrpcRequestData {
    use prost::Message;

    let req = userservice::CreateUserRequest {
        name: name.to_string(),
        email: email.to_string(),
    };
    let mut buf = Vec::new();
    req.encode(&mut buf).expect("failed to encode protobuf request");

    let mut metadata = MetadataMap::new();
    if let Some(token) = auth_token {
        metadata.insert("authorization", token.parse().expect("failed to parse authorization header"));
    }

    GrpcRequestData {
        service_name: "userservice.v1.UserService".to_string(),
        method_name: "CreateUser".to_string(),
        payload: Bytes::from(buf),
        metadata,
    }
}

#[tokio::test]
async fn test_with_helpers() {
    let handler = create_handler();

    let request = create_get_user_request(1);
    let response = handler.call(request).await.expect("handler call failed");

    let user = userservice::User::decode(response.payload).expect("failed to decode response payload");
    assert_eq!(user.id, 1);
}
```

## Running Tests

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_get_user_success

# Run tests matching pattern
cargo test test_get_user

# Run with coverage (requires cargo-tarpaulin)
cargo tarpaulin --out Html
```
