<?php

declare(strict_types=1);

namespace Spikard\Testing;

use RuntimeException;
use Spikard\App;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class TestClient
{
    private function __construct(private readonly App $app)
    {
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
        $handler = $this->app->findHandler($method, $path);
        if ($handler === null) {
            throw new RuntimeException(\sprintf('No handler registered for %s %s', $method, $path));
        }

        /** @var array<string, string> $headers */
        $headers = \is_array($options['headers'] ?? null) ? $options['headers'] : [];
        /** @var array<string, string> $cookies */
        $cookies = \is_array($options['cookies'] ?? null) ? $options['cookies'] : [];
        $body = $options['body'] ?? null;
        $queryParams = $this->parseQueryParams($path);

        $request = new Request(
            method: \strtoupper($method),
            path: $path,
            body: $body,
            headers: $headers,
            cookies: $cookies,
            queryParams: $queryParams,
            pathParams: [],
            dependencies: null,
        );

        return $handler->handle($request);
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
        // placeholder for resource cleanup once HTTP runtime is wired
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
        \parse_str($parsed, $output);
        foreach ($output as $key => $value) {
            if (\is_array($value)) {
                $coerced = [];
                foreach ($value as $item) {
                    if (\is_scalar($item) || $item === null) {
                        $coerced[] = (string) $item;
                    }
                }
                $result[(string) $key] = $coerced;
            } elseif (\is_scalar($value)) {
                $result[(string) $key] = [(string) $value];
            }
        }

        return $result;
    }
}
