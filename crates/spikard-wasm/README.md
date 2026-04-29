# spikard-wasm

Wasm-safe surface of spikard for browsers and edge runtimes.

Spikard's server runtime (`spikard`, `spikard-http`, `spikard-graphql`)
depends on `axum`, `tokio`, and `mio` — none of which compile to
`wasm32-unknown-unknown`. This crate exposes a deliberately narrow,
transport-free subset of `spikard-core` over `wasm-bindgen`.

## What's exposed

- `validateJsonSchema(schemaJson, dataJson)` — validate JSON Schema against
  data; returns `null` on success or a JSON-encoded `ValidationError`.
- `problemDetails(typeUri, title, status, detail?)` — RFC 7807
  `application/problem+json` builder, returns serialised JSON.
- `problemNotFound`, `problemBadRequest`, `problemInternalServerError` —
  convenience helpers for the common status codes.

## Build

```sh
cargo build -p spikard-wasm --target wasm32-unknown-unknown --release
# or via wasm-pack for npm packaging:
wasm-pack build crates/spikard-wasm --target bundler
```

## Why a separate crate

Adding a `wasm` feature to the umbrella `spikard` crate would need extensive
`cfg`-gating across the server module tree. Keeping the wasm surface in a
sibling crate that depends only on `spikard-core` (with no default features)
isolates the wasm-safe code path and makes the constraint legible at the
Cargo manifest level.
