<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * CORS middleware configuration.
 *
 * Configures Cross-Origin Resource Sharing for the HTTP server.
 */
final class CorsConfig
{
    /**
     * @param bool $enabled Enable CORS middleware
     * @param list<string> $allowedOrigins Allowed origins
     * @param list<string> $allowedMethods Allowed HTTP methods
     * @param list<string> $allowedHeaders Allowed request headers
     * @param list<string> $exposedHeaders Exposed response headers
     * @param bool $allowCredentials Allow credentials in cross-origin requests
     * @param int $maxAgeSeconds Max age for preflight cache (seconds)
     */
    public function __construct(
        public readonly bool $enabled = true,
        public readonly array $allowedOrigins = ['*'],
        public readonly array $allowedMethods = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'],
        public readonly array $allowedHeaders = ['*'],
        public readonly array $exposedHeaders = [],
        public readonly bool $allowCredentials = true,
        public readonly int $maxAgeSeconds = 600,
    ) {
    }

    public static function builder(): CorsConfigBuilder
    {
        return new CorsConfigBuilder();
    }
}

/**
 * Builder for CorsConfig.
 *
 * Provides a fluent interface for constructing CorsConfig instances.
 */
final class CorsConfigBuilder
{
    private bool $enabled = true;
    /** @var list<string> */
    private array $allowedOrigins = ['*'];
    /** @var list<string> */
    private array $allowedMethods = ['GET', 'POST', 'PUT', 'PATCH', 'DELETE', 'OPTIONS'];
    /** @var list<string> */
    private array $allowedHeaders = ['*'];
    /** @var list<string> */
    private array $exposedHeaders = [];
    private bool $allowCredentials = true;
    private int $maxAgeSeconds = 600;

    public function withEnabled(bool $enabled): self
    {
        $this->enabled = $enabled;
        return $this;
    }

    /**
     * @param list<string> $origins
     */
    public function withAllowedOrigins(array $origins): self
    {
        $this->allowedOrigins = $origins;
        return $this;
    }

    /**
     * @param list<string> $methods
     */
    public function withAllowedMethods(array $methods): self
    {
        $this->allowedMethods = $methods;
        return $this;
    }

    /**
     * @param list<string> $headers
     */
    public function withAllowedHeaders(array $headers): self
    {
        $this->allowedHeaders = $headers;
        return $this;
    }

    /**
     * @param list<string> $headers
     */
    public function withExposedHeaders(array $headers): self
    {
        $this->exposedHeaders = $headers;
        return $this;
    }

    public function withAllowCredentials(bool $allow): self
    {
        $this->allowCredentials = $allow;
        return $this;
    }

    public function withMaxAgeSeconds(int $seconds): self
    {
        $this->maxAgeSeconds = $seconds;
        return $this;
    }

    public function build(): CorsConfig
    {
        return new CorsConfig(
            enabled: $this->enabled,
            allowedOrigins: $this->allowedOrigins,
            allowedMethods: $this->allowedMethods,
            allowedHeaders: $this->allowedHeaders,
            exposedHeaders: $this->exposedHeaders,
            allowCredentials: $this->allowCredentials,
            maxAgeSeconds: $this->maxAgeSeconds,
        );
    }
}
