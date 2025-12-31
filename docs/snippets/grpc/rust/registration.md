```rust
use spikard_http::grpc::GrpcServiceRegistry;
use std::sync::Arc;

// Create handler
let user_repository = Arc::new(UserRepositoryImpl::new());
let handler = Arc::new(UserServiceHandler::new(user_repository));

// Register with gRPC runtime
let mut registry = GrpcServiceRegistry::new();
registry.register(handler);

// Runtime ready to serve
```
