# PHP Binding

Spikard's PHP binding uses ext-php-rs with PSR-compliant APIs and structured error handling. Handlers receive typed Request objects; responses are plain PHP arrays or Response objects with automatic JSON serialization. The Rust core handles routing, middleware, and streaming.

## Quickstart

```php
<?php
declare(strict_types=1);

require_once __DIR__ . '/vendor/autoload.php';

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Http\Response;

$config = new ServerConfig(port: 8000);
$app = new App($config);

$app = $app->addRoute('GET', '/', function () {
    return Response::text('Hello, World!');
});

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

Register routes with HTTP methods and handler callables:

```php
$app = $app->addRoute('GET', '/users', function () {
    return Response::json(['users' => []]);
});

$app = $app->addRoute('POST', '/users', function (Request $request) {
    $data = $request->body;
    return Response::json(['id' => 1] + $data, 201);
});

$app = $app->addRoute('GET', '/users/{id:int}', function (Request $request) {
    $id = $request->pathParams['id'];
    return Response::json(['id' => $id, 'name' => 'Alice']);
});
```

## Request & Response

Handlers receive `Spikard\Http\Request` objects and return responses:

```php
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = $app->addRoute('POST', '/data', function (Request $request) {
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
});
```

## Validation

Validate request bodies manually or use schema validation:

```php
use Spikard\Http\Request;
use Spikard\Http\Response;

$app = $app->addRoute('POST', '/users', function (Request $request) {
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
});
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
use Spikard\Http\Response;

$app = $app->addRoute('GET', '/data/{id:int}', function (Request $request) {
    $id = $request->pathParams['id'];

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
});
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

## Standards Compliance

- **PSR-4**: Autoloading via Composer `spikard/spikard` namespace
- **PSR-12**: Code style enforced via php-cs-fixer
- **PSR-7**: HTTP message interfaces via Request/Response classes
- **Static Analysis**: PHPStan level max (`composer run lint`)
- **Testing**: PHPUnit tests in `packages/php/tests/` with 80%+ coverage

For more details, see the [PHP examples](/examples/php/).
