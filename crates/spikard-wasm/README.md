# spikard-wasm

WebAssembly bindings that wrap the core Spikard APIs with `wasm-bindgen` for browser and Node targets.

## Build & Test
- Install deps once with `pnpm install` in the repository root.
- Generate artifacts via `pnpm run build:all` (or `task build:wasm`), which emits bundles under `dist-node/` and `dist-web/`.
- Execute `pnpm run test` to run the wasm-pack Node test harness.

## Notes
- Release builds enable `wasm-opt` with `-O3` and bulk-memory support; adjust under `[package.metadata.wasm-pack.profile.*]` if necessary.
- Keep the exported surface minimal—business logic should remain in `crates/spikard` and flow through thin `wasm-bindgen` shims here.
