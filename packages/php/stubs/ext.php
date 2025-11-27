<?php

declare(strict_types=1);

// Stub definitions for static analysis when the native extension is absent.
if (false) {
    function spikard_version(): string
    {
        return '0.0.0';
    }

    /**
     * @param array<int, array{method: string, path: string, handler: object}> $routes
     * @param array<string, mixed> $config
     * @param array<string, callable> $lifecycle
     */
    function spikard_start_server(array $routes, array $config, array $lifecycle): int
    {
        return 1;
    }

    function spikard_stop_server(int $handle): bool
    {
        return true;
    }
}

namespace Spikard\Native {
    class TestClient
    {
        /** @param array<int, array{method: string, path: string, handler?: object, websocket?: bool, sse?: bool}> $routes */
        public function __construct(array $routes) {}
        /** @param array<string, mixed> $options */
        public function request(string $method, string $path, array $options = []): object {}
        public function websocket(string $path, ?string $sendText = null): WebSocket {}
        public function sse(string $path): SseStream {}
    }

    class WebSocket
    {
        public function recv_text(): ?string {}
        public function send_text(string $message): bool {}
    }

    class SseStream
    {
        /** @return array<int, string> */
        public function events(): array {}
    }
}
