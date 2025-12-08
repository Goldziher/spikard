# Spikard WASM on Cloudflare Workers

A minimal example demonstrating Spikard WebAssembly bindings running on Cloudflare Workers. This example showcases how to build and deploy a high-performance HTTP API using Spikard's WASM runtime.

## Architecture

This example implements a simple HTTP API with the following features:

- **Routes**: Home, API data retrieval, echo endpoint, and health check
- **Type Safety**: Full TypeScript with strict compiler settings
- **Error Handling**: Structured JSON error responses
- **Performance**: Optimized WASM binary size and execution
- **Cloudflare Integration**: Native Workers runtime with environment bindings support

## Quick Start

### Prerequisites

- Node.js 18+ with pnpm 8+
- Cloudflare Wrangler CLI
- Rust toolchain (for WASM compilation, if building from source)

### Installation

```bash
# Install dependencies
pnpm install

# Install Wrangler if not already available globally
pnpm add -D wrangler
```

### Development

Run the local development server:

```bash
pnpm run dev
```

This starts a local server on `http://localhost:8787` that mimics Cloudflare Workers behavior.

### Testing Routes

Once the development server is running:

```bash
# Get home information
curl http://localhost:8787/

# Get sample data
curl http://localhost:8787/api/data

# Echo request (POST)
curl -X POST http://localhost:8787/api/echo \
  -H "Content-Type: application/json" \
  -d '{"message": "Hello Spikard!", "metadata": {"user": "dev"}}'

# Health check
curl http://localhost:8787/health
```

### Deployment

Deploy to Cloudflare Workers:

```bash
pnpm run deploy
```

By default, this deploys to a development environment. To deploy to production:

```bash
wrangler deploy --env production
```

## Project Structure

```
wasm-cloudflare/
├── src/
│   └── index.ts          # Main handler with route definitions
├── package.json          # Dependencies and scripts
├── tsconfig.json         # TypeScript configuration (strict mode)
├── wrangler.toml         # Cloudflare Workers configuration
└── README.md             # This file
```

## API Endpoints

### GET /

Home endpoint returning available routes.

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Spikard WASM on Cloudflare Workers",
    "routes": ["GET /", "GET /api/data", "POST /api/echo", "GET /health"]
  },
  "timestamp": "2024-12-08T12:34:56.789Z"
}
```

### GET /api/data

Retrieve sample data with unique identifiers.

**Response:**
```json
{
  "success": true,
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "spikard-cloudflare",
    "version": "0.3.7",
    "timestamp": "2024-12-08T12:34:56.789Z"
  },
  "timestamp": "2024-12-08T12:34:56.789Z"
}
```

### POST /api/echo

Echo back the request body with metadata.

**Request:**
```json
{
  "message": "Hello Spikard!",
  "metadata": {
    "user": "developer",
    "source": "example"
  }
}
```

**Response:**
```json
{
  "success": true,
  "data": {
    "message": "Hello Spikard!",
    "metadata": {
      "user": "developer",
      "source": "example"
    },
    "received_at": "2024-12-08T12:34:56.789Z"
  },
  "timestamp": "2024-12-08T12:34:56.789Z"
}
```

### GET /health

Health check endpoint for monitoring.

**Response:**
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "checks": {
      "wasm": "operational",
      "cloudflare": "operational"
    }
  },
  "timestamp": "2024-12-08T12:34:56.789Z"
}
```

## Configuration

### tsconfig.json

Strict TypeScript settings enforce type safety:
- `strict: true` - All strict type checking options
- `noUncheckedIndexedAccess: true` - Prevent unsafe array access
- `exactOptionalPropertyTypes: true` - Precise optional property handling

### wrangler.toml

Cloudflare Workers configuration:
- `compatibility_date: 2024-12-02` - Latest runtime compatibility
- `node_compat: true` - Enable Node.js API compatibility
- Two environments: `development` and `production`

## Error Handling

All endpoints return structured JSON responses with error information:

```json
{
  "success": false,
  "error": "Invalid request body. Expected { message: string, metadata?: object }",
  "timestamp": "2024-12-08T12:34:56.789Z"
}
```

## Performance Considerations

1. **WASM Binary Size**: The `@spikard/wasm` package is optimized with:
   - Tree-shaking of unused code
   - Optimized build level `z` with LTO
   - Multiple targets (bundler, web, nodejs)

2. **Caching Headers**: Endpoints use appropriate Cache-Control headers:
   - `/` and `/api/data`: 30-60 second cache
   - `/health`: no-cache (always fresh)
   - `/api/echo`: no cache (request-dependent)

3. **Async Handlers**: All I/O operations use native async/await

## Development

### Adding New Routes

1. Create a new handler function in `src/index.ts`
2. Add route pattern matching in the main fetch handler
3. Import and use Spikard WASM utilities as needed
4. Test locally with `pnpm run dev`

### Type Safety

All request/response types are defined as TypeScript interfaces for full type checking at compile time.

## Deployment Notes

- Cloudflare automatically provisions SSL/TLS certificates
- Workers have cold start times typically under 5ms
- Global edge network distribution for low-latency responses
- Environment-specific secrets can be configured in `wrangler.toml`

## Dependencies

- `@spikard/wasm`: Spikard WebAssembly bindings
- `wrangler`: Cloudflare Workers development and deployment CLI
- `@cloudflare/workers-types`: TypeScript type definitions for Workers API
- `typescript`: TypeScript compiler with strict settings

## Resources

- [Spikard Documentation](https://github.com/Goldziher/spikard)
- [Cloudflare Workers Guide](https://developers.cloudflare.com/workers/)
- [wasm-pack Documentation](https://rustwasm.org/docs/wasm-pack/)
- [TypeScript Strict Mode](https://www.typescriptlang.org/tsconfig#strict)

## License

MIT - See repository root LICENSE file
