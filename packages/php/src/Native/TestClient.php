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
     * Create a new test client.
     *
     * @param array<int, array<string, mixed>>|null $routes HTTP routes configuration
     * @param array<string, mixed>|null $config Server configuration options
     *
     * @throws RuntimeException if Spikard PHP extension is not loaded
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

    /**
     * Send a GraphQL query or mutation.
     *
     * @param string $query GraphQL query/mutation string
     * @param array<string, mixed>|null $variables Optional GraphQL variables
     * @param string|null $operationName Optional operation name for multi-operation documents
     */
    public function graphql(string $query, ?array $variables = null, ?string $operationName = null): Response
    {
        unset($query, $variables, $operationName);
        throw new RuntimeException('Spikard PHP extension is not loaded.');
    }

    /**
     * Send a GraphQL query and get HTTP status and body separately.
     *
     * @param string $query GraphQL query/mutation string
     * @param array<string, mixed>|null $variables Optional GraphQL variables
     * @param string|null $operationName Optional operation name for multi-operation documents
     *
     * @return array<int, string|int>
     */
    public function graphqlWithStatus(string $query, ?array $variables = null, ?string $operationName = null): array
    {
        unset($query, $variables, $operationName);
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
