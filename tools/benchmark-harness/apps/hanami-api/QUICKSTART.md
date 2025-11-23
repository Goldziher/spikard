# Hanami API Benchmark Server - Quick Start

## Installation
```bash
bundle install
```

## Running

### Method 1: Direct Ruby execution (recommended)
```bash
./server.rb          # Port 8000
./server.rb 9000     # Custom port
```

### Method 2: Using Rackup
```bash
rackup config.ru -p 8000
```

## Testing

### Health Check
```bash
curl http://localhost:8000/health
# {"status":"ok"}
```

### JSON Validation (Valid)
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget","description":"A test widget","price":19.99}'
# {"name":"Widget","description":"A test widget","price":19.99}
```

### JSON Validation (Invalid - Missing Required Fields)
```bash
curl -X POST http://localhost:8000/json/small \
  -H "Content-Type: application/json" \
  -d '{"name":"Widget"}'
# {"errors":{"description":["is missing"],"price":["is missing"]}}
```

### Path Parameters
```bash
curl http://localhost:8000/path/simple/123
# {"id":"123"}

curl http://localhost:8000/path/multiple/user456/post789
# {"user_id":"user456","post_id":"post789"}
```

### Query Parameters
```bash
curl 'http://localhost:8000/query/few?q=search&page=1&limit=20'
# {"q":"search","page":1,"limit":20}
```

## All 18 Endpoints

### JSON Body (POST)
1. `/json/small` - ~100 bytes with validation
2. `/json/medium` - ~1KB with nested address
3. `/json/large` - ~10KB with items array
4. `/json/very-large` - ~100KB with records

### Path Parameters (GET)
5. `/path/simple/:id`
6. `/path/multiple/:user_id/:post_id`
7. `/path/deep/:org/:team/:project/:resource/:id`
8. `/path/int/:id`
9. `/path/uuid/:uuid`
10. `/path/date/:date`

### Query Parameters (GET)
11. `/query/few`
12. `/query/medium`
13. `/query/many`

### URL-Encoded Forms (POST)
14. `/urlencoded/simple`
15. `/urlencoded/complex`

### Multipart Forms (POST)
16. `/multipart/small`
17. `/multipart/medium`
18. `/multipart/large`

## Key Features

- **Dry::Schema Validation**: All POST JSON endpoints validate with Dry::Schema
- **Error Handling**: Invalid requests return 400 with detailed errors
- **High Performance**: Single-threaded Puma, silent logging
- **Rack Compatible**: Works with any Rack handler
- **Pattern Matching**: Schemas match Python Pydantic patterns

## Dependencies

- hanami-api ~> 0.3 - Lightweight HTTP API framework
- dry-schema ~> 1.14 - Validation library
- puma ~> 7.1 - Web server
- rack ~> 3.2 - Web server interface
- rackup ~> 2.2 - Server management

## Configuration

- Host: 0.0.0.0 (all interfaces)
- Port: CLI argument or 8000
- Threads: 1:1 (single-threaded)
- Logging: Silent mode
