# gRPC Status Codes Reference

This document provides a comprehensive reference for all 17 standard gRPC status codes defined in the gRPC specification. Understanding these codes is essential for proper error handling and client-server communication in gRPC applications.

## Status Code Overview

gRPC uses a standardized set of status codes to communicate the outcome of RPC calls. These codes are consistent across all gRPC implementations and language bindings, making them ideal for cross-language microservice communication.

## Complete Status Code Reference

| Code | Numeric | When to Use | HTTP Equivalent |
|------|---------|-------------|-----------------|
| OK | 0 | Request completed successfully | 200 OK |
| CANCELLED | 1 | Operation was cancelled (typically by caller) | 499 Client Closed Request |
| UNKNOWN | 2 | Unknown error or unmapped status from another system | 500 Internal Server Error |
| INVALID_ARGUMENT | 3 | Client specified an invalid argument (validation errors) | 400 Bad Request |
| DEADLINE_EXCEEDED | 4 | Operation deadline was exceeded before completion | 504 Gateway Timeout |
| NOT_FOUND | 5 | Requested entity (e.g., file, user) was not found | 404 Not Found |
| ALREADY_EXISTS | 6 | Entity that client attempted to create already exists | 409 Conflict |
| PERMISSION_DENIED | 7 | Caller lacks permission for the operation | 403 Forbidden |
| RESOURCE_EXHAUSTED | 8 | Resource has been exhausted (quota, rate limit) | 429 Too Many Requests |
| FAILED_PRECONDITION | 9 | Operation rejected because system not in required state | 400 Bad Request |
| ABORTED | 10 | Operation aborted due to concurrency issues | 409 Conflict |
| OUT_OF_RANGE | 11 | Operation attempted past valid range | 400 Bad Request |
| UNIMPLEMENTED | 12 | Operation is not implemented or not supported | 501 Not Implemented |
| INTERNAL | 13 | Internal server error | 500 Internal Server Error |
| UNAVAILABLE | 14 | Service is currently unavailable (temporary condition) | 503 Service Unavailable |
| DATA_LOSS | 15 | Unrecoverable data loss or corruption | 500 Internal Server Error |
| UNAUTHENTICATED | 16 | Request missing or invalid authentication credentials | 401 Unauthorized |

## How to Throw Status Codes by Language

### Python

```python
import grpc

# INVALID_ARGUMENT
context.abort(grpc.StatusCode.INVALID_ARGUMENT, "User ID must be positive")

# NOT_FOUND
context.abort(grpc.StatusCode.NOT_FOUND, "User not found")

# PERMISSION_DENIED
context.abort(grpc.StatusCode.PERMISSION_DENIED, "Insufficient permissions")

# INTERNAL
context.abort(grpc.StatusCode.INTERNAL, "Database connection failed")
```

### TypeScript

```typescript
import * as grpc from '@grpc/grpc-js';

// INVALID_ARGUMENT
callback({
  code: grpc.status.INVALID_ARGUMENT,
  message: 'User ID must be positive'
});

// NOT_FOUND
callback({
  code: grpc.status.NOT_FOUND,
  message: 'User not found'
});

// PERMISSION_DENIED
callback({
  code: grpc.status.PERMISSION_DENIED,
  message: 'Insufficient permissions'
});

// INTERNAL
callback({
  code: grpc.status.INTERNAL,
  message: 'Database connection failed'
});
```

### Ruby

```ruby
require 'grpc'

# INVALID_ARGUMENT
raise GRPC::InvalidArgument.new('User ID must be positive')

# NOT_FOUND
raise GRPC::NotFound.new('User not found')

# PERMISSION_DENIED
raise GRPC::PermissionDenied.new('Insufficient permissions')

# INTERNAL
raise GRPC::Internal.new('Database connection failed')
```

### PHP

```php
use Grpc\Status;

// INVALID_ARGUMENT
return [
    'code' => Status::INVALID_ARGUMENT,
    'details' => 'User ID must be positive'
];

// NOT_FOUND
return [
    'code' => Status::NOT_FOUND,
    'details' => 'User not found'
];

// PERMISSION_DENIED
return [
    'code' => Status::PERMISSION_DENIED,
    'details' => 'Insufficient permissions'
];

// INTERNAL
return [
    'code' => Status::INTERNAL,
    'details' => 'Database connection failed'
];
```

### Rust

```rust
use tonic::{Status, Code};

// INVALID_ARGUMENT
Err(Status::new(Code::InvalidArgument, "User ID must be positive"))

// NOT_FOUND
Err(Status::not_found("User not found"))

// PERMISSION_DENIED
Err(Status::permission_denied("Insufficient permissions"))

// INTERNAL
Err(Status::internal("Database connection failed"))
```

## Practical Examples

### INVALID_ARGUMENT - Input Validation

Use when client provides invalid input that fails validation rules.

**Scenario**: User registration with invalid email format

```python
# Python
def CreateUser(self, request, context):
    if not is_valid_email(request.email):
        context.abort(
            grpc.StatusCode.INVALID_ARGUMENT,
            "Email format is invalid"
        )
```

### NOT_FOUND - Missing Resources

Use when a requested resource does not exist.

**Scenario**: Retrieving a user by ID that doesn't exist

```typescript
// TypeScript
getUser(call: ServerUnaryCall<GetUserRequest, User>, callback: sendUnaryData<User>) {
  const user = database.findUser(call.request.userId);
  if (!user) {
    callback({
      code: grpc.status.NOT_FOUND,
      message: `User with ID ${call.request.userId} not found`
    });
    return;
  }
  callback(null, user);
}
```

### PERMISSION_DENIED - Authorization Failures

Use when an authenticated user lacks permission to perform an operation.

**Scenario**: Regular user attempting to delete another user's account

```ruby
# Ruby
def delete_user(request, _call)
  current_user = authenticate_user(_call)
  target_user = User.find(request.user_id)

  unless current_user.admin? || current_user.id == target_user.id
    raise GRPC::PermissionDenied.new(
      'You do not have permission to delete this user'
    )
  end

  target_user.delete
  DeleteUserResponse.new(success: true)
end
```

### INTERNAL - Server-Side Errors

Use for unexpected server-side errors that are not the client's fault.

**Scenario**: Database connection failure

```rust
// Rust
async fn get_user(&self, request: Request<GetUserRequest>) -> Result<Response<User>, Status> {
    let user_id = request.into_inner().user_id;

    match database.find_user(user_id).await {
        Ok(Some(user)) => Ok(Response::new(user)),
        Ok(None) => Err(Status::not_found("User not found")),
        Err(e) => {
            error!("Database error: {}", e);
            Err(Status::internal("An internal error occurred"))
        }
    }
}
```

## Best Practices

1. **Choose the most specific code**: Use the most descriptive status code that accurately represents the error condition.

2. **Provide helpful messages**: Include clear, actionable error messages that help clients understand and resolve the issue.

3. **Never expose sensitive information**: Don't include stack traces, database errors, or internal system details in error messages.

4. **Use INTERNAL for unexpected errors**: When encountering unexpected server errors, return INTERNAL and log the details server-side.

5. **Distinguish UNAUTHENTICATED vs PERMISSION_DENIED**: Use UNAUTHENTICATED for missing/invalid credentials, PERMISSION_DENIED for authenticated users lacking permissions.

6. **Consider retry behavior**: Clients may automatically retry certain codes (UNAVAILABLE, DEADLINE_EXCEEDED) but not others (INVALID_ARGUMENT, PERMISSION_DENIED).

## See Also

- [gRPC Error Handling Guide](https://grpc.io/docs/guides/error/)
- [ADR 0010: gRPC Status Code Guidelines](/docs/adr/0010-grpc-status-codes.md)
