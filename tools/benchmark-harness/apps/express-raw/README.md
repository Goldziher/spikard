# Express RAW Benchmark Server

Express.js server **without validation** for measuring raw framework performance.

## Purpose

This server implements all 18 workload endpoints identical to the standard Express benchmark server, but **removes all Zod validation overhead**. This allows measuring Express's baseline performance without validation costs.

## Differences from Standard Express Server

- ❌ **NO Zod validation** - accepts any JSON body
- ✅ **All 18 workload endpoints** - same routes as standard server
- ✅ **Same response format** - echoes back request data
- ✅ **express.json() middleware** - JSON parsing only

## Installation

```bash
npm install
```

## Running

```bash
# Default port 8000
npm start

# Custom port
npm start 8001
```

## Endpoints

### JSON Body Workloads (4)
- `POST /json/small` - Small payload (no validation)
- `POST /json/medium` - Medium payload (no validation)
- `POST /json/large` - Large payload (no validation)
- `POST /json/very-large` - Very large payload (no validation)

### Multipart Form Workloads (3)
- `POST /multipart/small` - Returns mock file stats
- `POST /multipart/medium` - Returns mock file stats
- `POST /multipart/large` - Returns mock file stats

### URL Encoded Form Workloads (2)
- `POST /urlencoded/simple` - Echoes form data
- `POST /urlencoded/complex` - Echoes form data

### Path Parameter Workloads (6)
- `GET /path/simple/:id` - Single parameter
- `GET /path/multiple/:user_id/:post_id` - Two parameters
- `GET /path/deep/:org/:team/:project/:resource/:id` - Five parameters
- `GET /path/int/:id` - Integer parsing
- `GET /path/uuid/:uuid` - UUID parameter
- `GET /path/date/:date` - Date parameter

### Query Parameter Workloads (3)
- `GET /query/few` - Few query params
- `GET /query/medium` - Medium query params
- `GET /query/many` - Many query params

### Health Check (2)
- `GET /health` - Returns `{"status": "ok"}`
- `GET /` - Returns `{"status": "ok"}`

## Performance Testing

This server is designed to be benchmarked alongside the standard Express server to measure:

1. **Raw Express performance** (this server)
2. **Express + Zod validation overhead** (standard server)
3. **Validation cost** = (Standard - Raw)

## Dependencies

- **express** ^5.1.0 - Web framework
- **tsx** ^4.20.6 - TypeScript execution (dev)
- **@types/express** ^5.0.0 - TypeScript types (dev)
- **@types/node** ^22.19.1 - Node.js types (dev)

## Architecture

- Pure Express.js with minimal middleware
- TypeScript for type safety during development
- No validation libraries (intentionally omitted)
- Direct request/response handling
- Port configurable via command line
