# Changelog

All notable changes to the Spikard project are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.10.2] - 2026-02-03

### Fixed

- **Ruby gem packaging**: Fixed `vendor-crates.sh` to patch `ext/spikard_rb/Cargo.toml` path from workspace-relative (`../../../../crates/spikard-rb`) to vendored (`../../vendor/crates/spikard-rb`). This was causing the published gem to fail installation with "failed to load manifest for dependency `spikard-rb`".

## [0.10.1] - 2026-02-02

### Performance

- **Static response fast-path with HashMap router**: Added optimized routing path for all bindings, reducing dispatch overhead for static routes.
- **Reduced per-request allocations across HTTP pipeline**: Minimized allocations in the core HTTP pipeline and all language bindings.
- **Sync benchmark handlers**: Converted benchmark handlers from async to sync where unnecessary, improving benchmark accuracy.

### Fixed

- **Python API**: Critical fixes to align Python bindings with new API surface; resolved mypy errors in SSE and testing modules.
- **Python TestClient**: Added cookies support to TestClient.
- **Node.js bindings**: Fixed build configuration and tsx dependency for benchmark app.
- **Ruby bindings**: Stabilized lockfile for CI.
- **Linting**: Applied ruff and biome auto-fixes across Python and TypeScript.

### Documentation

- **Comprehensive docs audit**: Deleted orphaned files, consolidated duplicates, expanded all binding docs (Python, TypeScript, Ruby, PHP) with full API coverage.
- **New GraphQL guide**: Added complete user guide with multi-language examples.
- **Navigation cleanup**: Added 10 missing pages to mkdocs nav, fixed 15+ broken internal links.
- **Removed WASM references**: Cleaned all user-facing docs of stale WASM mentions.
- **Benchmark results**: Updated benchmark results and added performance comparison tables to all README files.

## [0.10.0] - 2026-01-30

### Removed

- **WASM bindings**: The `spikard-wasm` crate has been removed from the workspace. WASM support will return in a future release targeting WASIp3 HTTP components.

### Fixed

- **Node.js bindings**: Resolved JSON deserialization regression in Node.js bindings.
- **Ruby bindings**: Fixed lifecycle hooks implementation and improved test coverage.

### Changed

- **Dependencies**: Updated across all ecosystems (Rust, Python, Node.js, Ruby, PHP).

## [0.9.2] - 2026-01-22

### Performance

- **Arc::try_unwrap Pattern Across All Bindings**: Eliminates unnecessary clones when Arc has unique ownership
  - Applied to Python, Ruby, PHP, and Node.js bindings for all RequestData fields
  - 30-40% reduction in FFI conversion overhead measured in Python bindings
  - Static singletons for empty collections in request_extraction.rs (shared Arc instances)
  - OnceLock caching optimizations in Python handler_request.rs (eliminated double-clone patterns)

### Changed

- **BREAKING: RequestData Value Fields Arc-Wrapped** (Phase 2 Performance Optimization)
  - `query_params: Value` → `query_params: Arc<Value>` reduces per-request clones from 24-30 to 0-6
  - `body: Value` → `body: Arc<Value>` enables zero-copy when refcount is 1 via Arc::try_unwrap
  - `validated_params: Option<Value>` → `validated_params: Option<Arc<Value>>` maintains consistency
  - All RequestData construction sites updated across spikard-http crate
  - Test helpers and fixtures updated to wrap Value fields in Arc::new()
  - This is a pre-1.0 experimental breaking change; language bindings updated in this release

### Fixed

- Ruby handler: Eliminated double-clone in validated_params handling (line 274)
- Ruby handler: Fixed raw body clone by using serde_json::from_slice directly
- PHP response: Changed with_cookies to use &mut self (PHP heap-allocated object constraint)
- All binding tests: Updated to match Arc-wrapped RequestData struct changes
- Clippy lints: Fixed ptr_cast_constness and unnecessary_option_map_or_else warnings

## [0.9.1] - 2026-01-12

### Fixed

- Added skill descriptions to AI-Rulez metadata so Codex skill YAML satisfies the required `description` field.

## [0.9.0] - 2026-01-10

### Added

- **Ruby gRPC Streaming Handlers**: Full implementation of all 4 streaming modes for Ruby bindings
  - RubyGrpcRequest/RubyGrpcResponse wrapper types for Ruby FFI
  - RubyGrpcHandler implementing GrpcHandler trait with unary, server, client, and bidirectional streaming
  - DOS protection limits (payload size, metadata entries/size, stream message count, total bytes)
  - Handler timeout (30 seconds) to prevent hung handlers
  - SAFETY-documented unsafe blocks for Ruby GVL handling
  - Error message sanitization (log full errors, return generic messages to clients)
  - Magnus Ruby FFI integration with Opaque<Value> for proper GVL management

- **PHP gRPC Streaming Handlers**: Full implementation of all 4 streaming modes for PHP bindings
  - PhpGrpcRequest/PhpGrpcResponse wrapper types for PHP FFI
  - PhpGrpcHandler implementing GrpcHandler trait
  - DOS protection and timeout handling matching Ruby implementation

- **gRPC Streaming Fixture Integration** (Phases 1-4): Complete end-to-end fixture infrastructure for all 4 streaming modes
  - 30+ JSON fixture files covering Unary, ServerStreaming, ClientStreaming, and BidirectionalStreaming modes
  - Schema validation with semantic cross-reference checks (`testing_data/grpc/schema_definitions.json`)
  - Fixture validation script (`scripts/validate_fixtures.py`) with comprehensive error reporting
  - Cross-language parity tests verifying identical behavior across all 5 languages
  - Metadata and timeout support in all gRPC clients (Python, TypeScript, Ruby, PHP, Rust)
  - Stream generators for large fixture tests (sequential, random, timestamp-based patterns)
  - Helper functions eliminating test duplication across languages
  - CI workflow (`ci-grpc-fixtures.yaml`) for automated gRPC fixture validation

- **gRPC Fixture Testing Suite** (120+ cross-language tests):
  - Python: 30+ parametrized pytest tests with fixture loading, 80%+ code coverage
  - TypeScript: 30+ vitest tests with metadata support and stream assertions
  - Ruby: 30+ RSpec tests with block-based connection management and cleanup
  - PHP: 30+ PHPUnit tests with PSR-12 compliance and 85%+ coverage
  - All tests use shared fixture files for consistent validation across ecosystems

- **gRPC server streaming support**: Full server streaming RPC implementation
  - Added `RpcMode` enum for declaring handler capabilities (Unary, ServerStreaming, ClientStreaming, BidirectionalStreaming)
  - Added `call_server_stream()` trait method to `GrpcHandler` for streaming implementations
  - Added `StreamingResponse` type with optional trailers field for response metadata after stream completion
  - GrpcRegistry now stores (handler, RpcMode) tuples for proper request routing
  - Streaming utilities: `message_stream_from_vec()`, `empty_message_stream()`, `single_message_stream()`, `error_stream()` helpers

### Changed

- **GrpcHandler trait breaking changes** (semver major):
  - Removed `supports_streaming_requests()` method (replaced by `rpc_mode()`)
  - Removed `supports_streaming_responses()` method (replaced by `rpc_mode()`)
  - Added `rpc_mode() -> RpcMode` method with default implementation returning `RpcMode::Unary`
  - Added `call_server_stream()` method with UNIMPLEMENTED default for backward compatibility
  - GrpcRegistry registration now requires RpcMode parameter: `registry.register(handler, rpc_mode)`

### Migration Guide

For existing unary handlers:
```rust
// Before
if !handler.supports_streaming_requests() {
    // route to unary
}

// After
match handler.rpc_mode() {
    RpcMode::Unary => { /* unary routing */ }
    RpcMode::ServerStreaming => { /* server streaming routing */ }
    _ => { /* other modes */ }
}
```

For implementing server streaming:
```rust
fn rpc_mode(&self) -> RpcMode {
    RpcMode::ServerStreaming
}

fn call_server_stream(
    &self,
    request: GrpcRequestData,
) -> Pin<Box<dyn Future<Output = Result<MessageStream, tonic::Status>> + Send>> {
    Box::pin(async {
        // Create and return message stream
    })
}
```


- **gRPC Client Streaming Support**: Full implementation of gRPC client streaming mode
  - `call_client_stream()` trait method for handlers receiving message streams
  - `StreamingRequest` type containing service/method names, message stream, and metadata
  - HTTP/2 gRPC frame parsing with per-message size validation
  - Smart routing dispatches ClientStreaming mode to appropriate handler
  - Frame parser enforces `max_message_size` on each message in stream (not total body)
  - Helper utilities: `parse_grpc_client_stream()` for frame parsing with validation

### Security

- **Per-message size enforcement**: Client streaming now validates each gRPC frame against `max_message_size`, preventing resource exhaustion from large individual messages in multi-message streams
- **Stream resource limits**: Handlers can return early errors without consuming entire stream, preventing memory buildup

### Migration Guide - Client Streaming

To implement a client streaming handler:

```rust
use futures_util::StreamExt;

impl GrpcHandler for MyHandler {
    fn rpc_mode(&self) -> RpcMode {
        RpcMode::ClientStreaming
    }

    fn call_client_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = GrpcHandlerResult> + Send>> {
        Box::pin(async move {
            let mut stream = request.message_stream;
            let mut total = 0;

            // Consume stream message-by-message
            while let Some(msg_result) = stream.next().await {
                match msg_result {
                    Ok(message) => {
                        // Process message (size already validated)
                        total += decode_number(&message);
                    }
                    Err(status) => {
                        // Stream error (e.g., size limit exceeded)
                        return Err(status);
                    }
                }
            }

            Ok(GrpcResponseData {
                payload: encode_result(total),
                metadata: MetadataMap::new(),
            })
        })
    }
}

// Register with RpcMode::ClientStreaming
registry.register("mypackage.MyService", Arc::new(MyHandler), RpcMode::ClientStreaming);
```

- **gRPC Bidirectional Streaming Support**: Full implementation of gRPC bidirectional streaming mode
  - `call_bidi_stream()` trait method for handlers with full-duplex message streams
  - Full-duplex communication: both client and server send streams of messages concurrently
  - Independent request and response streams with proper backpressure handling
  - Smart routing dispatches BidirectionalStreaming mode to appropriate handler
  - Reuses HTTP/2 frame parser with per-message size validation from client streaming
  - Supports chat, collaborative editing, and real-time bidirectional data flows

### Migration Guide - Bidirectional Streaming

To implement a bidirectional streaming handler:

```rust
use futures_util::StreamExt;

impl GrpcHandler for MyHandler {
    fn rpc_mode(&self) -> RpcMode {
        RpcMode::BidirectionalStreaming
    }

    fn call_bidi_stream(
        &self,
        request: StreamingRequest,
    ) -> Pin<Box<dyn Future<Output = Result<MessageStream, Status>> + Send>> {
        Box::pin(async move {
            let request_stream = request.message_stream;

            // Echo each message back as it arrives
            let response_stream = request_stream.map(|msg_result| {
                match msg_result {
                    Ok(msg) => Ok(msg), // Echo back
                    Err(e) => Err(e),
                }
            });

            Ok(Box::pin(response_stream) as MessageStream)
        })
    }
}

// Register with RpcMode::BidirectionalStreaming
registry.register("mypackage.ChatService", Arc::new(MyHandler), RpcMode::BidirectionalStreaming);
```

**Advanced pattern with async processing**:

```rust
fn call_bidi_stream(
    &self,
    request: StreamingRequest,
) -> Pin<Box<dyn Future<Output = Result<MessageStream, Status>> + Send>> {
    Box::pin(async move {
        let mut request_stream = request.message_stream;
        let (tx, rx) = tokio::sync::mpsc::channel(100);

        // Spawn task to process incoming messages asynchronously
        tokio::spawn(async move {
            while let Some(msg_result) = request_stream.next().await {
                match msg_result {
                    Ok(msg) => {
                        let response = process_message(msg);
                        let _ = tx.send(Ok(response)).await;
                    }
                    Err(e) => {
                        let _ = tx.send(Err(e)).await;
                        break;
                    }
                }
            }
        });

        // Convert mpsc receiver to MessageStream
        let stream = tokio_stream::wrappers::ReceiverStream::new(rx);
        Ok(Box::pin(stream) as MessageStream)
    })
}
```

## [0.8.3] - 2026-01-05

### Fixed

- **Workspace lints**: Added proper workspace lints configuration to eliminate unexpected `tarpaulin_include` cfg condition errors across all crates
- **Clippy warnings**: Resolved 500+ clippy warnings in core crates (spikard, spikard-core, spikard-codegen, benchmark-harness, spikard-bindings-shared) to maintain zero-warning policy
- **Language binding lifetime parameters**: Fixed lifetime parameter errors in all language binding crates (Python/PyO3, TypeScript/NAPI-RS, Ruby/Magnus, PHP/ext-php-rs, WASM/wasm-bindgen)
- **FFI binding crates**: Added comprehensive clippy allow attributes for FFI-specific crates to suppress intentional FFI-related warnings
- **Ruby vendoring**: Updated vendoring script to preserve workspace lints configuration during gem dependency vendoring
- **Code formatting**: Fixed formatting inconsistencies across all binding crates to comply with project standards

## [0.8.2] - 2026-01-02

### Fixed

- **Homebrew bottles**: Fixed bottle directory structure to include formula name and version path, resolving installation failures

## [0.8.1] - 2026-01-01

### Added

- **Homebrew bottles**: Pre-built binaries for macOS arm64 (Sonoma, Sequoia) for faster `brew install`/`upgrade`

### Fixed

- **Homebrew formula**: Fixed missing SHA256 checksum that was causing installation failures

## [0.8.0] - 2025-12-31

### Added

- **gRPC/Protobuf Support**: Full gRPC server implementation with code generation
  - Supports all 5 languages: Python, TypeScript, Ruby, PHP, Rust
  - Complete proto3 schema parsing and type generation
  - All 17 gRPC status codes with proper error handling
  - Unary RPC support (streaming modes planned for future releases)
  - FFI bindings for all language runtimes using Tonic
  - Comprehensive test coverage (672+ tests across all layers)
  - Complete documentation suite with 9 guides and 3 ADRs

- **Documentation Enhancements**: Complete documentation transformation
  - Created 239 reusable code snippets across 5 languages
  - Added comprehensive testing guide with all languages
  - Added troubleshooting and code generation guides
  - Refactored all major guides with snippet extraction (77% line reduction)
  - Achieved full language parity (Python, TypeScript, Ruby, PHP, Rust) in all guides
  - Added 9 gRPC-specific guides with complete examples
  - Split init command into quickstart and reference documentation

## [0.7.5] - 2025-12-31

### Fixed

- **Ruby bindings**: Avoid double-defining `StreamingResponse` to eliminate runtime redefinition warnings.
- **Release automation**: Use Packagist username + token when triggering refreshes and skip gracefully if missing.
- **Test apps**: Use registry-only pnpm installs, add uv fallback to pip, and validate npm/Packagist endpoints correctly.

## [0.7.4] - 2025-12-31

### Fixed

- **Ruby TestClient**: Always execute handler calls under the GVL so Ruby VM access is valid when invoked from Rust threads.

## [0.7.3] - 2025-12-31

### Fixed

- **Ruby gem vendoring**: Align tower-http features, tracing-subscriber env-filter, and tower-governor version so native builds match the workspace.
- **Cloudflare test generator**: Keep workers-types version aligned with the lockfile to avoid frozen install failures.

## [0.7.2] - 2025-12-30

### Fixed

- **Ruby vendoring**: Avoid rewriting `spikard-http` as `http` so the Ruby gem builds against the vendored crates.

## [0.7.1] - 2025-12-30

### Fixed

- **WASM test client**: Load bundled WASM bindings from the package dist output so published builds work in tests.

## [0.7.0] - 2025-12-30

### Added

- **GraphQL code generation**: Full support for generating typed GraphQL server code from schema files
  - Supports all 5 languages: Python, TypeScript, Ruby, PHP, Rust
  - Generates three output types: types, resolvers, and schema
  - Type-safe resolver signatures with proper parent/context/info parameters
  - Automatic RBS type definitions for Ruby
  - Strict type checking compliance (no `Any` types in Python, TypeScript)
  - Quality validation with mypy, TypeScript compiler, Steep, PHPStan
  - SDL schema reconstruction for runtime validation

- **`spikard init` command**: Project scaffolding for new Spikard projects
  - Supports all 5 languages: Python, TypeScript, Ruby, PHP, Rust
  - Language-specific project structure generation
  - Automatic dependency initialization (pip, npm/pnpm, gem, composer, cargo)
  - Example handler files following language-specific patterns
  - Optional schema file integration for code generation

- **Quality validation framework**: Automated validation of generated code
  - Language-specific syntax validation
  - Type checking integration (mypy, TypeScript, Steep, PHPStan)
  - Linting with native tools (Ruff, Biome, Rubocop, PHP-CS-Fixer)
  - Structured validation reports with detailed error messages

### Changed

- **Code generation architecture refactored**: All generators now use shared utilities
  - Centralized case conversion (snake_case, camelCase, PascalCase, kebab-case)
  - Unified string escaping for different contexts (JSON, GraphQL SDL, docstrings)
  - Consistent identifier sanitization with language-specific rules
  - Improved code quality and consistency across all generators

### Fixed

- **OpenAPI generators**: Critical bug fixes affecting generated code quality
  - Ruby: Fixed multi-line comment handling causing syntax errors
  - PHP: Corrected parameter ordering violations
  - TypeScript: Resolved forward reference errors in type definitions

- **OpenRPC generators**: Fixed serialization issues causing double JSON encoding

- **AsyncAPI generators**: Fixed critical type mapping issues across all languages

## [0.6.2] - 2025-12-28

### Fixed
- Version bump and test app updates for consistency

## [0.6.1] - Previous Release

See git history for detailed changes.

---

## Project Structure

### Codegen Modules
```
crates/spikard-cli/src/codegen/
├── common/              # Shared utilities (case conversion, escaping, sanitization)
├── quality/             # Quality validation framework
├── formatters/          # Language-specific formatters
├── graphql/             # GraphQL schema generators
├── openapi.rs           # OpenAPI generators
├── openrpc/             # OpenRPC generators
├── asyncapi/            # AsyncAPI generators
└── [language].rs        # Individual language generators
```

### Init Module
```
crates/spikard-cli/src/init/
├── engine.rs            # Core initialization orchestration
├── scaffolder.rs        # ProjectScaffolder trait
└── [language].rs        # Language-specific scaffolders
```

## Contributing

When adding new features or generators:

1. Use shared utilities in `codegen/common/` for case conversion and escaping
2. Validate generated code using `codegen/quality/QualityValidator`
3. Add fixtures to `testing_data/` for new scenarios
4. Update this changelog with all changes
5. Run `task test` to ensure quality gates pass

For detailed guidelines, see:
- [Code Generation Architecture](docs/adr/0004-code-generation.md)
- [Project Initialization Guide](docs/init-command.md)
- [Codegen Modernization](docs/codegen-modernization.md)
