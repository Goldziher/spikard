# Spikard PHP

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![Packagist Version](https://img.shields.io/packagist/v/spikard/spikard.svg)](https://packagist.org/packages/spikard/spikard)
[![PHP Version](https://img.shields.io/badge/php-%3E%3D%208.2-blue.svg)](https://www.php.net/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

High-performance PHP bindings for Spikard. Build REST APIs, WebSockets, and SSE services with modern PHP 8.2+ patterns backed by a Rust core via ext-php-rs.

## Installation

### Via Composer

```bash
composer require spikard/spikard
```

### From Source (Development)

```bash
cd packages/php
composer install
cargo build --release --manifest-path ../../crates/spikard-php/Cargo.toml
```

### Requirements

- PHP 8.2+
- Composer 2.0+
- Rust stable (for building the native extension)

## Quick Start

```php
<?php

declare(strict_types=1);

require_once 'vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UserController
{
    #[Get('/users/{id}')]
    public function get(Request $request): Response
    {
        $userId = (int) $request->pathParams['id'];
        return Response::json([
            'id' => $userId,
            'name' => 'Alice',
            'email' => 'alice@example.com',
        ]);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $data = $request->jsonBody();

        // Automatic validation
        return Response::json([
            'id' => 1,
            'name' => $data['name'],
            'email' => $data['email'],
        ], 201);
    }
}

$config = new ServerConfig(port: 8000);
$app = (new App($config))->registerController(new UserController());
$app->run();
```

## Key Features

- **ext-php-rs FFI**: Zero-copy data exchange between PHP and Rust
- **Type-safe**: PHP 8.2+ with strict types and PHPStan level max
- **Multiple protocols**: HTTP/1.1, WebSockets, Server-Sent Events
- **Middleware stack**: Compression, rate limiting, auth, CORS, timeouts
- **Zero-copy serialization**: Direct type conversion without JSON overhead
- **Lifecycle hooks**: onRequest, preValidation, preHandler, onResponse, onError
- **Built-in testing**: TestClient for unit and integration tests
- **OpenAPI/GraphQL support**: Full spec compatibility (see main docs)

## Usage

Routes are defined with attributes on controller methods:

```php
use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class UsersController
{
    #[Get('/users/{id}')]
    public function get(Request $request): Response
    {
        return Response::json(['id' => $request->pathParams['id']]);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        return Response::json($request->jsonBody(), 201);
    }
}

$app = (new App())->registerController(new UsersController());
$app->run();
```

Supported HTTP methods: GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS.

For query parameters, use `$request->query`. For path parameters, use `$request->pathParams`. Request bodies are validated against JSON schemas.

See [examples/php/](../../examples/php/) for additional patterns: dependency injection, file uploads, streaming responses, WebSockets, and Server-Sent Events.

## Configuration

```php
use Spikard\Config\ServerConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\RateLimitConfig;

$config = new ServerConfig(
    host: '0.0.0.0',
    port: 8080,
    workers: 4,
    compression: new CompressionConfig(gzip: true, brotli: true),
    rateLimit: new RateLimitConfig(perSecond: 100, burst: 200)
);

$app = new App($config);
$app->run();
```

Configuration options: `host`, `port`, `workers`, `maxBodySize`, `requestTimeout`, `compression`, `rateLimit`, `jwt`, `apiKey`, `staticFiles`, `openapi`, and `hooks`.

See [Rust core docs](../../crates/spikard/) for complete ServerConfig options.

## Advanced Features

**Lifecycle Hooks:** Register `onRequest`, `preValidation`, `preHandler`, `onResponse`, and `onError` callbacks via `ServerConfig`.

**WebSockets & Server-Sent Events:** Full support via `addWebSocket()` and `addSse()` methods.

**Background Tasks:** Schedule non-blocking work via `BackgroundTask::run()`.

**Dependency Injection:** Register values and factories via `DependencyContainer`.

See [examples/php/](../../examples/php/) for complete implementations.

## Testing

Use PHPUnit with the built-in `TestClient`:

```php
use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;

final class ApiTest extends TestCase
{
    public function testGetUser(): void
    {
        $app = (new App())->registerController(new class () {
            #[Get('/users/{id}')]
            public function user(Request $request): Response
            {
                return Response::json(['id' => $request->pathParams['id']]);
            }
        });

        $client = TestClient::create($app);
        $response = $client->request('GET', '/users/123');

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => '123'], $response->body);
    }
}
```

TestClient supports HTTP requests, WebSocket connections, and SSE streams.

## Type Safety

All code uses PHP 8.2+ features: strict types, readonly properties, typed properties, return types, and PHPStan level max static analysis.

```php
<?php declare(strict_types=1);

final class UserHandler
{
    public function handle(Request $request): Response
    {
        $user = $request->jsonBody();
        return Response::json(['id' => 1, ...$user], 201);
    }
}
```

## Examples

See [examples/php/](../../examples/php/) for runnable PHP examples and [examples/README.md](../../examples/README.md) for code generation from OpenAPI/GraphQL schemas.

## Documentation

- [Main README](../../README.md) - Spikard overview and quick start
- [Rust Core](../../crates/spikard/) - Core library documentation and API reference
- [Architecture Decision Records](../../docs/adr/) - Design decisions
- [Contributing Guide](../../CONTRIBUTING.md) - Development workflow

See [README.md](../../README.md) for documentation on code generation from OpenAPI and GraphQL schemas.

## License

MIT - See [LICENSE](LICENSE) for details

## Ecosystem

Spikard is available for Python, TypeScript, Ruby, PHP, WebAssembly, and Rust. See [spikard.dev](https://spikard.dev) for all language bindings.
