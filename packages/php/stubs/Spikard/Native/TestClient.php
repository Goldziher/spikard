<?php

declare(strict_types=1);

namespace Spikard\Native;

use Spikard\Response;
use Spikard\Testing\WebSocketTestConnection;
use Spikard\Testing\SseStream;

/**
 * Native test client for PHP using the full Rust HTTP stack.
 *
 * Provides HTTP testing capabilities against a Spikard server without network overhead.
 * Routes are executed directly through the Rust server implementation.
 */
class TestClient
{
    /**
     * Create a new test client.
     *
     * @param array<int, array{method: string, path: string, handler_name: string, handler?: object, websocket?: bool, sse?: bool}> $routes HTTP routes configuration
     * @param array<string, mixed> $config Optional server configuration
     *
     * @throws \Exception if routes or config are invalid
     */
    public function __construct(array $routes, ?array $config = null)
    {
    }

    /**
     * Execute an HTTP request using the full Rust HTTP stack.
     *
     * @param string $method HTTP method (GET, POST, PUT, PATCH, DELETE, etc)
     * @param string $path Request path
     * @param array<string, mixed> $options Request options (body, headers, cookies, etc)
     */
    public function request(string $method, string $path, array $options = []): Response
    {
    }

    /**
     * Send a GraphQL query or mutation.
     *
     * @param string $query GraphQL query/mutation string
     * @param array<string, mixed> $variables Optional GraphQL variables
     * @param string $operationName Optional operation name for multi-operation documents
     */
    public function graphql(string $query, ?array $variables = null, ?string $operationName = null): Response
    {
    }

    /**
     * Send a GraphQL query and get HTTP status and body separately.
     *
     * @param string $query GraphQL query/mutation string
     * @param array<string, mixed> $variables Optional GraphQL variables
     * @param string $operationName Optional operation name for multi-operation documents
     *
     * @return array<int, mixed> [status, body]
     */
    public function graphqlWithStatus(string $query, ?array $variables = null, ?string $operationName = null): array
    {
    }

    /**
     * Connect to a WebSocket endpoint for testing.
     *
     * @param string $path WebSocket endpoint path
     * @param string $sendText Optional text message to send immediately
     */
    public function websocket(string $path, ?string $sendText = null): WebSocketTestConnection
    {
    }

    /**
     * Connect to a Server-Sent Events endpoint for testing.
     *
     * @param string $path SSE endpoint path
     */
    public function sse(string $path): SseStream
    {
    }

    /**
     * Close the test client and release resources.
     */
    public function close(): void
    {
    }
}
