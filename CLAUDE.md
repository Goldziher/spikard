<!--
🤖 AI-RULEZ :: GENERATED FILE — DO NOT EDIT DIRECTLY
Project: spikard
Generated: 2025-11-20 15:32:22
Source of truth: ai-rulez.yaml
Target file: CLAUDE.md
Content summary: rules=24, sections=5, agents=14

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

Spikard is a Rust-centric multi-language toolkit that provides a core library, command-line interface, and HTTP runtime with tower-http middleware surfaced through Python (PyO3), Node.js (napi-rs), Ruby (magnus), and WebAssembly bindings to build and validate typed web services. The monorepo couples Rust 2024 workspace crates with Python/uv, Node.js/pnpm, Ruby, and taskfile automation, backed by extensive fixture-driven tests to ensure consistent request handling across platforms. Future bindings planned for PHP (ext-php-rs).

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

Respect the project’s zero-copy serialization choices—keep Rust structs `serde`-ready so bindings reuse them, lean on the adapters documented in `docs/adr/0003-validation-and-fixtures.md`, and wrap heavy work in async-safe boundaries (`tokio::task::spawn_blocking` in Rust, `pyo3::Python::allow_threads` when calling back into Python). Reuse fixture loaders such as `packages/python/tests/fixture_app.py` instead of re-parsing schema files per request.

### Consistent Tooling
**Priority:** medium

Honor the repo formatters and linters before committing: run `cargo fmt` (configured by `rustfmt.toml`), rely on Biome per `biome.json` for JavaScript/TypeScript, and manage Python tooling through `uv` (`uv.lock`). Invoke them via `task lint` so local changes match the CI configuration.

### Cross-Target Performance
**Priority:** medium

Consolidate heavy computation inside the shared Rust core (crates/spikard) and expose thin bindings in crates/spikard-py, packages/python, crates/spikard-node, crates/spikard-rb, and crates/spikard-wasm; stress-test large or deeply nested payloads with testing_data/edge_cases and verify optimized builds via task build:rust, task build:py, task build:node, and task build:ruby.

### Lint & Formatting Discipline
**Priority:** medium

Before committing, run task lint to honor the Biome settings in biome.json for JS/TS, format Rust with cargo fmt --manifest-path Cargo.toml configured by rustfmt.toml, and sync Python dependencies with uv so pyproject.toml and uv.lock stay consistent—avoid introducing divergent toolchains or unchecked formatting drift.

### Optimized Serialization Path
**Priority:** medium

Follow the conversion patterns captured in `docs/adr/0003-validation-and-fixtures.md` so data exchanged between Rust and Python leverages `msgspec` without extra JSON hops. Share zero-copy buffers where possible, and use `task build:rust` (release mode) when benchmarking or publishing bindings to avoid debug-performance regressions.

### Workspace Separation
**Priority:** medium

Keep language-neutral logic inside `crates/spikard/src` and limit each binding crate (`spikard-py`, `spikard-node`, `spikard-wasm`) to thin adapters over that core. When introducing new modules, register them in the relevant `Cargo.toml`, mirror usage in `examples/`, and avoid duplicating business rules across bindings.

### Binding-Level Configuration Only
**Priority:** high

Language bindings (spikard-py, spikard-node, spikard-rb) must NOT duplicate middleware logic. All middleware lives in Rust (tower-http). Bindings only expose configuration APIs that construct ServerConfig and pass it to the Rust server. Ruby bindings use magnus for FFI, TypeScript uses napi-rs, Python uses PyO3. Future PHP bindings will use ext-php-rs.

### HTTP Error Contracts
**Priority:** high

When updating handlers in crates/spikard-http, translate domain failures into the JSON payloads maintained under testing_data/status_codes and testing_data/validation_errors; add the matching fixture files and assertions in packages/python/tests/test_all_fixtures.py or the focused integration suites, and keep every testing_data/**/schema.json aligned with the new variants.

### HTTP Input Validation
**Priority:** high

Handlers under `crates/spikard-http/src` must validate headers, cookies, and payloads against the schemas in `testing_data/headers`, `testing_data/cookies`, and `testing_data/json_bodies`. Reject unexpected or malformed values with structured errors returned to the caller, and cover each guard with an integration test tied to the corresponding fixture set.

### Header & Cookie Security
**Priority:** high

Authentication, header, and cookie code must enforce the scenarios captured in testing_data/headers and testing_data/cookies—reject deviations from those schemas, add explicit fixtures plus assertions in packages/python/tests/test_integration_query_params.py for new header names or cookie attributes, and keep Secure/HttpOnly/SameSite defaults intact.

### Layered Code Organization
**Priority:** high

Implement cross-cutting logic in `crates/spikard/src` and expose it through thin adapters in `crates/spikard-http`, `crates/spikard-py`, `crates/spikard-node`, `crates/spikard-rb`, and `crates/spikard-wasm`. Keep build metadata confined to each binding's manifest (`pyproject.toml`, `crates/spikard-node/package.json`, `crates/spikard-wasm/package.json`) and register new workflows in `Taskfile.yaml` so `task build`/`task lint` continue to orchestrate the monorepo.

### Lifecycle Hooks Implementation
**Priority:** high

Lifecycle hooks (onRequest, preValidation, preHandler, onResponse, onError) must follow the zero-cost design in `docs/adr/0005-lifecycle-hooks.md`: use Option<Arc<dyn Fn>> for conditional execution (<1ns when not registered), provide async support via pyo3_async_runtimes for Python and ThreadsafeFunction for TypeScript, and allow hooks to short-circuit with early responses. Implement HookResult enum with Continue/ShortCircuit variants.

### Request Surface Security
**Priority:** high

Guard every HTTP-facing change with the validation strategy captured in `docs/adr/0003-validation-and-fixtures.md`: enforce cookie rules via `testing_data/cookies/*.json`, headers/auth via `testing_data/headers/*.json`, and CORS expectations via `testing_data/cors/*.json`. Strip secrets from logs and ensure new handlers never bypass the existing validator layer before reaching business logic.

### Tower-HTTP Middleware Stack
**Priority:** high

All standard middleware (compression, rate limiting, timeouts, graceful shutdown, static files, request IDs) is implemented in Rust using tower-http and exposed via typed ServerConfig. Configuration structs (CompressionConfig, RateLimitConfig, StaticFilesConfig, etc.) must be forwarded to Python/TypeScript/Ruby bindings with proper type safety. See `docs/adr/0002-runtime-and-middleware.md` for the complete middleware stack order and configuration options.

### Workspace Organization
**Priority:** high

Place reusable domain types and logic in crates/spikard/src/ and keep feature-specific glue isolated within sibling crates (spikard-http, spikard-cli, bindings); mirror module changes across Cargo manifests and refresh the relevant docs/adr/* notes whenever the layering or routing changes.

### Zero-Copy JSON to Python Conversion
**Priority:** high

Convert `serde_json::Value` to Python objects using direct PyO3 type construction (PyDict::new, PyList::empty, PyString::new, etc.) instead of serialize-to-JSON-string then json.loads. This zero-copy approach in `crates/spikard-py/src/handler.rs::json_to_python()` eliminates 30-40% conversion overhead. Match on Value variants and recursively build native Python objects.

### Cross-Language Error Boundaries
**Priority:** critical

Rust code in `crates/spikard`, `crates/spikard-http`, and the binding crates must avoid panics; expose fallible APIs as `Result<T, E>` and propagate with `?`. When exporting to Python (`crates/spikard-py/src`), always return `PyResult<T>` and convert domain failures with `PyErr::new_err(...)`; for Node (`crates/spikard-node/src`), return `napi::Result<T>` and build errors via `napi::Error::from_reason`. Never let an unwrap cross the FFI boundary.

### Fixture-Aligned Error Handling
**Priority:** critical

Keep every fallible path in the Rust workspace (`crates/spikard`, `crates/spikard-http`, bindings crates) returning the structured payload described in `testing_data/validation_errors/schema.json`. Reuse the shared error constructor so Python (`crates/spikard-py`) and Node (`crates/spikard-node`) adapters raise translated host-language errors while preserving the same JSON body that `packages/python/tests/test_all_fixtures.py` asserts on.

### Fixture-Backed Testing
**Priority:** critical

Every feature change must expand the Python-driven integration suite in `packages/python/tests/` and keep the JSON fixtures under `testing_data/` in sync. Prefer validating new scenarios by adding fixture files and asserting them in `packages/python/tests/test_all_fixtures.py`. Run `task test` locally before merging so the Rust, Python, and JavaScript checks that CI executes stay green.

### Fixture-Driven Testing
**Priority:** critical

Every feature change must extend the pytest suites in packages/python/tests/ by loading fixtures through packages/python/tests/conftest.py and invoking task test before merging; new fixture collections belong in testing_data/ with a runnable illustration under examples/ so automated coverage, demos, and docs stay synchronized.

### Fixture-First Testing
**Priority:** critical

When adding behavior, introduce or update fixtures under the relevant `testing_data/*` directory and extend the parametrized suites in `packages/python/tests/test_all_fixtures.py`, `packages/python/tests/test_integration_query_params.py`, and peers. Do not ship without running `task test` plus the language targets (`task test:rust`, `task test:python`) so local runs match CI.

### Handler Trait Abstraction
**Priority:** critical

In `crates/spikard-http`, define language-agnostic Handler trait with `Pin<Box<dyn Future<Output = HandlerResult> + Send>>` return type. Language bindings (`spikard-py`, `spikard-node`, `spikard-rb`) implement this trait with Arc<dyn Handler> wrappers. The HTTP server accepts `Vec<(Route, Arc<dyn Handler>)>` enabling clean separation: spikard-http has zero FFI dependencies, all Python/Node/Ruby/WASM code lives in binding crates.

### PyO3 Async Performance
**Priority:** critical

For async Python handlers in `crates/spikard-py/src/handler.rs`, use `pyo3_async_runtimes::tokio::into_future()` to convert Python coroutines directly to Rust futures, eliminating spawn_blocking overhead. Initialize the event loop once with `TaskLocals` stored in a `OnceCell` to avoid per-request event loop creation. Ensure GIL is released before awaiting Rust futures: `Python::attach(|py| {...}).await` not `Python::with_gil(|py| {...}).await`.

### PyO3 Extension Module Management
**Priority:** critical

The `extension-module` feature in `crates/spikard-py/Cargo.toml` must NOT be in default features—it breaks linking for binaries that embed Python (like spikard-cli). Configure maturin in `pyproject.toml` with `features = ["extension-module"]` so Python extension modules build correctly. Binaries (CLI, tests) build without extension-module to link libpython; extensions (maturin builds) enable it for manylinux compliance.

## Sections

### Architecture Overview
**Priority:** medium

## Rust Workspace Layout
- Core library: `crates/spikard/Cargo.toml`.
- CLI frontend: `crates/spikard-cli/`.
- Server with tower-http middleware: `crates/spikard-http/`.
- Bindings: `crates/spikard-{py,node,rb,wasm}/` for Python (PyO3), Node.js (napi-rs), Ruby (magnus), and WebAssembly (wasm-bindgen).
- Future bindings: PHP via ext-php-rs.

## Python & Testing
- Python package scaffold: `packages/python/spikard`.
- Shared integration fixtures: `testing_data/` with schemas per scenario.
- Pytests live in `packages/python/tests/`, using fixture helpers (`packages/python/tests/fixture_app.py`) and sample apps in `examples/`.

## Reference Docs
- High-level design: `docs/adr/0001-architecture-and-principles.md`.
- Validation strategy: `docs/adr/0003-validation-and-fixtures.md`.
- Middleware & lifecycle: `docs/adr/0002-runtime-and-middleware.md` and `docs/adr/0005-lifecycle-hooks.md`.
- Streaming & async protocols: `docs/adr/0006-streaming-and-async-protocols.md`.

### Development Workflow
**Priority:** medium

## Task Runner
- The root `Taskfile.yaml` standardizes workflows; rely on the named tasks below to mirror CI in `.github/workflows/ci.yaml`.
```bash
task setup       # hydrate toolchains + build bindings
task update      # refresh dependencies + hooks
task build       # Rust + Python + JS + WASM builds
task lint        # Rust/Python/JS/Ruby + PHPStan + Prek
task test        # Rust + Python + JS + Ruby + e2e suites
task format      # rustfmt + ruff + biome + rubocop
```

## Targeted Builds
- Use Cargo directly when iterating on a single crate:
```bash
cargo build -p spikard-http
cargo run -p spikard-cli -- --help
```
- Python examples under `examples/` assume the bindings built from `crates/spikard-py`; run them with the synced uv environment after building the crate.
- Design docs live in `docs/adr/` (e.g., `docs/adr/0001-architecture-and-principles.md`, `docs/adr/0002-runtime-and-middleware.md`) and must stay in sync with code changes.

### Testing Strategy
**Priority:** medium

## End-to-End Runs
- Execute the full suite with the task runner so Rust, Python, JS, Ruby, and the fixture-driven e2e suites stay in parity with CI:
```bash
task test
```

## Focused Testing
- Python fixtures live in `testing_data/` and tests in `packages/python/tests/`; iterate quickly with pytest:
```bash
PYTHONPATH=. uv run pytest packages/python/tests/test_integration_query_params.py
```
- Validate Rust crates individually to isolate failures:
```bash
cargo test -p spikard -- --nocapture
cargo test -p spikard-http
```
- When adding new fixture scenarios, mirror the schema files in `testing_data/*/schema.json` so validation stays consistent with generated data.

### Environment Setup
**Priority:** high

## Prerequisites
- Rust toolchain for workspace crates (`Cargo.toml`)
- Node.js 18+ with `pnpm` (`package.json`, `pnpm-workspace.yaml`)
- Python 3.10+ with `uv` (`pyproject.toml`, `uv.lock`)
- `task` CLI to use `Taskfile.yaml` automation

## Install Dependencies
- Run `task setup` to install and verify all toolchains in one shot (wraps `pnpm install`, `uv sync`, ruby bundler install, and the initial PyO3 build).

## Verify Tooling
```bash
task build
```

### Workspace Setup
**Priority:** high

## Tooling
- Rust workspace declared in `Cargo.toml` and per-crate manifests under `crates/`; install the stable toolchain with `rustup` and add required targets as you touch bindings.
- JavaScript tooling is managed by `pnpm-workspace.yaml`; install pnpm ≥8 so shared dependencies resolve correctly.
- Python environments lock via `uv.lock` and `packages/python/pyproject.toml`; install the `uv` CLI to keep wheels in sync.

## Install Once Per Machine
```bash
# from the repository root
pnpm install
uv sync
cargo check
```
- `task setup` installs uv/pnpm/rbenv dependencies, bootstraps developer tooling, and builds the PyO3 bindings so every language stack is ready.
- `task update` is the canonical dependency bump path; it sequentially refreshes Python (uv), Rust (cargo), JavaScript (pnpm), Ruby (bundler), rebuilds the Python bindings, and updates Prek hooks.
- `cargo check` remains the fastest Rust-only validation when iterating on a single crate.

## Agents

### docs-scribe
**Priority:** medium

Generates agent handbooks and CLAUDE briefs that summarize current rules and workflows.

### docs-strategist
**Priority:** medium

Maintains developer guides, upgrade notes, and architecture references so contributors can follow cross-language patterns.

### interop-build-engineer
**Priority:** medium

Ensures binding build scripts for PyO3, napi-rs, magnus/rb-sys, and wasm-pack stay in sync and optimized. Plans for future ext-php-rs integration.

### php-engineer
**Priority:** medium

(FUTURE) Implements and maintains PHP bindings via ext-php-rs, ensuring PSR compliance and idiomatic PHP patterns while integrating with Rust core.

### wasm-engineer
**Priority:** medium

(FUTURE) Maintains WebAssembly bindings via wasm-bindgen for browser and server-side WASM runtimes.

### build-and-ci-ops
**Priority:** high

Curates Taskfile, CI, and release automation to keep multi-language toolchains reproducible.

### fixture-tester
**Priority:** high

Evolves `testing_data` fixtures and validation schemas so every handler respects documented contracts.

### integration-qa
**Priority:** high

Expands fixture-driven coverage and hunts for regressions across Rust, Python, Node, and Ruby integration suites.

### middleware-architect
**Priority:** high

Implements tower-http middleware stack, lifecycle hooks, and authentication middleware while maintaining zero-overhead design principles.

### python-engineer
**Priority:** high

Implements and maintains Python bindings via PyO3, ensuring Pythonic patterns with async/await support, proper type hints, and zero-copy msgspec integration.

### ruby-engineer
**Priority:** high

Implements and maintains Ruby bindings via magnus/rb-sys, ensuring idiomatic Ruby patterns while preserving Rust performance and safety guarantees.

### typescript-engineer
**Priority:** high

Implements TypeScript type definitions and ensures napi-rs Node.js bindings provide full type safety with proper JSDoc annotations and .d.ts generation.

### workspace-architect
**Priority:** high

Structures Cargo workspaces, feature flags, and dependency graph to avoid duplication across crates.

### rust-polyglot-architect
**Priority:** critical

Designs Rust-first APIs and keeps Python, Node, Ruby, and WASM bindings aligned with shared memory-safety and error contracts. Plans for future PHP bindings.

## MCP Servers

### ai-rulez
AI-Rulez MCP server for configuration management
- Transport: stdio
- Command: npx
- Args: -y, ai-rulez@latest, mcp
