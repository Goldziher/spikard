<?php

declare(strict_types=1);

namespace {
    // Stub definitions for static analysis when the native extension is absent.
    if (false) {
        /**
         * Get the Spikard version string.
         */
        function spikard_version(): string
        {
            return '0.0.0';
        }

        /**
         * Start a Spikard HTTP server.
         *
         * @param array<int, array{method: string, path: string, handler: object}> $routes HTTP routes
         * @param array<string, mixed> $config Server configuration
         * @param array<string, callable> $lifecycle Lifecycle hooks
         * @param object|array<string, mixed>|null $dependencies Handler dependencies (DependencyContainer|array|null)
         */
        function spikard_start_server(array $routes, array $config, array $lifecycle, object|array|null $dependencies): int
        {
            return 1;
        }

        /**
         * Stop a running Spikard HTTP server.
         *
         * @param int $handle Server handle from spikard_start_server()
         */
        function spikard_stop_server(int $handle): bool
        {
            return true;
        }
    }
}

namespace Spikard {
    class Response
    {
        /** @param array<string, string> $headers */
        public function __construct(string $body, int $status = 200, array $headers = []) {}
        public function getStatus(): int {}
        public function getStatusCode(): int {}
        public function getBody(): string {}
        /** @return array<string, mixed> */
        public function json(): array {}
        /** @return array<string, string> */
        public function getHeaders(): array {}
        public function getHeader(string $name): ?string {}
        public function isSuccess(): bool {}
        public function isRedirect(): bool {}
        public function isClientError(): bool {}
        public function isServerError(): bool {}
        /** @return array<string, mixed> */
        public function graphqlData(): array {}
        /** @return array<int, array<string, mixed>> */
        public function graphqlErrors(): array {}
    }
}

namespace Spikard\Native {
    class TestClient
    {
        /**
         * @param array<int, array{method: string, path: string, handler_name: string, handler?: object, websocket?: bool, sse?: bool}>|null $routes
         * @param array<string, mixed>|null $config
         */
        public function __construct(?array $routes = null, ?array $config = null) {}
        /** @param array<string, mixed> $options */
        public function request(string $method, string $path, array $options = []): \Spikard\Response {}
        /** @param array<string, mixed>|null $variables */
        public function graphql(string $query, ?array $variables = null, ?string $operationName = null): \Spikard\Response {}
        /** @param array<string, mixed>|null $variables
         * @return array<int, string|int>
         */
        public function graphqlWithStatus(string $query, ?array $variables = null, ?string $operationName = null): array {}
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
        /** @return array<string, mixed> */
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
        /** @return array<int, array<string, mixed>> */
        public function eventsAsJson(): array {}
        public function body(): string {}
        public function count(): int {}
    }

    class SseEvent
    {
        public function getData(): string {}
        /** @return array<string, mixed> */
        public function asJson(): array {}
        public function getEventType(): ?string {}
        public function getId(): ?string {}
    }
}
