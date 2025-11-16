# Ruby Bindings Review

## API Design

- **Test client architecture** – Native bindings now keep two `axum_test::TestServer` instances: the default in-memory transport for standard HTTP requests and an HTTP transport instance dedicated to WebSocket/SSE flows. This keeps synchronous handler execution on the Ruby VM thread while still allowing real socket semantics where required.
- **Runtime threading model** – The global Tokio runtime remains single-threaded so `Ruby::get()` always executes on the interpreter thread. WebSocket tasks are spawned explicitly to avoid nested runtime issues.
- **Error surface** – HTTP middleware always returns RFC7807 `ProblemDetails`. Tests were updated to assert the structured payload rather than ad-hoc hashes so all bindings align with the canonical format.

## Ruby Code Review

- `Spikard::Response` continues to act as a thin DTO. Cookie/header normalisation is covered by new unit tests to make sure no extra work leaks into Ruby handlers.
- `Spikard::Testing::TestClient` now delegates routing, compression and validation to Rust; Ruby simply shims keyword args and wraps responses. No extra JSON parsing happens on the Ruby side beyond `Response#json`.
- `Spikard::Schema.extract_json_schema` supports plain hashes, `Dry::Schema` and `Dry::Struct` exporters. Specs use lightweight stubs so the behaviour is verified without pulling the actual Dry gems.

## DTO / Dry::Schema Usage

- Dry schemas are only used to author request/response contracts. Extraction converts them to plain JSON Schema once, before being handed to Rust validators, avoiding runtime conversions per request.
- Dry structs map directly to JSON Schema definitions by inspecting attribute types. Unknown/optional attributes default to permissive object schemas, mirroring the Python/Node behaviour.

## Rust Binding Review

- `RubyHandler` already performs request/response schema validation plus JSON/body serialisation, so Ruby code only marshals the Proc result.
- `ClientInner` now separates HTTP and socket transports which keeps content-length validation aligned with other languages while still allowing live WebSocket/SSE tests.
- HTTP middleware gained unit tests around `validate_content_length`, guarding the regression we just fixed and ensuring future transport tweaks cannot silently change semantics.

## Request Builder Consolidation

- Multipart encoding logic is now implemented once in `spikard_http::testing::build_multipart_body`; Ruby, Node, and the Python native client all delegate to it, so payloads (boundary names, header ordering, files) are identical.
- URL-encoded/form encoding is also shared via `spikard_http::testing::encode_urlencoded_body`, removing the final bit of duplicated serialization from every binding.

## Follow-ups

1. Extend the schema extractor to surface Dry type metadata (format enums, arrays) when Dry exposes it.
2. Add snapshot-driven tests for streaming/SSE helpers to guard future transport toggles.
3. Evaluate whether the new in-process Python test client can cover more fixtures (e.g., WebSocket edge cases) and document when to prefer it over the subprocess runner.
