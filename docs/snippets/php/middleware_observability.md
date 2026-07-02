```php
<?php

declare(strict_types=1);

use Monolog\Logger;
use Monolog\Handler\StreamHandler;
use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;
use Spikard\Http\Response;

// PSR-3 compatible logger (Monolog)
$logger = new Logger('app');
$logger->pushHandler(new StreamHandler('php://stdout', Logger::INFO));

$hooks = LifecycleHooks::builder()
    ->withOnRequest(function (Request $request) use ($logger): HookResult {
        // Generate or propagate request ID
        $requestId = $request->headers['x-request-id'] ?? uniqid('req_', true);

        // Inject into context for handlers to use
        $request->context['request_id'] = $requestId;

        // Log request with structured data
        $logger->info('request_started', [
            'request_id' => $requestId,
            'method' => $request->method,
            'path' => $request->path,
            'user_agent' => $request->headers['user-agent'] ?? null,
        ]);

        return HookResult::continue();
    })
    ->withOnResponse(function (Request $request, Response $response) use ($logger): HookResult {
        $requestId = $request->context['request_id'] ?? 'unknown';

        $logger->info('request_completed', [
            'request_id' => $requestId,
            'status' => $response->status,
            'duration_ms' => $response->durationMs ?? null,
        ]);

        // Propagate request ID in response headers
        $response->headers['X-Request-ID'] = $requestId;

        return HookResult::continue();
    })
    ->build();

$app = (new App(new ServerConfig(port: 8000)))
    ->withLifecycleHooks($hooks);
```
