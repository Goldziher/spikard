```rust
// Direct tonic::Status
return Err(Status::invalid_argument("Invalid ID"));
return Err(Status::not_found("User not found"));
return Err(Status::unauthenticated("Auth required"));
return Err(Status::permission_denied("Access denied"));
return Err(Status::internal("Internal error"));

// With .map_err()
user_repository.find_by_id(id)
    .await
    .map_err(|e| Status::internal(format!("DB error: {}", e)))?;
```

Type-safe `Result<T, Status>` pattern.
