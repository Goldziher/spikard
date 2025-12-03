<?php

declare(strict_types=1);

namespace Spikard\Generated;

final class Response
{
    public function __construct(
        public readonly mixed $body = null,
        public readonly int $statusCode = 200,
        /** @var array<string, string> */

        public readonly array $headers = [],
        /** @var array<string, string> */

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
        return new self(
            body: $this->body,
            statusCode: $this->statusCode,
            headers: $this->headers,
            cookies: $cookies
        );
    }

    public function getStatus(): int
    {
        return $this->statusCode;
    }

    public function getStatusCode(): int
    {
        return $this->statusCode;
    }

    public function getBody(): string
    {
        if (\is_string($this->body)) {
            return $this->body;
        }

        return (string) \json_encode($this->body);
    }

    /** @return array<string, string> */
    public function getHeaders(): array
    {
        return $this->headers;
    }

    /**
     * Convenience accessor to decode JSON body when returned as a string.
     *
     * @return array<string, mixed>|null
     */
    public function jsonBody(): ?array
    {
        if (\is_array($this->body)) {
            return $this->body;
        }

        if (\is_string($this->body)) {
            $decoded = \json_decode($this->body, true);
            if (\is_array($decoded)) {
                return $decoded;
            }
        }

        return null;
    }

    public function __call(string $name, array $args): mixed
    {
        if ($name === 'json') {
            return $this->jsonBody();
        }

        throw new \BadMethodCallException('Undefined method ' . __CLASS__ . '::' . $name);
    }
}
