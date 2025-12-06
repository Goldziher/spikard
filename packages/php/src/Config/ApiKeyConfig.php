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

    public static function builder(): ApiKeyConfigBuilder
    {
        return new ApiKeyConfigBuilder();
    }
}

/**
 * Builder for ApiKeyConfig.
 *
 * Provides a fluent interface for constructing ApiKeyConfig instances.
 */
final class ApiKeyConfigBuilder
{
    /** @var list<string> */
    private array $keys = [];
    private string $headerName = 'X-API-Key';

    /**
     * @param list<string> $keys
     */
    public function withKeys(array $keys): self
    {
        $this->keys = $keys;
        return $this;
    }

    public function withHeaderName(string $headerName): self
    {
        $this->headerName = $headerName;
        return $this;
    }

    public function build(): ApiKeyConfig
    {
        return new ApiKeyConfig(
            keys: $this->keys,
            headerName: $this->headerName,
        );
    }
}
