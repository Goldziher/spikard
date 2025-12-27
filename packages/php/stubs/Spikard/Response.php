<?php

declare(strict_types=1);

namespace Spikard;

/**
 * HTTP Response object exposed to PHP.
 *
 * Provides access to HTTP response status, headers, and body,
 * with convenient methods for JSON parsing and GraphQL extraction.
 */
class Response
{
    /**
     * Create a new Response.
     *
     * @param string $body Response body content
     * @param int $status HTTP status code (default 200)
     * @param array<string, string> $headers Response headers (default empty)
     */
    public function __construct(string $body, int $status = 200, array $headers = [])
    {
    }

    /**
     * Get the HTTP status code.
     */
    public function getStatus(): int
    {
    }

    /**
     * Alias for getStatus().
     */
    public function getStatusCode(): int
    {
    }

    /**
     * Get the response body as a string.
     */
    public function getBody(): string
    {
    }

    /**
     * Get the response body parsed as JSON.
     *
     * @return array<string, mixed>
     */
    public function json(): array
    {
    }

    /**
     * Get response headers as a PHP array.
     *
     * @return array<string, string>
     */
    public function getHeaders(): array
    {
    }

    /**
     * Get a specific header value (case-insensitive).
     */
    public function getHeader(string $name): ?string
    {
    }

    /**
     * Check if response was successful (2xx status).
     */
    public function isSuccess(): bool
    {
    }

    /**
     * Check if response was a redirect (3xx status).
     */
    public function isRedirect(): bool
    {
    }

    /**
     * Check if response was a client error (4xx status).
     */
    public function isClientError(): bool
    {
    }

    /**
     * Check if response was a server error (5xx status).
     */
    public function isServerError(): bool
    {
    }

    /**
     * Extract GraphQL data from response.
     *
     * @return array<string, mixed>
     *
     * @throws \Exception if response has no 'data' field
     */
    public function graphqlData(): array
    {
    }

    /**
     * Extract GraphQL errors from response.
     *
     * @return array<int, array<string, mixed>>
     */
    public function graphqlErrors(): array
    {
    }
}
