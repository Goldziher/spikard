<?php

declare(strict_types=1);

namespace Spikard\Http;

// When the native extension is loaded, the Response class is provided by the extension.
if (\class_exists(__NAMESPACE__ . '\Response', false)) {
    return;
}

final class Response
{
    /**
     * @param array<string, string> $headers
     * @param array<string, string> $cookies
     */
    public function __construct(
        public readonly mixed $body = null,
        public readonly int $statusCode = 200,
        public readonly array $headers = [],
        public readonly array $cookies = [],
    ) {
    }

    /** @param array<string, string> $headers */
    public static function json(mixed $data, int $status = 200, array $headers = []): self
    {
        $mergedHeaders = \array_merge(['Content-Type' => 'application/json'], $headers);
        return new self(body: $data, statusCode: $status, headers: $mergedHeaders);
    }

    /** @param array<string, string> $headers */
    public static function text(string $body, int $status = 200, array $headers = []): self
    {
        $mergedHeaders = \array_merge(['Content-Type' => 'text/plain; charset=utf-8'], $headers);
        return new self(body: $body, statusCode: $status, headers: $mergedHeaders);
    }

    /** @param array<string, string> $cookies */
    public function withCookies(array $cookies): self
    {
        return new self(body: $this->body, statusCode: $this->statusCode, headers: $this->headers, cookies: $cookies);
    }

    public function getStatus(): int
    {
        return $this->statusCode;
    }

    public function getStatusCode(): int
    {
        return $this->statusCode;
    }

    /** @return array<string, string> */
    public function getHeaders(): array
    {
        return $this->headers;
    }

    /** @return array<string, string> */
    public function getCookies(): array
    {
        return $this->cookies;
    }

    public function getBody(): string
    {
        if (\is_string($this->body)) {
            return $this->body;
        }

        $encoded = \json_encode($this->body);
        return $encoded === false ? '' : $encoded;
    }

    /**
     * Check if response was successful (2xx status).
     */
    public function isSuccess(): bool
    {
        return $this->statusCode >= 200 && $this->statusCode < 300;
    }

    /**
     * Check if response was a redirect (3xx status).
     */
    public function isRedirect(): bool
    {
        return $this->statusCode >= 300 && $this->statusCode < 400;
    }

    /**
     * Check if response was a client error (4xx status).
     */
    public function isClientError(): bool
    {
        return $this->statusCode >= 400 && $this->statusCode < 500;
    }

    /**
     * Check if response was a server error (5xx status).
     */
    public function isServerError(): bool
    {
        return $this->statusCode >= 500 && $this->statusCode < 600;
    }

    /**
     * Parse response body as JSON.
     *
     * @return array<string, mixed>
     *
     * @throws \Exception if JSON is invalid
     */
    public function parseJson(): array
    {
        $body = $this->getBody();
        /** @var array<string, mixed>|null $decoded */
        $decoded = \json_decode($body, true);
        if (!\is_array($decoded)) {
            throw new \Exception('Invalid JSON body: ' . \json_last_error_msg());
        }
        return $decoded;
    }

    /**
     * Extract GraphQL data from response.
     *
     * @return array<string, mixed>
     *
     * @throws \Exception if response has no 'data' field or JSON is invalid
     */
    public function graphqlData(): array
    {
        $data = $this->parseJson();
        if (!\array_key_exists('data', $data)) {
            throw new \Exception("No 'data' field in GraphQL response");
        }
        /** @var array<string, mixed> $graphqlData */
        $graphqlData = $data['data'];
        return $graphqlData;
    }

    /**
     * Extract GraphQL errors from response.
     *
     * @return array<int, array<string, mixed>>
     *
     * @throws \Exception if JSON is invalid
     */
    public function graphqlErrors(): array
    {
        $data = $this->parseJson();
        if (!\array_key_exists('errors', $data)) {
            return [];
        }
        /** @var array<int, array<string, mixed>> $errors */
        $errors = \is_array($data['errors']) ? $data['errors'] : [];
        return $errors;
    }

    /**
     * Get a specific header value (case-insensitive).
     */
    public function getHeader(string $name): ?string
    {
        $name_lower = \strtolower($name);
        foreach ($this->headers as $key => $value) {
            if (\strtolower($key) === $name_lower) {
                return $value;
            }
        }
        return null;
    }
}
