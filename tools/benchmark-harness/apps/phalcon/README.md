# Phalcon Benchmark Server

High-performance REST API benchmark server built with the Phalcon PHP framework.

## Overview

This benchmark application demonstrates Phalcon's HTTP handling capabilities through a minimal but complete REST API. It implements the standard benchmark schema with user management endpoints and in-memory storage.

## Requirements

- PHP 8.2 or higher
- Phalcon 5.9 or higher (installed as PHP extension)
- Composer

## Installation

```bash
composer install
```

This will install Phalcon framework as a dependency. Note that Phalcon requires the PHP extension to be installed system-wide.

## Building Phalcon Extension

If you don't have Phalcon installed as a PHP extension:

```bash
# Clone Phalcon
git clone https://github.com/phalcon/cphalcon.git

# Navigate to build directory
cd cphalcon/build

# Build extension
./install

# Add to php.ini
extension=phalcon.so
```

## Running the Server

### Using PHP Built-in Server

```bash
php server.php 8000
```

The server will start on `http://0.0.0.0:8000`

### Via Docker

```bash
docker run -p 8000:8000 \
  -v $(pwd):/app \
  -w /app \
  php:8.2-cli-alpine \
  php server.php 8000
```

### Via Nginx

Configure Nginx to route requests to the application:

```nginx
server {
    listen 8000;
    root /path/to/phalcon;

    location / {
        if (!-e $request_filename) {
            rewrite ^/(.*)$ /index.php/$1 last;
        }
    }

    location ~ \.php$ {
        fastcgi_pass 127.0.0.1:9000;
        fastcgi_index index.php;
        fastcgi_param SCRIPT_FILENAME $document_root$fastcgi_script_name;
        include fastcgi_params;
    }
}
```

## API Endpoints

### Health Check

```http
GET /health
```

Response:
```json
{
  "status": "ok"
}
```

### Create User

```http
POST /users
Content-Type: application/json

{
  "name": "John Doe",
  "email": "john@example.com",
  "age": 30
}
```

Response (201 Created):
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com",
  "age": 30
}
```

### Get User

```http
GET /users/{id}
```

Response:
```json
{
  "id": 1,
  "name": "John Doe",
  "email": "john@example.com",
  "age": 30
}
```

### Update User

```http
PUT /users/{id}
Content-Type: application/json

{
  "name": "Jane Doe",
  "email": "jane@example.com"
}
```

Response:
```json
{
  "id": 1,
  "name": "Jane Doe",
  "email": "jane@example.com",
  "age": 30
}
```

### Delete User

```http
DELETE /users/{id}
```

Response:
```json
{
  "message": "User deleted"
}
```

## Architecture

### Components

- **UserStore**: In-memory user storage with auto-incrementing IDs
- **Micro Application**: Phalcon's lightweight HTTP router and dispatcher
- **Request/Response**: Standard Phalcon HTTP abstractions
- **Dependency Injection**: Phalcon's DI container for service management

### Code Structure

```
phalcon/
├── composer.json          # Phalcon dependency configuration
├── server.php             # Main server implementation
├── index.php              # Entry point for web servers
├── README.md              # This file
└── vendor/                # Composer dependencies
    └── phalcon/           # Phalcon framework
```

## Performance Characteristics

### Strengths

- **Native Performance**: Compiled C extension provides near-native performance
- **Low Memory**: Minimal overhead compared to pure PHP frameworks
- **Fast Routing**: Optimized routing layer with path parameter extraction
- **Built-in Features**: Comprehensive framework without external dependencies

### Benchmark Notes

- Single worker for consistent benchmarking
- In-memory user storage (no database I/O)
- No middleware overhead (minimal routing)
- Direct response serialization

## Code Quality

### Standards Compliance

- **PSR-4**: Strict autoloading compliance
- **PSR-12**: Code style following modern PHP standards
- **Strict Types**: All files declare `strict_types=1`
- **Type Safety**: Full type hints on all method signatures

### Static Analysis

```bash
# Run PHPStan for static analysis
./vendor/bin/phpstan analyse server.php
```

## Benchmarking

### Load Testing Example

Using Apache Bench:

```bash
# Health check (warmup)
ab -n 100 -c 10 http://localhost:8000/health

# User creation benchmark
ab -n 1000 -c 50 -p payload.json -T application/json http://localhost:8000/users

# User retrieval benchmark
ab -n 1000 -c 50 http://localhost:8000/users/1
```

### Comparison with Other Frameworks

Run the benchmark harness to compare Phalcon with other frameworks:

```bash
# From repository root
task benchmark
```

## Troubleshooting

### Phalcon Extension Not Found

```
Fatal error: Class 'Phalcon\Mvc\Micro' not found
```

Solution: Install Phalcon extension and verify it's loaded:

```bash
php -m | grep phalcon
php -i | grep "Phalcon Version"
```

### Permission Denied on Port 8000

```bash
# Use a port > 1024
php server.php 8080

# Or use sudo for ports < 1024
sudo php server.php 8000
```

### Memory Issues

Increase PHP memory limit if needed:

```bash
php -d memory_limit=512M server.php 8000
```

## Additional Resources

- [Phalcon Documentation](https://phalcon.io/docs/)
- [Phalcon API Reference](https://phalcon.io/en/api/)
- [GitHub Repository](https://github.com/phalcon/cphalcon)

## License

This benchmark implementation is provided as-is for comparison purposes. Phalcon framework is licensed under the BSD-3-Clause license.
