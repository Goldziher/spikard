# gRPC Handlers Across All Languages

This guide demonstrates how to implement gRPC service handlers in all five Spikard-supported languages: Python, TypeScript, Ruby, PHP, and Rust. We'll build the same `UserService` with `GetUser` and `CreateUser` methods in each language, showing identical functionality with language-specific idioms.

## Overview

Spikard uses a **language-agnostic FFI pattern** where:

1. **Rust gRPC Runtime** (Tonic) handles all network I/O, HTTP/2, and wire protocol
2. **Language Handlers** receive binary protobuf payloads via FFI
3. **Handlers** deserialize, process, and serialize responses back to binary
4. **No gRPC codegen needed** in each language - just protobuf serialization

This architecture means:

- Single high-performance runtime shared across all languages
- Minimal FFI overhead (binary payloads only)
- Language-native protobuf libraries for serialization
- Consistent behavior and error handling

## Common Proto Definition

All examples use this protobuf schema:

--8<-- "snippets/grpc/proto/userservice.md"

## Architecture

The FFI data flow for all languages:

```
┌──────────────────────────────────────────────────┐
│  Language Handler (Python/TS/Ruby/PHP/Rust)     │
│  ┌────────────────────────────────────────┐     │
│  │ 1. Receive GrpcRequest                 │     │
│  │    - service_name: string              │     │
│  │    - method_name: string               │     │
│  │    - payload: bytes                    │     │
│  │    - metadata: dict/map                │     │
│  └────────────────────────────────────────┘     │
│  ┌────────────────────────────────────────┐     │
│  │ 2. Deserialize using protobuf library  │     │
│  │    req = GetUserRequest.parse(payload) │     │
│  └────────────────────────────────────────┘     │
│  ┌────────────────────────────────────────┐     │
│  │ 3. Business logic                      │     │
│  │    user = db.get_user(req.id)          │     │
│  └────────────────────────────────────────┘     │
│  ┌────────────────────────────────────────┐     │
│  │ 4. Serialize response                  │     │
│  │    payload = user.serialize()          │     │
│  └────────────────────────────────────────┘     │
│  ┌────────────────────────────────────────┐     │
│  │ 5. Return GrpcResponse                 │     │
│  │    - payload: bytes                    │     │
│  │    - metadata: dict/map (optional)     │     │
│  └────────────────────────────────────────┘     │
└──────────────────────────────────────────────────┘
                      ↕ FFI
┌──────────────────────────────────────────────────┐
│  Rust gRPC Runtime (Tonic)                       │
│  - HTTP/2 multiplexing                           │
│  - Compression                                   │
│  - Metadata/headers                              │
│  - Stream management                             │
└──────────────────────────────────────────────────┘
```

## Handler Implementations

### Complete Handler

=== "Python"

    --8<-- "snippets/grpc/python/handler_complete.md"

=== "TypeScript"

    --8<-- "snippets/grpc/typescript/handler_complete.md"

=== "Ruby"

    --8<-- "snippets/grpc/ruby/handler_complete.md"

=== "PHP"

    --8<-- "snippets/grpc/php/handler_complete.md"

=== "Rust"

    --8<-- "snippets/grpc/rust/handler_complete.md"

### Using Helper Functions (TypeScript)

TypeScript provides convenience helpers for simpler code:

--8<-- "snippets/grpc/typescript/handler_helpers.md"

### Registration

=== "Python"

    --8<-- "snippets/grpc/python/registration.md"

=== "TypeScript"

    --8<-- "snippets/grpc/typescript/registration.md"

=== "Ruby"

    --8<-- "snippets/grpc/ruby/registration.md"

=== "PHP"

    --8<-- "snippets/grpc/php/registration.md"

=== "Rust"

    --8<-- "snippets/grpc/rust/registration.md"

### Key Patterns

=== "Python"

    --8<-- "snippets/grpc/python/key_patterns.md"

=== "TypeScript"

    --8<-- "snippets/grpc/typescript/key_patterns.md"

=== "Ruby"

    --8<-- "snippets/grpc/ruby/key_patterns.md"

=== "PHP"

    --8<-- "snippets/grpc/php/key_patterns.md"

=== "Rust"

    --8<-- "snippets/grpc/rust/key_patterns.md"

## Error Handling Comparison

How each language maps errors to gRPC status codes:

=== "Python"

    --8<-- "snippets/grpc/python/error_handling.md"

=== "TypeScript"

    --8<-- "snippets/grpc/typescript/error_handling.md"

=== "Ruby"

    --8<-- "snippets/grpc/ruby/error_handling.md"

=== "PHP"

    --8<-- "snippets/grpc/php/error_handling.md"

=== "Rust"

    --8<-- "snippets/grpc/rust/error_handling.md"

## Metadata Handling

### Request Metadata (Headers)

| Language   | Access Pattern                                      |
|------------|-----------------------------------------------------|
| Python     | `request.get_metadata("key")` -> `str \| None`      |
| TypeScript | `request.metadata["key"]` -> `string \| undefined`  |
| Ruby       | `request.get_metadata("key")` -> `String \| nil`    |
| PHP        | `$request->getMetadata("key")` -> `?string`         |
| Rust       | `request.metadata.get("key")` -> `Option<&str>`     |

### Response Metadata (Headers)

| Language   | Set Pattern                                               |
|------------|-----------------------------------------------------------|
| Python     | `GrpcResponse(payload=..., metadata={"key": "value"})`    |
| TypeScript | `return { payload: ..., metadata: { "key": "value" } }`   |
| Ruby       | `response.metadata = { "key" => "value" }`                |
| PHP        | `new Response(payload: ..., metadata: ["key" => "val"])`  |
| Rust       | `metadata.insert("key", "value".parse().unwrap())`        |

### Common Metadata Keys

```
authorization: Bearer <token>      # Authentication
x-request-id: <uuid>               # Request tracing
x-user-id: <id>                    # User context
content-type: application/grpc     # gRPC content type
grpc-status: <status_code>         # Error status (response only)
grpc-message: <error_message>      # Error message (response only)
```

## Best Practices

### 1. Validation

Always validate inputs before processing:

=== "Python"

    ```python
    if req.id <= 0:
        raise ValueError("User ID must be positive")
    ```

=== "TypeScript"

    ```typescript
    if (req.id <= 0) {
      throw new GrpcError(GrpcStatusCode.INVALID_ARGUMENT, 'Invalid ID');
    }
    ```

=== "Ruby"

    ```ruby
    raise ArgumentError, 'User ID must be positive' if req.id <= 0
    ```

=== "PHP"

    ```php
    if ($req->getId() <= 0) {
        return Response::error('User ID must be positive');
    }
    ```

=== "Rust"

    ```rust
    if req.id <= 0 {
        return Err(Status::invalid_argument("User ID must be positive"));
    }
    ```

### 2. Error Context

Provide helpful error messages:

=== "Ruby"

    ```ruby
    # Good
    Response.error("User #{req.id} not found")

    # Bad
    Response.error("Not found")
    ```

=== "Python"

    ```python
    # Good
    raise ValueError(f"User {req.id} not found")

    # Bad
    raise ValueError("Not found")
    ```

### 3. Metadata for Observability

Add tracing metadata:

=== "PHP"

    ```php
    return new Response(
        payload: $data,
        metadata: [
            'x-request-id' => $requestId,
            'x-processing-time-ms' => (string)$duration,
        ]
    );
    ```

=== "Python"

    ```python
    return GrpcResponse(
        payload=data,
        metadata={
            "x-request-id": request_id,
            "x-processing-time-ms": str(duration),
        }
    )
    ```

### 4. Dependency Injection

Inject dependencies in constructor:

=== "TypeScript"

    ```typescript
    class UserServiceHandler implements GrpcHandler {
      constructor(
        private userRepository: UserRepository,
        private logger: Logger,
        private metrics: Metrics
      ) {}
    }
    ```

=== "Python"

    ```python
    class UserServiceHandler(GrpcHandler):
        def __init__(self, user_repository, logger, metrics):
            self.user_repository = user_repository
            self.logger = logger
            self.metrics = metrics
    ```

### 5. Method Routing

Use clear routing patterns:

=== "Python"

    ```python
    async def handle_request(self, request: GrpcRequest) -> GrpcResponse:
        match request.method_name:
            case "GetUser":
                return await self._get_user(request)
            case "CreateUser":
                return await self._create_user(request)
            case _:
                raise NotImplementedError(f"Unknown: {request.method_name}")
    ```

=== "TypeScript"

    ```typescript
    async handleRequest(request: GrpcRequest): Promise<GrpcResponse> {
      switch (request.methodName) {
        case 'GetUser':
          return this.getUser(request);
        case 'CreateUser':
          return this.createUser(request);
        default:
          throw new GrpcError(GrpcStatusCode.UNIMPLEMENTED, 'Unknown method');
      }
    }
    ```

### 6. Testing

Test handlers with mock requests:

=== "TypeScript"

    ```typescript
    const request: GrpcRequest = {
      serviceName: 'userservice.UserService',
      methodName: 'GetUser',
      payload: Buffer.from(encoded),
      metadata: {},
    };

    const response = await handler.handleRequest(request);
    expect(response.payload).toBeDefined();
    ```

=== "Python"

    ```python
    request = GrpcRequest(
        service_name="userservice.UserService",
        method_name="GetUser",
        payload=encoded,
        metadata={},
    )

    response = await handler.handle_request(request)
    assert response.payload is not None
    ```

### 7. Logging

Log important operations:

=== "Rust"

    ```rust
    tracing::info!(
        user_id = req.id,
        method = "GetUser",
        "Processing user request"
    );
    ```

=== "Python"

    ```python
    logger.info("Processing user request", user_id=req.id, method="GetUser")
    ```

### 8. Security

Always validate authentication:

=== "PHP"

    ```php
    $authToken = $request->getMetadata('authorization');
    if (!$authToken || !$this->validateToken($authToken)) {
        return Response::error(
            'Unauthorized',
            ['grpc-status' => 'UNAUTHENTICATED']
        );
    }
    ```

=== "Python"

    ```python
    auth_token = request.get_metadata("authorization")
    if not auth_token or not self.validate_token(auth_token):
        raise PermissionError("Unauthorized")
    ```

## Summary

All five languages follow the same architectural pattern:

1. **Receive** binary protobuf payload via FFI
2. **Deserialize** using language-native protobuf library
3. **Process** business logic
4. **Serialize** response to binary protobuf
5. **Return** binary payload with optional metadata

The Rust gRPC runtime handles all network concerns (HTTP/2, compression, streaming) while language handlers focus purely on business logic. This separation provides:

- **Consistency**: Same behavior across all languages
- **Performance**: Single high-performance runtime
- **Simplicity**: No gRPC code generation needed
- **Flexibility**: Choose the right language for each service

For more details:

- Python: `packages/python/spikard/grpc.py`
- TypeScript: `packages/node/src/grpc.ts`
- Ruby: `packages/ruby/lib/spikard/grpc.rb`
- PHP: `packages/php/docs/GRPC.md`
- Rust: `crates/spikard-http/src/grpc/`
- FFI Details: `docs/adr/0011-grpc-ffi-bindings-strategy.md`
