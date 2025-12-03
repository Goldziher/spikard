<?php

declare(strict_types=1);

// Stub definitions for static analysis when the native extension is absent.
if (false) {
    function spikard_version(): string
    {
        return '0.0.0';
    }

    /**
     * @param array<int, array{method: string, path: string, handler?: object, handler_name?: string, request_schema?: array<mixed>|null, response_schema?: array<mixed>|null, parameter_schema?: array<mixed>|null, websocket?: bool, sse?: bool}> $routes
     * @param array<string, mixed> $config
     * @param array<string, callable> $lifecycle
     */
    function spikard_start_server(array $routes, array $config, array $lifecycle): int
    {
        return 1;
    }
    /**
     * @return array<string, mixed>
     */
    function spikard_config_defaults(): array {}
    /**
     * @return array<string, mixed>
     */
    function spikard_request_dto_shape(): array {}
    /**
     * @return array<string, mixed>
     */
    function spikard_response_dto_shape(): array {}
    /**
     * @return array<int, array{name: string, kind: string, fields: array<int, array{name: string, php_doc: string, rust_type: string, optional: bool, description: string}>}>
     */
    function spikard_dto_definitions(): array {}
    /**
     * @param array<int, array<string, mixed>> $routes
     * @return array<int, array<string, mixed>>
     */
    function spikard_normalize_routes(array $routes): array {}

    function spikard_stop_server(int $handle): bool
    {
        return true;
    }
}

namespace Spikard\Native {
    class TestClient
    {
        /** @param array<int, array{method: string, path: string, handler?: object, handler_name?: string, request_schema?: array<mixed>|null, response_schema?: array<mixed>|null, parameter_schema?: array<mixed>|null, websocket?: bool, sse?: bool}> $routes */
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
