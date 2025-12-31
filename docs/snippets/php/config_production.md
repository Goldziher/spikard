```php
<?php

declare(strict_types=1);

use Spikard\App;
use Spikard\Config\CompressionConfig;
use Spikard\Config\OpenApiConfig;
use Spikard\Config\RateLimitConfig;
use Spikard\Config\ServerConfig;

// Production error handling
error_reporting(E_ALL);
ini_set('display_errors', '0');
ini_set('log_errors', '1');

$config = ServerConfig::builder()
    ->withHost('0.0.0.0')
    ->withPort(8080)
    ->withWorkers(4)
    ->withRequestTimeout(60)
    ->withMaxBodySize(10 * 1024 * 1024)

    // High-quality compression
    ->withCompression(
        CompressionConfig::builder()
            ->withGzip(true)
            ->withBrotli(true)
            ->withMinSize(1024)
            ->withQuality(6)
            ->build()
    )

    // Protect against abuse
    ->withRateLimit(
        RateLimitConfig::builder()
            ->withPerSecond(100)
            ->withBurst(200)
            ->withIpBased(true)
            ->build()
    )

    // Auto-generated docs
    ->withOpenApi(
        OpenApiConfig::builder()
            ->withEnabled(true)
            ->withTitle('Production API')
            ->withVersion('1.0.0')
            ->build()
    )

    // Graceful shutdown
    ->withGracefulShutdown(true)
    ->withShutdownTimeout(30)
    ->build();

$app = new App($config);
```
