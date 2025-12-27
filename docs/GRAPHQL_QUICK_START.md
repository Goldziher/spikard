# GraphQL Integration - Quick Start Guide

## Overview

Spikard now provides seamless GraphQL integration through the `spikard-graphql` crate, which works with the `spikard-http` HTTP server and all tower-http middleware.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
spikard-graphql = "0.6"
spikard-http = "0.6"
spikard-core = "0.6"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"
```

## Minimal Example

```rust
use spikard_graphql::{GraphQLHandler, GraphQLExecutor, QueryOnlyConfig, SchemaBuilder};
use spikard_http::{Handler, Route, Router, Server, ServerConfig};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct Query;

impl Query {
    pub fn hello(&self) -> String {
        "Hello GraphQL!".to_string()
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create schema
    let schema = SchemaBuilder::new()
        .query_only(QueryOnlyConfig::builder().build())
        .build();

    // Create handler
    let executor = Arc::new(GraphQLExecutor::<Query, (), ()>::new(schema));
    let handler = Arc::new(GraphQLHandler::new(executor));

    // Register route
    let route = Route::new(
        "/graphql".to_string(),
        spikard_http::Method::Post,
        handler as Arc<dyn Handler>,
    );

    // Start server
    let mut router = Router::new();
    router.register_route(route);

    let config = ServerConfig::builder()
        .host("127.0.0.1")
        .port(8000)
        .build();

    Server::new(config, router).run().await?;

    Ok(())
}
```

## Testing the Server

```bash
# Simple query
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{"query":"{ hello }"}'

# Expected response:
# {"data":{"hello":"Hello GraphQL!"}}
```

## Configuration Options

### Route Configuration

Use `GraphQLRouteConfig` for declarative setup:

```rust
use spikard_graphql::routes::GraphQLRouteConfig;

let config = GraphQLRouteConfig::new()
    .path("/api/v1/graphql")
    .method("POST")
    .description("GraphQL API")
    .enable_playground(false);
```

### Server Configuration

Enable middleware with `ServerConfig`:

```rust
use spikard_http::{ServerConfig, CompressionConfig, RateLimitConfig};

let config = ServerConfig::builder()
    .host("0.0.0.0")
    .port(3000)
    .compression(Some(CompressionConfig::default()))
    .rate_limit(Some(RateLimitConfig::default()))
    .enable_http_trace(true)
    .build();
```

## Advanced Topics

### Adding Mutations

```rust
#[derive(Debug, Clone)]
pub struct Mutation;

impl Mutation {
    pub fn create_user(&self, name: String) -> String {
        format!("Created user: {}", name)
    }
}

let schema = SchemaBuilder::new()
    .query_mutation(QueryMutationConfig::builder()
        .build())
    .build();
```

### Using GraphQL Variables

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query GetUser($id: ID!) { user(id: $id) { name } }",
    "variables": {"id": "123"}
  }'
```

### Multiple Operations

Specify which operation to execute:

```bash
curl -X POST http://localhost:8000/graphql \
  -H "Content-Type: application/json" \
  -d '{
    "query": "query A { users { id } } query B { posts { title } }",
    "operationName": "A"
  }'
```

## Error Handling

GraphQL errors follow the GraphQL specification:

```json
{
  "data": null,
  "errors": [
    {
      "message": "Field 'unknown' doesn't exist on type 'Query'",
      "locations": [{"line": 1, "column": 3}]
    }
  ]
}
```

## Integration with Middleware

All tower-http middleware works transparently:

```rust
let config = ServerConfig::builder()
    // Compression (gzip, brotli)
    .compression(Some(CompressionConfig::default()))
    // Rate limiting
    .rate_limit(Some(RateLimitConfig::default()))
    // Request timeout
    .request_timeout(Some(30))
    // Enable tracing
    .enable_http_trace(true)
    // Custom headers
    // JWT/API key auth
    .build();
```

## Testing with GraphQL

### Unit Testing

```rust
#[test]
fn test_query() {
    let query = Query;
    assert_eq!(query.hello(), "Hello GraphQL!");
}
```

### Integration Testing

Use fixtures from `testing_data/graphql/`:

```bash
cargo test -p spikard-graphql
```

## Real-World Example

See complete example with error handling, mutations, and middleware in:
- `examples/graphql/main.rs`
- `examples/graphql/README.md`

## Architecture

```
Client Request
    ↓
Spikard HTTP Server
    ↓
Tower-HTTP Middleware
    ↓
GraphQL Handler (Router)
    ↓
GraphQL Executor
    ↓
Resolver Functions
    ↓
JSON Response
```

## Key Types

| Type | Purpose |
|------|---------|
| `GraphQLHandler<Q, M, S>` | Implements Handler trait for HTTP routing |
| `GraphQLExecutor<Q, M, S>` | Executes queries against schema |
| `GraphQLRouteConfig` | Declares route configuration |
| `SchemaBuilder` | Builds GraphQL schema |

## Common Patterns

### Multiple Endpoints

```rust
// API endpoint
let api_route = Route::new(
    "/api/graphql".to_string(),
    spikard_http::Method::Post,
    api_handler as Arc<dyn Handler>,
);

// Admin endpoint
let admin_route = Route::new(
    "/admin/graphql".to_string(),
    spikard_http::Method::Post,
    admin_handler as Arc<dyn Handler>,
);

router.register_route(api_route);
router.register_route(admin_route);
```

### Combined with REST

GraphQL and REST endpoints can coexist:

```rust
// REST endpoint
let rest_route = Route::new(
    "/api/users".to_string(),
    spikard_http::Method::Get,
    rest_handler as Arc<dyn Handler>,
);

// GraphQL endpoint
let graphql_route = Route::new(
    "/api/graphql".to_string(),
    spikard_http::Method::Post,
    graphql_handler as Arc<dyn Handler>,
);

router.register_route(rest_route);
router.register_route(graphql_route);
```

## Performance

- **Zero-copy serialization** - Efficient JSON conversion
- **Async-first** - Full tokio async/await support
- **Middleware optimized** - All middleware applied at transport layer
- **Lazy validation** - Only validate requested fields

## Documentation

- **API Docs:** `cargo doc -p spikard-graphql --open`
- **Architecture:** `docs/adr/0007-graphql-http-integration.md`
- **Examples:** `examples/graphql/`
- **Tests:** `cargo test -p spikard-graphql`

## Troubleshooting

### GraphQL query not working

1. Check route path matches request URL
2. Verify HTTP method is POST
3. Ensure `Content-Type: application/json` header is set
4. Check server logs with `RUST_LOG=debug`

### Middleware not applied

Middleware is applied at the HTTP layer before routing. All middleware automatically applies to GraphQL routes.

### Performance issues

1. Enable release builds: `cargo build --release`
2. Use compression: `CompressionConfig::default()`
3. Monitor with HTTP trace: `enable_http_trace(true)`
4. Check GraphQL complexity limits

## FAQ

**Q: Can I use GraphQL with Python/Node/Ruby bindings?**
A: Yes! The GraphQL handler works transparently through all language bindings.

**Q: Can I combine GraphQL with other route handlers?**
A: Yes! GraphQL routes are registered just like REST routes.

**Q: Is there a playground UI?**
A: Enable with `GraphQLRouteConfig::enable_playground(true)` (future enhancement).

**Q: Can I use subscriptions?**
A: Yes, via WebSocket. See `websocket_handler` in spikard-http.

**Q: How do I secure my GraphQL endpoint?**
A: Use `jwt_auth`, `api_key_auth`, or custom middleware in `ServerConfig`.

## Next Steps

1. **Read** [ADR 0007](docs/adr/0007-graphql-http-integration.md) for architecture details
2. **Run** the example: `cargo run --example graphql`
3. **Test** with curl or GraphQL client (Insomnia, Postman, Apollo Studio)
4. **Integrate** into your application
5. **Monitor** with HTTP tracing and metrics

## Support

- **Issues:** Report bugs in GitHub issues
- **Documentation:** Check docs/adr/ and README.md files
- **Examples:** See examples/graphql/ for working code
- **Tests:** Run `cargo test` for validation

---

Happy GraphQL development with Spikard!
