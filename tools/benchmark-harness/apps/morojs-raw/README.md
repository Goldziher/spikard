# MoroJS Raw (No Validation)

Benchmark application using MoroJS framework in raw mode without validation.

## Framework

- **MoroJS 1.7+**: TypeScript-first framework built on uWebSockets.js
- **Node.js 20+**: Modern JavaScript runtime

## Raw Mode

This app demonstrates MoroJS in **raw performance mode** - no validation is performed on incoming requests. All routes use `.handler()` directly without `.body()` schema validation.

### Key Differences from morojs-validation

- **No Zod schemas**: Removed all `z.object()` definitions
- **No `.body()` chains**: Routes use `.handler()` directly instead of `.body(Schema).handler()`
- **Direct body access**: `req.body` is accessed without validation
- **Pure performance**: Measures MoroJS framework overhead without validation cost

## Routes

All routes return data without validation:

### JSON Body Routes
- `POST /json/small` - Echo small JSON payload
- `POST /json/medium` - Echo medium JSON payload
- `POST /json/large` - Echo large JSON payload
- `POST /json/very-large` - Echo very large JSON payload

### Other Routes
- Multipart form routes (`/multipart/*`)
- URL-encoded routes (`/urlencoded/*`)
- Path parameter routes (`/path/*`)
- Query parameter routes (`/query/*`)
- Health check routes (`/health`, `/`)

## Installation

```bash
pnpm install
```

## Running

```bash
pnpm start [port]
```

Default port: 8000

## Comparison with morojs-validation

- **morojs-validation**: Uses Zod schemas with `.body()` validation
- **morojs-raw** (this app): No validation, direct request handling

This pair demonstrates the performance impact of MoroJS's built-in Zod validation support.
