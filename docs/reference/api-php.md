# PHP API Reference

The PHP binding exposes the Rust runtime through a thin, PSR-compliant surface with strict types and dependency injection.

## Package

- Install: `composer require spikard/spikard`
- PHP requirement: 8.2+
- Namespace: `Spikard\`

## Core Classes

### App

Main application class for registering routes and starting the server.

```php
use Spikard\App;
use Spikard\Config\ServerConfig;

$config = new ServerConfig(port: 8000);
$app = new App($config);

// Register routes
$app = $app->addRoute('GET', '/health', function () {
    return Response::json(['status' => 'ok']);
});

// Run server
$app->run();
```

**Methods:**

- `addRoute(string $method, string $path, callable $handler): self` – Register HTTP route
- `withDependencies(DependencyContainer $container): self` – Set up dependency injection
- `onRequest(callable $hook): self` – Register pre-request hook
- `preValidation(callable $hook): self` – Register pre-validation hook
- `preHandler(callable $hook): self` – Register pre-handler hook
- `onResponse(callable $hook): self` – Register post-response hook
- `onError(callable $hook): self` – Register error hook
- `run(): void` – Start the server

### Request

Represents an incoming HTTP request. Passed to route handlers.

```php
use Spikard\Http\Request;

function handler(Request $request) {
    $method = $request->method;           // 'GET', 'POST', etc.
    $path = $request->path;               // '/users'
    $headers = $request->headers;         // ['content-type' => 'application/json']
    $query = $request->query;             // ['page' => '1']
    $pathParams = $request->pathParams;   // ['id' => '123']
    $body = $request->body;               // Parsed JSON as array
    $rawBody = $request->rawBody;         // Raw request body string
}
```

**Properties:**

- `method: string` – HTTP method (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
- `path: string` – URL path
- `headers: array<string, string>` – Request headers
- `query: array<string, string>` – Query parameters
- `pathParams: array<string, mixed>` – Path parameters (typed)
- `body: array<string, mixed> | null` – Parsed JSON body
- `rawBody: string` – Raw request body
- `cookies: array<string, string>` – Request cookies

### Response

Represents an HTTP response returned from handlers.

```php
use Spikard\Http\Response;

// Text response
return Response::text('Hello, World!');

// JSON response
return Response::json(['key' => 'value']);

// JSON with status code
return Response::json(['error' => 'Not found'], 404);

// Custom headers
return Response::json(['data' => []], 200, ['X-Custom' => 'header']);
```

**Static Methods:**

- `text(string $content, int $status = 200): self` – Text response
- `json(mixed $data, int $status = 200, array $headers = []): self` – JSON response
- `html(string $content, int $status = 200): self` – HTML response
- `stream(callable $generator): self` – Server-Sent Events stream
- `empty(int $status = 204): self` – Empty response

## Configuration

### ServerConfig

Server configuration passed to `App` constructor.

```php
use Spikard\Config\ServerConfig;

$config = new ServerConfig(
    port: 8000,
    host: '127.0.0.1',
    workers: 4,
);
```

**Constructor Parameters:**

- `port: int` – Listen port (default: 8000)
- `host: string` – Listen address (default: '127.0.0.1')
- `workers: int | null` – Number of worker threads

## Dependency Injection

### DependencyContainer

Manages value and factory dependencies.

```php
use Spikard\DI\DependencyContainer;
use Spikard\DI\Provide;

$container = new DependencyContainer(
    values: [
        'app_name' => 'My App',
        'debug' => true,
    ],
    factories: [
        'db' => new Provide(
            factory: function (string $app_name): Database {
                return new Database('localhost');
            },
            dependsOn: ['app_name'],
            singleton: true
        ),
    ]
);

$app = (new App($config))->withDependencies($container);
```

**Constructor Parameters:**

- `values: array<string, mixed>` – Value dependencies
- `factories: array<string, Provide>` – Factory dependencies

### Provide

Factory definition for creating dependencies.

```php
use Spikard\DI\Provide;

$factory = new Provide(
    factory: function (Database $db): Cache {
        return new Cache($db);
    },
    dependsOn: ['database'],
    singleton: true
);
```

**Constructor Parameters:**

- `factory: callable` – Factory function
- `dependsOn: string[]` – List of dependency names
- `singleton: bool` – Create once and reuse (default: true)

## Error Handling

### Error Structure

Errors return structured JSON payloads:

```php
{
    "error": "Validation failed",
    "code": "INVALID_INPUT",
    "details": {
        "field": "email",
        "reason": "invalid format"
    }
}
```

**Return from handlers:**

```php
$app = $app->addRoute('POST', '/users', function (Request $request) {
    if (!isset($request->body['email'])) {
        return Response::json(
            [
                'error' => 'Missing required field',
                'code' => 'MISSING_FIELD',
                'details' => ['field' => 'email']
            ],
            400
        );
    }

    return Response::json(['id' => 1], 201);
});
```

## Lifecycle Hooks

All hooks receive request or response arrays and return them (possibly modified).

```php
$app = $app->onRequest(function (array $request): array {
    // Modify or inspect request
    error_log("Request: {$request['method']} {$request['path']}");
    return $request;
});

$app = $app->preValidation(function (array $request): array {
    // Before validation
    return $request;
});

$app = $app->preHandler(function (array $request): array {
    // Before handler execution
    return $request;
});

$app = $app->onResponse(function (array $response): array {
    // After handler, modify response
    return $response;
});

$app = $app->onError(function (array $error): array {
    // On handler error
    return $error;
});
```

## Code Quality

- **Type Safety**: All public APIs are fully typed (`declare(strict_types=1)`)
- **Static Analysis**: Use `composer run lint` (PHPStan level max)
- **Code Style**: Format with `composer run format` (php-cs-fixer)
- **Testing**: `composer test` runs PHPUnit with 80%+ coverage

## Examples

Comprehensive examples are available in `/examples/php/`:

- `01-hello-world.php` – Basic text response
- `02-json-api.php` – JSON endpoints with request bodies
- `03-background-tasks.php` – Long-running tasks
- `04-streaming-sse.php` – Server-Sent Events
- `05-dependency-injection.php` – DI container setup
