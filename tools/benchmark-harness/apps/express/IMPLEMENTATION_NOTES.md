# Express TypeScript Implementation Notes

## TypeScript vs JavaScript Differences

### Type Safety

**TypeScript (server.ts):**
```typescript
import express, { type Request, type Response, type NextFunction } from 'express';
import { z, type ZodSchema } from 'zod';

function validateBody<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    // Full type inference and safety
  };
}

type SmallPayload = z.infer<typeof SmallPayloadSchema>;

app.post('/json/small', validateBody(SmallPayloadSchema), (req: Request, res: Response): void => {
  res.json(req.body as SmallPayload); // Type-safe after validation
});
```

**JavaScript (server.js):**
```javascript
const express = require('express');
const { z } = require('zod');

function validateBody(schema) {
  return (req, res, next) => {
    // No type checking
  };
}

app.post('/json/small', validateBody(SmallPayloadSchema), (req, res) => {
  res.json(req.body); // No type safety
});
```

### Null Safety

**TypeScript:**
```typescript
// Uses nullish coalescing (??) for strict null checks
res.json(req.body ?? {});
res.json({ id: Number.parseInt(req.params.id ?? '0', 10) });
```

**JavaScript:**
```javascript
// Uses logical OR (||) which treats empty strings/0 as falsy
res.json(req.body || {});
res.json({ id: parseInt(req.params.id, 10) });
```

### Error Handling

**TypeScript:**
```typescript
function validateBody<T>(schema: ZodSchema<T>) {
  return (req: Request, res: Response, next: NextFunction): void => {
    try {
      req.body = schema.parse(req.body);
      next();
    } catch (error) {
      if (error instanceof z.ZodError) {
        // Type-narrowed error handling
        res.status(400).json({
          error: 'Validation failed',
          details: error.errors,
        });
      } else {
        res.status(400).json({
          error: 'Validation failed',
          details: String(error),
        });
      }
    }
  };
}
```

**JavaScript:**
```javascript
function validateBody(schema) {
  return (req, res, next) => {
    try {
      req.body = schema.parse(req.body);
      next();
    } catch (error) {
      // No type narrowing
      res.status(400).json({
        error: 'Validation failed',
        details: error.errors
      });
    }
  };
}
```

### Unused Parameter Handling

**TypeScript:**
```typescript
// Explicit unused parameter naming with underscore
app.post('/multipart/small', (_req: Request, res: Response): void => {
  res.json({ files_received: 1, total_bytes: 1024 });
});
```

**JavaScript:**
```javascript
// No convention for unused parameters
app.post('/multipart/small', (req, res) => {
  res.json({ files_received: 1, total_bytes: 1024 });
});
```

## Zod Schema Patterns

### Nested Objects
```typescript
const AddressSchema = z.object({
  street: z.string(),
  city: z.string(),
  state: z.string(),
  zip_code: z.string(),
});

const MediumPayloadSchema = z.object({
  // ... other fields
  address: AddressSchema, // Nested schema
  tags: z.array(z.string()),
});
```

### Optional Fields
```typescript
const SmallPayloadSchema = z.object({
  name: z.string(),
  description: z.string(),
  price: z.number(),
  tax: z.number().optional(), // Optional field
});
```

### Arrays and Records
```typescript
const VeryLargePayloadSchema = z.object({
  data: z.array(z.record(z.unknown())), // Array of records
  metadata: z.record(z.unknown()),      // Record of unknown
});
```

## Validation Middleware Pattern

The Express validation pattern differs significantly from FastAPI/Robyn:

1. **FastAPI/Robyn**: Validation is built into route decorators via Pydantic models
2. **Express**: Requires explicit middleware functions wrapping Zod schemas

### Why Middleware?

Express has no built-in validation, so we use middleware to:
- Intercept requests before handlers
- Parse and validate request bodies
- Transform validated data into typed objects
- Return validation errors with proper HTTP status codes
- Allow `next()` to pass control to the handler

### Validation Flow

```
Request → express.json() → validateBody(schema) → Handler → Response
           ↓                       ↓                  ↓
       Parse JSON          Validate with Zod    Use typed data
                                 ↓
                         Error? Return 400
```

## TypeScript Strict Mode Benefits

With `tsconfig.json` strict settings:

1. **noUncheckedIndexedAccess**: Prevents unsafe `req.params.id` access without null checks
2. **exactOptionalPropertyTypes**: Ensures optional fields are exactly `T | undefined`, not `T | null`
3. **noImplicitReturns**: All handler functions must have explicit return types (`: void`)
4. **noUnusedParameters**: Catches unused request parameters (forces `_req` naming)
5. **strict**: Enables all strict type-checking options

## Running Both Versions

### TypeScript (Development)
```bash
npm run start:ts      # Direct execution with tsx
npm run dev          # Watch mode with auto-reload
```

### TypeScript (Production)
```bash
npm run build        # Compile to JavaScript
node dist/server.js  # Run compiled version
```

### JavaScript (Legacy)
```bash
npm start            # Run server.js directly
node server.js 8001  # Custom port
```

## Performance Considerations

### TypeScript Runtime
- `tsx` adds minimal overhead (~5-10ms startup)
- No performance difference after startup (both run as JavaScript)
- Compiled TypeScript is identical to hand-written JavaScript

### Validation Overhead
- Zod validation adds overhead to every POST request
- Nested schemas (Medium/Large payloads) are more expensive
- Trade-off: Runtime safety vs. raw performance

## Integration with Benchmark Harness

The benchmark harness can test both versions:

```bash
# Test TypeScript version
npm run start:ts 8001 &
python -m benchmark_harness.cli run --server express --port 8001

# Test JavaScript version
npm start 8002 &
python -m benchmark_harness.cli run --server express --port 8002
```

Both should produce identical results since they compile to the same runtime code.

## Future Enhancements

Potential improvements for the TypeScript version:

1. **Request Type Guards**: Use Zod's type inference for better handler typing
2. **Generic Handler Types**: Create `Handler<TBody, TParams, TQuery>` types
3. **Validation Error Formatting**: Match exact error format from other frameworks
4. **Schema Registry**: Centralize schemas for reuse across routes
5. **OpenAPI Generation**: Generate OpenAPI specs from Zod schemas

## Conclusion

The TypeScript implementation provides:
- **Type Safety**: Catch errors at compile time
- **Better IDE Support**: IntelliSense and autocomplete
- **Refactoring Safety**: Rename/move code with confidence
- **Documentation**: Types serve as inline documentation
- **Zero Runtime Cost**: Compiles to identical JavaScript

Trade-offs:
- **Build Step**: Requires compilation (or tsx runtime)
- **Complexity**: More verbose than JavaScript
- **Learning Curve**: Developers must understand TypeScript and Zod
