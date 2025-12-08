# Spikard WASM + Rollup Example

A minimal Spikard HTTP application built with **TypeScript**, **Rollup bundler**, and the **Spikard WASM** framework. This example demonstrates how to build and bundle a Spikard application for both browser and Node.js environments with strict TypeScript configuration and production-ready bundling.

## Features

- **Strict TypeScript**: Full strict mode with all compiler flags enabled
- **Rollup Bundling**: Dual ESM and CommonJS builds optimized for different environments
- **Type-Safe Framework**: Uses Spikard's type-safe routing and handler system
- **Development Mode**: Watch-mode rebuilds during active development
- **Production Optimization**: Minification and tree-shaking for production builds
- **Sourcemaps**: Full sourcemap support for debugging in both dev and production

## Project Structure

```
examples/wasm-rollup/
├── src/
│   └── index.ts                 # Main application with 3 routes
├── package.json                 # Project metadata and dependencies
├── tsconfig.json                # Strict TypeScript configuration
├── rollup.config.js             # Rollup bundler configuration
└── README.md                     # This file
```

## Routes

The example includes the following endpoints:

| Method | Path          | Description                        |
| ------ | ------------- | ---------------------------------- |
| GET    | `/`           | Welcome message with server info   |
| GET    | `/api/data`   | Returns sample framework metadata  |
| POST   | `/api/echo`   | Echoes back the request body       |

## Installation

Install dependencies using pnpm:

```bash
pnpm install
```

This will:
1. Install Rollup and its plugins
2. Install TypeScript and dev tooling
3. Link the workspace `@spikard/wasm` package

## Development

Start the development server with watch-mode rebuilds:

```bash
pnpm run dev
```

This will:
- Watch `src/**/*.ts` for changes
- Automatically rebuild on file changes
- Generate ESM and CommonJS bundles in `dist/`

## Building

Build the project for production:

```bash
pnpm run build
```

This will:
1. Compile TypeScript to JavaScript
2. Bundle dependencies with Rollup
3. Generate ESM bundle (`dist/index.mjs`)
4. Generate CommonJS bundle (`dist/index.cjs`)
5. Minify output with terser (production only)
6. Create TypeScript declaration files

## Output

After building, the `dist/` directory contains:

```
dist/
├── index.mjs                    # ESM bundle for browsers and bundlers
├── index.cjs                    # CommonJS bundle for Node.js
├── index.d.ts                   # TypeScript type declarations
└── [sourcemaps]                 # Source maps for debugging (dev builds only)
```

## Usage

### In Node.js

```typescript
import app from "./dist/index.mjs";

// Start the server on custom port
app.run({ port: 3000, host: "0.0.0.0" });
```

### CommonJS Environments

```javascript
const app = require("./dist/index.cjs").default;

// Start the server
app.run({ port: 8080 });
```

### Creating Custom Routes

```typescript
import { Spikard, get, post } from "@spikard/wasm";
import type { HandlerFunction } from "@spikard/wasm";

const app = new Spikard();

const myHandler: HandlerFunction = async (request) => {
	return {
		statusCode: 200,
		headers: { "content-type": "application/json" },
		body: { message: "Hello from Spikard" },
	};
};

get("/my-route")(myHandler);
post("/my-post-route")(myHandler);

app.run({ port: 8080 });
```

## TypeScript Configuration

The project uses **maximal TypeScript strictness**:

```json
{
	"strict": true,
	"noImplicitAny": true,
	"noUncheckedIndexedAccess": true,
	"exactOptionalPropertyTypes": true,
	"noUnusedLocals": true,
	"noUnusedParameters": true,
	"strictNullChecks": true,
	"strictFunctionTypes": true,
	"strictBindCallApply": true,
	"strictPropertyInitialization": true
}
```

This ensures:
- No implicit `any` types
- Full type safety for all operations
- Unused code is flagged as errors
- Optional properties must be handled explicitly

## Rollup Configuration

The build produces two separate bundles optimized for different runtime environments:

### ESM Bundle (`dist/index.mjs`)
- **Target**: Browsers, modern bundlers, edge runtimes (Cloudflare Workers, Deno)
- **Format**: ES modules with top-level await support
- **Optimization**: Tree-shaking, code splitting, production minification
- **Dependencies**: All bundled (no externals) for maximum portability
- **Sourcemaps**: Included for debugging (dev builds only)

### CommonJS Bundle (`dist/index.cjs`)
- **Target**: Node.js environments, legacy JavaScript ecosystems
- **Format**: CommonJS with named exports
- **Module Resolution**: Uses Node.js native resolution
- **External Dependencies**: `@spikard/wasm` resolved from node_modules
- **Sourcemaps**: Included for debugging (dev builds only)

Both bundles support the same API and can be used interchangeably depending on your runtime.

## Dependencies

### Production
- `@spikard/wasm`: Spikard HTTP framework for WebAssembly runtimes

### Development
- `rollup`: JavaScript module bundler
- `@rollup/plugin-node-resolve`: Node.js module resolution
- `@rollup/plugin-typescript`: TypeScript compilation
- `@rollup/plugin-terser`: Code minification
- `typescript`: TypeScript compiler
- `tslib`: TypeScript runtime helpers
- `@types/node`: Node.js type definitions

## Next Steps

1. **Modify routes**: Edit `src/index.ts` to add your own endpoints using `get()`, `post()`, `put()`, `patch()`, `del()` decorators
2. **Add handlers**: Create new `HandlerFunction` implementations with full type safety
3. **Bundle for production**: Run `NODE_ENV=production pnpm run build`
4. **Deploy**: Use the generated bundles:
   - ESM bundle for edge runtimes (Cloudflare Workers, Vercel Edge, etc.)
   - CommonJS bundle for Node.js or traditional servers

## Advanced Usage

### Lifecycle Hooks

```typescript
app.onRequest(async (request) => {
	console.log(`${request.method} ${request.path}`);
	return request;
});

app.onResponse(async (response) => {
	console.log(`Response: ${response.statusCode}`);
	return response;
});
```

### Response Streaming

```typescript
import { StreamingResponse } from "@spikard/wasm";

const handleStream: HandlerFunction = async () => {
	return new StreamingResponse(
		async function* () {
			for (let i = 0; i < 5; i++) {
				yield `data: ${JSON.stringify({ count: i })}\n\n`;
				await new Promise((r) => setTimeout(r, 100));
			}
		},
		{
			statusCode: 200,
			headers: {
				"content-type": "text/event-stream",
				"cache-control": "no-cache",
			},
		}
	);
};
```

## Learn More

- [Spikard Documentation](https://github.com/Goldziher/spikard)
- [Rollup Documentation](https://rollupjs.org/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/)
- [@spikard/wasm Package](https://github.com/Goldziher/spikard/tree/main/packages/wasm)

## License

MIT
