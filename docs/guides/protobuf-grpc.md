# Protobuf/gRPC Guide

Build high-performance, type-safe gRPC services with Spikard. This guide covers proto3 syntax, code generation, and service implementation across all supported languages.

## What You'll Learn

- Write `.proto` files with proto3 syntax
- Generate type-safe code with `spikard generate protobuf`
- Implement gRPC service handlers
- Handle errors with gRPC status codes
- Use streaming patterns

## Why gRPC and Protobuf?

**Protocol Buffers** provide type safety, 3-5x smaller payloads than JSON, cross-language code generation, and schema evolution support.

**gRPC** adds HTTP/2 multiplexing, four streaming modes, standardized error codes, and metadata support.

## Part 1: Proto3 Syntax

### Basic Service Definition

--8<-- "grpc/proto/user_service_basic.md"

**Key concepts**:
1. **`syntax = "proto3"`**: Required (Spikard only supports proto3)
2. **`package`**: Namespace for types (use versioning: `v1`, `v2`)
3. **Field numbers**: Unique identifiers (1-536870911) that must never change
4. **Field labels**: `optional`, `repeated`, or none

### Scalar Types

--8<-- "grpc/common/proto_scalar_types.md"

### Enums

```protobuf
enum UserStatus {
  USER_STATUS_UNSPECIFIED = 0;  // Always have a zero value
  ACTIVE = 1;
  INACTIVE = 2;
  SUSPENDED = 3;
}
```

**Best practices**: Always include a zero-value, use UPPER_SNAKE_CASE, prefix values with enum name.

### Service Definitions

```protobuf
service UserService {
  // Unary RPC
  rpc GetUser(GetUserRequest) returns (User) {}

  // Server streaming
  rpc ListUsers(ListUsersRequest) returns (stream User) {}

  // Client streaming
  rpc CreateUsers(stream CreateUserRequest) returns (CreateUsersResponse) {}

  // Bidirectional streaming
  rpc Chat(stream ChatMessage) returns (stream ChatMessage) {}
}
```

## Part 2: Type Mapping

--8<-- "grpc/common/type_mapping_table.md"

## Part 3: Code Generation

### Installation

```bash
cargo install spikard-cli
```

### Generate Code

--8<-- "grpc/common/codegen_commands.md"

## Part 4: Implementing Handlers

=== "Python"
    --8<-- "grpc/python/handler_basic.md"

=== "TypeScript"
    --8<-- "grpc/typescript/handler_basic.md"

=== "Ruby"
    --8<-- "grpc/ruby/handler_basic.md"

=== "PHP"
    --8<-- "grpc/php/handler_basic.md"

=== "Rust"
    --8<-- "grpc/rust/handler_basic.md"

## Part 5: Error Handling

### gRPC Status Codes

--8<-- "grpc/common/status_codes_table.md"

### Error Handling Patterns

=== "Python"
    --8<-- "grpc/python/error_handling.md"

=== "TypeScript"
    --8<-- "grpc/typescript/error_handling.md"

=== "Ruby"
    --8<-- "grpc/ruby/error_handling.md"

=== "PHP"
    --8<-- "grpc/php/error_handling.md"

=== "Rust"
    --8<-- "grpc/rust/error_handling.md"

## Part 6: Testing

=== "Python"
    --8<-- "grpc/python/test_handler.md"

=== "TypeScript"
    --8<-- "grpc/typescript/test_handler.md"

=== "Ruby"
    --8<-- "grpc/ruby/test_handler.md"

=== "PHP"
    --8<-- "grpc/php/test_handler.md"

=== "Rust"
    --8<-- "grpc/rust/test_handler.md"

## Part 7: Streaming

--8<-- "grpc/common/streaming_modes.md"

### Streaming Handler Implementations

Complete examples of client streaming and bidirectional streaming handlers:

=== "Python"
    --8<-- "grpc/python/handler_streaming.md"

=== "TypeScript"
    --8<-- "grpc/typescript/handler_streaming.md"

=== "Ruby"
    --8<-- "grpc/ruby/handler_streaming.md"

=== "PHP"
    --8<-- "grpc/php/handler_streaming.md"

=== "Rust"
    --8<-- "grpc/rust/handler_streaming.md"

## Part 8: Best Practices

### Project Structure

```
project/
+-- proto/
|   +-- user/v1/
|   |   +-- user.proto
|   |   +-- user_service.proto
|   +-- common/v1/
|       +-- types.proto
+-- generated/
    +-- python/
    +-- typescript/
    +-- ruby/
```

### Schema Evolution

1. **Use packages with versioning**: `package company.user.v1;`
2. **Reserve deleted field numbers**: `reserved 2, 15, 9 to 11;`
3. **Document everything**: Add comments to services and fields
4. **Design for evolution**: Wrapper responses allow adding metadata later

## Next Steps

- [Getting Started with gRPC](./grpc-getting-started.md) - Step-by-step tutorial
- [ADR 0010](../adr/0010-protobuf-grpc-code-generation.md) - Implementation details
- [Proto3 Language Guide](https://protobuf.dev/programming-guides/proto3/)
- [gRPC Core Concepts](https://grpc.io/docs/what-is-grpc/core-concepts/)

## Summary

You've learned proto3 syntax, type mapping across languages, code generation, handler implementation, error handling, and testing patterns. gRPC with Spikard gives you type-safe, high-performance APIs with cross-language support.
