# spikard-node

Node.js bindings for Spikard built with `napi-rs` and packaged through `@napi-rs/cli`.

## Build & Test
- Install deps once with `pnpm install` in the repository root.
- Compile release artifacts using `pnpm run build` (or `task build:node`).
- Run the Vitest suite with `pnpm run test` to verify the generated `.node` modules.

## Notes
- The crate produces a `cdylib` with napi8 ABI support and enables `mimalloc` on 64-bit desktop targets for predictable latency.
- Build artifacts are emitted under `npm/` (one directory per target triple) and are published through the workspace package `@spikard/node`.
