<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * API Key authentication configuration.
 *
 * Configures API key authentication middleware for the server.
 */
final class ApiKeyConfig
{
    /**
     * @param list<string> $keys Valid API keys
     * @param string $headerName Header name to check (e.g., "X-API-Key")
     */
    public function __construct(
        public readonly array $keys,
        public readonly string $headerName = 'X-API-Key',
    ) {
    }
}
