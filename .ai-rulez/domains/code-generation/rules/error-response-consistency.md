---
name: error-response-consistency
priority: high
---

Generated handlers must return consistent error responses:

- OpenAPI: ProblemDetails (application/problem+json)
- GraphQL: GraphQL errors array with extensions
- AsyncAPI: Channel error messages
- OpenRPC: JSON-RPC 2.0 error object with code/message/data
