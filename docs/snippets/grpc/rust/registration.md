```rust
use spikard_http::grpc::{GrpcRegistry, RpcMode};
use std::sync::Arc;

// Create handler
let user_repository = Arc::new(UserRepositoryImpl::new());
let handler = Arc::new(UserServiceHandler::new(user_repository));

// Register with gRPC runtime
let mut registry = GrpcRegistry::new();
registry.register("userservice.UserService", handler, RpcMode::Unary);

// Runtime ready to serve
```
