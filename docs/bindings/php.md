# PHP Binding

Spikard's PHP binding uses ext-php-rs with PSR-compliant APIs and structured error handling. Handlers receive typed Request objects; responses are plain PHP arrays or Response objects with automatic JSON serialization. The Rust core handles routing, middleware, and streaming.

## Quickstart

```php
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

final class HelloController
{
    #[Get('/')]
    public function index(): Response
    {
        return Response::text('Hello, World!');
    }
}

$config = new ServerConfig(port: 8000);
$app = (new App($config))->registerController(new HelloController());
$app->run();
```

## Installation

Install via Composer:

```bash
composer require spikard/spikard
```

Requires PHP 8.2+ and Rust toolchain (for ext-php-rs native extension build).

## Configuration

```php
use Spikard\Config\ServerConfig;

$config = new ServerConfig(
    port: 8000,
    host: '127.0.0.1',
    workers: 4,  // Optional: worker threads
);

$app = new App($config);
```

## Routing

Register routes with controller attributes:

```php
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;

final class UsersController
{
    #[Get('/users')]
    public function list(): Response
    {
        return Response::json(['users' => []]);
    }

    #[Post('/users')]
    public function create(Request $request): Response
    {
        $data = $request->body;
        return Response::json(['id' => 1] + $data, 201);
    }

    #[Get('/users/{id:int}')]
    public function show(Request $request): Response
    {
        $id = $request->pathParams['id'];
        return Response::json(['id' => $id, 'name' => 'Alice']);
    }
}
```

## Request & Response

Handlers receive `Spikard\Http\Request` objects and return responses:

```php
use Spikard\Attributes\Post;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class DataController
{
    #[Post('/data')]
    public function store(Request $request): Response
    {
        // Access request data
        $method = $request->method;
        $path = $request->path;
        $headers = $request->headers;
        $query = $request->query;
        $pathParams = $request->pathParams;
        $body = $request->body;  // Array from JSON payload

        // Return responses
        return Response::text('Plain text');
        return Response::json(['key' => 'value']);
        return Response::json(['error' => 'Not found'], 404);
    }
}
```

## Validation

Validate request bodies manually or use schema validation:

```php
use Spikard\Attributes\Post;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class ValidationController
{
    #[Post('/users')]
    public function create(Request $request): Response
    {
        $data = $request->body;

        // Manual validation
        if (!isset($data['name'], $data['email'])) {
            return Response::json(
                ['error' => 'Missing required fields: name, email'],
                400
            );
        }

        if (!filter_var($data['email'], FILTER_VALIDATE_EMAIL)) {
            return Response::json(
                ['error' => 'Invalid email format'],
                400
            );
        }

        return Response::json(['id' => 1, 'name' => $data['name']], 201);
    }
}
```

## Dependency Injection

Register values and factories in a DependencyContainer:

```php
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;

$container = new DependencyContainer(
    values: [
        'app_name' => 'My API',
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

$app = (new App($config))->withDependencies($container);
```

## Lifecycle Hooks

Register hooks for cross-cutting behavior:

```php
// Called on every request
$app = $app->onRequest(function (array $request): array {
    error_log("{$request['method']} {$request['path']}");
    return $request;
});

// Called before validation
$app = $app->preValidation(function (array $request): array {
    return $request;
});

// Called before handler
$app = $app->preHandler(function (array $request): array {
    return $request;
});

// Called after handler completes
$app = $app->onResponse(function (array $response): array {
    return $response;
});

// Called on handler error
$app = $app->onError(function (array $error): array {
    error_log("Error: " . json_encode($error));
    return $error;
});
```

## Error Handling

Handlers should return error responses with appropriate status codes:

```php
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Attributes\Get;

final class DataController
{
    #[Get('/data/{id:int}')]
    public function show(Request $request): Response
    {
        $id = (int) ($request->pathParams['id'] ?? 0);

        if ($id < 1) {
            return Response::json(
                [
                    'error' => 'Invalid ID',
                    'code' => 'INVALID_INPUT',
                    'details' => ['id' => 'must be positive']
                ],
                400
            );
        }

        return Response::json(['id' => $id, 'data' => 'content']);
    }
}

$app = $app->registerController(DataController::class);
```

## Response Types

Handlers return Response objects with flexible body and header support:

```php
use Spikard\Http\Response;

// JSON response (auto-serialized)
return Response::json(['id' => 1, 'name' => 'Alice']);
return Response::json(['error' => 'Not found'], 404);

// Plain text response
return Response::text('Plain text content');

// Custom response with headers and cookies
return new Response(
    body: ['custom' => 'data'],
    statusCode: 201,
    headers: ['X-Custom-Header' => 'value'],
    cookies: ['session' => 'abc123']
);

// Response builder methods
$response = Response::json(['items' => []])
    ->withCookies(['auth' => 'token123']);
```

### Streaming Responses

For large files, real-time data, or Server-Sent Events:

```php
use Spikard\Http\StreamingResponse;

#[Get('/events')]
public function stream(): StreamingResponse
{
    $events = function(): Generator {
        for ($i = 0; $i < 5; $i++) {
            yield "data: " . json_encode(['count' => $i]) . "\n\n";
            sleep(1);
        }
    };

    return StreamingResponse::sse($events());
}

#[Get('/download')]
public function downloadFile(): StreamingResponse
{
    return StreamingResponse::file(
        '/path/to/large-file.zip',
        chunkSize: 65536
    );
}

#[Get('/records')]
public function streamJsonLines(): StreamingResponse
{
    $records = function(): Generator {
        foreach ($this->database->fetchLargeResult() as $row) {
            yield $row;
        }
    };

    return StreamingResponse::jsonLines($records());
}
```

## Request Parameter Extraction

Extract typed parameters from requests with optional validation:

```php
use Spikard\Http\Params\{Query, Path, Header, Body, Cookie};
use Spikard\Attributes\Post;

final class ItemController
{
    #[Post('/items')]
    public function create(
        array $body = new Body(
            schema: [
                'type' => 'object',
                'required' => ['name', 'price'],
                'properties' => [
                    'name' => ['type' => 'string'],
                    'price' => ['type' => 'number', 'minimum' => 0],
                ],
            ]
        )
    ): Response {
        return Response::json(['id' => 1] + $body, 201);
    }

    #[Get('/items')]
    public function list(
        int $limit = new Query(default: 10),
        int $offset = new Query(default: 0),
        array $tags = new Query(defaultFactory: fn() => [])
    ): Response {
        return Response::json(['items' => []]);
    }

    #[Get('/items/{id:int}')]
    public function show(
        int $id = new Path()
    ): Response {
        return Response::json(['id' => $id]);
    }

    #[Get('/items/{id:int}')]
    public function withHeader(
        int $id = new Path(),
        string $token = new Header(default: '')
    ): Response {
        return Response::json(['authorized' => !empty($token)]);
    }
}
```

## Server Configuration

Complete configuration options via ServerConfig:

```php
use Spikard\Config\{
    ServerConfig,
    CompressionConfig,
    RateLimitConfig,
    CorsConfig,
    JwtConfig,
    ApiKeyConfig,
    StaticFilesConfig,
    OpenApiConfig,
};

$config = ServerConfig::builder()
    ->withHost('0.0.0.0')
    ->withPort(8080)
    ->withWorkers(4)
    ->withMaxBodySize(52428800) // 50 MB
    ->withRequestTimeout(60)
    ->withCompression(new CompressionConfig(
        enabled: true,
        minBodySize: 1024,
        level: 6,
    ))
    ->withRateLimit(new RateLimitConfig(
        requestsPerSecond: 100,
        burst: 50,
    ))
    ->withCors(new CorsConfig(
        allowedOrigins: ['https://app.example.com'],
        allowedMethods: ['GET', 'POST', 'PUT'],
        allowedHeaders: ['Content-Type', 'Authorization'],
        allowCredentials: true,
        maxAge: 3600,
    ))
    ->withJwtAuth(new JwtConfig(
        secret: $_ENV['JWT_SECRET'],
        algorithms: ['HS256'],
        issuer: 'api.example.com',
    ))
    ->withStaticFiles(new StaticFilesConfig(
        directory: __DIR__ . '/public',
        urlPath: '/static',
    ))
    ->withOpenApi(new OpenApiConfig(
        enabled: true,
        title: 'My API',
        version: '1.0.0',
    ))
    ->build();

$app = (new App($config))->registerController(new ItemController());
$app->run();
```

## File Uploads

Handle multipart form data with file uploads:

```php
use Spikard\Attributes\Post;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class FileController
{
    #[Post('/upload')]
    public function upload(Request $request): Response
    {
        $files = $request->files;

        if (empty($files)) {
            return Response::json(['error' => 'No files provided'], 400);
        }

        $uploaded = [];
        foreach ($files as $fieldName => $fileArray) {
            // $fileArray is ['name' => '...', 'tmp_name' => '...', 'size' => ..., ...]
            $uploaded[] = [
                'field' => $fieldName,
                'name' => $fileArray['name'],
                'size' => $fileArray['size'],
            ];
        }

        return Response::json(['uploaded' => $uploaded], 201);
    }
}
```

## WebSocket Support

Real-time bidirectional communication with WebSocket handlers:

```php
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Attributes\Route;

final class ChatWebSocketHandler implements WebSocketHandlerInterface
{
    private array $connections = [];

    #[Route('/ws/chat', methods: ['WEBSOCKET'])]
    public function onConnect(): void
    {
        error_log("Client connected");
    }

    public function onMessage(string $message): void
    {
        // Broadcast to all connections
        error_log("Received: {$message}");
        // Send back confirmation
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        error_log("Client disconnected: {$code} - {$reason}");
    }
}

$app->registerWebSocketHandler(new ChatWebSocketHandler());
```

## Server-Sent Events (SSE)

Stream events to clients with automatic formatting:

```php
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Http\StreamingResponse;
use Spikard\Attributes\Get;
use Generator;

final class NotificationController
{
    #[Get('/events/notifications')]
    public function stream(): StreamingResponse
    {
        $producer = new class implements SseEventProducerInterface {
            public function __invoke(): Generator
            {
                for ($i = 0; $i < 10; $i++) {
                    yield "data: " . json_encode([
                        'timestamp' => time(),
                        'message' => "Event {$i}",
                    ]) . "\n\n";
                    sleep(1);
                }
            }
        };

        return StreamingResponse::sse($producer());
    }
}
```

## gRPC Services

Define gRPC handlers for protocol buffer services:

```php
use Spikard\Grpc;
use Spikard\Grpc\{Service, Request, Response, HandlerInterface};

final class UserServiceHandler implements HandlerInterface
{
    public function handleRequest(Request $request): Response
    {
        if ($request->methodName === 'GetUser') {
            // Decode protobuf payload
            $userId = unpack('N', substr($request->payload, 0, 4))[1];

            // Encode response
            $responseData = pack('N', $userId) . 'Alice';
            return new Response($responseData);
        }

        return Response::error('Unknown method');
    }
}

$grpcService = Grpc::createService();
$grpcService->registerHandler('users.UserService', new UserServiceHandler());

// In controller
#[Post('/users.UserService/GetUser')]
public function grpcHandler(Request $request, Service $service): Response
{
    $grpcRequest = Grpc::createRequest(
        'users.UserService',
        'GetUser',
        $request->body
    );

    return $service->handleRequest($grpcRequest);
}
```

## Background Tasks

Execute work asynchronously without blocking responses:

```php
use Spikard\Background\BackgroundTask;
use Spikard\Attributes\Post;
use Spikard\Http\Response;

final class EmailController
{
    #[Post('/send-email')]
    public function sendEmail(): Response
    {
        // Return immediately
        BackgroundTask::run(function($email, $subject) {
            error_log("Sending email to: {$email}");
            sleep(2); // Simulated work
            error_log("Email sent");
        }, ['user@example.com', 'Welcome']);

        return Response::json(['queued' => true], 202);
    }
}
```

## Testing

Write integration tests with TestClient:

```php
use PHPUnit\Framework\TestCase;
use Spikard\Testing\TestClient;
use Spikard\App;
use Spikard\Config\ServerConfig;

final class ApiTest extends TestCase
{
    private TestClient $client;

    protected function setUp(): void
    {
        $config = new ServerConfig(port: 8000);
        $app = (new App($config))->registerController(new ItemController());
        $this->client = TestClient::create($app);
    }

    protected function tearDown(): void
    {
        $this->client->close();
    }

    public function testGetItems(): void
    {
        $response = $this->client->get('/items?limit=5');

        $this->assertEquals(200, $response->getStatusCode());
        $data = $response->parseJson();
        $this->assertArrayHasKey('items', $data);
    }

    public function testCreateItem(): void
    {
        $response = $this->client->post('/items', [
            'name' => 'Widget',
            'price' => 9.99,
        ]);

        $this->assertEquals(201, $response->getStatusCode());
        $data = $response->parseJson();
        $this->assertEquals('Widget', $data['name']);
    }

    public function testItemNotFound(): void
    {
        $response = $this->client->get('/items/9999');

        $this->assertEquals(404, $response->getStatusCode());
    }

    public function testWebSocketConnection(): void
    {
        $ws = $this->client->connectWebSocket('/ws/chat');
        // Test WebSocket communication
    }

    public function testSseStream(): void
    {
        $stream = $this->client->connectSse('/events/notifications');
        // Test SSE event stream
    }
}
```

## Deployment

- Local development: `php app.php`
- Production: Set environment variables `SPIKARD_PORT` and `SPIKARD_HOST`
- Requires PHP 8.2+ and compiled ext-php-rs extension

## Troubleshooting

- **Extension not loading**: Ensure ext-php-rs is compiled and loaded. Run `composer install` to trigger post-install build.
- **Type errors**: Enable strict_types=1 and use PHPStan for static analysis: `composer run phpstan`.
- **Port in use**: Change port in ServerConfig or set `SPIKARD_PORT` env variable.
- **Request parsing fails**: Check Content-Type headers and JSON payload validity.
- **WebSocket/SSE unavailable**: WebSocket and SSE testing requires native extension. Set `SPIKARD_TEST_CLIENT_FORCE_PHP=1` to disable.

## Standards Compliance

- **PSR-4**: Autoloading via Composer `spikard/spikard` namespace
- **PSR-12**: Code style enforced via php-cs-fixer
- **PSR-7**: HTTP message interfaces via Request/Response classes
- **Static Analysis**: PHPStan level max (`composer run lint`)
- **Testing**: PHPUnit tests in `packages/php/tests/` with 80%+ coverage

For more details, see the PHP examples.
