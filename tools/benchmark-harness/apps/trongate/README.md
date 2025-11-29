# Trongate Benchmark Application

This is a minimal Trongate benchmark application for the spikard workload harness. It implements a simple HTTP API server that serves as a performance baseline for the Trongate PHP framework.

## About Trongate

Trongate is a PHP framework that emphasizes pure PHP over PSR-4 autoloading and Composer. It uses traditional module-based architecture and is designed for rapid API development. More information at [trongate.io](https://trongate.io).

## Architecture

This benchmark application uses a lightweight implementation that focuses on performance testing rather than the full Trongate framework features. It consists of:

- **Simple Router**: Basic request routing without full Trongate framework overhead
- **UserStore**: In-memory user storage for CRUD operations
- **HTTP Handlers**: Dedicated handlers for each endpoint type
- **Socket Server**: Direct PHP socket implementation for maximum performance

## Features

### Endpoints

#### Health Check
- `GET /health` - Returns `{"status": "ok"}`

#### User CRUD
- `POST /users` - Create user (expects JSON body)
- `GET /users/:id` - Get user by ID
- `PUT /users/:id` - Update user
- `DELETE /users/:id` - Delete user

#### Echo Endpoints (JSON Body)
- `POST /items`
- `POST /items/nested`
- `POST /items/list`
- `POST /payment`
- `POST /billing`
- `POST /api/v1/data`
- `POST /config`
- `POST /data`
- `POST /events/`
- And 20+ more echo endpoints

#### Form Data Endpoints
- `POST /login/`
- `POST /register/`
- `POST /form/`
- `POST /profile`
- `POST /accounts`
- And more...

#### File Upload Endpoints
- `POST /files/upload`
- `POST /files/image`
- `POST /files/document`
- `POST /upload`
- And more...

## Requirements

- PHP 8.2 or higher
- No external dependencies (except PHP socket extension, which is built-in)

## Installation

```bash
cd tools/benchmark-harness/apps/trongate
composer install 2>/dev/null || echo "Composer not required for benchmark"
```

## Running the Server

```bash
# Start on default port 8000
php server.php

# Start on custom port
php server.php 9000
```

The server will output:
```
[trongate] Starting server on 0.0.0.0:8000
[trongate] Server listening on 0.0.0.0:8000
```

## Verification

Run the verification script to test all endpoints:

```bash
./verify.sh
```

Or with custom domain/port:

```bash
DOMAIN=http://127.0.0.1 PORT=9000 ./verify.sh
```

## Implementation Details

### In-Memory Storage

User data is stored in memory using a simple array-based storage system:

```php
class UserStore {
    private static array $users = [];
    private static int $nextId = 1;

    public static function create(array $data): array { ... }
    public static function get(int $id): ?array { ... }
    public static function update(int $id, array $data): ?array { ... }
    public static function delete(int $id): bool { ... }
}
```

### Request Parsing

The server parses HTTP requests directly:

1. **HTTP Request Line**: Extracts method, path, and query string
2. **Headers**: Parses Content-Type and Content-Length
3. **Body**: Handles JSON, form-urlencoded, and multipart/form-data
4. **Path Parameters**: Extracts ID from path patterns like `/users/{id}`

### Response Format

All responses are JSON with appropriate HTTP status codes:

```json
{
  "status": 200,
  "headers": {
    "Content-Type": "application/json"
  },
  "body": { ... }
}
```

### Workload Categories

1. **Health Check** - Simple status response (100 bytes)
2. **CRUD Operations** - User creation, retrieval, update, delete
3. **Small JSON Payloads** - ~100-500 bytes
4. **Medium JSON Payloads** - ~1-10 KB (nested objects)
5. **Large JSON Payloads** - ~10-100 KB (arrays/lists)
6. **Form-Encoded Data** - 3-20 fields
7. **File Uploads** - Multipart form data (1KB - 100KB)

## Performance Considerations

- **No Composer autoloading**: Direct file execution for minimal overhead
- **Native sockets**: Uses PHP socket extension directly
- **Single-threaded**: Single worker process for benchmarking consistency
- **In-memory storage**: No database overhead
- **Direct routing**: Simple pattern matching instead of complex routing layers

## Code Quality

- **PSR-12**: Follows PSR-12 coding standards
- **Strict types**: Uses `declare(strict_types=1)`
- **Type hints**: All function parameters and returns are typed
- **Error handling**: Proper HTTP status codes and JSON error responses

## Files

- `server.php` - Main server implementation with all endpoints and handlers
- `composer.json` - Project metadata (composer.json not required for operation)
- `verify.sh` - Verification script to test all endpoints
- `README.md` - This file

## Differences from Full Trongate

This benchmark implementation differs from full Trongate in several ways:

1. **No Module System**: Uses simple routing instead of Trongate's module-based architecture
2. **No Template Engine**: No view rendering capability
3. **No Database Layer**: In-memory storage only
4. **No ORM**: Direct PHP arrays for data storage
5. **Minimal Config**: Hardcoded endpoint configuration
6. **Direct Sockets**: Raw PHP socket API instead of web server

These simplifications are intentional to measure framework overhead separately from HTTP handling performance.

## Troubleshooting

### Port Already in Use

If the server fails to bind to the port:

```bash
# Kill process on port 8000
lsof -ti :8000 | xargs kill -9

# Try a different port
php server.php 9000
```

### Permission Denied

If you get permission errors:

```bash
chmod +x verify.sh
chmod 755 server.php
```

### Connection Refused

Make sure the server is running before running verification:

```bash
# Terminal 1
php server.php

# Terminal 2
./verify.sh
```

## Performance Tips

For optimal benchmark results:

1. Use a dedicated benchmarking tool (Apache Bench, wrk, hey)
2. Disable logging during benchmarks
3. Run on a dedicated machine without other workloads
4. Use the same port and connections as other benchmark apps
5. Run multiple iterations and average the results

Example benchmark run:

```bash
# Terminal 1
php server.php 8000

# Terminal 2 (using wrk benchmark tool)
wrk -t4 -c100 -d30s http://localhost:8000/health
```

## License

This benchmark application is part of the spikard project and follows the same license.

## Related

- Trongate Framework: https://github.com/trongate/trongate-framework
- Spikard Benchmark Suite: https://github.com/your-org/spikard
- Benchmark Results: ../results/
