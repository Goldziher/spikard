# Types Reference

The runtime normalizes request/response types so bindings can stay in sync. Use these shapes to model DTOs and to align generated code.

## Request Context
- **Method/Path**: HTTP verb and path template
- **Params**: path params with optional converters (int, uuid, etc.)
- **Query**: map/object of query params
- **Headers/Cookies**: case-insensitive accessors
- **Body**: JSON by default; form/multipart helpers per binding

## Responses
- **JSON**: `Json<T>` in Rust, plain objects/structs in other bindings
- **Streams**: async iterators/generators for chunked responses
- **Errors**: typed error envelopes with status codes

## DTOs
- **Python**: msgspec `Struct`, optional Pydantic/dataclasses
- **TypeScript**: Zod schemas with inferred types
- **Ruby**: RBS signatures and runtime validators
- **Rust**: serde structs + JSON Schema derivation (planned)

## Errors Shape
See [Errors](errors.md) for the canonical response body and RFC 9457 mapping.
