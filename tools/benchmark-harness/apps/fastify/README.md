# Fastify Benchmark Server

High-performance Node.js benchmark server using Fastify with built-in JSON Schema validation.

## Features

- **Native JSON Schema Validation**: All POST endpoints use Fastify's built-in schema validation
- **18 Workload Endpoints**: Complete coverage of all benchmark scenarios
- **TypeScript**: Fully typed with strict TypeScript configuration
- **Zero External Validators**: Uses only Fastify's native validation (based on Ajv)

## Installation

```bash
npm install
```

## Usage

```bash
# Default port 8000
npm start

# Custom port
npm start 3000
```

## Endpoints

### JSON Body Workloads
- `POST /json/small` - Small JSON payload (~100 bytes)
- `POST /json/medium` - Medium JSON payload (~1KB)
- `POST /json/large` - Large JSON payload (~10KB)
- `POST /json/very-large` - Very large JSON payload (~100KB)

### Path Parameter Workloads
- `GET /path/simple/:id` - Single path parameter
- `GET /path/multiple/:user_id/:post_id` - Multiple path parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Deep nested parameters
- `GET /path/int/:id` - Path parameter with integer
- `GET /path/uuid/:uuid` - Path parameter with UUID
- `GET /path/date/:date` - Path parameter with date

### Query Parameter Workloads
- `GET /query/few` - Few query parameters (1-2)
- `GET /query/medium` - Medium query parameters (3-5)
- `GET /query/many` - Many query parameters (6-10)

### URL Encoded Form Workloads
- `POST /urlencoded/simple` - Simple URL-encoded form
- `POST /urlencoded/complex` - Complex URL-encoded form

### Multipart Form Workloads
- `POST /multipart/small` - Small multipart form (~1KB)
- `POST /multipart/medium` - Medium multipart form (~10KB)
- `POST /multipart/large` - Large multipart form (~100KB)

### Health Check
- `GET /health` - Health check endpoint
- `GET /` - Root endpoint

## JSON Schema Validation

All POST endpoints with JSON bodies use Fastify's built-in JSON Schema validation:

```typescript
fastify.post("/json/small", {
  schema: {
    body: {
      type: "object",
      required: ["name", "description", "price"],
      properties: {
        name: { type: "string" },
        description: { type: "string" },
        price: { type: "number" },
        tax: { type: "number", nullable: true },
      },
    },
  },
  handler: async (request, reply) => {
    return request.body;
  },
});
```

Invalid requests return a 400 error with validation details:
```json
{
  "statusCode": 400,
  "code": "FST_ERR_VALIDATION",
  "error": "Bad Request",
  "message": "body must have required property 'description'"
}
```

## Performance Features

- **No logging overhead**: `logger: false` for benchmark accuracy
- **Native validation**: Uses Fastify's built-in Ajv-based JSON Schema validation
- **TypeScript**: Compiled with tsx for efficient execution
- **Minimal dependencies**: Only Fastify core + TypeScript tooling
