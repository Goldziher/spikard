# spikard-graphql

GraphQL support for Spikard with async-graphql integration.

This crate provides GraphQL schema execution and integration with Spikard's HTTP server and tower-http middleware stack.

## Features

- **GraphQL Execution**: Execute queries and mutations against a schema
- **Schema Building**: Fluent API for constructing GraphQL schemas with Query, Mutation, and Subscription types
- **HTTP Integration**: Seamless integration with Spikard's HTTP runtime via the Handler trait
- **Error Handling**: Structured error responses compatible with GraphQL and HTTP specifications
- **Federation** (optional): Support for Apollo Federation via feature flag
- **Subscriptions** (optional): GraphQL subscription support via feature flag

## Modules

- `executor`: Core GraphQL execution engine
- `handler`: HTTP request handling for GraphQL queries
- `schema`: Schema builder with introspection and limit controls
- `error`: Error types and HTTP conversion

## Dependencies

- `async-graphql`: GraphQL library
- `spikard-http`: HTTP server integration
- `tokio`: Async runtime
- `serde_json`: JSON serialization
