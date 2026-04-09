---
name: graphql-execution-rules
priority: high
---

GraphQL execution must:

- Parse SDL schemas correctly
- Execute queries with proper field resolution
- Validate mutations with input type schemas
- Support introspection queries
- Return errors in GraphQL format (errors array)
- Implement Handler trait for HTTP integration
- Support query complexity limits via middleware
