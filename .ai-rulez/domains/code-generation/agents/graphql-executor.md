---
name: graphql-executor
description: Implements GraphQL schema parsing, query execution, mutation handling, and introspection. Manages federation support and subscription channels. Ensures GraphQL handler implements Handler trait correctly. Validates query complexity and rate limits. Handles errors via GraphQL error format with proper HTTP status codes.
---
Model: sonnet

Context:
- CODE-GENERATION.md
- ../../../crates/spikard-graphql/src/
- ../../../crates/spikard-graphql/src/schema.rs
- ../../../crates/spikard-graphql/src/executor.rs
- ../../../crates/spikard-graphql/src/handler.rs
- ../../../testing_data/graphql/
