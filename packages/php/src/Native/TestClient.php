<?php

declare(strict_types=1);

namespace Spikard\Native;

use function file_get_contents;
use function json_decode;

use RuntimeException;
use Spikard\Http\Response;

/**
 * Placeholder definition for the native test client when the extension
 * is not loaded. The actual implementation is provided by ext-php-rs.
 */
final class TestClient
{
    /**
     * @param array<int, mixed>|null $routes
     * @param array<string, mixed>|null $config
     */
    public function __construct(?array $routes = null, ?array $config = null)
    {
        unset($routes, $config);
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

    public function websocket(string $path, ?string $sendText = null): object
    {
        unset($path, $sendText);
        throw new RuntimeException('Spikard PHP extension is not loaded.');
    }

    public function sse(string $path): object
    {
        unset($path);
        throw new RuntimeException('Spikard PHP extension is not loaded.');
    }

    public function close(): void
    {
        throw new RuntimeException('Spikard PHP extension is not loaded.');
    }
}
