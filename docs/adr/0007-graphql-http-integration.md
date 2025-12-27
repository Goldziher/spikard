# ADR 0007: GraphQL HTTP Integration

**Status:** Accepted

**Date:** 2025-12-27

**Deciders:** AI Assistant

**Type:** Feature Integration

## Context

Spikard provides a powerful, multi-language toolkit for building typed web services. GraphQL is a popular query language and runtime for APIs that complements REST and JSON-RPC. The spikard-graphql crate already provides a high-level GraphQL implementation with async-graphql integration, but it was not formally integrated with the spikard-http HTTP server.

This ADR documents how GraphQL support is integrated into Spikard's HTTP runtime while maintaining:
- Zero circular dependencies in the crate dependency graph
- Compatibility with tower-http middleware stack
- The Handler trait abstraction for language-agnostic routing
- Consistent error handling across platforms

## Decision

We integrate GraphQL with spikard-http by:

1. **Placing integration utilities in spikard-graphql** - The routes module lives in spikard-graphql (which depends on spikard-http), not in spikard-http itself, avoiding circular dependencies.

2. **Leveraging existing Handler trait** - GraphQLHandler already implements Handler, enabling transparent integration with Spikard's HTTP server and middleware stack.

3. **Providing route configuration builder** - GraphQLRouteConfig in the routes module provides a declarative way to configure GraphQL endpoints.

4. **Maintaining middleware compatibility** - All tower-http middleware (compression, rate limiting, request IDs, etc.) applies transparently to GraphQL requests.

## Consequences

### Positive

- **No circular dependencies** - spikard-graphql can safely depend on spikard-http without creating dependency cycles
- **Clean architecture** - Integration code lives in the crate that owns the GraphQL implementation
- **Handler reuse** - GraphQLHandler implements Handler from day one (no wrapper needed)
- **Middleware transparency** - All HTTP middleware works with GraphQL routes automatically
- **First-class citizen** - GraphQL routes are registered exactly like REST routes via Router
- **Type safety** - GraphQLRouteConfig uses builder pattern for declarative configuration
- **Language-agnostic** - Integration strategy works for all language bindings (Python, Node, Ruby, PHP, WASM)

### Negative

- **Slight import discrepancy** - Developers must import from spikard_graphql::routes instead of spikard_http::graphql
  - Mitigation: Clear documentation and examples in each location

### Neutral

- **Optional dependency** - While spikard-graphql doesn't depend on spikard-http optionally, routes are unconditionally available to users who depend on both
  - This is acceptable because spikard-graphql requires spikard-http; no added optionality is needed

## Implementation

### Core Components

1. **GraphQLHandler** (existing) - Implements Handler trait for GraphQL requests
2. **GraphQLRouteConfig** (new) - Declarative configuration builder
3. **routes module** (new) - Integration utilities and documentation

### Request Flow

```
HTTP POST /graphql
    ↓
Tower-HTTP Middleware (compression, rate limiting, auth, etc.)
    ↓
Spikard Router.route_request()
    ↓
GraphQLHandler::call() (implements Handler)
    ↓
Parse GraphQL request (query, variables, operation name)
    ↓
GraphQLExecutor::execute()
    ↓
Format GraphQL response (data + errors)
    ↓
HTTP Response (200 OK with GraphQL JSON)
```

### Example Integration

```rust
use spikard_graphql::{GraphQLHandler, GraphQLExecutor, QueryOnlyConfig, SchemaBuilder};
use spikard_http::{Handler, Route, Router, Server, ServerConfig};
use std::sync::Arc;

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

let config = ServerConfig::builder().port(8000).build();
let server = Server::new(config, router);
server.run().await?;
```

## Error Handling

GraphQL errors follow the GraphQL specification while respecting HTTP conventions:

| Error Type | HTTP Status | Example |
|-----------|------------|---------|
| Invalid JSON | 400 Bad Request | Invalid request body |
| GraphQL validation | 200 OK | Invalid query syntax (per GraphQL spec) |
| Query execution | 200 OK | Field not found, type mismatch (per GraphQL spec) |
| Internal server error | 500 Internal Server Error | Unexpected panic or resource exhaustion |

The GraphQL response always includes an `errors` field when applicable, following the GraphQL over HTTP specification.

## Testing

The integration is validated through:

1. **Unit tests** in routes.rs for GraphQLRouteConfig
2. **Handler tests** existing in handler.rs (unchanged)
3. **Integration examples** in examples/graphql/
4. **Fixture-based tests** in testing_data/graphql/ (following project convention)

## Alternative Approaches Considered

### 1. Add GraphQL as optional feature in spikard-http
**Rejected** because:
- Creates optional dependency coupling
- spikard-http doesn't own GraphQL domain
- Adds bloat to spikard-http if feature is enabled

### 2. Create separate spikard-http-graphql crate
**Rejected** because:
- GraphQL already depends on spikard-http
- Would create unnecessary indirection
- Splits related functionality across crates

### 3. Keep everything in spikard-graphql (chosen)
**Selected** because:
- spikard-graphql already depends on spikard-http
- Clean separation: routing helpers in domain crate
- No circular dependencies
- Follows project layering pattern

## Related Decisions

- **ADR 0002: Runtime and Middleware** - tower-http middleware stack
- **ADR 0001: Architecture and Layering** - Thin binding pattern

## Future Enhancements

1. **GraphQL Playground UI** - Optional endpoint at /graphql/ui (when GraphQLRouteConfig.enable_playground = true)
2. **Apollo Federation support** - Via spikard-graphql federation feature
3. **Subscription support** - WebSocket integration (already supported in spikard-http)
4. **Persisted queries** - Query ID mapping for reduced payload size
5. **Complexity analysis** - Configurable complexity limits per query
6. **Language-specific bindings** - GraphQL route helpers for Python/Node/Ruby/PHP (thin adapters around Rust implementation)

## References

- [GraphQL over HTTP spec](https://graphql.org/learn/serving-over-http/)
- [async-graphql crate](https://docs.rs/async-graphql/)
- [Tower-HTTP middleware](https://github.com/tower-rs/tower-http)
- [Spikard Handler trait](../../crates/spikard-http/src/handler_trait.rs)
