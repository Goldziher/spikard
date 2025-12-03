<?php

declare(strict_types=1);

namespace Spikard\Generated;

use Spikard\DI\ResolvedDependencies;

final class Request
{
    public function __construct(
        public readonly string $method,
        public readonly string $path,
        public readonly mixed $body,
        /** @var array<string, string> */

        public readonly array $headers = [],
        /** @var array<string, string> */

        public readonly array $cookies = [],
        /** @var array<string, array<int, string>> */

        public readonly array $queryParams = [],
        /** @var array<string, string> */

        public readonly array $pathParams = [],
        /** @var array<string, mixed> */

        public readonly array $files = [],
        public readonly ?string $rawBody = null,
        /** @var array<string, array<int, string>> */

        public readonly ?array $rawQueryParams = null,
        /** @var ResolvedDependencies|null */

        public readonly ?ResolvedDependencies $dependencies = null,
    ) {
    }

    /** @param array<string, mixed> $options */
    public static function fromHttp(string $method, string $path, array $options = []): self
    {
        $headers = self::normalizeStringMap($options['headers'] ?? []);
        $cookies = self::normalizeStringMap($options['cookies'] ?? []);
        $files = self::normalizeMixedMap($options['files'] ?? []);
        $queryParams = self::parseQueryParams($path);
        $pathOnly = \explode('?', $path, 2)[0];
        $body = $options['body'] ?? null;

        if ($body === null && $files !== []) {
            $body = $files;
        }

        $rawBody = \is_string($body)
            ? $body
            : ((\is_scalar($body) && !\is_bool($body)) ? (string) $body : null);

        return new self(
            method: \strtoupper($method),
            path: $pathOnly,
            body: $body,
            headers: $headers,
            cookies: $cookies,
            queryParams: $queryParams,
            pathParams: self::normalizeStringMap($options['pathParams'] ?? []),
            files: $files,
            rawBody: $rawBody,
            rawQueryParams: $queryParams,
            dependencies: $options['dependencies'] ?? null,
        );
    }

    public function query(string $name): ?string
    {
        $values = $this->queryParams[$name] ?? $this->rawQueryParams[$name] ?? null;
        if (\is_array($values)) {
            foreach ($values as $value) {
                if (\is_string($value)) {
                    return $value;
                }
            }
        }

        return null;
    }

    /** @return array<string, string> */
    private static function normalizeStringMap(mixed $input): array
    {
        if (!\is_array($input)) {
            return [];
        }

        $normalized = [];
        foreach ($input as $key => $value) {
            if (!\is_string($key) || (!\is_string($value) && !\is_numeric($value))) {
                continue;
            }
            $normalized[$key] = (string) $value;
        }

        return $normalized;
    }

    /** @return array<string, mixed> */
    private static function normalizeMixedMap(mixed $input): array
    {
        if (!\is_array($input)) {
            return [];
        }

        $normalized = [];
        foreach ($input as $key => $value) {
            if (!\is_string($key)) {
                continue;
            }
            $normalized[$key] = $value;
        }

        return $normalized;
    }

    /** @return array<string, array<int, string>> */
    private static function parseQueryParams(string $path): array
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

            [$rawKey, $rawValue] = \array_pad(\explode('=', $pair, 2), 2, '');
            $key = \urldecode($rawKey);
            $value = \urldecode($rawValue);

            if ($key === '') {
                continue;
            }

            if (!\array_key_exists($key, $result)) {
                $result[$key] = [];
            }

            $result[$key][] = $value;
        }

        return $result;
    }
}
