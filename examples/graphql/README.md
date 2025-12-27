# GraphQL Example for Spikard

This example demonstrates how to integrate a GraphQL server with Spikard's HTTP runtime and tower-http middleware stack.

## Features

- GraphQL query execution via the Handler trait
- Integration with Spikard's HTTP server and middleware
- Support for queries, mutations, and subscriptions
- Request/response handling with proper error formats
- Zero-copy serialization for performance

## Structure

- `main.rs` - Basic GraphQL server setup
- `schema.rs` - GraphQL schema definition

## Running the Example

```bash
cargo run --example graphql
```

The server will listen on `http://localhost:8000/graphql`

## Making GraphQL Requests

Using cURL:

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ hello }"}'
```

Using GraphQL variables:

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetUser($id: ID!) { user(id: $id) { name } }",
    "variables": {"id": "123"}
  }'
```

## Integration Points

The GraphQL handler integrates with:

1. **Handler Trait** - Implements `Handler` for route handling
2. **HTTP Middleware** - Works with tower-http middleware stack
3. **Error Handling** - Follows Spikard's error response format
4. **Request/Response** - Uses Spikard's `RequestData` and response types

## Architecture

```
Client
  |
  v
Spikard HTTP Server
  |
  v
Tower-HTTP Middleware Stack
  |
  v
GraphQL Handler (implements Handler trait)
  |
  v
GraphQL Executor
  |
  v
GraphQL Schema
```

The GraphQL handler is a thin adapter that:
1. Parses the incoming HTTP request
2. Extracts the GraphQL query and variables
3. Executes the query via the GraphQL executor
4. Returns a properly formatted GraphQL response

## Configuration

Configure the GraphQL route using `GraphQLRouteConfig`:

```rust
use spikard_graphql::routes::GraphQLRouteConfig;

let config = GraphQLRouteConfig::new()
    .path("/graphql")
    .method("POST")
    .description("Main GraphQL API");
```

## See Also

- [GraphQL Handler Documentation](../../crates/spikard-graphql/src/handler.rs)
- [Route Configuration](../../crates/spikard-graphql/src/routes.rs)
- [Spikard HTTP Server](../../crates/spikard-http/README.md)
