<!--
🤖 AI-RULEZ :: GENERATED FILE — DO NOT EDIT DIRECTLY
Project: spikard
Generated: 2025-11-22 09:42:09
Source of truth: ai-rulez.yaml
Target file: CLAUDE.md
Content summary: rules=24, sections=11, agents=14

UPDATE WORKFLOW
1. Modify ai-rulez.yaml
2. Run `ai-rulez generate` to refresh generated files
3. Commit regenerated outputs together with the config changes

AI ASSISTANT SAFEGUARDS
- Treat ai-rulez.yaml as the canonical configuration
- Never overwrite CLAUDE.md manually; regenerate instead
- Surface changes as patches to ai-rulez.yaml (include doc/test updates)

Need help? /capability-plan or https://github.com/Goldziher/ai-rulez
-->

# spikard

Spikard is a Rust-centric multi-language toolkit that provides a core library,
command-line interface, and HTTP runtime with tower-http middleware surfaced
through Python (PyO3), Node.js (napi-rs), Ruby (magnus), and WebAssembly bindings
to build and validate typed web services. The monorepo couples Rust 2024 workspace
crates with Python/uv, Node.js/pnpm, Ruby, and taskfile automation, backed by
extensive fixture-driven tests to ensure consistent request handling across platforms.
Future bindings planned for PHP (ext-php-rs).


Version: 1.0.0

## Governance

- Source of truth: ai-rulez.yaml
- Generated output: CLAUDE.md
- Update workflow:
  1. Edit the source configuration above.
  2. Run ai-rulez generate to refresh generated files.
  3. Commit the regenerated files alongside the configuration change.
- AI assistants must propose edits to the source configuration, not this file.

## Rules

### Async-Friendly Performance
**Priority:** medium

Respect the project's zero-copy serialization choices—keep Rust structs `serde`-ready
so bindings reuse them, lean on the adapters documented in `docs/adr/0003-validation-and-fixtures.md`,
and wrap heavy work in async-safe boundaries (`tokio::task::spawn_blocking` in Rust,
`pyo3::Python::allow_threads` when calling back into Python). Reuse fixture loaders such
as `packages/python/tests/fixture_app.py` instead of re-parsing schema files per request.


### Consistent Tooling
**Priority:** medium

Honor the repo formatters and linters before committing: run `cargo fmt` (configured by
`rustfmt.toml`), rely on Biome per `biome.json` for JavaScript/TypeScript, and manage
Python tooling through `uv` (`uv.lock`). Invoke them via `task lint` so local changes
match the CI configuration.


### Cross-Target Performance
**Priority:** medium

Consolidate heavy computation inside the shared Rust core (crates/spikard) and expose
thin bindings in crates/spikard-py, packages/python, crates/spikard-node, crates/spikard-rb,
and crates/spikard-wasm; stress-test large or deeply nested payloads with
testing_data/edge_cases and verify optimized builds via task build:rust, task build:py,
task build:node, and task build:ruby.


### Lint & Formatting Discipline
**Priority:** medium

Before committing, run task lint to honor the Biome settings in biome.json for JS/TS,
format Rust with cargo fmt --manifest-path Cargo.toml configured by rustfmt.toml, and
sync Python dependencies with uv so pyproject.toml and uv.lock stay consistent—avoid
introducing divergent toolchains or unchecked formatting drift.


### Optimized Serialization Path
**Priority:** medium

Follow the conversion patterns captured in `docs/adr/0003-validation-and-fixtures.md`
so data exchanged between Rust and Python leverages `msgspec` without extra JSON hops.
Share zero-copy buffers where possible, and use `task build:rust` (release mode) when
benchmarking or publishing bindings to avoid debug-performance regressions.


### Workspace Separation
**Priority:** medium

Keep language-neutral logic inside `crates/spikard/src` and limit each binding crate
(`spikard-py`, `spikard-node`, `spikard-wasm`) to thin adapters over that core. When
introducing new modules, register them in the relevant `Cargo.toml`, mirror usage in
`examples/`, and avoid duplicating business rules across bindings.


### Binding-Level Configuration Only
**Priority:** high

Language bindings (spikard-py, spikard-node, spikard-rb) must NOT duplicate middleware
logic. All middleware lives in Rust (tower-http). Bindings only expose configuration APIs
that construct ServerConfig and pass it to the Rust server. Ruby bindings use magnus for
FFI, TypeScript uses napi-rs, Python uses PyO3. Future PHP bindings will use ext-php-rs.


### HTTP Error Contracts
**Priority:** high

When updating handlers in crates/spikard-http, translate domain failures into the JSON
payloads maintained under testing_data/status_codes and testing_data/validation_errors;
add the matching fixture files and assertions in packages/python/tests/test_all_fixtures.py
or the focused integration suites, and keep every testing_data/**/schema.json aligned
with the new variants.


### HTTP Input Validation
**Priority:** high

Handlers under `crates/spikard-http/src` must validate headers, cookies, and payloads
against the schemas in `testing_data/headers`, `testing_data/cookies`, and
`testing_data/json_bodies`. Reject unexpected or malformed values with structured
errors returned to the caller, and cover each guard with an integration test tied to
the corresponding fixture set.


### Header & Cookie Security
**Priority:** high

Authentication, header, and cookie code must enforce the scenarios captured in
testing_data/headers and testing_data/cookies—reject deviations from those schemas, add
explicit fixtures plus assertions in packages/python/tests/test_integration_query_params.py
for new header names or cookie attributes, and keep Secure/HttpOnly/SameSite defaults
intact.


### Layered Code Organization
**Priority:** high

Implement cross-cutting logic in `crates/spikard/src` and expose it through thin adapters
in `crates/spikard-http`, `crates/spikard-py`, `crates/spikard-node`, `crates/spikard-rb`,
and `crates/spikard-wasm`. Keep build metadata confined to each binding's manifest
(`pyproject.toml`, `crates/spikard-node/package.json`, `crates/spikard-wasm/package.json`)
and register new workflows in `Taskfile.yaml` so `task build`/`task lint` continue to
orchestrate the monorepo.


### Lifecycle Hooks Implementation
**Priority:** high

Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError) must follow
the zero-cost design in `docs/adr/0005-lifecycle-hooks.md`: use Option<Arc<dyn Fn>> for
conditional execution (<1ns when not registered), provide async support via
pyo3_async_runtimes for Python and ThreadsafeFunction for TypeScript, and allow hooks to
short-circuit with early responses. Implement HookResult enum with Continue/ShortCircuit
variants.


### Request Surface Security
**Priority:** high

Guard every HTTP-facing change with the validation strategy captured in
`docs/adr/0003-validation-and-fixtures.md`: enforce cookie rules via
`testing_data/cookies/*.json`, headers/auth via `testing_data/headers/*.json`, and
CORS expectations via `testing_data/cors/*.json`. Strip secrets from logs and ensure
new handlers never bypass the existing validator layer before reaching business logic.


### Tower-HTTP Middleware Stack
**Priority:** high

All standard middleware (compression, rate limiting, timeouts, graceful shutdown, static
files, request IDs) is implemented in Rust using tower-http and exposed via typed
ServerConfig. Configuration structs (CompressionConfig, RateLimitConfig, StaticFilesConfig,
etc.) must be forwarded to Python/TypeScript/Ruby bindings with proper type safety.
See `docs/adr/0002-runtime-and-middleware.md` for the complete middleware stack order
and configuration options.


### Workspace Organization
**Priority:** high

Place reusable domain types and logic in crates/spikard/src/ and keep feature-specific
glue isolated within sibling crates (spikard-http, spikard-cli, bindings); mirror module
changes across Cargo manifests and refresh the relevant docs/adr/* notes whenever the
layering or routing changes.


### Zero-Copy JSON to Python Conversion
**Priority:** high

Convert `serde_json::Value` to Python objects using direct PyO3 type construction
(PyDict::new, PyList::empty, PyString::new, etc.) instead of serialize-to-JSON-string
then json.loads. This zero-copy approach in `crates/spikard-py/src/handler.rs::json_to_python()`
eliminates 30-40% conversion overhead. Match on Value variants and recursively build
native Python objects.


### Cross-Language Error Boundaries
**Priority:** critical

Rust code in `crates/spikard`, `crates/spikard-http`, and the binding crates must
avoid panics; expose fallible APIs as `Result<T, E>` and propagate with `?`. When
exporting to Python (`crates/spikard-py/src`), always return `PyResult<T>` and convert
domain failures with `PyErr::new_err(...)`; for Node (`crates/spikard-node/src`),
return `napi::Result<T>` and build errors via `napi::Error::from_reason`. Never let
an unwrap cross the FFI boundary.


### Fixture-Aligned Error Handling
**Priority:** critical

Keep every fallible path in the Rust workspace (`crates/spikard`, `crates/spikard-http`,
bindings crates) returning the structured payload described in
`testing_data/validation_errors/schema.json`. Reuse the shared error constructor so
Python (`crates/spikard-py`) and Node (`crates/spikard-node`) adapters raise translated
host-language errors while preserving the same JSON body that
`packages/python/tests/test_all_fixtures.py` asserts on.


### Fixture-Backed Testing
**Priority:** critical

Every feature change must expand the Python-driven integration suite in
`packages/python/tests/` and keep the JSON fixtures under `testing_data/` in sync.
Prefer validating new scenarios by adding fixture files and asserting them in
`packages/python/tests/test_all_fixtures.py`. Run `task test` locally before merging
so the Rust, Python, and JavaScript checks that CI executes stay green.


### Fixture-Driven Testing
**Priority:** critical

Every feature change must extend the pytest suites in packages/python/tests/ by loading
fixtures through packages/python/tests/conftest.py and invoking task test before merging;
new fixture collections belong in testing_data/ with a runnable illustration under
examples/ so automated coverage, demos, and docs stay synchronized.


### Fixture-First Testing
**Priority:** critical

When adding behavior, introduce or update fixtures under the relevant `testing_data/*`
directory and extend the parametrized suites in `packages/python/tests/test_all_fixtures.py`,
`packages/python/tests/test_integration_query_params.py`, and peers. Do not ship without
running `task test` plus the language targets (`task test:rust`, `task test:python`) so
local runs match CI.


### Handler Trait Abstraction
**Priority:** critical

In `crates/spikard-http`, define language-agnostic Handler trait with
`Pin<Box<dyn Future<Output = HandlerResult> + Send>>` return type. Language bindings
(`spikard-py`, `spikard-node`, `spikard-rb`) implement this trait with Arc<dyn Handler>
wrappers. The HTTP server accepts `Vec<(Route, Arc<dyn Handler>)>` enabling clean
separation: spikard-http has zero FFI dependencies, all Python/Node/Ruby/WASM code
lives in binding crates.


### PyO3 Async Performance
**Priority:** critical

For async Python handlers in `crates/spikard-py/src/handler.rs`, use
`pyo3_async_runtimes::tokio::into_future()` to convert Python coroutines directly to
Rust futures, eliminating spawn_blocking overhead. Initialize the event loop once with
`TaskLocals` stored in a `OnceCell` to avoid per-request event loop creation. Ensure
GIL is released before awaiting Rust futures: `Python::attach(|py| {...}).await`
not `Python::with_gil(|py| {...}).await`.


### PyO3 Extension Module Management
**Priority:** critical

The `extension-module` feature in `crates/spikard-py/Cargo.toml` must NOT be in default
features—it breaks linking for binaries that embed Python (like spikard-cli). Configure
maturin in `pyproject.toml` with `features = ["extension-module"]` so Python extension
modules build correctly. Binaries (CLI, tests) build without extension-module to link
libpython; extensions (maturin builds) enable it for manylinux compliance.


## Sections

### Documentation & Architecture Decision Records
**Priority:** high

**ADRs in docs/adr/ · Examples in examples/ · rustdoc/JSDoc/docstrings**

ADRs (docs/adr/): 0001 (architecture, layering), 0002 (tower-http, config), 0003 (fixtures, msgspec), 0005 (lifecycle hooks), 0006 (async, streaming). Update when architecture changes.

Code Documentation: Rust rustdoc on ALL public items with examples. Python docstrings (NumPy style) with type hints. TypeScript JSDoc; .d.ts auto-generated by napi-rs. Ruby RBS files with YARD docs.

Examples (examples/): Runnable illustrations for Python, Node, Ruby, WASM. Load fixtures from testing_data/; show error handling.


### HTTP Routing & Middleware Design
**Priority:** high

**Tower-HTTP middleware stack · OpenAPI codegen · Lifecycle hooks**

Middleware Stack: Compression → RateLimit → Timeout → RequestId → Auth → UserAgent → Handler. All configurable via CompressionConfig, RateLimitConfig. Auth validates against testing_data/headers/*.json.

Handler Trait: Language-agnostic `trait Handler { fn handle(&self, req: Request) -> Pin<Box<dyn Future<...>>> }`. Binding wrappers implement Arc<dyn Handler>. HTTP server accepts `Vec<(Route, Arc<dyn Handler>)>`.

Lifecycle Hooks (docs/adr/0005-lifecycle-hooks.md): onRequest, preValidation, preHandler, onResponse, onError. Zero-cost: Option<Arc<dyn Fn>>. Async via pyo3_async_runtimes (Python) and ThreadsafeFunction (TypeScript).


### Ruby 3.2+ with RBS & Steep
**Priority:** high

**Ruby 3.2+ · RBS type definitions · Steep · RSpec · Rubocop**
- Ruby 3.2+ with .ruby-version; rbenv for version management
- RBS files in sig/ directory parallel to source: lib/foo.rb → sig/foo.rbs
- Steep for type checking; avoid Any types, use union types explicitly
- RSpec testing: 80%+ coverage, function-like tests
- Rubocop with auto-fix: line length ≤120
- Code quality: methods <10 lines, guard clauses, modules for mixins


### Spikard Workspace Architecture
**Priority:** high

**Multi-crate Rust workspace · Layered binding architecture · Fixture-driven testing**

Rust Workspace: `crates/spikard/` (core), `crates/spikard-http/` (tower-http), `crates/spikard-cli/` (CLI), `crates/spikard-py/` (PyO3), `crates/spikard-node/` (napi-rs), `crates/spikard-rb/` (magnus), `crates/spikard-wasm/` (wasm-bindgen).

Binding Principles: All middleware in Rust; bindings expose config APIs only. Language-neutral Handler trait: `Pin<Box<dyn Future<Output = HandlerResult> + Send>>`. Each binding wraps with Arc<dyn Handler>. No PyO3/napi/magnus in core crates.

Python & Testing: Package scaffold `packages/python/spikard`; integration tests in `packages/python/tests/` with conftest.py. Shared fixtures `testing_data/` with schema.json per scenario. Fixture-driven: `testing_data/{headers,cookies,json_bodies,validation_errors,edge_cases}/`. Coverage: 95% Rust core, 80%+ Python/JS/Ruby.


### Task Automation & Build Orchestration
**Priority:** high

**Taskfile.yaml · Multi-language coordination · Dependency management · CI/CD parity**

Root Commands: `task setup` (install tooling, build bindings), `task update` (upgrade all), `task build` (all languages), `task lint` (mypy, clippy, biome, steep), `task format` (all), `task test` (all suites).

Language-Specific: `task rust:build`, `task python:build` (maturin), `task python:test`, `task js:build`, `task js:test`, `task ruby:build`, `task wasm:build`.

Dependency Files (committed): Cargo.lock, uv.lock, pnpm-lock.yaml, Gemfile.lock. All mandatory in version control.


### WebAssembly & wasm-bindgen Standards
**Priority:** high

**WebAssembly · wasm-bindgen · wasm-pack · WASI compatibility**
- wasm-bindgen for FFI to JavaScript; wasm-pack for bundling
- Minimize binary size: tree-shake unused code, opt-level=z with lto=true
- No blocking operations: all I/O async or via workers
- Testing: wasm-pack test for unit tests, browser/node environments
- Type safety: proper type boundaries at JS/WASM interface, no Any types
- Never: spawn threads (limited WASM threading), blocking allocations


### Cross-Language Error Handling
**Priority:** critical

**Structured error payloads · FFI boundaries · Validation schema alignment**

Error Structure: All errors return JSON { "error": string, "code": string, "details": {} }. Rust uses Result<T, E> with thiserror. Python: PyResult<T> → PyErr. Node: napi::Result<T> → napi::Error. Ruby: raise_error. All preserve same JSON payload.

FFI Boundaries: Rust core returns Result<T, E> to handlers. Adapters (PyO3/napi/magnus) convert to language errors while preserving JSON. Python: PyErr::new_err(json_string). Node: Error::from_reason(json_string). Never let unwrap cross FFI boundary.

Validation: HTTP handlers validate against testing_data/{headers,cookies,json_bodies}. Reject with errors matching testing_data/validation_errors/schema.json. Assert in packages/python/tests/test_all_fixtures.py. Keep schema.json in sync with handler code.


### Fixture-First Testing Strategy
**Priority:** critical

**Fixture-driven · Multi-language parity · 95% coverage · Real infrastructure**

Fixture Organization: Central `testing_data/` with JSON files per scenario (headers, cookies, bodies, errors, edge_cases). Each directory has schema.json. Python tests parametrized: test_all_fixtures.py loads all JSONs. Rust: unit tests embed JSON; integration tests load from testing_data/.

Coverage: Rust 95% minimum (tarpaulin). Python/JS/Ruby 80%+ minimum. Enforce in CI; fail if < threshold.

Three-Tier Testing: Unit (pure functions, fast), Integration (real DB, PostgreSQL, fixtures), E2E (full HTTP stack, all bindings).

Running: `cargo test -p spikard`, `uv run pytest packages/python/tests/test_all_fixtures.py`, `pnpm test`, `bundle exec rspec`. All: `task test`.


### Python Modern & Performance Standards
**Priority:** critical

**Python 3.10+ · Functional-first · msgspec · Fully async · Strict typing**
- Python 3.10+; match/case, union types (X | Y), structural pattern matching
- msgspec ONLY (NEVER pydantic); msgspec.Struct with slots=True, frozen=True
- Full type hints: ParamSpec, TypeVar/Generic[T], mypy --strict; never use Any
- Functional patterns: pure functions, composition, immutability
- Fully async: anyio.Path, httpx AsyncClient, asyncpg, asyncio.gather
- Function-based tests ONLY (*_test.py); pytest fixtures, 95% coverage
- Never: class tests, pydantic, sync I/O in async, Any type, Optional[T]


### Rust Latest Edition Standards
**Priority:** critical

**Rust 2024 · High strictness · clippy -D warnings · 95% coverage · Zero unwrap**
- Rust 2024; cargo fmt, clippy -D warnings (zero tolerance)
- Result<T, E> for errors; thiserror for custom errors; NEVER .unwrap() in production
- Testing: 95% minimum coverage (tarpaulin); unit/integration/doc tests
- Async: Tokio 1.x, 'static constraints, Send+Sync bounds
- FFI: isolated modules, pointer validation, SAFETY comments, error conversion at boundaries
- Code quality: RAII, explicit lifetimes, builder pattern, no panics


### TypeScript Strictest Standards
**Priority:** critical

**TypeScript 5.x · Strictest typing · No any/object · Generics required**
- Enable ALL strict flags: strict, noUncheckedIndexedAccess, exactOptionalPropertyTypes
- Ban any and object types; use unknown with guards
- Generics with constraints, satisfies operator, const assertions
- Tests: .spec.ts next to source files; vitest, 80%+ coverage
- Functional: pure functions, map/filter/reduce, immutability, readonly
- Biome for linting/formatting; pnpm ≥10.17, pnpm-lock.yaml committed
- React: function components, custom hooks, proper prop typing
- Never: any/object types, non-null assertions !, || for defaults


## Agents

### build-and-ci-ops
**Priority:** medium

Curates Taskfile.yaml, CI workflows, and release automation to keep multi-language
toolchains reproducible. Maintains lock files (Cargo.lock, uv.lock, pnpm-lock.yaml,
Gemfile.lock) and ensures task commands mirror CI execution.


### docs-scribe
**Priority:** medium

Generates agent handbooks and CLAUDE briefs that summarize current rules and workflows.
Keeps generated CLAUDE.md in sync via ai-rulez generate and documents agent responsibilities.


### docs-strategist
**Priority:** medium

Maintains developer guides, upgrade notes, and architecture references (docs/adr/)
so contributors can follow cross-language patterns. Ensures ADRs stay synchronized
with code changes and examples remain runnable.


### fixture-tester
**Priority:** medium

Evolves testing_data/ fixtures and validation schemas so every handler respects
documented contracts. Maintains schema.json files across all fixture directories
and coordinates fixture-first test expansion with integration-qa.


### integration-qa
**Priority:** medium

Expands fixture-driven coverage and hunts for regressions across Rust, Python, Node,
and Ruby integration suites. Manages testing_data/ fixture schemas and ensures all
handlers respect documented contracts. Enforces 95% Rust / 80%+ language coverage.


### interop-build-engineer
**Priority:** medium

Ensures binding build scripts for PyO3, napi-rs, magnus/rb-sys, and wasm-pack stay
in sync and optimized. Manages Cargo.toml/pyproject.toml/package.json/Gemfile manifest
coordination and plans for future ext-php-rs integration.


### middleware-architect
**Priority:** medium

Implements tower-http middleware stack, lifecycle hooks, and authentication
middleware while maintaining zero-overhead design principles. Designs ServerConfig
structures and ensures configuration APIs are properly exposed to all bindings.


### php-engineer
**Priority:** medium

(FUTURE) Implements and maintains PHP bindings via ext-php-rs, ensuring PSR compliance
and idiomatic PHP patterns while integrating with Rust core. Will follow similar FFI
isolation and error handling conventions as Python/Node/Ruby bindings.


### python-engineer
**Priority:** medium

Implements and maintains Python bindings via PyO3, ensuring Pythonic patterns with
async/await support via pyo3_async_runtimes, proper type hints, and zero-copy msgspec
integration. Manages extension-module feature gate for maturin vs. CLI binaries.


### ruby-engineer
**Priority:** medium

Implements and maintains Ruby bindings via magnus/rb-sys, ensuring idiomatic Ruby
patterns while preserving Rust performance and safety guarantees. Manages RBS type
files and Steep type checking for binding APIs.


### rust-polyglot-architect
**Priority:** medium

Designs Rust-first APIs and keeps Python, Node, Ruby, and WASM bindings aligned
with shared memory-safety and error contracts. Plans for future PHP bindings.
Responsible for workspace structure, Handler trait design, FFI boundaries, and
cross-language error handling semantics.


### typescript-engineer
**Priority:** medium

Implements TypeScript type definitions and ensures napi-rs Node.js bindings provide
full type safety with proper JSDoc annotations and .d.ts generation. Maintains
ServerConfig configuration APIs in TypeScript with runtime type safety.


### wasm-engineer
**Priority:** medium

Maintains WebAssembly bindings via wasm-bindgen for browser and server-side WASM
runtimes. Optimizes WASM binary size, manages wasm-pack bundling, and ensures
JavaScript/TypeScript interop with proper type safety.


### workspace-architect
**Priority:** medium

Structures Cargo workspaces, feature flags, and dependency graph to avoid duplication
across crates. Manages crate layering (core → http → bindings) and ensures clean FFI
isolation. Maintains Taskfile.yaml task definitions for multi-language orchestration.


## MCP Servers

### ai-rulez
AI-Rulez MCP server for configuration management
- Transport: stdio
- Command: npx
- Args: -y, ai-rulez@latest, mcp
