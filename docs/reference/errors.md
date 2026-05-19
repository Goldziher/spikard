---
title: "Error Reference"
---

## Error Reference

All error types thrown by the library across all languages.

### GraphQLError

Errors that can occur during GraphQL operations

These errors are compatible with async-graphql error handling and can be
converted to structured HTTP responses matching the project's error fixtures.

| Variant | Message | Description |
|---------|---------|-------------|
| `ExecutionError` | execution error: {0} | Error during schema execution Occurs when the GraphQL executor encounters a runtime error during query execution. |
| `SchemaBuildError` | schema build error: {0} | Error during schema building Occurs when schema construction fails due to invalid definitions or conflicts. |
| `RequestHandlingError` | request handling error: {0} | Error during request handling Occurs when the HTTP request cannot be properly handled or parsed. |
| `SerializationError` | serialization error: {0} | Serialization error Occurs during JSON serialization/deserialization of GraphQL values. |
| `JsonError` | JSON error: {0} | JSON parsing error Occurs when JSON input cannot be parsed. |
| `ValidationError` | GraphQL validation error: {0} | GraphQL validation error Occurs when a GraphQL query fails schema validation. |
| `ParseError` | GraphQL parse error: {0} | GraphQL parse error Occurs when the GraphQL query string cannot be parsed. |
| `AuthenticationError` | Authentication error: {0} | Authentication error Occurs when request authentication fails. |
| `AuthorizationError` | Authorization error: {0} | Authorization error Occurs when user lacks required permissions. |
| `NotFound` | Not found: {0} | Not found error Occurs when a requested resource is not found. |
| `RateLimitExceeded` | Rate limit exceeded: {0} | Rate limit error Occurs when rate limit is exceeded. |
| `InvalidInput` | Invalid input: {message} | Invalid input error with validation details Occurs during input validation with detailed error information. |
| `ComplexityLimitExceeded` | Query complexity limit exceeded | Query complexity limit exceeded Occurs when a GraphQL query exceeds the configured complexity limit. |
| `DepthLimitExceeded` | Query depth limit exceeded | Query depth limit exceeded Occurs when a GraphQL query exceeds the configured depth limit. |
| `InternalError` | Internal server error: {0} | Internal server error Occurs when an unexpected internal error happens. |

---

### SchemaError

Error type for schema building operations

| Variant | Message | Description |
|---------|---------|-------------|
| `BuildingFailed` | Schema building failed: {0} | Generic schema building error |
| `ValidationError` | Configuration validation failed: {0} | Configuration validation error |
| `ComplexityLimitExceeded` | Query complexity limit exceeded: limit={limit}, actual={actual} | Complexity limit exceeded |
| `DepthLimitExceeded` | Query depth limit exceeded: limit={limit}, actual={actual} | Depth limit exceeded |

---
