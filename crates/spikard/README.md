# spikard

Core Rust library that defines the shared error types, request primitives, and helper traits consumed by every Spikard binding.

## Responsibilities
- Provide the canonical `SpikardError` and `Result<T>` aliases used across FFI layers.
- Host cross-language safe logic so adapters in `spikard-http`, `spikard-py`, `spikard-node`, and `spikard-wasm` can stay thin.
- Maintain panic-free APIs; everything returned to bindings must surface as structured errors.

## Development
- Build with `cargo build -p spikard` (or `task build:rust` for the full workspace).
- Run unit tests via `cargo test -p spikard`.
- Enforce formatting with `cargo fmt` and stay aligned with workspace-wide Clippy settings.
