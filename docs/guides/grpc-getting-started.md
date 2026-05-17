# Getting Started with gRPC

**Quick Start**: In 30 seconds, you'll have a working gRPC service handler.

=== "Python"
--8<-- "snippets/grpc/python/quickstart.md"

=== "TypeScript"
--8<-- "snippets/grpc/typescript/quickstart.md"

=== "Ruby"
--8<-- "snippets/grpc/ruby/quickstart.md"

=== "PHP"
--8<-- "snippets/grpc/php/quickstart.md"

=== "Rust"
--8<-- "snippets/grpc/rust/quickstart.md"

That's it! Now let's build a complete gRPC service from scratch.

---

## What is gRPC in Spikard?

Spikard's gRPC support lets you write type-safe service handlers in Python, TypeScript, Ruby, PHP, or Rust that integrate with a high-performance Rust runtime. You write handlers in your language of choice, Spikard handles the protocol details.

### Architecture

--8<-- "snippets/grpc/common/architecture_diagram.md"

**Key insight**: Your handler receives raw protobuf bytes and returns raw protobuf bytes. Spikard handles HTTP/2, routing, and status codes.

---

## Prerequisites

--8<-- "snippets/grpc/common/prerequisites.md"

---

## Step-by-Step Tutorial

### Step 1: Write a .proto File

Create `user_service.proto`:

--8<-- "snippets/grpc/proto/userservice.md"

**Proto3 Key Concepts**:

- **messages**: Data structures (like structs/classes)
- **fields**: Each has a type, name, and unique number
- **optional**: Field may or may not be present
- **repeated**: Array/list of values
- **service**: Defines RPC methods (input -> output)

### Step 2: Generate Code

--8<-- "snippets/grpc/common/codegen_commands.md"

### Step 3: Implement a Handler

=== "Python"
--8<-- "snippets/grpc/python/handler_basic.md"

=== "TypeScript"
--8<-- "snippets/grpc/typescript/handler_basic.md"

=== "Ruby"
--8<-- "snippets/grpc/ruby/handler_basic.md"

=== "PHP"
--8<-- "snippets/grpc/php/handler_basic.md"

=== "Rust"
--8<-- "snippets/grpc/rust/handler_basic.md"

### Step 4: Register the Handler

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

### Step 5: Test the Handler

=== "Python"
--8<-- "snippets/grpc/python/test_handler.md"

=== "TypeScript"
--8<-- "snippets/grpc/typescript/test_handler.md"

=== "Ruby"
--8<-- "snippets/grpc/ruby/test_handler.md"

=== "PHP"
--8<-- "snippets/grpc/php/test_handler.md"

=== "Rust"
--8<-- "snippets/grpc/rust/test_handler.md"

---

## Complete Handler Examples

Full-featured handler implementations showing routing, validation, business logic, and metadata:

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

---

## Common Patterns

### Key Patterns by Language

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

### Error Handling

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

### Status Codes Reference

--8<-- "snippets/grpc/common/status_codes_table.md"

---

## Next Steps

1. **Streaming RPCs**: Server, client, and bidirectional streaming
2. **Authentication**: Implement auth using metadata headers
3. **Observability**: Add request tracing and logging

### Learn More

- [Protobuf/gRPC Guide](./protobuf-grpc.md) - Comprehensive reference
- [Proto3 Language Guide](https://protobuf.dev/programming-guides/proto3/)
- [gRPC Core Concepts](https://grpc.io/docs/what-is-grpc/core-concepts/)

---

## Summary

You've learned:

1. **What Spikard gRPC is**: Handler-focused gRPC with a shared Rust runtime
2. **How to write .proto files**: Define messages and services
3. **Code generation**: Use protoc to generate language-specific types
4. **Handler implementation**: Deserialize -> Process -> Serialize pattern
5. **Testing**: Write comprehensive tests for your handlers

**Key Takeaway**: Spikard gRPC lets you focus on business logic. The runtime handles HTTP/2, gRPC protocol, routing, and status codes.
