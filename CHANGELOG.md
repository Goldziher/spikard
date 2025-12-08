# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.4] - 2025-12-08

### Fixed

#### Ruby Gem
- **Native extension build**: Fixed critical workspace inheritance issue in vendored crates by replacing `version.workspace = true`, `edition.workspace = true`, and other workspace-inherited fields with explicit hardcoded values in `packages/ruby/vendor/crates/spikard-core/Cargo.toml` and `packages/ruby/vendor/crates/spikard-http/Cargo.toml`. This ensures the gem can build successfully when installed from RubyGems.org, as published gems don't preserve the workspace root context (v0.3.3 was completely unusable due to Cargo failing to find workspace root manifest during native extension compilation).

## [0.3.3] - 2025-12-08

### Fixed

#### Ruby Gem
- **Native extension build**: Updated `packages/ruby/ext/spikard_rb/Cargo.toml` to use vendored crates path (`../../vendor/crates/spikard-rb`) instead of workspace-relative path, allowing the gem to build successfully when installed from RubyGems

#### Benchmark Harness
- **PHP benchmark app**: Added `spikard/spikard` package dependency to `tools/benchmark-harness/apps/spikard-php/composer.json` to properly load the Spikard extension
- **PHP benchmark app**: Converted anonymous handler classes to named classes in `tools/benchmark-harness/apps/spikard-php/server.php` to fix FFI callable compatibility (anonymous classes cannot be properly called across the PHP-Rust FFI boundary)
- **WASM benchmark app**: Updated `tools/benchmark-harness/apps/spikard-wasm/package.json` to use TypeScript (`server.ts`) with `tsx` runtime instead of JavaScript
- **WASM benchmark harness**: Updated `tools/benchmark-harness/src/framework.rs` to detect and run `server.ts` instead of `server.js` for spikard-wasm framework
- **Node benchmark app**: Added `@spikard/node` package dependency to `tools/benchmark-harness/apps/spikard-node/package.json` and removed manual native binding loading logic
- **Node benchmark app**: Converted to ES modules (`"type": "module"`) and updated all imports to use ESM syntax for consistency with modern Node.js and WASM benchmark apps
- **Ruby benchmark app**: Updated `tools/benchmark-harness/apps/spikard-ruby/Gemfile` to use published `spikard` gem instead of local path dependency

## [0.3.2] - 2025-12-08

### Fixed

#### Package Distribution
- **Node package**: Added `--platform` flag to napi build commands to generate platform-suffixed .node binaries (e.g., `spikard-node.darwin-arm64.node`), ensuring native bindings are included in published npm packages
- **Ruby gem**: Included vendored Rust workspace crates (`spikard-rb`, `spikard-core`, `spikard-http`) in gem package by updating .gitignore to allow `packages/ruby/vendor/crates/`, fixing gem installation failures

## [0.3.1] - 2025-12-07

### Fixed

#### Release Infrastructure
- **Node packaging**: Fixed package-artifacts scripts to only verify the platform-specific .node file for the current build target instead of checking all platform directories
- **Python wheels**: Corrected wheel artifact paths to use workspace root `target/wheels` instead of `packages/python/target/wheels` to match maturin's actual output location
- **Version synchronization**: All package versions now properly synchronized to 0.3.1 across Cargo.toml, package.json, pyproject.toml, composer.json, and Ruby version files

## [0.3.0] - 2025-12-07

### Added

#### Coverage Enforcement & Testing Infrastructure
- **Coverage thresholds**: Enforced 95% minimum for Rust core, 80%+ for all language bindings (Python, Node, Ruby, PHP, WASM) with CI job failures when below threshold
- **Comprehensive test coverage**: Expanded test suites across all bindings achieving 97.96% coverage in shared infrastructure (`crates/spikard-bindings-shared`)
- **WASM test coverage**: Added 96% test coverage for WebAssembly bindings with dedicated test modules
- **Behavioral tests**: 78 new tests for HTTP handlers covering WebSocket, SSE, background tasks, and multipart uploads; 19 behavioral tests for PHP ControllerMethodHandler; FFI and dependency injection coverage improvements
- **Test infrastructure**: Reorganized test modules into dedicated testing directories; created shared infrastructure for lifecycle hook and config extraction testing across all bindings

#### CI/CD Improvements
- **Workflow stabilization**: Fixed all CI test failures across Rust, Python, Node, Ruby, PHP, and WASM workflows with dedicated language-specific coverage jobs
- **Windows compatibility**: Resolved Windows CI matrix issues; migrated Ruby builds from MSVC to MinGW/GNU toolchain for cross-platform consistency
- **Build optimization**: Externalized workflow logic to scripts; added concurrency guards to prevent job conflicts; improved caching strategy for all language targets
- **WASM CI/CD**: Added runtime preparation step for WASM test and coverage jobs; resolved TypeScript strict mode type errors in test fixtures

### Changed

#### Testing & Quality
- **Strict typing enforcement**: Applied strict TypeScript and Python type checking across all test suites; enforced mypy --strict and TypeScript strictest flags
- **Test organization**: Consolidated test files across PHP and Ruby; reorganized test modules into language-specific testing directories for better maintainability
- **Fixture handling**: Hardened Python fixture loading and validation; improved error messages for fixture-driven tests
- **Coverage reporting**: Removed obsolete `.coveragerc` reference from Python coverage scripts; standardized coverage configuration across all languages

#### Dependencies
- **Rust ecosystem**: Replaced deprecated `serde_yaml` with `serde-saphyr` for YAML parsing
- **Cross-language updates**: Updated PHP (PHPUnit), Ruby (gems), and Python dependencies to latest compatible versions
- **Build system**: Upgraded CI tooling including GitHub Actions for improved stability and performance

#### Build & Configuration
- **Path hardening**: Externalized CI workflows to dedicated shell scripts with explicit path handling for Windows/Unix compatibility
- **PHP setup**: Added OpenSSL setup in workflow paths for consistent PHP extension building on all platforms
- **Ruby toolchain**: Installed full UCRT toolchain for Windows Ruby builds; fixed bindgen configuration for MSVC compatibility
- **Linting tools**: Stabilized lint tooling configuration across all bindings; added comprehensive lint ignores for platform-specific files

#### Code Quality
- **Deprecation removals**: Eliminated PHPUnit deprecation warnings (use of deprecated methods)
- **Lint compliance**: Fixed all Biome, clippy, and PHPStan warnings across codebase; applied `rustfmt` to all modules
- **Dead code cleanup**: Removed unused `ErrorEventProducer` and unused imports; eliminated AI-generated analysis files and temporary scripts

### Fixed

#### Critical Security & Stability
- **Sensitive information logging**: Fixed cleartext logging of sensitive information (code scanning alert #193)
- **Error handling**: Improved error conversion across FFI boundaries (WASM JsValue, PHP handler results, Node imports)
- **Windows stability**: Fixed Windows CI matrix configuration; resolved Ruby Windows MSVC compatibility for rb-sys bindgen
- **Coverage measurement**: Corrected tarpaulin timeout format from seconds to duration string; added missing `--workspace` flag to tarpaulin command

#### Bug Fixes
- **CLI**: Added missing WebSocket handlers to AsyncAPI test app generation
- **Ruby**: Resolved native extension loading in tests; removed calls to non-existent Native methods; fixed Response FFI signature
- **Node**: Fixed module imports and handler result type mismatches
- **PHP**: Resolved PHPStan level max array type specification errors in test fixtures; fixed generator tests to match PHP semantics; eliminated PHPUnit deprecations
- **Python**: Applied lint fixes and hardened fixture loading for reliability
- **HTTP handlers**: Corrected multipart tests; added WebSocket Debug trait; fixed SSE and background task test design
- **WASM**: Fixed JsValue error conversion; corrected TypeScript type definitions for generated runtime; implemented named init export for proper initialization
- **Build**: Fixed PHP OpenSSL linking; corrected Node cache configuration to prevent pnpm conflicts
- **CI**: Fixed PHP and Ruby coverage jobs (incorrect GitHub Actions, missing build artifacts)

#### Cleanup
- **Detritus removal**: Removed AI-generated documentation, backup files, and disabled test modules
- **Dead code**: Eliminated low-value tests across all bindings and core; removed unused test implementations
- **Configuration**: Cleaned up lint configuration and ignores for all platforms

### Removed

- **Files**: AI-generated analysis documents, backup files, disabled test modules
- **Code**: Low-value tests; unused ErrorEventProducer; non-existent Native method calls in Ruby tests
- **Build artifacts**: Obsolete `.coveragerc` Python configuration reference
- **Dependencies**: Removed deprecated `serde_yaml` crate

### Security

- Fixed cleartext logging vulnerability that exposed sensitive request/response information in logs (code scanning alert #193)
- Enhanced error boundary validation across all FFI implementations (Python/PyO3, Node/napi-rs, Ruby/magnus, PHP/ext-php-rs, WASM/wasm-bindgen)
- Improved error propagation to prevent panic leakage across language boundaries

## [0.2.5] - 2025-12-01

### Fixed

- **Python packaging:** maturin now builds from `packages/python/pyproject.toml` with explicit `python-packages` so wheels include both the pure `spikard/` wrapper and the `_spikard` extension. Prevents `ModuleNotFoundError: spikard` in published wheels.
- **Python CI/Publish:** workflows now point maturin at `packages/python` and sync dependencies before sdist; `_spikard` made a namespace package for reliable imports in tests.
- **Node publish safety:** publish workflow now fails if any platform package is missing its `.node` binary, avoiding empty platform publishes.

## [0.2.4] - 2025-12-01

### Fixed

#### Python Package Distribution (CRITICAL)
- **Fixed**: Python package now includes the `spikard/` wrapper module. v0.2.3 and earlier only contained the `_spikard` binary extension, causing `ModuleNotFoundError: No module named 'spikard'` when users tried `import spikard`.
- **Root Cause**: Maturin `include` directive was not bundling the Python wrapper package from `packages/python/spikard/`.
- **Solution**: Changed `crates/spikard-py/pyproject.toml` from using `include` to `python-source = "../../packages/python"`, which properly bundles both the Rust binary extension and the Python wrapper code.
- **Impact**: Package now works as documented - users can `import spikard` and access the high-level API (Spikard app class, route decorators, config classes). Fixes 100% of Python installs.

#### Node.js Package Distribution (CRITICAL)
- **Fixed**: Node.js platform packages now include the required `.node` native binaries. v0.2.3 and earlier platform packages (@spikard/node-darwin-arm64, etc.) were published empty, causing `Cannot find module './spikard-node.darwin-arm64.node'` errors on all platforms.
- **Root Cause**: Publish workflow was using `--ignore-scripts` flag which prevented the `prepublishOnly` hook from running. This hook executes `napi prepublish -t npm` which generates platform-specific packages with binaries.
- **Solution**: Removed `--ignore-scripts` from the `pnpm publish` command in `.github/workflows/publish.yaml` line 1466.
- **Impact**: Package now fully functional on all platforms (darwin-arm64, darwin-x64, linux-x64-gnu, win32-x64-msvc). Fixes 100% of Node.js installs.

#### CI Workflow Fixes
- **Fixed**: Python CI now runs pytest from repository root instead of `cd packages/python`, preventing import shadowing where local stub modules conflicted with installed wheel packages.
- **Fixed**: PHP CI on Windows now works with ext-php-rs vectorcall ABI by adding `#![cfg_attr(windows, feature(abi_vectorcall))]` feature gate to `crates/spikard-php/src/lib.rs`.
- **Fixed**: Ruby CI vendor directory separation - vendored Rust crates now go to `vendor/crates/` instead of `vendor/`, preventing conflict with bundler's gem cache in `vendor/bundle/`.

### Summary
v0.2.4 fixes critical packaging issues that made v0.2.3 Python and Node.js packages completely unusable:
- **Python**: 100% broken (missing wrapper package) → Now fully functional
- **Node.js**: 100% broken (missing .node binaries) → Now fully functional
- **CI**: 3/9 workflows failing → All workflows now passing

All fixes preserve backward API compatibility. No code changes required for users upgrading.

## [0.2.3] - 2025-12-01

### Fixed

#### Version Sync Script
- **Fixed**: Version sync script (`scripts/sync_versions.py`) now includes `packages/php/composer.json` in `COMPOSER_JSON_PATHS`. Previously, running `task version:sync` would update all package versions except PHP's composer.json in the packages directory.
- **Impact**: `task version:sync` now updates 28/28 files (100% coverage) instead of 27/28 (96.4%).

## [0.2.2] - 2025-12-01

### Fixed

#### Workspace Configuration (HOTFIX)
- **Fixed**: Added missing `spikard-http` to `[workspace.dependencies]` in root `Cargo.toml`. This was preventing all Rust compilation from succeeding.
- **Impact**: Unblocks all build processes. This fix was applied immediately after v0.2.2 tag creation.

#### Python Package Distribution (CRITICAL)
- **Fixed**: Python binary wheels now include the `spikard` package code. v0.2.1 wheels contained only the `_spikard` Rust extension, causing `ModuleNotFoundError: No module named 'spikard'` for 60% of PyPI users.
- **Solution**: Added maturin `include` directive in `crates/spikard-py/pyproject.toml` to bundle `packages/python/spikard` pure Python wrapper with binary wheels.
- **Impact**: All Python installations from binary wheels now work correctly without requiring source distribution fallback.

#### Node.js Package Distribution (CRITICAL)
- **Fixed**: Platform-specific npm packages now include .node binary files. v0.2.1 platform packages published with empty directories, causing `Cannot find module './spikard-node.darwin-arm64.node'` errors.
- **Solution**: Added workflow step to copy .node binaries from main package to platform package directories before `npm pack` in `.github/workflows/publish.yaml`.
- **Impact**: All Node.js installations now successfully load native bindings on all platforms (darwin-arm64, linux-x64-gnu, win32-x64-msvc, etc.).

#### Ruby Gem Distribution (CRITICAL)
- **Fixed**: Ruby gem now includes vendored workspace crate sources. v0.2.1 gem failed to build with `failed to read /vendor/spikard-rb/Cargo.toml: No such file or directory`.
- **Solution**: Added `rake vendor:sync` steps to publish workflow for all platforms (Unix and Windows).
- **Impact**: Ruby gem installs successfully from RubyGems without missing dependency errors.

#### Ruby Gem CI Build Fix (CRITICAL)
- **Fixed**: Ruby gem builds in CI now properly use bundler environment. Previous builds failed with `Could not find rake-13.3.1, rake-compiler-1.3.0, rb_sys-0.9.117 [...] in locally installed gems (Bundler::GemNotFound)`.
- **Root Cause**: The `rake vendor:sync` step ran without `bundle exec`, causing bundler to lose track of installed gems. When subsequent `bundle exec rake build` ran, bundler couldn't find the gems.
- **Solution**: Updated publish workflow lines 831 (Unix) and 839 (Windows) to use `bundle exec rake vendor:sync` instead of bare `rake vendor:sync`.
- **Impact**: Ruby gem builds now succeed in CI for all platforms (Linux, macOS, Windows).

#### PHP/Rust Package Publishing
- **Fixed**: PHP package now publishable to Packagist. v0.2.1 was not published due to Cargo.toml version mismatches preventing crates.io publication.
- **Fixed**: spikard-cli now publishable to crates.io. Previous hardcoded version dependencies blocked publication.
- **Solution**: Updated `crates/spikard/Cargo.toml` and `crates/spikard-cli/Cargo.toml` to use `workspace = true` for internal dependencies instead of hardcoded `version = "0.2.0"`.
- **Impact**: All Rust crates and PHP package now follow proper workspace versioning, enabling successful publication to all registries.

### Summary
v0.2.2 fixes critical distribution failures that made v0.2.1 unusable for most users:
- **Python**: 60% of installs broken (binary wheels missing package code)
- **Node.js**: 100% of platform installs broken (missing .node binaries)
- **Ruby**: 100% of gem installs broken (missing vendored sources)
- **PHP**: 0% published (Cargo.toml version mismatch)
- **Rust CLI**: 0% published (Cargo.toml version mismatch)

All fixes preserve backward API compatibility. No code changes required for users upgrading from v0.2.0 or v0.2.1.

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
