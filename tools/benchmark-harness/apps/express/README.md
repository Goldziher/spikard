# Express Benchmark Server

Express comparison server implementing all 18 workload endpoints with Zod validation.

## Features

- **Zod Validation**: Request body validation using Zod schemas
- **TypeScript Support**: Full TypeScript implementation with strict type checking
- **Middleware Pattern**: Validation middleware for clean separation of concerns
- **All Workloads**: Complete implementation of JSON, multipart, URL-encoded, path, and query parameter workloads

## Files

- `server.ts` - TypeScript implementation with full type safety
- `server.js` - JavaScript implementation (legacy, kept for compatibility)
- `package.json` - Dependencies and scripts
- `tsconfig.json` - TypeScript configuration with strictest settings

## Quick Start

### Using TypeScript (Recommended)

```bash
# Install dependencies
npm install

# Run with tsx (no build step needed)
npm run start:ts

# Run on custom port
npm run start:ts 8001

# Development mode with auto-reload
npm run dev

# Build TypeScript to JavaScript
npm run build
```

### Using JavaScript

```bash
npm start
# or
node server.js 8001
```

## Endpoints

### JSON Body Workloads (with Zod validation)
- `POST /json/small` - Small payload (~100 bytes)
- `POST /json/medium` - Medium payload (~1KB)
- `POST /json/large` - Large payload (~10KB)
- `POST /json/very-large` - Very large payload (~100KB)

### Multipart Form Workloads
- `POST /multipart/small` - Small multipart (~1KB)
- `POST /multipart/medium` - Medium multipart (~10KB)
- `POST /multipart/large` - Large multipart (~100KB)

### URL-Encoded Form Workloads
- `POST /urlencoded/simple` - Simple form
- `POST /urlencoded/complex` - Complex form

### Path Parameter Workloads
- `GET /path/simple/:id` - Single path parameter
- `GET /path/multiple/:user_id/:post_id` - Multiple path parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Deep nested paths
- `GET /path/int/:id` - Integer path parameter
- `GET /path/uuid/:uuid` - UUID path parameter
- `GET /path/date/:date` - Date path parameter

### Query Parameter Workloads
- `GET /query/few` - Few query parameters (1-2)
- `GET /query/medium` - Medium query parameters (3-5)
- `GET /query/many` - Many query parameters (6-10)

### Health Checks
- `GET /health` - Health check endpoint
- `GET /` - Root endpoint

## Validation Approach

Express has no native request validation, so we use Zod with a middleware pattern:

```typescript
function validateBody<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    try {
      req.body = schema.parse(req.body);
      next();
    } catch (error) {
      if (error instanceof z.ZodError) {
        res.status(400).json({
          error: 'Validation failed',
          details: error.errors,
        });
      }
    }
  };
}

// Usage:
app.post('/json/small', validateBody(SmallPayloadSchema), (req, res) => {
  res.json(req.body);
});
```

## Zod Schemas

All POST endpoints with JSON bodies use Zod schemas for validation:

- `SmallPayloadSchema` - name, description, price, optional tax
- `MediumPayloadSchema` - user_id, username, email, is_active, address (nested), tags array
- `LargePayloadSchema` - order_id, customer_name, items array (nested), total, notes
- `VeryLargePayloadSchema` - data array of records, metadata object

## TypeScript Configuration

Strictest TypeScript settings enabled:

- `strict: true` - All strict type-checking options
- `noUncheckedIndexedAccess: true` - Prevent unsafe array/object access
- `exactOptionalPropertyTypes: true` - Exact optional property types
- `noImplicitReturns: true` - All code paths must return
- `noUnusedLocals: true` - No unused variables
- `noUnusedParameters: true` - No unused parameters

## Dependencies

### Production
- `express` 5.1.0 - Web framework
- `zod` 3.24.1 - Schema validation

### Development
- `typescript` 5.7.2 - TypeScript compiler
- `tsx` 4.19.2 - TypeScript execution (no build step)
- `@types/express` 5.0.0 - Express type definitions
- `@types/node` 22.0.0 - Node.js type definitions

## Comparison with Other Frameworks

Express is chosen as a baseline comparison because:

1. **No Native Validation**: Requires third-party validation (Zod)
2. **Middleware Pattern**: Different from FastAPI/Robyn/Spikard's decorator/route patterns
3. **Mature Ecosystem**: Industry standard with extensive middleware
4. **Performance Baseline**: Good for comparing against newer frameworks

Unlike FastAPI (Pydantic) or Spikard (built-in), Express requires explicit validation middleware setup.

## Running Benchmarks

From the benchmark harness root:

```bash
# Start Express server
cd apps/express
npm run start:ts 8001

# Run benchmarks against Express
cd ../..
python -m benchmark_harness.cli run --server express --port 8001
```

## Notes

- Port defaults to 8000, configurable via command line argument
- Server binds to `0.0.0.0` for Docker compatibility
- Uses `express.json()` and `express.urlencoded()` middleware for body parsing
- Validation errors return 400 with Zod error details
- TypeScript version uses strict null checks (`??` instead of `||`)
