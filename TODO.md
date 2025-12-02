# Binding Review TODO

Purpose: track the systematic refactor of Rust core and language bindings for DRY structure, minimal binding logic, and shared behavior.

## Review Status
- [x] spikard-core (Rust core) — reviewed 2025-11-28; todos below.
- [x] spikard-http (Rust transport/middleware) — reviewed 2025-11-28; todos below.
- [ ] spikard-py (Python/PyO3)
- [ ] spikard-node (TypeScript/napi-rs)
- [ ] spikard-rb (Ruby/magnus)
- [ ] spikard-php (ext-php-rs)
- [ ] spikard-wasm (wasm-bindgen)

## spikard-core (Rust)
- Precompile parameter validation: ParameterValidator recompiles SchemaValidator on every request after stripping `source`; cache the derived schema and compiled validator at construction to avoid per-call work and schema cloning.
- ParameterSource ergonomics: centralize source→label/header-name logic (query/path/headers/cookie) to remove repeated matches and ensure consistent `loc`/error messages across bindings.
- Error mapping reuse: SchemaValidator builds error details with large string/regex checks; refactor into data-driven lookups keyed by schema pointers to cut duplication and keep fixture-aligned error payloads consistent across bindings.
- Error payload contract: ProblemDetails currently mirrors RFC 9457; reconcile with the repo-wide `{error, code, details}` contract so bindings surface a single shared error constructor instead of re-encoding per language.
- Debug logging: debug.rs prints via `eprintln!` with `[spikard-http]` prefix; route through `tracing` with a crate-appropriate target and remove accidental transport coupling from the core crate.
- Compression robustness: RawResponse compression swallows encoder errors (`unwrap_or_else(|_| Vec::new())`); return a Result or skip compression on failure to avoid empty/corrupted bodies and make errors observable.
- RequestData serde: manual Serialize/Deserialize duplicates field lists; consider a helper/derive-friendly wrapper to reduce maintenance and keep DI/non-DI feature parity.
- Tests vs invariants: strengthen zero-unwrap policy—core files still use `unwrap_or_else`/`unwrap` when finishing gzip and parsing StatusCode; add Result surfaces or map_err to preserve FFI safety.

## spikard-http (Rust transport)
- RequestData duplication: HTTP defines its own RequestData with manual serde; replace with the spikard-core carrier (or a thin Axum-specific wrapper) to avoid drift and ensure bindings share one schema.
- Error shape drift: Lifecycle error handling builds ad-hoc `{"error": ...}` JSON strings; route through the shared structured error constructor (ProblemDetails or `{error, code, details}`) so bindings surface identical payloads.
- Debug logging duplication: http/debug.rs mirrors core debug with `eprintln!`; centralize on the core logger or `tracing` to avoid diverging prefixes and configuration.
- Response types sprawl: Response, HandlerResponse, and RawResponse coexist; collapse onto a single response container in core plus lightweight adapters for Axum/streaming to keep bindings thin.
- Request extraction/validation: JSON body parsing and validation happen in ValidatingHandler with per-request serde roundtrips; reuse SchemaValidator::validate_json (or a shared extractor) to avoid double parsing and keep errors consistent.
- Content-Type checks: validation middleware performs separate JSON/form charset/boundary checks with mixed error shapes; consolidate into the shared validator path used by the handler wrapper to maintain fixture-aligned responses.
- Re-exports without layering: many modules re-export spikard-core counterparts; audit remaining unique logic (middleware/query parsing/openapi) to ensure no drift and move shared pieces into core to keep bindings thin.

## spikard-py (Python)
- Route metadata extraction: converts Py objects to JSON via `json.dumps` + serde parse; replace with zero-copy Py→Value construction (per zero-copy rule) to avoid double-serialization and drift with fixtures. ✅ addressed with `py_to_json_value`.
- Query params + validation: PythonHandler flattens `raw_query_params` to first value, diverging from multi-valued support and core fixtures; reuse the shared lossless converter from core. ✅ addressed.
- Async runtime strategy: custom OnceCell + run_coroutine_threadsafe thread loop instead of `pyo3_async_runtimes::tokio::into_future` + TaskLocals as per ADR; align to avoid extra threads and to release GIL correctly before awaits.
- Debug noise: `create_test_client` writes to /tmp and unconditional `eprintln!` markers; gate behind debug flag/tracing or remove. ✅ removed.
- Error payloads: bindings return RFC 9457 ProblemDetails JSON; wire through the shared `{error, code, details}` constructor to match cross-language contract (non-validation paths still use ad-hoc strings).
- Dead code/artifacts: `handler.rs.bak` sits in src; remove to avoid drift and ensure one source of truth for handler logic. ✅ removed.
- Msgspec usage: keep support for dataclasses/TypedDict/NamedTuple/Pydantic/msgspec.Struct, but ensure serialization/deserialization paths consistently use msgspec (no `json.dumps`/`json.loads`, no double serialization).
- Test client JSON bridge: `python_to_json_value`/`json_value_to_python` in `test_client.rs` round-trip through `json.dumps`/`json.loads` instead of zero-copy msgspec conversions, adding overhead and breaking the msgspec-only rule—replace with direct Value↔Py objects to keep fixtures aligned.
- Python `packages/python` TestClient: spins a subprocess server via cloudpickle and shell script instead of using the shared Rust test client; introduces flakiness, port leaks, and drifts middleware config—replace with Rust-backed in-process client consistent with other bindings.
- Python request stubs: `packages/python/spikard/request.py` is a hand-written stub with no alignment to the actual Rust request shape (missing multi-value query/raw body/accessors); generate/request types from Rust metadata to avoid drift.
- Python app metadata: `packages/python/spikard/app.py` extracts schemas/parameters with Python reflection and builds route metadata in Python, creating a second source of truth for ParameterValidator/file params/body param names; move metadata construction into Rust and make decorators thin, fixture-driven shims.
- Lifecycle hooks duplication: Python app collects lifecycle hooks in Python and passes them to Rust; should delegate hook registration to the Rust core to avoid divergent semantics and ensure zero-cost hook design.

## spikard-node (TypeScript)
- Request marshalling: NodeHandler serializes RequestData to JSON string then parses response JSON back; replace with zero-copy Value passthrough (or shared struct) to avoid double encode/decode and keep parity with fixtures.
- Query params: flattens `raw_query_params` to first value; reuse shared multi-value converter so validation and handlers see the full set.
- Error contract: binding wraps internal failures in ad-hoc strings (`Handler ... failed`); surface shared `{error, code, details}` payloads and avoid panic-y `unwrap_or`.
- Dependency injection: DI block assumes values are JSON strings; provide typed schema-aware decoding or structured map to avoid runtime parse errors and “stringly-typed” dependencies.
- Streaming responses: HandlerReturnValue routing is manual; unify with core RawResponse/HandlerResponse to reduce duplication and ensure compression/headers behavior matches Rust/other bindings.
- Panic shielding: wrap handler invocation with catch_unwind + StructuredError mapping to prevent panics from crossing the napi boundary.
- Missing validation: runtime `NodeHandler` never invokes request/response/parameter validators (only test client does), so HTTP paths bypass schema enforcement; thread the validators into the handler wrapper.
- Content-type defaults: `NodeHandler` builds responses without guaranteeing `content-type`, producing bare JSON strings that don’t match fixture expectations—default to application/json when body exists.
- TypeScript bridge losses: `wrapHandler`/`wrapBodyHandler` in `packages/node` parse and stringify JSON strings and assume `Record<string,string>` query params, dropping multi-values and adding double-serialization overhead; move to structured Value inputs and multi-map support.
- Native loader drift: `packages/node/server.ts` hardcodes Darwin/Generic `.node` names and falls back to throwing; misses Linux/Windows targets and bypasses Rust server when missing—align loader to platform detection or reuse Rust CLI startup path.
- Package-level request model: `packages/node/src/request.ts` redefines Request with string-only query/headers/cookies and local parsing; should be derived from Rust RequestData (multi-map, raw body) to keep behavior in sync and reduce JS-side logic.
- Config/types duplication: `packages/node/src/config.ts` and `index.ts` hand-maintain ServerConfig/RouteMetadata/Schema types; generate from Rust schema registry or share types emitted from core to prevent drift and reduce JS-only logic.
- Test client abstraction: `packages/node/testing.ts` constructs a JS-only mock WebSocket and stringifies routes/handlers before handing to native; use Rust test client directly and drop JS-side multipart/form encoding logic to avoid divergent behavior.

## spikard-rb (Ruby)
- Request/response conversions: multiple ad-hoc JSON/Ruby conversions (json_to_ruby, map_to_ruby_hash, multimap) duplicating core logic; move shared conversions to core and ensure zero-copy where possible.
- Query params: raw_query_params flattened to first value before validation; keep multi-value fidelity consistent with fixtures and other bindings.
- Error payloads: internal errors return ad-hoc strings (e.g., “Handler X failed”), not the shared structured payload; align with `{error, code, details}` contract and ProblemDetails adapter.
- Streaming: streaming enumerator handling builds custom async_stream loops; consolidate with core HandlerResponse/RawResponse to share header/status/compression logic.

## spikard-php (PHP)
- Runtime safety: GLOBAL_RUNTIME uses unwrap_or_else+panic; switch to fallible init that surfaces structured errors to PHP instead of panicking.
- Handler registry: thread_local RefCell Vec<Zval> with manual indices—add bounds checks, cleanup hooks, and avoid leaks; consider a typed registry in Rust core to share across bindings.
- Error payloads: handler invocation returns ad-hoc strings; route through shared `{error, code, details}` payload and avoid leaking Debug of Zval errors.
- Request/response conversion: PhpRequest/interpret_php_response likely duplicate parsing logic; centralize in core and ensure zero-copy where possible.
- Coverage: ensure PSR/middleware parity tests exist in packages/php; add binding-level tests for handler errors and streaming to maintain fixture alignment.
- Panic shielding: add centralized panic catch/convert layer for all PHP entrypoints so Rust panics surface as structured errors rather than unwinding into PHP.
- Callable registration: register_from_zval currently returns plain string errors; convert to StructuredError and enforce bounds checks on registry lookups/cleanup.
- Lifecycle hooks inert: `PhpServer` stores lifecycle hooks but never wires them into router construction, so PHP onRequest/onResponse hooks are dead code; plumb hooks through `build_router_with_handlers_and_config`.
- Config JSON handling: `PhpRequest::new` and handler bridging still parse bodies/schemas via JSON strings with silent fallback, adding double-serialization and inconsistent error surfaces; switch to structured Value passing in line with zero-copy rules.
- PHP package controllers: `packages/php` ControllerMethodHandler/HandlerInterface duplicate routing/parameter extraction/response building in PHP (reflection defaults, attribute matching) instead of using Rust ParameterValidator/schema validation; shift to Rust-driven extraction to avoid drift and enforce fixtures/PSR constraints.
- PHP package Request/Response: PHP-layer request classes and response conversion logic reimplement content-type/status defaults and multi-value query handling, risking divergence; consolidate with Rust-generated DTOs or thin proxies over the native extension.
- PHP lifecycle hooks: package-level lifecycle hooks/live server wiring live only in PHP classes and are not enforced by Rust; move hook registration/execution into Rust to match ADR zero-cost design and remove PHP-only control flow.
- PHP route/config duplication: `packages/php` App/Config classes rebuild route metadata, middleware config, and DI wiring in PHP, creating a second source of truth from Rust manifests; generate PHP facades from Rust schema/metadata to keep parity.

## spikard-wasm (WebAssembly)
- Serialization: TestClient new() expects routes/config as JSON strings and re-parses; consider passing structured Value to avoid double parse and ensure parity with other bindings.
- Error contract: wasm path surfaces JsValue strings for errors; adapt shared structured error payloads for JS side to keep fixtures consistent.
- Rate limiting/state: local Rc<RefCell> rate state in TestClient duplicates server logic; centralize in core or share implementation with native to avoid behavior drift.
- Query params/body: ensure ParameterValidator uses full multi-value inputs; audit matching/matching.rs build_params for flattening.
- Lifecycle hooks: parse_hooks wiring should mirror core zero-cost design; confirm HookResult ShortCircuit/Continue semantics match native and bindings.
- Panic shielding: wrap dispatch/handler execution with catch_unwind and convert to structured JS errors; unify error shape helper with core StructuredError mapping.
- Query fidelity: `matching::parse_query` overwrites duplicate keys in `normalized`, and parameter validation uses that lossy map, dropping multi-valued params compared to fixtures—keep raw multi-map through validation and handler payloads.
- Config shape: WASM TestClient requires config as a JSON string and ignores plain JS objects, forcing extra serialization and diverging from other bindings; accept structured objects for config/hooks.

## Cross-binding consistency
- Implement centralized panic shielding and error surfacing: add a shared wrapper in core that catches panics/FFI boundary failures and converts them into the structured `{error, code, details}` payload for all bindings (Python/Node/Ruby/PHP/WASM), eliminating ad-hoc `panic!`/`unwrap` paths and ensuring consistent host-language exceptions/errors.

## spikard-cli (Rust CLI)
- DTO/style selection duplication: main.rs and apply_dto_selection hand-map DTO variants per language; centralize in a data table to avoid diverging defaults when languages add new DTO styles.
- CodegenEngine file writing: repeated `create_dir_all`/`fs::write` blocks across handlers/fixtures/bundle—extract a single atomic writer with consistent error contexts and optional dry-run to reduce IO divergence.
- AsyncAPI validation UX: `ValidateAsyncapi` prints hardcoded version info (3.0.0) instead of the spec’s actual version and always reports “Schema validated successfully” without structured exit codes; use parsed metadata and return non-zero on validation errors.
- Progress/log output: println!-heavy paths with inconsistent prefixes; route through a small logger (or clap/indicatif styles) for uniform messaging and machine-readable modes.
- Testing gap: CLI has no integration/unit tests covering commands; add snapshot tests for CodegenEngine execution and ValidateAsyncapi to guard DTO routing and file emission across bindings.
