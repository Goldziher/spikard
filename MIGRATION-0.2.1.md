# Migration Guide: v0.2.0 → v0.2.1

## Breaking Changes

All npm packages have been moved to the `@spikard` organization scope.

### Node.js Package

**Before:**
```bash
npm install spikard
```
```typescript
import { Spikard } from 'spikard';
```

**After:**
```bash
npm install @spikard/node
```
```typescript
import { Spikard } from '@spikard/node';
```

### WebAssembly Package

**Before:**
```bash
npm install spikard-wasm
```
```typescript
import * as spikard from 'spikard-wasm';
```

**After:**
```bash
npm install @spikard/wasm
```
```typescript
import * as spikard from '@spikard/wasm';
```

## Migration Steps

1. **Update package.json dependencies:**

   Replace:
   ```json
   {
     "dependencies": {
       "spikard": "^0.2.0",
       "spikard-wasm": "^0.2.0"
     }
   }
   ```

   With:
   ```json
   {
     "dependencies": {
       "@spikard/node": "^0.2.1",
       "@spikard/wasm": "^0.2.1"
     }
   }
   ```

2. **Update all import statements:**

   Find and replace in your codebase:
   - `from 'spikard'` → `from '@spikard/node'`
   - `from 'spikard-wasm'` → `from '@spikard/wasm'`
   - `require('spikard')` → `require('@spikard/node')`
   - `require('spikard-wasm')` → `require('@spikard/wasm')`

3. **Run install:**
   ```bash
   npm install
   # or
   pnpm install
   # or
   yarn install
   ```

## Why This Change?

- **Prevents npm spam detection:** Platform-specific packages (e.g., `@spikard/darwin-arm64`) no longer trigger npm's spam detection when published under an organization scope
- **Establishes proper package organization:** All Spikard packages are now clearly grouped under the `@spikard` namespace
- **Follows napi-rs best practices:** Aligns with the recommended approach for publishing native Node.js addons with platform-specific binaries

## Other Language Bindings

No changes are required for other language bindings:

- **Python:** `pip install spikard` (unchanged)
- **Ruby:** `gem install spikard` (unchanged)
- **PHP:** `composer require spikard/spikard` (unchanged)
- **Rust:** `cargo add spikard` (unchanged)

## Questions?

If you encounter any issues during migration, please:
- Check the [installation guide](docs/getting-started/installation.md)
- Review the [TypeScript binding documentation](docs/bindings/typescript.md)
- Open an issue on [GitHub](https://github.com/Goldziher/spikard/issues)
