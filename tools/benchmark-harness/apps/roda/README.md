# Roda Benchmark Application

Routing tree web toolkit implementation for benchmarking against spikard-ruby.

## Framework Overview

**Roda** is a routing tree web framework emphasizing simplicity and performance.

- **Architecture**: Tree-based routing for efficient URL matching
- **Performance**: Fastest Ruby web framework per r10k benchmark
- **Philosophy**: Simple, usable, productive
- **Validation**: Dry::Schema integration
- **Plugins**: json, all_verbs, type_routing

## Installation

```bash
cd tools/benchmark-harness/apps/roda
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
- **Frozen**: App frozen in production for thread safety

## Routing Tree Architecture

Roda uses a tree-based routing system:

```ruby
r.on 'json' do
  r.post 'small' do
    # Handler for /json/small
  end
end
```

This allows:
- Efficient URL matching
- Shared logic at branch points
- Type-safe parameter extraction
- Low per-request overhead

## Schema Alignment

All schemas match spikard-ruby exactly for fair comparison:
- Field names use snake_case
- Types match: String, Integer, Float, Boolean, Hash, Array
- Optional fields handled with `maybe`
- Nested validation for complex structures
