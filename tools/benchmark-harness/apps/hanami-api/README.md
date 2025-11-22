# Hanami API Benchmark Application

Lightweight HTTP API framework implementation for benchmarking against spikard-ruby.

## Framework Overview

**Hanami API** is a minimal, extremely fast Ruby framework optimized for HTTP APIs.

- **Performance**: 14,290+ req/s with 10,000 routes
- **Memory**: 53,988 bytes for 10,000 routes
- **Architecture**: Block-based routing with minimal overhead
- **Validation**: Dry::Schema integration

## Installation

```bash
cd tools/benchmark-harness/apps/hanami-api
bundle install
```

## Running

```bash
# Default port 8000
./server.rb

# Custom port
./server.rb 9000
```

## Implemented Endpoints

### JSON Bodies (POST)
- `/json/small` - Small payload with Dry::Schema validation
- `/json/medium` - Medium payload with nested address
- `/json/large` - Large payload with items array
- `/json/very-large` - Very large payload with metadata

### Multipart Forms (POST)
- `/multipart/small` - Small file upload
- `/multipart/medium` - Medium file upload
- `/multipart/large` - Large file upload

### URL-Encoded Forms (POST)
- `/urlencoded/simple` - Simple form with name/email
- `/urlencoded/complex` - Nested form with user/preferences

### Path Parameters (GET)
- `/path/simple/:id` - Single parameter
- `/path/multiple/:user_id/:post_id` - Multiple parameters
- `/path/deep/:org/:team/:project/:resource/:id` - Deep nesting
- `/path/int/:id` - Integer conversion
- `/path/uuid/:uuid` - UUID validation
- `/path/date/:date` - Date parameter

### Query Parameters (GET)
- `/query/few` - Few parameters (q, page, limit)
- `/query/medium` - Medium parameter count
- `/query/many` - Many parameters

### Health Checks
- `/health` - Health check
- `/` - Root endpoint

## Configuration

- **Host**: 0.0.0.0 (all interfaces)
- **Port**: Command-line argument (default: 8000)
- **Threads**: Single-threaded (1:1) for consistent benchmarking
- **Logging**: Silent mode enabled
- **Validation**: Dry::Schema for all request bodies

## Schema Alignment

All schemas match spikard-ruby exactly for fair comparison:
- Field names use snake_case
- Types match: String, Integer, Float, Boolean, Hash, Array
- Optional fields handled with `maybe`
- Nested validation for complex structures
