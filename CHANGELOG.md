# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.2.1] - 2025-11-30

### Fixed

#### Python Package Distribution
- **Critical**: Fixed PyPI package architecture - Removed `python-source` configuration from `crates/spikard-py/pyproject.toml`. The correct architecture publishes two wheels under the `spikard` package name: (1) Platform-specific binary wheels containing only the `_spikard` Rust extension (built by maturin from `crates/spikard-py`), and (2) Pure Python wheel containing the `spikard` package code (built by hatchling from `packages/python`). Previous attempts to bundle both in maturin caused StripPrefixError panics during sdist generation. PyPI correctly serves both wheels when users `pip install spikard`.

#### Ruby Gem Distribution
- **Critical**: Fixed Ruby gem installation failures due to missing workspace crates - Implemented workspace crate vendoring following rb-sys best practices. Added `rake vendor:sync` task to bundle spikard-rb, spikard-core, and spikard-http source files into `packages/ruby/vendor/`. Updated `ext/spikard_rb/Cargo.toml` to reference vendored crates and gemspec to include vendor directory in gem distribution. Previous releases failed to install from RubyGems with "no such file or directory" errors for workspace path dependencies.

#### npm Package Migration
- **Changed**: Migrated npm packages to @spikard organization scope to prevent spam detection on platform-specific binaries:
  - `spikard` → `@spikard/node` (Node.js bindings)
  - `spikard-wasm` → `@spikard/wasm` (WebAssembly bindings)
  - Platform packages → `@spikard/darwin-arm64`, `@spikard/linux-x64-gnu`, `@spikard/win32-x64-msvc`
- Updated all documentation, examples, and tests to use new scoped package names
- Added migration guide (`MIGRATION-0.2.1.md`) for users upgrading from unscoped packages

#### Crates.io Compliance
- **Fixed**: Reduced keywords to 5 maximum across all crates to comply with crates.io publishing requirements:
  - `spikard`: 8→5 keywords (removed "web", "web-framework", "rest")
  - `spikard-node`: 6→5 keywords (removed "bindings")
  - `spikard-cli`: 6→5 keywords (removed "framework")
  - `spikard-py`: 6→5 keywords (removed "bindings")

#### CI/CD Improvements
- Fixed PHP Windows builds to use nightly Rust (required by ext-php-rs)
- Fixed PHP macOS linking by adding `-undefined dynamic_lookup` linker flags
- Fixed Ruby Windows builds by adding MSVC compatibility headers for `strings.h`
- Fixed Python CI tests to use `uv pip install` for proper wheel installation
- Re-enabled path filters for validated CI workflows (ci-validate, ci-rust, ci-node, ci-ruby, ci-cli, ci-php, ci-python, docs)

### Infrastructure
- Removed npm package deprecation job from publish workflow (user feedback)
- Improved documentation deployment with optimized path filters
- Enhanced sccache integration across all build jobs

## [0.1.0] - 2025-11-23

### Added

#### Core Framework (Rust)
- High-performance HTTP framework built on Axum and Tower-HTTP
- Type-safe routing with path parameter extraction
- JSON Schema validation via schemars
- RequestContext API for accessing request data (body, query, path params, headers, cookies)
- WebSocket support with trait-based handler abstraction
- Server-Sent Events (SSE) support with event producer pattern
- File upload handling with UploadFile type
- Streaming response support
- Testing utilities with in-memory TestServer
- Integration with custom Axum routers via `merge_axum_router`

#### Middleware Stack (Tower-HTTP)
- Compression middleware (gzip, brotli) with configurable quality and thresholds
- Rate limiting with per-second and burst controls
- Request timeout handling
- Request ID generation and tracking
- JWT authentication with multiple algorithm support (HS256, HS384, HS512, RS256)
- Static file serving with cache control and index file support
- CORS configuration
- Graceful shutdown handling

#### Lifecycle Hooks
- Zero-cost lifecycle hook system with Option<Arc<dyn Fn>> design
- Hook stages: `on_request`, `pre_validation`, `pre_handler`, `on_response`, `on_error`
- Support for short-circuiting with early responses
- Async hook support across all language bindings

#### OpenAPI Support
- Automatic OpenAPI 3.1 schema generation from route definitions
- Swagger UI integration at configurable path
- ReDoc integration at configurable path
- Schema extraction from schemars types and raw JSON schemas

#### Python Bindings (PyO3)
- FastAPI/Litestar-style decorator API (`@app.get`, `@post`, etc.)
- Standalone decorators (`@get`, `@post`, etc.)
- Path parameter extraction with type conversion
- Query parameter support with `Query` helper
- Request body validation with multiple backends:
  - msgspec.Struct (recommended, zero-copy, fastest)
  - Pydantic v2 BaseModel
  - dataclasses
  - TypedDict
  - NamedTuple
  - attrs classes
  - Raw JSON Schema dicts
- WebSocket support via `@websocket` decorator with async function handlers
- SSE support via `@sse` decorator with async generator pattern
- Background task execution with `background.run()`
- Custom response objects (Response, StreamingResponse)
- File upload handling with async `UploadFile.aread()`
- TestClient for integration testing with WebSocket and SSE support
- Zero-copy JSON-to-Python conversion via direct PyO3 type construction
- Async/await support via pyo3_async_runtimes
- Full type hints with mypy --strict compliance

#### Node.js Bindings (napi-rs)
- Manual route registration via `app.addRoute(metadata, handler)`
- Route metadata decorators for type safety
- Zod schema validation support
- Raw JSON Schema support
- Handler wrappers (`wrapHandler`, `wrapBodyHandler`) for parameter extraction
- Request context with query, path params, headers, form data
- File upload support with UploadFile type
- Streaming responses with async generators
- Lifecycle hooks with async callbacks
- Background task support
- TestClient with WebSocket and SSE testing utilities
- Full TypeScript type definitions generated by napi-rs
- Zero-copy FFI via ThreadsafeFunction for async JavaScript callbacks

#### Ruby Bindings (magnus)
- Sinatra-style block-based routing (`app.get`, `app.post`, etc.)
- Single request hash argument with unified interface
- Request hash structure with `:method`, `:path`, `:path_params`, `:query`, `:headers`, `:cookies`, `:body`, `:params`
- dry-schema validation support
- dry-struct validation support
- Raw JSON Schema support
- WebSocket support with class-based handlers
- SSE support with event producer classes
- Streaming responses with Enumerator
- Background task execution with `Spikard::Background.run`
- TestClient with WebSocket and SSE support
- RBS type definitions for type checking with Steep
- File upload handling with UploadFile

#### WebAssembly Bindings
- wasm-bindgen integration for browser and server-side WASM
- wasm-pack bundling support
- Optimized binary size with tree-shaking
- JavaScript/TypeScript interop with proper type safety

#### Testing Infrastructure
- Fixture-driven testing strategy with 400+ JSON fixtures
- Centralized `testing_data/` directory with categorized fixtures:
  - `headers/` - HTTP header validation scenarios
  - `cookies/` - Cookie handling scenarios
  - `json_bodies/` - Request body validation
  - `validation_errors/` - Error response formats
  - `status_codes/` - HTTP status code scenarios
  - `edge_cases/` - Edge case coverage
  - `cors/` - CORS configuration scenarios
- Schema validation for all fixture directories
- Parametrized pytest suites for comprehensive coverage
- 95% minimum Rust coverage (tarpaulin)
- 80%+ minimum coverage for Python/Node/Ruby bindings

#### Build & Automation
- Taskfile.yaml for cross-language build orchestration
- Task commands: `setup`, `build`, `test`, `lint`, `format`, `update`
- Language-specific tasks: `rust:build`, `python:build`, `node:build`, `ruby:build`, `wasm:build`
- Multi-language testing: `test:rust`, `test:python`, `test:node`, `test:ruby`
- Dependency management with committed lock files (Cargo.lock, uv.lock, pnpm-lock.yaml, Gemfile.lock)

#### Documentation
- Architecture Decision Records (ADRs) in `docs/adr/`:
  - 0001: Architecture and layering
  - 0002: Tower-HTTP middleware and configuration
  - 0003: Validation and fixtures strategy
  - 0005: Lifecycle hooks design
  - 0006: Async and streaming support
- Language-specific READMEs for Rust, Python, Node, Ruby
- Runnable examples in `examples/` for all language bindings
- Comprehensive API documentation with rustdoc, Python docstrings, JSDoc, RBS

### Changed
- N/A (initial release)

### Deprecated
- N/A (initial release)

### Removed
- N/A (initial release)

### Fixed

#### Documentation Corrections
- **Ruby README**: Fixed catastrophic handler signature errors (70% of examples incorrect)
  - Corrected from `|params, query, body|` to single `|request|` hash argument
  - Added comprehensive "Request Hash Structure" documentation
- **Node README**: Fixed Quick Start example showing non-existent auto-registration
  - Corrected to use manual `app.addRoute(metadata, handler)` pattern
  - Clarified that decorators only add metadata, don't auto-register
- **Python README**: Fixed 7 critical API mismatches
  - WebSocket: Changed from class-based to decorator-based async function
  - SSE: Changed from class with `next_event()` to async generator pattern
  - Background tasks: Fixed to use awaitable coroutine, not lambda
  - TestClient WebSocket: Corrected method names to `send()`/`recv()`
  - UploadFile: Added async `aread()` example
  - OpenApiConfig: Fixed import path to `from spikard.config`
  - SSE testing: Corrected event iteration pattern
- **Rust README**: Fixed 3 type errors in configuration examples
  - `StaticFilesConfig.index_file`: Corrected from `Option<bool>` to `bool`
  - `OpenApiConfig` paths: Corrected from `Option<String>` to `String`
  - Trait implementations: Fixed syntax from `async fn` to `-> impl Future`

### Security
- JWT validation with configurable algorithms and claims verification
- Secure cookie defaults (HttpOnly, Secure, SameSite) in Ruby bindings
- Header and cookie validation against fixture-based schemas
- Input validation at system boundaries (user input, external APIs)
- CORS middleware with configurable allowed origins, methods, headers

### Known Limitations
- PHP bindings planned but not yet implemented (ext-php-rs integration pending)
- OpenAPI documentation generation requires schemars types (raw JSON schemas supported but don't contribute to OpenAPI)
- WebAssembly bindings have limited threading support due to WASM runtime constraints
- Background tasks in Python require explicit awaitable coroutines (no automatic lambda wrapping)

## [0.2.1] - 2025-11-30

### Changed

#### Breaking Changes - npm Package Scope Migration

All npm packages have been migrated to the `@spikard` organization scope:

- **Node.js bindings**: `spikard` → `@spikard/node`
- **WebAssembly bindings**: `spikard-wasm` → `@spikard/wasm`
- **Platform packages**: Automatically scoped under `@spikard` (e.g., `@spikard/darwin-arm64`, `@spikard/linux-x64-gnu`)

**Migration required for TypeScript/JavaScript users:**
```bash
# Old installation
npm install spikard
npm install spikard-wasm

# New installation
npm install @spikard/node
npm install @spikard/wasm
```

Update all imports:
```typescript
// Before
import { Spikard } from 'spikard';
import * as wasm from 'spikard-wasm';

// After
import { Spikard } from '@spikard/node';
import * as wasm from '@spikard/wasm';
```

See [MIGRATION-0.2.1.md](MIGRATION-0.2.1.md) for detailed migration instructions.

**Rationale:**
- Prevents npm spam detection on platform-specific packages
- Establishes proper package organization under `@spikard` namespace
- Follows napi-rs best practices for native Node.js addons

**Other language bindings unchanged:**
- Python: `pip install spikard` (no change)
- Ruby: `gem install spikard` (no change)
- PHP: `composer require spikard/spikard` (no change)
- Rust: `cargo add spikard` (no change)

### Deprecated

- `spikard@0.2.0` on npm (use `@spikard/node@0.2.1`)
- `spikard-wasm@0.2.0` on npm (use `@spikard/wasm@0.2.1`)

## [0.2.0] - TBD

### Added

#### PHP Bindings (ext-php-rs)
- Complete PHP 8.2+ bindings via ext-php-rs with 100% test coverage
- Native HTTP server with full middleware stack support
- Parameter extraction helpers (Query, Path, Header, Cookie, Body)
- Lifecycle hooks implementation (on_request, pre_validation, pre_handler, on_response, on_error)
- Streaming responses and WebSocket support
- Server-Sent Events (SSE) support with producer pattern
- Background task infrastructure with message queue pattern and serialization
- Dependency injection with thread-local architecture
- AsyncAPI integration and test generation
- PHP app generator for benchmark harness
- Comprehensive test coverage with native PHP test client
- Full configuration API support matching Python/Node/Ruby bindings

#### Dependency Injection (DI)
- Cross-language DI framework implementation across all bindings
- Handler parameter extraction with type-safe context passing
- Language-specific DI container implementations:
  - Python: PyO3 integration with proper GIL handling
  - Node.js: napi-rs with ThreadsafeFunction support
  - Ruby: Magnus with proper memory management
  - PHP: ext-php-rs with Zval thread-safety
- 100% test pass rates achieved across all bindings (18/18 Python, 442/442 Node, 478/478 Ruby, 100+ PHP)
- Request context exposure to handlers for accessing parsed parameters

#### Documentation & Examples
- MkDocs site setup with comprehensive playbooks and guides
- Architecture diagram and reference matrices
- Binding support matrix and feature parity audit
- Playbooks for authentication, uploads, streaming, background tasks, and error handling
- Runnable examples for all bindings including WASM/Deno
- Comprehensive README updates for all language bindings with testable snippets
- PHP language standards and agent configuration

### Changed

#### PHP Implementation
- Parameter extraction system now utilizes DI framework for type-safe handler access
- Streaming and WebSocket handlers refined for production stability
- Background task system improved with proper serialization and error handling
- Thread-local architecture adopted for DI container management

#### Build & Testing
- PHP build and test tasks fully enabled with native extension loading
- E2E generation now supports PHP with schema-aware generation
- Test generator enhanced for PHP with ServerConfig, schema validation, and response verification

### Fixed

#### PHP Bindings
- Threading issues resolved - proper async handling without deadlocks
- ARM Mac compilation errors fixed with runtime feature enablement
- Zval thread safety achieved through thread-local architecture
- Parameter passing in background tasks corrected
- Query string handling and null body processing fixed
- All test failures resolved (query strings, streaming, WebSocket)

#### Bindings Integration
- Python server config properly propagates DI container
- Node DI context correctly flows into handlers
- Ruby DI snippet evaluation bound to local app
- API compatibility across all bindings standardized

#### Build System
- Biome formatting alignment across codebase
- Lint issues addressed in all binding implementations
- PHP e2e generator headers normalization and proper escaping
- Dependency management stabilized with locked versions

#### Documentation
- PHP snippet API updated to match actual implementation
- Handler signatures corrected across all binding READMEs
- TypeScript examples aligned with current request shape
- WebSocket support status clarified
- Background task caveats documented

### Security
- Zval thread-safety in PHP bindings ensures safe concurrent access
- Proper error propagation across FFI boundaries prevents panic leakage
- DI container thread-local storage prevents cross-request contamination

### Infrastructure
- Dependency updates: hono and various GitHub Actions
- Extended testing coverage across all bindings
- CI/CD setup for PHP native extension building

## [0.1.1] - 2025-11-23

### Added
- Homebrew formula publishing automation in release workflow
- Automatic Homebrew tap updates via `mislav/bump-homebrew-formula-action@v3`
- `homebrew` target to release workflow (can be used with `targets: all` or `targets: homebrew`)

### Fixed
- Package publishing configuration for all language bindings
- README.md inclusion in Python, Node.js, and WASM package manifests
- CLI crate (`spikard-cli`) now properly configured for crates.io publishing

## [Unreleased]

### Planned
- PHP bindings via ext-php-rs
- Enhanced OpenAPI customization options
- GraphQL support
- Additional middleware: CSRF protection, session management
- Performance benchmarks and comparison documentation

---

[0.1.1]: https://github.com/Goldziher/spikard/releases/tag/v0.1.1
[0.1.0]: https://github.com/Goldziher/spikard/releases/tag/v0.1.0
