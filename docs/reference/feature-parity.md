# Feature parity matrix

Spikard is a Rust core plus 14 language bindings. This matrix shows which server-side features are available in each, with the core itself listed first for comparison.

| Binding | Server | HTTP Routing | WebSocket | SSE | gRPC | GraphQL | JSON-RPC | OpenAPI | JWT Auth | API Key Auth | CORS | Compression | Rate Limit | Static Files | TestClient |
|---------|--------|-----|-----------|-----|------|---------|----------|---------|----------|--------------|------|-------------|------------|--------------|------------|
| Rust | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Python | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Node/TS | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Ruby | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| PHP | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Elixir | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Go | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Java | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| C# | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Kotlin | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Dart | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Swift | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| Zig | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| C FFI | ✅ | ✅ | ✅ | ✅ | ⚠️ | ⚠️ | ⚠️ | ⚠️ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| WASM | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |

## Legend

- ✅ **Fully supported** — feature is available with idiomatic bindings in the language.
- ⚠️ **Partial** — feature is available but may require manual setup or schema definition not yet codegen'd.
- ❌ **Not yet** — feature exists in Rust core but no binding implementation.

## Notes on partial support

**gRPC, GraphQL, JSON-RPC, OpenAPI** across all bindings except Rust are marked ⚠️ because:

- Rust core has full implementations of all handlers and schema validators.
- Bindings inherit server routing and middleware through the FFI layer.
- Schema codegen (protobuf → Rust → language stubs) is WIP. Bindings support registering handlers and returning responses; code generation of client stubs and schema validation is being rolled out incrementally.

**TestClient** is marked ❌ for C FFI because the C ABI provides no idiomatic way to construct test fixtures; the parent language (Go, C#, Rust, etc.) using the C FFI should wrap testing via its own test utilities.

**WASM** is client-side only. No server binding exists; the package provides type stubs and serialization helpers for talking to a remote Spikard server.

## Feature deep-dives

See language-specific binding references for handler syntax, error handling, and middleware attachment:

- [Python API](api-python.md)
- [TypeScript / Node API](api-typescript.md)
- [Ruby API](api-ruby.md)
- [PHP API](api-php.md)
- [Elixir API](api-elixir.md)
- [Go API](api-go.md)
- [Java API](api-java.md)
- [C# API](api-csharp.md)
- [Rust API](api-rust.md)
