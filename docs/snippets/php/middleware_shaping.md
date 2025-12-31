```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\HookResult;
use Spikard\Http\Request;
use Spikard\Http\Response;

// Server-side rate limiting configuration (uses Rust pipeline)
$rateLimit = RateLimitConfig::builder()
    ->withPerSecond(100)
    ->withBurst(200)
    ->withIpBased(true)
    ->build();

$hooks = LifecycleHooks::builder()
    ->withOnRequest(function (Request $request): HookResult {
        // 1. Normalize headers (lowercase keys)
        $normalizedHeaders = [];
        foreach ($request->headers as $key => $value) {
            $normalizedHeaders[strtolower($key)] = $value;
        }
        $request->headers = $normalizedHeaders;

        // 2. Inject tenant from subdomain
        $host = $request->headers['host'] ?? '';
        $tenant = str_contains($host, '.') ? explode('.', $host)[0] : 'default';
        $request->context['tenant'] = $tenant;

        // 3. Feature flags from query params
        $featureStr = $request->query['features'] ?? '';
        $features = array_filter(explode(',', $featureStr));
        $request->context['features'] = $features;

        return HookResult::continue();
    })
    ->withOnResponse(function (Request $request, Response $response): HookResult {
        // Response compression for large payloads (handled by Rust layer)
        // Add custom headers if needed
        $response->headers['X-Tenant'] = $request->context['tenant'] ?? 'default';

        return HookResult::continue();
    })
    ->build();

$config = ServerConfig::builder()
    ->withPort(8000)
    ->withRateLimit($rateLimit)
    ->withMaxBodySize(10 * 1024 * 1024)  // 10 MB limit
    ->withCompression(true)               // Enable gzip compression
    ->build();

$app = (new App($config))
    ->withLifecycleHooks($hooks);
```
