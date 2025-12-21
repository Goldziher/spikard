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
        public function websocket(string $path, ?string $sendText = null): \Spikard\Testing\WebSocketTestConnection {}
        public function sse(string $path): \Spikard\Testing\SseStream {}
        public function close(): void {}
    }
}

namespace Spikard\Testing {
    class WebSocketTestConnection
    {
        public function sendText(string $message): void {}
        public function sendJson(string $payload): void {}
        public function receiveText(): string {}
        public function receiveJson(): array {}
        /** @return array<int, int> */
        public function receiveBytes(): array {}
        public function close(): void {}
        public function isClosed(): bool {}
    }

    class SseStream
    {
        /** @return array<int, SseEvent> */
        public function events(): array {}
        public function eventsAsJson(): array {}
        public function body(): string {}
        public function count(): int {}
    }

    class SseEvent
    {
        public function getData(): string {}
        public function asJson(): array {}
        public function getEventType(): ?string {}
        public function getId(): ?string {}
    }
}
