<?php

declare(strict_types=1);

namespace Spikard\Testing;

/**
 * WebSocket test connection for PHP.
 *
 * Provides methods to send and receive WebSocket messages in tests.
 */
class WebSocketTestConnection
{
    /**
     * Send a text message to the WebSocket.
     *
     * @param string $text Message content
     *
     * @throws \Exception if connection is closed
     */
    public function sendText(string $text): void
    {
    }

    /**
     * Send a JSON message to the WebSocket.
     *
     * @param string $data JSON string payload
     *
     * @throws \Exception if connection is closed or JSON is invalid
     */
    public function sendJson(string $data): void
    {
    }

    /**
     * Receive a text message from the WebSocket.
     *
     * @throws \Exception if connection is closed or receive not supported
     */
    public function receiveText(): string
    {
    }

    /**
     * Receive a JSON message from the WebSocket.
     *
     * @return array<string, mixed>
     *
     * @throws \Exception if connection is closed or JSON is invalid
     */
    public function receiveJson(): array
    {
    }

    /**
     * Receive raw bytes from the WebSocket.
     *
     * @return array<int, int>
     *
     * @throws \Exception if connection is closed or receive not supported
     */
    public function receiveBytes(): array
    {
    }

    /**
     * Close the WebSocket connection.
     */
    public function close(): void
    {
    }

    /**
     * Check if the connection is closed.
     */
    public function isClosed(): bool
    {
    }
}
