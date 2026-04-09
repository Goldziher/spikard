---
name: openapi-codegen-rules
priority: high
---

OpenAPI code generation must:

- Convert Route definitions to OpenAPI 3.0 spec
- Generate parameter validators from JSON schemas
- Support Swagger UI and ReDoc UI
- Include security schemes for auth middleware
- Generate server URLs from ServerInfo config
- Produce valid OpenAPI that passes validation
