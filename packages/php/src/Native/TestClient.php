<?php

declare(strict_types=1);

namespace Spikard\Native;

use RuntimeException;
use Spikard\Http\Response;

/**
 * Placeholder definition for the native test client when the extension
 * is not loaded. The actual implementation is provided by ext-php-rs.
 */
final class TestClient
{
    /** @param array<int, mixed> $routes */
    public function __construct(array $routes)
    {
        unset($routes);
        if (!\function_exists('spikard_version')) {
            throw new RuntimeException('Spikard PHP extension is not loaded.');
        }
    }

    /** @param array<string, mixed> $options */
    public function request(string $method, string $path, array $options = []): Response
    {
        unset($method, $path, $options);
        throw new RuntimeException('Spikard PHP extension is not loaded.');
    }
}
