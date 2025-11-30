# Spikard PHP

[![Documentation](https://img.shields.io/badge/docs-spikard.dev-58FBDA)](https://spikard.dev)
[![Packagist Version](https://img.shields.io/packagist/v/spikard/spikard.svg)](https://packagist.org/packages/spikard/spikard)
[![Packagist Downloads](https://img.shields.io/packagist/dt/spikard/spikard.svg)](https://packagist.org/packages/spikard/spikard)
[![PHP Version](https://img.shields.io/badge/php-%3E%3D%208.2-blue.svg)](https://www.php.net/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CI Status](https://github.com/Goldziher/spikard/workflows/CI/badge.svg)](https://github.com/Goldziher/spikard/actions)
[![PyPI](https://img.shields.io/pypi/v/spikard.svg)](https://pypi.org/project/spikard/)
[![npm](https://img.shields.io/npm/v/spikard.svg)](https://www.npmjs.com/package/spikard)
[![Crates.io](https://img.shields.io/crates/v/spikard.svg)](https://crates.io/crates/spikard)
[![RubyGems](https://img.shields.io/gem/v/spikard.svg)](https://rubygems.org/gems/spikard)

High-performance PHP web framework with a Rust core. Build REST APIs, WebSockets, and SSE services with modern PHP 8.2+ patterns backed by Axum and Tower-HTTP via ext-php-rs. Part of the multi-language Spikard ecosystem.

## Installation

### Via Composer

```bash
composer require spikard/spikard
```

### From Source (Development)

```bash
cd packages/php
composer install
```

### Extension Compilation & Loading

The Spikard PHP binding is a native Rust extension built with `ext-php-rs`. The extension is compiled as part of the build process:

**Build the extension:**

```bash
cargo build --release --manifest-path ../../crates/spikard-php/Cargo.toml
```

**Load the extension in php.ini:**

```ini
; Find your php.ini location
php -i | grep "php.ini"

; Add this line to your php.ini:
extension=spikard
```

**Verify installation:**

```bash
php -m | grep spikard
php -i | grep "Spikard"
```

### Requirements

- **PHP**: 8.2 or higher (strict_types enforced)
- **Composer**: 2.0 or higher (PSR-4 autoloading)
- **Rust toolchain**: Latest stable (for building native extension)
- **Build tools**:
  - macOS: Xcode Command Line Tools
  - Linux: build-essential, libssl-dev
  - Windows: Visual Studio Build Tools

## Quick Start

```php
<?php

declare(strict_types=1);

require_once 'vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Request;
use Spikard\Http\Response;

$config = new ServerConfig(port: 8000);
$app = new App($config);

$app = $app->addRoute('GET', '/users/{id}', function (Request $request) {
    $userId = (int) $request->pathParams['id'];
    return Response::json([
        'id' => $userId,
        'name' => 'Alice',
        'email' => 'alice@example.com',
    ]);
});

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $data = $request->jsonBody();

    // Automatic validation
    return Response::json([
        'id' => 1,
        'name' => $data['name'],
        'email' => $data['email'],
    ], 201);
});

$app->run();
```

## Features

### High-Performance Rust Core

- **ext-php-rs FFI**: Zero-copy data exchange between PHP and Rust
- **Tower-HTTP middleware**: Industry-standard async HTTP middleware stack
- **Tokio async runtime**: Non-blocking I/O throughout request lifecycle
- **Direct type conversion**: No JSON serialization overhead for internal communication
- **Optimized bindings**: Compiled release builds with aggressive optimization flags
- **Request/response streaming**: Efficient handling of large payloads

### Developer Experience

- **Type-safe**: PHP 8.2+ strict_types with PHPStan level max analysis
- **Modern patterns**: Named parameters, readonly properties, union types
- **Dependency Injection**: Built-in container with factories and singletons
- **Flexible routing**: Path parameters, query strings, request body validation
- **Lifecycle hooks**: onRequest, preValidation, preHandler, onResponse, onError
- **Testing utilities**: Built-in TestClient for unit and integration testing

### Protocol Support

- **HTTP/1.1**: Full compliance with RFC 9110
- **WebSockets**: Full-duplex real-time communication
- **Server-Sent Events (SSE)**: Efficient server-to-client streaming
- **Static files**: Optimized static asset serving with caching
- **Request/response streaming**: Generator-based streaming responses

### Security & Validation

- **JWT authentication**: HS256, HS384, HS512, RS256, and more
- **API Key authentication**: Header-based API key validation
- **CORS support**: Configurable cross-origin resource sharing
- **Rate limiting**: Per-IP or global rate limit configuration
- **Header validation**: Strict header schema enforcement
- **Cookie security**: Secure, HttpOnly, SameSite attributes

### Middleware & Configuration

- **Compression**: Gzip and Brotli with quality control
- **Rate limiting**: Token bucket algorithm with burst allowance
- **Request IDs**: Automatic request tracking
- **Timeout enforcement**: Global and per-route request timeouts
- **OpenAPI documentation**: Auto-generated API documentation with Swagger UI
- **User-Agent parsing**: Built-in user-agent detection

## Core Features

### Route Registration

Routes are registered using `addRoute()` with method, path, and handler:

```php
use Spikard\App;
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = new App();

$app = $app->addRoute('GET', '/users', function (Request $request) {
    return Response::json(['users' => []]);
});

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $user = $request->jsonBody();
    return Response::json($user, 201);
});

$app = $app->addRoute('PUT', '/users/{id}', function (Request $request) {
    $userId = $request->pathParams['id'];
    $data = $request->jsonBody();
    return Response::json(['id' => $userId, ...$data]);
});

$app = $app->addRoute('DELETE', '/users/{id}', function (Request $request) {
    return Response::json(['deleted' => true], 200);
});
```

**Supported HTTP Methods:**
- `GET` - Retrieve resources
- `POST` - Create resources
- `PUT` - Replace resources
- `PATCH` - Update resources
- `DELETE` - Delete resources
- `HEAD` - Get headers only
- `OPTIONS` - Get allowed methods
- `TRACE` - Echo the request

### Path Parameters

```php
$app = $app->addRoute('GET', '/users/{user_id}', function (Request $request) {
    $userId = (int) $request->pathParams['user_id'];
    return Response::json(['id' => $userId]);
});

$app = $app->addRoute('GET', '/posts/{post_id}/comments/{comment_id}',
    function (Request $request) {
        return Response::json([
            'post_id' => (int) $request->pathParams['post_id'],
            'comment_id' => (int) $request->pathParams['comment_id'],
        ]);
    }
);
```

### Query Parameters

```php
$app = $app->addRoute('GET', '/search', function (Request $request) {
    $q = $request->query['q'] ?? '';
    $limit = (int) ($request->query['limit'] ?? 10);
    $offset = (int) ($request->query['offset'] ?? 0);

    return Response::json([
        'query' => $q,
        'limit' => $limit,
        'offset' => $offset,
    ]);
});
```

### Request Body Validation

Spikard validates request bodies against JSON schemas. PHP 8.2+ with strict types provides natural validation:

```php
$app = $app->addRoute('POST', '/posts', function (Request $request) {
    $data = $request->jsonBody();

    // Validate required fields
    if (!isset($data['title'], $data['content'])) {
        return Response::json([
            'error' => 'Missing required fields: title, content',
        ], 400);
    }

    return Response::json([
        'id' => 1,
        'title' => $data['title'],
        'content' => $data['content'],
        'tags' => $data['tags'] ?? [],
    ], 201);
});
```

**JSON Schema validation:**

```php
$postSchema = [
    'type' => 'object',
    'properties' => [
        'title' => ['type' => 'string', 'minLength' => 1],
        'content' => ['type' => 'string'],
        'tags' => ['type' => 'array', 'items' => ['type' => 'string']],
    ],
    'required' => ['title', 'content'],
];

$app = $app->addRouteWithSchemas(
    method: 'POST',
    path: '/posts',
    handler: function (Request $request) {
        $post = $request->jsonBody();
        return Response::json($post, 201);
    },
    requestSchema: $postSchema,
    responseSchema: null,
    parameterSchema: null
);
```

### Dependency Injection

Register values or factories and prepare them for injection:

```php
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;

// Simulated database class
class Database {
    public function __construct(
        public readonly string $host,
        public readonly int $port
    ) {}

    public function query(string $sql): array {
        return [['id' => 1, 'name' => 'Alice']];
    }
}

$container = new DependencyContainer(
    values: [
        'app_name' => 'My Spikard App',
        'db_config' => ['host' => 'localhost', 'port' => 5432],
    ],
    factories: [
        'database' => new Provide(
            factory: function (array $db_config): Database {
                return new Database($db_config['host'], $db_config['port']);
            },
            dependsOn: ['db_config'],
            singleton: true
        ),
    ]
);

$app = $app->withDependencies($container);

// Note: P0.2 complete (DI system registered)
// Full auto-injection coming in P1.4
```

### File Uploads

```php
use Spikard\Http\UploadFile;

$app = $app->addRoute('POST', '/upload', function (Request $request) {
    $data = $request->jsonBody();
    $file = $data['file']; // UploadFile instance

    return Response::json([
        'filename' => $file->filename,
        'size' => $file->size,
        'content_type' => $file->contentType,
        'content' => $file->read(),
    ]);
});
```

### Custom Responses

```php
use Spikard\Http\Response;

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $user = $request->jsonBody();

    return new Response(
        content: json_encode($user),
        statusCode: 201,
        headers: ['X-Custom' => 'value']
    );
});

// Convenience methods
Response::json(['key' => 'value'], 200);           // JSON response
Response::text('Hello, World!');                    // Plain text
Response::html('<h1>Hello</h1>');                   // HTML
Response::empty(204);                               // Empty response
```

### Streaming Responses

```php
use Spikard\Http\StreamingResponse;

$app = $app->addRoute('GET', '/stream', function () {
    $generator = function (): Generator {
        for ($i = 0; $i < 10; $i++) {
            yield "data: {$i}\n";
            sleep(1);
        }
    };

    return new StreamingResponse(
        stream: $generator(),
        statusCode: 200,
        headers: ['Content-Type' => 'text/plain']
    );
});
```

## Configuration

```php
use Spikard\Config\ServerConfig;
use Spikard\Config\CompressionConfig;
use Spikard\Config\RateLimitConfig;

$config = new ServerConfig(
    host: '0.0.0.0',
    port: 8080,
    workers: 4,
    enableRequestId: true,
    maxBodySize: 10 * 1024 * 1024,  // 10 MB
    requestTimeout: 30,              // seconds
    compression: new CompressionConfig(
        gzip: true,
        brotli: true,
        quality: 6,
        minSize: 1024
    ),
    rateLimit: new RateLimitConfig(
        perSecond: 100,
        burst: 200,
        ipBased: true
    )
);

$app = new App($config);
```

### Middleware Configuration

**Compression:**

```php
use Spikard\Config\CompressionConfig;

$compression = new CompressionConfig(
    gzip: true,          // Enable gzip
    brotli: true,        // Enable brotli
    minSize: 1024,       // Min bytes to compress
    quality: 6           // 0-11 for brotli, 0-9 for gzip
);
```

**Rate Limiting:**

```php
use Spikard\Config\RateLimitConfig;

$rateLimit = new RateLimitConfig(
    perSecond: 100,     // Max requests per second
    burst: 200,         // Burst allowance
    ipBased: true       // Per-IP rate limiting
);
```

**JWT Authentication:**

```php
use Spikard\Config\JwtConfig;

$jwt = new JwtConfig(
    secret: 'your-secret-key',
    algorithm: 'HS256',  // HS256, HS384, HS512, RS256, etc.
    audience: ['api.example.com'],
    issuer: 'auth.example.com',
    leeway: 30  // seconds
);
```

**API Key Authentication:**

```php
use Spikard\Config\ApiKeyConfig;

$apiKey = new ApiKeyConfig(
    keys: ['sk_test_123456', 'sk_prod_789012'],
    headerName: 'X-API-Key'
);
```

**Static Files:**

```php
use Spikard\Config\StaticFilesConfig;

$static = new StaticFilesConfig(
    enabled: true,
    root: './public',
    indexFile: 'index.html',
    cache: true
);

$config = new ServerConfig(staticFiles: $static);
```

**OpenAPI Documentation:**

```php
use Spikard\Config\OpenApiConfig;

$openapi = new OpenApiConfig(
    enabled: true,
    title: 'My API',
    version: '1.0.0',
    description: 'API documentation',
    swaggerUiPath: '/docs',
    redocPath: '/redoc',
    openapiJsonPath: '/openapi.json'
);

$config = new ServerConfig(openapi: $openapi);
```

## Lifecycle Hooks

```php
use Spikard\Config\LifecycleHooks;

$hooks = new LifecycleHooks(
    onRequest: function (Request $request) {
        error_log("{$request->method} {$request->path}");
        return $request;  // Must return request to continue
    },

    preValidation: function (Request $request) {
        $token = $request->headers['Authorization'] ?? null;
        if (!$token) {
            return Response::json(['error' => 'Unauthorized'], 401);
        }
        return $request;
    },

    preHandler: function (Request $request) {
        // Additional checks before handler
        return $request;
    },

    onResponse: function (Response $response) {
        $response->headers['X-Frame-Options'] = 'DENY';
        return $response;
    },

    onError: function (Response $response) {
        error_log("Error: {$response->statusCode}");
        return $response;
    }
);

$config = new ServerConfig(hooks: $hooks);
$app = new App($config);
```

## WebSockets

```php
use Spikard\Handlers\WebSocketHandlerInterface;

class ChatHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
        error_log('Client connected');
    }

    public function onMessage(string $message): void
    {
        $data = json_decode($message, true);
        error_log("Received: {$data['message']}");

        // Echo back the message
        $response = json_encode(['echo' => $data]);
        // Response sent automatically
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        error_log("Client disconnected: {$code}");
    }
}

$app = $app->addWebSocket('/ws', new ChatHandler());
```

**Alternative: Inline WebSocket handler:**

```php
// Using foreach pattern for message iteration
$app = $app->addRoute('GET', '/chat', function () {
    foreach ($messages as $message) {
        $data = json_decode($message, true);
        yield json_encode(['echo' => $data]);
    }
});
```

## Server-Sent Events (SSE)

```php
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Http\SseEvent;

class NotificationProducer implements SseEventProducerInterface
{
    private int $count = 0;

    public function onConnect(): void
    {
        error_log('Client connected to SSE stream');
    }

    public function nextEvent(): ?SseEvent
    {
        sleep(1);

        if ($this->count >= 10) {
            return null;  // End stream
        }

        $event = new SseEvent(
            data: json_encode([
                'count' => $this->count,
                'message' => "Event #{$this->count}",
                'timestamp' => time(),
            ]),
            eventType: 'notification',
            id: (string) $this->count,
            retryMs: 3000
        );

        $this->count++;
        return $event;
    }

    public function onDisconnect(): void
    {
        error_log('Client disconnected from SSE');
    }
}

$app = $app->addSse('/notifications', new NotificationProducer());
```

**Alternative: Inline SSE with StreamingResponse:**

```php
use Spikard\Http\StreamingResponse;

$app = $app->addRoute('GET', '/events', function () {
    $generator = function (): Generator {
        for ($i = 0; $i < 10; $i++) {
            $data = json_encode([
                'count' => $i,
                'message' => "Event #{$i}",
            ]);

            yield "data: {$data}\n\n";
            sleep(1);
        }
    };

    return StreamingResponse::sse($generator());
});
```

## Background Tasks

```php
use Spikard\Background\BackgroundTask;

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $user = $request->jsonBody();

    // Schedule background work (doesn't block response)
    BackgroundTask::run(function () use ($user) {
        // Send welcome email
        sleep(2);
        error_log("Sent email to {$user['email']}");
    });

    BackgroundTask::run(function () use ($user) {
        // Log analytics
        sleep(1);
        error_log("Logged user creation: {$user['name']}");
    });

    // Return immediately
    return Response::json([
        'user' => $user,
        'message' => 'User created. Welcome email will be sent shortly.',
    ], 201);
});
```

## Testing

```php
use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;

final class ApiTest extends TestCase
{
    private TestClient $client;

    protected function setUp(): void
    {
        $app = new App();
        $app = $app->addRoute('GET', '/users/{id}', function (Request $request) {
            return Response::json(['id' => $request->pathParams['id']]);
        });

        $this->client = TestClient::create($app);
    }

    public function testGetUser(): void
    {
        $response = $this->client->request('GET', '/users/123', []);

        $this->assertSame(200, $response->statusCode);
        $this->assertEquals(['id' => '123'], $response->body);
    }

    public function testCreateUser(): void
    {
        $response = $this->client->request(
            method: 'POST',
            path: '/users',
            options: [
                'json' => ['name' => 'Alice', 'email' => 'alice@example.com'],
            ]
        );

        $this->assertSame(201, $response->statusCode);
    }

    protected function tearDown(): void
    {
        $this->client->close();
    }
}
```

### WebSocket Testing

```php
public function testWebSocket(): void
{
    $app = $app->addWebSocket('/ws', new ChatHandler());
    $client = TestClient::create($app);

    $ws = $client->websocketConnect('/ws');
    $ws->sendJson(['message' => 'hello']);

    $response = $ws->receiveJson();
    $this->assertEquals('hello', $response['echo']['message']);

    $ws->close();
    $client->close();
}
```

### SSE Testing

```php
public function testSse(): void
{
    $app = $app->addSse('/events', new NotificationProducer());
    $client = TestClient::create($app);

    $response = $client->request('GET', '/events', []);
    $sse = new SseStream($response->text());

    $events = $sse->eventsAsJson();
    $this->assertGreaterThan(0, count($events));

    $client->close();
}
```

## Type Support

Spikard PHP uses PHP 8.2+ features for type safety:

- **Strict types**: `declare(strict_types=1)` in all files
- **Readonly properties**: Immutable configuration objects
- **Named parameters**: Clear constructor calls
- **Union types**: `string|int` for flexible parameters
- **Type hints**: Return types on all public methods
- **PHPDoc**: Complete annotations with `@psalm` tags
- **PHPStan**: Level max static analysis (zero errors)

**Example with full type safety:**

```php
<?php

declare(strict_types=1);

use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * @psalm-return Response
 */
function handleRequest(Request $request): Response
{
    /** @var array{name: string, email: string} $user */
    $user = $request->jsonBody();

    return Response::json($user, 201);
}
```

## Running the Server

```php
// Development
$app->run();

// Production with multiple workers
$config = new ServerConfig(
    host: '0.0.0.0',
    port: 8080,
    workers: 4
);
$app->run($config);

// Using ServerConfigBuilder for complex configs
$config = ServerConfig::builder()
    ->withHost('0.0.0.0')
    ->withPort(8080)
    ->withWorkers(4)
    ->withCompression(new CompressionConfig(gzip: true, brotli: true))
    ->withRateLimit(new RateLimitConfig(perSecond: 100, burst: 200))
    ->build();

$app->run($config);
```

## Performance

### ext-php-rs Advantage

The Spikard PHP binding uses **ext-php-rs** for direct Rust FFI, providing:

- **Zero-copy data exchange**: Shared memory between PHP and Rust (similar to PyO3 for Python)
- **No JSON serialization overhead**: Direct type conversion (Value → PHP array → Rust struct)
- **Compiled native code**: HTTP layer is pure Rust, not PHP
- **Async all the way**: Tokio async runtime with Hyper for HTTP, no PHP blocking
- **Optimized builds**: Release mode with LTO enabled for production deployments

### Architecture

All middleware (compression, rate limiting, JWT, CORS) runs in Rust. PHP only handles:

1. **Configuration**: Preparing ServerConfig with typed builders
2. **Business logic**: Your handler functions (PHP callables)
3. **Response construction**: Building JSON/HTML/streaming responses

### Benchmarks

Typical request overhead:
- Pure Rust core: < 1ms (routing + middleware)
- PHP handler invocation: 0.2-0.5ms (via ext-php-rs FFI)
- Response serialization: < 0.5ms (direct conversion, no JSON hops)

**Total typical latency: < 2ms** before your business logic runs

Compare to traditional PHP-FPM:
- PHP initialization: ~5-10ms
- Framework bootstrap: ~2-5ms
- Request parsing: ~1-2ms
- Handler execution: variable
- Response output: ~1-2ms

**Spikard removes the first 8-19ms overhead** by running HTTP layer in Rust.

## Route Attributes (PHP 8.0+ Syntax)

While Spikard uses programmatic route registration (fluent API), you can build a decorator layer:

```php
use Attribute;
use ReflectionClass;

#[Attribute(Attribute::TARGET_METHOD)]
final class Route
{
    public function __construct(
        public readonly string $method,
        public readonly string $path
    ) {}
}

final class PostController
{
    #[Route('GET', '/posts')]
    public function index(Request $request): Response
    {
        return Response::json(['posts' => []]);
    }

    #[Route('POST', '/posts')]
    public function create(Request $request): Response
    {
        $data = $request->jsonBody();
        return Response::json($data, 201);
    }

    #[Route('GET', '/posts/{id}')]
    public function show(Request $request): Response
    {
        $id = $request->pathParams['id'];
        return Response::json(['id' => $id]);
    }
}

// Bootstrap from attributes
$app = new App();
$controller = new PostController();
$reflection = new ReflectionClass($controller);

foreach ($reflection->getMethods() as $method) {
    foreach ($method->getAttributes(Route::class) as $attribute) {
        $route = $attribute->newInstance();
        $handler = $method->getClosure($controller);
        $app = $app->addRoute($route->method, $route->path, $handler);
    }
}

$app->run();
```

## Examples

The [examples directory](../../examples/) contains comprehensive demonstrations:

**PHP-specific examples:**
- [Hello World](../../examples/php/01-hello-world.php) - Basic server
- [JSON API](../../examples/php/02-json-api.php) - REST API with validation
- [Background Tasks](../../examples/php/03-background-tasks.php) - Background job processing
- [Streaming SSE](../../examples/php/04-streaming-sse.php) - Server-Sent Events
- [Dependency Injection](../../examples/php/05-dependency-injection.php) - DI with factories

**API Schemas** (language-agnostic, can be used with code generation):
- [Todo API](../../examples/schemas/todo-api.openapi.yaml) - REST CRUD with validation
- [File Service](../../examples/schemas/file-service.openapi.yaml) - File uploads/downloads
- [Auth Service](../../examples/schemas/auth-service.openapi.yaml) - JWT, API keys, OAuth
- [Chat Service](../../examples/schemas/chat-service.asyncapi.yaml) - WebSocket messaging
- [Event Streams](../../examples/schemas/events-stream.asyncapi.yaml) - SSE streaming

See [examples/README.md](../../examples/README.md) for code generation instructions.

## Documentation

### Project Resources

- [Main Project README](../../README.md) - Spikard monorepo overview
- [Contributing Guide](../../CONTRIBUTING.md) - Development workflow
- [Changelog](../../CHANGELOG.md) - Version history and breaking changes

### Architecture & Design

- [ADR 0001: Architecture & Layering](../../docs/adr/0001-architecture.md)
- [ADR 0002: Runtime & Middleware Stack](../../docs/adr/0002-runtime-and-middleware.md)
- [ADR 0003: Validation & Fixtures](../../docs/adr/0003-validation-and-fixtures.md)
- [ADR 0005: Lifecycle Hooks](../../docs/adr/0005-lifecycle-hooks.md)

### PHP-Specific Standards

- [PHP 8.2+ Standards](../../CLAUDE.md#php-82-with-phpstan--psr-standards) - Type safety, PSR compliance, PHPStan
- [Error Handling](../../CLAUDE.md#cross-language-error-boundaries) - FFI error propagation
- [Fixture Testing](../../CLAUDE.md#fixture-driven-testing) - Test strategy and patterns

### Cross-Language Reference

- [Python Bindings](../python/) - PyO3 implementation
- [Node.js Bindings](../../crates/spikard-node/) - napi-rs implementation
- [Ruby Bindings](../../crates/spikard-rb/) - magnus implementation
- [Rust Core](../../crates/spikard/) - Core library documentation

## Support

- GitHub Issues: [Report bugs](https://github.com/Goldziher/spikard/issues)
- GitHub Discussions: [Ask questions](https://github.com/Goldziher/spikard/discussions)

## License

MIT - See [LICENSE](LICENSE) for details

## Ecosystem

Spikard is available for multiple languages:

| Language | Package | Status |
|----------|---------|--------|
| Python | [spikard](https://pypi.org/project/spikard/) | Stable |
| Node.js | [@spikard/node](https://www.npmjs.com/package/@spikard/node) | Stable |
| Ruby | [spikard](https://rubygems.org/gems/spikard) | Stable |
| PHP | [spikard/spikard](https://packagist.org/packages/spikard/spikard) | Stable |
| WebAssembly | [@spikard/wasm](https://www.npmjs.com/package/@spikard/wasm) | Stable |
| Rust | [spikard](https://crates.io/crates/spikard) | Stable |
