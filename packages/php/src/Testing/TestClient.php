<?php

declare(strict_types=1);

namespace Spikard\Testing;

use RuntimeException;
use Spikard\App;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Handlers\HandlerInterface;

final class TestClient
{
    private bool $usedNative = false;

    private function __construct(private readonly App $app)
    {
        $this->assertNativeAvailable();
    }

    public static function create(App $app): self
    {
        return new self($app);
    }

    public function app(): App
    {
        return $this->app;
    }

    /**
     * @param array<string, mixed> $options
     */
    public function request(string $method, string $path, array $options = []): Response
    {
        $this->assertNativeAvailable();

        $headers = \is_array($options['headers'] ?? null) ? $options['headers'] : [];
        $cookies = \is_array($options['cookies'] ?? null) ? $options['cookies'] : [];
        $files = \is_array($options['files'] ?? null) ? $options['files'] : [];
        $queryParams = $this->parseQueryParams($path);
        $pathOnly = \explode('?', $path, 2)[0];
        $bodyPayload = $options['body'] ?? null;
        $rawBody = \is_string($bodyPayload)
            ? $bodyPayload
            : ((\is_scalar($bodyPayload) && !\is_bool($bodyPayload)) ? (string) $bodyPayload : null);

        if (\method_exists(Request::class, 'fromHttp')) {
            $request = Request::fromHttp(
                $method,
                $path,
                [
                    'body' => $bodyPayload,
                    'headers' => $headers,
                    'cookies' => $cookies,
                    'files' => $files,
                    'pathParams' => [],
                    'dependencies' => null,
                ]
            );
        } elseif (\class_exists(\Spikard\Internal\Request::class)) {
            /** @psalm-suppress MixedMethodCall runtime-provided constructor */
            $request = new Request(
                \strtoupper($method),
                $pathOnly,
                $bodyPayload,
                $rawBody,
                $headers,
                $cookies,
                $queryParams,
                []
            );
        } else {
            $request = new Request(
                method: \strtoupper($method),
                path: $pathOnly,
                body: $bodyPayload,
                headers: $headers,
                cookies: $cookies,
                queryParams: $queryParams,
                pathParams: [],
                files: $files,
                rawBody: $rawBody,
                rawQueryParams: $queryParams,
                dependencies: null,
            );
        }

        $normalized = $this->normalizeRequest($request);
        $route = $this->app->resolveRoute($normalized);
        if ($route === null) {
            throw new RuntimeException(\sprintf('No handler registered for %s %s', $method, $path));
        }
        $handler = $route['handler'];

        $this->usedNative = true;
        $native = \Spikard\Testing\NativeTestClient::new();
        $bridge = new class ($handler, $this) {
            public function __construct(private HandlerInterface $handler, private TestClient $client) {}
            public function handle(object $req): Response
            {
                $normalized = $this->client->normalizeRequest($req);
                return $this->handler->handle($normalized);
            }
        };

        $queryString = \parse_url($path, PHP_URL_QUERY);
        $hooksPayload = $this->app->lifecycleHooksPayload();
        try {
            $nativeResponse = $native->request(
                \strtoupper($method),
                $pathOnly,
                [$bridge, 'handle'],
                $bodyPayload,
                \is_string($queryString) ? $queryString : null,
                $headers,
                $route['request_schema'] ?? null,
                $route['parameter_schema'] ?? null,
                $hooksPayload,
                $route['path'] ?? null,
                $route['websocket'] ?? false,
                $route['sse'] ?? false
            );
        } catch (\Throwable $exception) {
            return Response::json([
                'error' => $exception->getMessage(),
                'code' => 'panic',
                'details' => new \stdClass(),
            ], 500);
        }

        $responseBody = null;
        try {
            $maybeJson = $nativeResponse->json();
            if (\is_array($maybeJson)) {
                $responseBody = $maybeJson;
            }
        } catch (\Throwable) {
            // Fall back to raw body below.
        }

        if ($responseBody === null) {
            $responseBody = $nativeResponse->getBody();
        }

        $nativeHeaders = $nativeResponse->getHeaders();
        return new Response(
            body: $responseBody,
            statusCode: $nativeResponse->getStatus(),
            headers: \is_array($nativeHeaders) ? $nativeHeaders : [],
            cookies: []
        );
    }

    /**
     * Connect to a WebSocket route (native path only).
     */
    public function connectWebSocket(string $path, ?string $sendText = null): object
    {
        $this->assertNativeAvailable();
        $this->usedNative = true;
        $handler = $this->app->websocketHandlers()[$path] ?? null;
        if ($handler === null) {
            throw new RuntimeException('No handler registered for WebSocket route.');
        }
        $native = \Spikard\Testing\HttpTestClient::new();
        /** @phpstan-ignore-next-line runtime extension method */
        $connection = $native->websocket($path, $handler);
        if ($sendText !== null) {
            /** @phpstan-ignore-next-line runtime extension method */
            $connection->sendText($sendText);
        }
        return $connection;
    }

    /**
     * Connect to an SSE route and retrieve the event stream (native path only).
     */
    public function connectSse(string $path): object
    {
        $this->assertNativeAvailable();
        $this->usedNative = true;
        $producer = $this->app->sseProducers()[$path] ?? null;
        if ($producer === null) {
            throw new RuntimeException('No SSE producer registered for route.');
        }
        $native = \Spikard\Testing\HttpTestClient::new();
        /** @phpstan-ignore-next-line runtime extension method */
        return $native->sse($path, $producer);
    }

    public function get(string $path): Response
    {
        return $this->request('GET', $path);
    }

    public function post(string $path, mixed $body = null): Response
    {
        return $this->request('POST', $path, ['body' => $body]);
    }

    public function usedNativeClient(): bool
    {
        return $this->usedNative;
    }

    public function close(): void
    {
        $this->app->close();
    }

    /**
     * @return array<string, array<int, string>>
     */
    private function parseQueryParams(string $path): array
    {
        $query = \parse_url($path, PHP_URL_QUERY);
        if (!\is_string($query) || $query === '') {
            return [];
        }

        $result = [];
        foreach (\explode('&', $query) as $pair) {
            if ($pair === '') {
                continue;
            }
            [$rawKey, $rawValue] = \array_pad(\explode('=', $pair, 2), 2, '');
            $key = \urldecode($rawKey);
            if ($key === '') {
                continue;
            }
            $value = \urldecode($rawValue);
            $result[$key] ??= [];
            $result[$key][] = $value;
        }

        return $result;
    }

    /**
     * Normalize any request-like object to Spikard\Http\Request.
     */
    public function normalizeRequest(object $request): Request
    {
        if ($request instanceof Request) {
            return $request;
        }

        $class = $request::class;
        // Best-effort mapping from internal request to generated DTO.
        if (\method_exists($request, 'getMethod') && \method_exists($request, 'getPath')) {
            $method = (string) $request->getMethod();
            $path = (string) $request->getPath();
            $body = null;
            if (\method_exists($request, 'getBody')) {
                $raw = $request->getBody();
                $decoded = \json_decode((string) $raw, true);
                $body = $decoded !== null ? $decoded : $raw;
            }

            $headers = \method_exists($request, 'getHeaders') ? (array) $request->getHeaders() : [];
            $cookies = \method_exists($request, 'getCookies') ? (array) $request->getCookies() : [];
            $queryParams = \method_exists($request, 'getQueryParams') ? (array) $request->getQueryParams() : [];
            $pathParams = \method_exists($request, 'getPathParams') ? (array) $request->getPathParams() : [];
            $rawBody = \method_exists($request, 'getRawBody') ? $request->getRawBody() : null;

            return new Request(
                method: $method,
                path: $path,
                body: $body,
                headers: $headers,
                cookies: $cookies,
                queryParams: $queryParams,
                pathParams: $pathParams,
                files: [],
                rawBody: \is_string($rawBody) ? $rawBody : null,
                rawQueryParams: $queryParams,
                dependencies: null,
            );
        }

        throw new RuntimeException(
            \sprintf('Unsupported request type %s; expected Spikard\Http\Request-compatible object.', $class)
        );
    }

    private function assertNativeAvailable(): void
    {
        if (!\class_exists('\\Spikard\\Testing\\NativeTestClient', false) || !\function_exists('spikard_start_server')) {
            throw new RuntimeException('Spikard PHP extension is required for testing.');
        }
    }
}
