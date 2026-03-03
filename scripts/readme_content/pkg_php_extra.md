## Usage

Composer installs Spikard as a Composer plugin and runs the bundled native extension installer automatically.
In CI or other non-interactive environments, allow the plugin explicitly first:

```bash
composer config allow-plugins.spikard/spikard true
composer require spikard/spikard
```

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
