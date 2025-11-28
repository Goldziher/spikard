# Spikard PHP

[![Discord](https://img.shields.io/badge/Discord-Join%20our%20community-7289da)](https://discord.gg/pXxagNK2zN)
[![PyPI](https://badge.fury.io/py/spikard.svg)](https://badge.fury.io/py/spikard)
[![npm](https://img.shields.io/npm/v/spikard)](https://www.npmjs.com/package/spikard)
[![npm (WASM)](https://img.shields.io/npm/v/spikard-wasm?label=npm%20%28wasm%29)](https://www.npmjs.com/package/spikard-wasm)
[![RubyGems](https://badge.fury.io/rb/spikard.svg)](https://rubygems.org/gems/spikard)
[![Packagist](https://img.shields.io/packagist/v/spikard/spikard)](https://packagist.org/packages/spikard/spikard)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

High-performance PHP web framework with a Rust core. Build REST APIs, WebSockets, and SSE services with modern PHP 8.2+ patterns backed by Axum and Tower-HTTP via ext-php-rs.

## Installation

**From source (currently):**

```bash
cd packages/php
composer install
# Build the Rust extension
cargo build --release --manifest-path ../../crates/spikard-php/Cargo.toml
```

**Requirements:**
- PHP 8.2+
- Composer 2.0+
- Rust toolchain (for building native extension)

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

PHP bindings use:
- **ext-php-rs** for zero-copy FFI (similar to PyO3 for Python)
- **Direct type conversion** without JSON serialization overhead
- **Rust-powered core** with Tokio and Hyper for HTTP
- **Tower middleware** for zero-overhead routing and middleware
- **No PHP overhead for HTTP layer** - pure Rust until handler invocation

All middleware (compression, rate limiting, JWT, CORS) runs in Rust. PHP only handles:
1. Configuration (preparing ServerConfig)
2. Business logic (your handler functions)
3. Response construction

## Examples

See `/examples/php/` for complete examples:
- `01-hello-world.php` - Basic server
- `02-json-api.php` - REST API with validation
- `03-background-tasks.php` - Background job processing
- `04-streaming-sse.php` - Server-Sent Events
- `05-dependency-injection.php` - DI with factories

## Documentation

- [Main Project README](../../README.md)
- [Contributing Guide](../../CONTRIBUTING.md)
- [Architecture Decision Records](../../docs/adr/)
- [PHP Language Standards](../../CLAUDE.md#php-82-with-phpstan--psr-standards)

## License

MIT
