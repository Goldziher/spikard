# ADR 0024: The WASM e2e suite is contract testing, not binding e2e

**Status**: Accepted
**Date**: 2026-08-13

## Context

Every other language in the e2e matrix follows the *server pattern*: the generated harness
constructs a real `App`, registers routes on a real `RouteBuilder`, serves HTTP, and the test suite
drives that process. A green suite therefore says something about the binding.

WASM cannot do this, by design. `crates/spikard-wasm` deliberately excludes `App` and `RouteBuilder`
from its exported surface (`alef.toml`, `[crates.wasm] exclude_types`), because the server pattern
requires binding a socket and running a Tokio reactor — neither of which is meaningful in a
`wasm32-unknown-unknown` target. WASM exposes the codec, validation, and configuration types; it does
not expose a server.

The generator already encodes this correctly. `crates/spikard-e2e-http/src/lang/wasm.rs` computes:

```rust
let use_server_pattern = has_http_fixtures && has_harness_imports && !app_class_excluded;
if !use_server_pattern { return Ok(Vec::new()); }
```

Since `App` is permanently in `exclude_types`, `app_class_excluded` is always true, so `emit()`
always returns no files and no WASM app harness is ever produced. The suite falls back to the
generic client pattern, whose `globalSetup.ts` spawns `e2e/rust/target/release/mock-server`.

The problem was never the behaviour — it was the label. The suite lived in the e2e matrix beside
thirteen binding suites, so a green WASM run read as "the WASM binding works". It does not say that.
Confirmed by inspection: none of the 36 test files under `e2e/wasm/tests/` import
`@spikard/node-wasm`, even though `e2e/wasm/package.json` declares it as a dependency.

## Decision

The WASM e2e suite is **mock-server contract testing** and is labelled as such. It verifies that the
fixture corpus and the mock server agree on the HTTP contract. It does not exercise the WASM binding
and must not be counted as binding coverage.

We do **not** build a WASM app harness. Exposing `App`/`RouteBuilder` to make one would mean shipping
a server API on a target that cannot serve, purely to satisfy a test-matrix shape.

The existing generator gate stays as the enforcement point: it is correct, and it fails closed.

## Consequences

- WASM coverage claims must be scoped to what the suite proves. "End-to-end parity across every
  binding" is accurate for the thirteen server-pattern languages; WASM is a fourteenth suite of a
  different kind and is described separately.
- The suite's value is real but narrow: it is a second independent reader of the fixture corpus, so
  it catches fixture/mock-server contract drift. Keep it.
- `e2e/wasm/package.json` depends on `@spikard/node-wasm` and `[crates.test.wasm].before` runs
  `wasm-pack build`, yet no test imports the package. The dependency is load-bearing only for
  `pnpm install` to resolve the `file:` reference. The `wasm-pack build` still earns its place as a
  build check — it proves the WASM package compiles — but it validates compilation, not behaviour.
- Genuine WASM binding coverage, if wanted later, belongs in a unit suite that imports the package
  and exercises the types WASM actually exports. That is a different suite with a different shape,
  not a variant of this one.
- `crates/spikard-e2e-http/src/lang/wasm.rs` remains in the tree with its gate intact. Its module
  documentation is corrected to say it is inert under Spikard's configuration, so a reader does not
  conclude a WASM harness is being produced.

## References

- `crates/spikard-e2e-http/src/lang/wasm.rs` — the `use_server_pattern` gate
- `alef.toml` — `[crates.wasm] exclude_types`, `[crates.test.wasm]`
- `e2e/wasm/globalSetup.ts` — spawns `mock-server`, not a harness
- [ADR 0022: Hybrid Service Testing](0022-hybrid-service-testing.md)
