# Validation Engine

Validation is contract-first: JSON Schema shapes everything from handler signatures to generated clients and tests.

## Principles
- **Schema everywhere** – request bodies, params, headers, cookies, and responses validate against JSON Schema.
- **Language-native types** – msgspec/Pydantic (Python), Zod/TypeScript types, serde (Rust), and RBS (Ruby) stay aligned with the canonical schema.
- **Fail fast** – invalid inputs never reach handlers; responses are checked before being sent back.

## Sources of Truth
- **Code-first**: derive schemas from DTOs in each language and register them with the runtime.
- **Spec-first**: feed OpenAPI/AsyncAPI into the CLI to generate DTOs/handlers, fixtures, and contract tests.

## Error Model
- Standardized error payloads with RFC 9457 alignment
- Clear pointer paths to failing fields and a consistent status code strategy (400/422 for validation, 500 for unhandled errors)

For implementation notes and trade-offs, see [ADR 0003](../adr/0003-validation-and-fixtures.md).
