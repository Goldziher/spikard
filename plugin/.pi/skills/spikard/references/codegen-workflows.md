<!--
AI-RULEZ :: GENERATED FILE — DO NOT EDIT
Content-Hash: blake3:4c716e43c759cf1965d31e5562dd3af2f2df993acfee6051c7ba481373431751
Source-Hash: blake3:abccd7eb49522b31a5fd250dd2cb28fb991f8bdb6528b9fd032c3d50821ccf1b
Schema-Version: v1
-->

# Spikard Codegen Workflows

Use Spikard as a schema-to-code generator first. The examples use the repository checkout's `testing_data/schemas/` fixtures. For standalone skill installations, replace those paths with your project's schema files.

## OpenAPI

Use for REST handler scaffolding.

```bash
spikard generate openapi testing_data/schemas/todo-api.openapi.yaml --lang python
spikard generate openapi testing_data/schemas/todo-api.openapi.yaml --lang typescript --dto zod --output app.ts
```

## AsyncAPI

Use for SSE or WebSocket handler scaffolding, fixtures, and test apps.

```bash
spikard generate asyncapi testing_data/schemas/chat-service.asyncapi.yaml --lang ruby --output app.rb
spikard testing asyncapi fixtures testing_data/schemas/chat-service.asyncapi.yaml --output testing_data
spikard testing asyncapi test-app testing_data/schemas/chat-service.asyncapi.yaml --lang python --output app.py
```

## OpenRPC / JSON-RPC

Use for JSON-RPC 2.0 handler generation.

```bash
spikard generate jsonrpc testing_data/schemas/calculator.openrpc.yaml --lang python --output handlers.py
```

## GraphQL

Use for types, resolvers, or schema output.

```bash
spikard generate graphql testing_data/schemas/blog.graphql --lang rust --target all --output generated.rs
```

## Protobuf

Use for messages and gRPC services.

```bash
spikard generate protobuf testing_data/schemas/greeter.proto --lang php --output src/Proto.php
spikard generate protobuf testing_data/schemas/greeter.proto --lang python --include testing_data/schemas/includes --output generated.py
```

## PHP DTO Helpers

Use when a PHP integration needs Spikard request/response helper classes.

```bash
spikard generate php-dto --output src/Generated
```

When an agent is driving generation repeatedly, prefer the MCP server so file outputs come back as structured tool results.

## SQL-to-HTTP

Annotate named SQL queries with `-- @http GET /items` (or the appropriate method and route), supply schema DDL, then generate route sidecars and an OpenAPI 3.1 document:

```bash
spikard generate sql queries --schema schema.sql --lang python --output generated
```

Choose the dialect explicitly when it is not PostgreSQL. Query data access remains in the generated language adapter; the HTTP runtime consumes the generated routes. Inspect output files before integrating them. Use `--strict` to reject unsupported neutral types instead of an any-JSON fallback.
