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
}
