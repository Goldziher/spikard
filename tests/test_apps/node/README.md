# Spikard Node.js Test App

## Purpose

Test application that validates the published `@spikard/node` npm package (v0.10.0) works correctly in a real Node.js environment.

## Setup

```bash
cd tests/test_apps/node
pnpm install
```

## Run Tests

```bash
pnpm test
```

## Troubleshooting

### Package not found
- Verify `@spikard/node@0.10.0` is published to npm
- Check registry access: `pnpm view @spikard/node versions`
- Try clearing cache: `pnpm store prune`

### TypeScript errors
- Ensure TypeScript 5.7+ is installed
- Check type definitions are included in package
- Verify `tsconfig.json` settings

### Test failures
- Confirm server starts on random port (0)
- Check fetch API availability (Node 18+)
- Verify handler signatures match exported types

### Import errors
- Ensure `"type": "module"` in package.json
- Use `.js` extensions in TypeScript imports
- Check napi-rs build artifacts are included
