# Spikard Codegen Workflows

Use Spikard as a schema-to-code generator first.

## OpenAPI

Use for REST handler scaffolding.

```bash
spikard generate openapi examples/schemas/todo-api.openapi.yaml --lang python
spikard generate openapi examples/schemas/todo-api.openapi.yaml --lang typescript --dto zod --output app.ts
```

## AsyncAPI

Use for SSE or WebSocket handler scaffolding, fixtures, and test apps.

```bash
spikard generate asyncapi examples/schemas/chat-service.asyncapi.yaml --lang ruby --output app.rb
spikard testing asyncapi fixtures examples/schemas/chat-service.asyncapi.yaml --output testing_data
spikard testing asyncapi test-app examples/schemas/chat-service.asyncapi.yaml --lang python --output app.py
```

## OpenRPC / JSON-RPC

Use for JSON-RPC 2.0 handler generation.

```bash
spikard generate jsonrpc examples/schemas/calculator.openrpc.yaml --lang python --output handlers.py
```

## GraphQL

Use for types, resolvers, or schema output.

```bash
spikard generate graphql examples/schemas/blog.graphql --lang rust --target all --output generated.rs
```

## Protobuf

Use for messages and gRPC services.

```bash
spikard generate protobuf examples/schemas/greeter.proto --lang php --output src/Proto.php
spikard generate protobuf examples/schemas/greeter.proto --lang python --include examples/schemas/includes --output generated.py
```

## PHP DTO Helpers

Use when a PHP integration needs Spikard request/response helper classes.

```bash
spikard generate php-dto --output src/Generated
```

When an agent is driving generation repeatedly, prefer the MCP server so file outputs come back as structured tool results.
