# Hono vs Hono-Raw Performance Comparison

## Overview

This document compares the **hono** (with Zod validation) and **hono-raw** (no validation) implementations.

## Key Differences

| Aspect | hono | hono-raw |
|--------|------|----------|
| Validation | ✅ Zod schemas with @hono/zod-validator | ❌ None - accepts any JSON |
| Dependencies | hono, @hono/node-server, @hono/zod-validator, zod | hono, @hono/node-server only |
| Request Parsing | `c.req.valid('json')` after validation | `c.req.json()` directly |
| Error Handling | Validation errors returned with 400 status | None - echoes any input |
| Type Safety | Runtime validation + type inference | None |
| Performance | Validation overhead included | Raw framework performance |

## Code Comparison

### hono (with validation)
```javascript
const SmallPayloadSchema = z.object({
  name: z.string(),
  description: z.string(),
  price: z.number(),
  tax: z.number().optional(),
});

app.post('/json/small', zValidator('json', SmallPayloadSchema), (c) => {
  const validated = c.req.valid('json');
  return c.json(validated);
});
```

### hono-raw (no validation)
```typescript
app.post('/json/small', async (c) => {
  const body = await c.req.json();
  return c.json(body);
});
```

## Endpoints Implemented

Both implementations have identical endpoint coverage:

### JSON Body Workloads (4 endpoints)
- `POST /json/small` - Small payload
- `POST /json/medium` - Medium payload
- `POST /json/large` - Large payload
- `POST /json/very-large` - Very large payload

### Multipart Form (3 endpoints)
- `POST /multipart/small` - 1 file, 1KB
- `POST /multipart/medium` - 2 files, 10KB
- `POST /multipart/large` - 5 files, 100KB

### URL Encoded Form (2 endpoints)
- `POST /urlencoded/simple`
- `POST /urlencoded/complex`

### Path Parameters (6 endpoints)
- `GET /path/simple/:id`
- `GET /path/multiple/:user_id/:post_id`
- `GET /path/deep/:org/:team/:project/:resource/:id`
- `GET /path/int/:id`
- `GET /path/uuid/:uuid`
- `GET /path/date/:date`

### Query Parameters (3 endpoints)
- `GET /query/few`
- `GET /query/medium`
- `GET /query/many`

**Total: 18 workload endpoints** (plus 2 health check endpoints)

## Expected Performance Impact

The performance difference between hono and hono-raw will reveal:

1. **Zod Validation Overhead**: Time spent validating JSON schemas
2. **Type Coercion Cost**: Converting and validating primitive types
3. **Nested Object Validation**: Cost of validating complex nested structures
4. **Array Validation**: Overhead of validating arrays of objects

## Running Benchmarks

```bash
# Start hono (with validation)
cd ../hono && pnpm start 8000

# Start hono-raw (no validation)
cd ../hono-raw && pnpm start 8001

# Run benchmarks against both
# Compare throughput, latency, and resource usage
```

## Expected Findings

- **hono-raw** should show higher throughput for JSON endpoints
- Validation overhead should be most visible on:
  - `/json/large` (many items to validate)
  - `/json/very-large` (deep object graphs)
  - `/json/medium` (nested objects + arrays)
- Path and query parameter performance should be similar (no complex validation)

## Use Cases

### Use hono (with validation) when:
- You need runtime type safety
- API contracts must be enforced
- Invalid data should be rejected with clear errors
- Type inference from schemas is valuable

### Use hono-raw (no validation) when:
- Maximum performance is critical
- Validation happens elsewhere (client-side, API gateway)
- Benchmarking framework overhead only
- Measuring raw routing/serialization performance
