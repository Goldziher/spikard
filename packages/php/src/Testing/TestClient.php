<?php

declare(strict_types=1);

namespace Spikard\Testing;

use RuntimeException;
use Spikard\App;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class TestClient
{
    private ?\Spikard\Native\TestClient $native = null;

    private function __construct(private readonly App $app)
    {
    }

    public static function create(App $app): self
    {
        return new self($app);
    }

    private function useNative(): bool
    {
        if (\getenv('SPIKARD_TEST_CLIENT_FORCE_PHP') === '1') {
            return false;
        }
        return \class_exists('\\Spikard\\Native\\TestClient') && \function_exists('spikard_version');
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
        if ($this->useNative()) {
            $nativeResponse = $this->nativeClient()->request($method, $path, $options);
            $headers = \method_exists($nativeResponse, 'getHeaders')
                ? $nativeResponse->getHeaders()
                : ($nativeResponse->headers ?? []);
            $cookies = \method_exists($nativeResponse, 'getCookies')
                ? $nativeResponse->getCookies()
                : ($nativeResponse->cookies ?? []);
            $statusCode = \method_exists($nativeResponse, 'getStatusCode')
                ? $nativeResponse->getStatusCode()
                : ($nativeResponse->statusCode ?? 200);
            $body = null;
            if (\method_exists($nativeResponse, 'getBody')) {
                $bodyString = $nativeResponse->getBody();
                if ($bodyString !== '') {
                    if ($this->isJsonResponse($headers)) {
                        $decoded = \json_decode($bodyString, true);
                        $body = $decoded !== null || \json_last_error() === \JSON_ERROR_NONE
                            ? $decoded
                            : $bodyString;
                    } else {
                        $body = $bodyString;
                    }
                }
            } else {
                $body = $nativeResponse->body ?? null;
            }

            return new Response(
                body: $body,
                statusCode: $statusCode,
                headers: $headers,
                cookies: $cookies,
            );
        }

        /** @var array<string, string> $headers */
        $headers = \is_array($options['headers'] ?? null) ? $options['headers'] : [];
        /** @var array<string, string> $cookies */
        $cookies = \is_array($options['cookies'] ?? null) ? $options['cookies'] : [];
        /** @var array<string, mixed> $files */
        $files = \is_array($options['files'] ?? null) ? $options['files'] : [];
        $queryParams = $this->parseQueryParams($path);
        $pathOnly = \explode('?', $path, 2)[0];
        $body = $options['body'] ?? null;
        if ($body === null && $files !== []) {
            $body = $files;
        }
        $validatedParams = null;
        $dependencies = null;

        $request = new Request(
            method: \strtoupper($method),
            path: $pathOnly,
            body: $body,
            headers: $headers,
            cookies: $cookies,
            queryParams: $queryParams,
            pathParams: [],
            validatedParams: $validatedParams,
            files: $files,
            dependencies: $dependencies,
        );

        $handler = $this->app->findHandler($request);
        if ($handler === null) {
            throw new RuntimeException(\sprintf('No handler registered for %s %s', $method, $path));
        }

        return $handler->handle($request);
    }

    /**
     * Connect to a WebSocket route (native path only).
     */
    public function connectWebSocket(string $path, ?string $sendText = null): object
    {
        if (!$this->useNative()) {
            throw new RuntimeException('WebSocket client requires the native extension.');
        }
        return $this->nativeClient()->websocket($path, $sendText);
    }

    /**
     * Connect to an SSE route and retrieve the event stream (native path only).
     */
    public function connectSse(string $path): object
    {
        if (!$this->useNative()) {
            throw new RuntimeException('SSE client requires the native extension.');
        }
        return $this->nativeClient()->sse($path);
    }

    public function get(string $path): Response
    {
        return $this->request('GET', $path);
    }

    public function post(string $path, mixed $body = null): Response
    {
        return $this->request('POST', $path, ['body' => $body]);
    }

    public function close(): void
    {
        if ($this->native !== null) {
            $this->native->close();
            $this->native = null;
        }
        $this->app->close();
    }

    private function nativeClient(): \Spikard\Native\TestClient
    {
        if ($this->native === null) {
            $routes = $this->app->nativeRoutes();
            $config = $this->app->nativeConfig();
            $this->native = new \Spikard\Native\TestClient($routes, $config);
        }

        return $this->native;
    }

    /**
     * @return array<string, array<int, string>>
     */
    private function parseQueryParams(string $path): array
    {
        $parsed = \parse_url($path, PHP_URL_QUERY);
        if (!\is_string($parsed) || $parsed === '') {
            return [];
        }

        $result = [];
        foreach (\explode('&', $parsed) as $pair) {
            if ($pair === '') {
                continue;
            }
            [$key, $value] = \array_pad(\explode('=', $pair, 2), 2, '');
            $decodedKey = \urldecode($key);
            $decodedValue = \urldecode($value);
            if ($decodedKey === '') {
                continue;
            }
            if (\array_key_exists($decodedKey, $result)) {
                $result[$decodedKey][] = $decodedValue;
            } else {
                $result[$decodedKey] = [$decodedValue];
            }
        }

        return $result;
    }

    /** @param array<string, string> $headers */
    private function isJsonResponse(array $headers): bool
    {
        foreach ($headers as $key => $value) {
            if (\strtolower($key) === 'content-type') {
                $contentType = \strtolower(\trim(\explode(';', $value, 2)[0]));
                if ($contentType === 'application/json' || $contentType === 'application/problem+json') {
                    return true;
                }
                return \str_ends_with($contentType, '+json');
            }
        }
        return false;
    }
}
