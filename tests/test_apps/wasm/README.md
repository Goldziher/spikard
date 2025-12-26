# Spikard WASM Test App

## Purpose

Test application that validates the published `@spikard/wasm` npm package (v0.6.1) works correctly in a WebAssembly environment.

## Setup

```bash
cd tests/test_apps/wasm
pnpm install
```

## Run Tests

```bash
pnpm test
```

## Troubleshooting

### Package not found
- Verify `@spikard/wasm@0.6.1` is published to npm
- Check registry access: `pnpm view @spikard/wasm versions`
- Try clearing cache: `pnpm store prune`

### WASM initialization errors
- Ensure wasm-bindgen version matches between build and runtime
- Check .wasm file is included in published package
- Verify wasm-pack bundled correctly with --target bundler

### Module loading errors
- Confirm ESM imports work: `"type": "module"` in package.json
- Check WASM file MIME type if serving via HTTP
- Ensure async init() completes before server creation

### Test failures
- Verify WASM module loads in Node.js environment
- Check fetch API is available (Node 18+)
- Ensure server starts on random port (0)

### Performance issues
- Use wasm-opt for smaller binary size
- Enable wasm-pack --release mode
- Avoid blocking operations in WASM handlers

### Browser compatibility
- Test with both Node.js and browser targets
- Check wasm-pack target matches your environment
- Verify no Node.js-specific APIs in browser code
