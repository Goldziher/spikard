<?php

declare(strict_types=1);

namespace Spikard\Config;

use Spikard\Config\LifecycleHooks;

/**
 * Server configuration.
 *
 * Configures all aspects of the Spikard HTTP server including host/port binding,
 * middleware settings, authentication, and lifecycle hooks.
 */
final class ServerConfig
{
    /**
     * @param string $host Host to bind to
     * @param int $port Port to bind to
     * @param int $workers Number of worker threads
     * @param bool $enableRequestId Enable request ID generation and propagation
     * @param int|null $maxBodySize Maximum request body size in bytes (null = unlimited, not recommended)
     * @param int|null $requestTimeout Request timeout in seconds (null = no timeout)
     * @param bool $gracefulShutdown Enable graceful shutdown on SIGTERM/SIGINT
     * @param int $shutdownTimeout Graceful shutdown timeout (seconds)
     * @param CompressionConfig|null $compression Enable compression middleware
     * @param RateLimitConfig|null $rateLimit Enable rate limiting
     * @param CorsConfig|null $cors CORS configuration
     * @param StaticFilesConfig|null $staticFiles Static file serving configuration
     * @param JwtConfig|null $jwtAuth JWT authentication configuration
     * @param ApiKeyConfig|null $apiKeyAuth API Key authentication configuration
     * @param OpenApiConfig|null $openapi OpenAPI documentation configuration
     * @param LifecycleHooks|null $hooks Lifecycle hooks for request/response processing
     */
    public function __construct(
        public readonly string $host = '127.0.0.1',
        public readonly int $port = 8000,
        public readonly int $workers = 1,
        public readonly bool $enableRequestId = true,
        public readonly ?int $maxBodySize = 10485760, // 10 MB
        public readonly ?int $requestTimeout = 30,
        public readonly bool $gracefulShutdown = true,
        public readonly int $shutdownTimeout = 30,
        public readonly ?CompressionConfig $compression = null,
        public readonly ?RateLimitConfig $rateLimit = null,
        public readonly ?CorsConfig $cors = null,
        public readonly ?StaticFilesConfig $staticFiles = null,
        public readonly ?JwtConfig $jwtAuth = null,
        public readonly ?ApiKeyConfig $apiKeyAuth = null,
        public readonly ?OpenApiConfig $openapi = null,
        public readonly ?LifecycleHooks $hooks = null,
    ) {
    }

    public static function builder(): ServerConfigBuilder
    {
        return new ServerConfigBuilder();
    }
}

/**
 * Builder for ServerConfig.
 *
 * Provides a fluent interface for constructing ServerConfig instances.
 */
final class ServerConfigBuilder
{
    private string $host = '127.0.0.1';
    private int $port = 8000;
    private int $workers = 1;
    private bool $enableRequestId = true;
    private ?int $maxBodySize = 10485760; // 10 MB
    private ?int $requestTimeout = 30;
    private bool $gracefulShutdown = true;
    private int $shutdownTimeout = 30;
    private ?CompressionConfig $compression = null;
    private ?RateLimitConfig $rateLimit = null;
    private ?CorsConfig $cors = null;
    private ?StaticFilesConfig $staticFiles = null;
    private ?JwtConfig $jwtAuth = null;
    private ?ApiKeyConfig $apiKeyAuth = null;
    private ?OpenApiConfig $openapi = null;
    private ?LifecycleHooks $hooks = null;

    public function withHost(string $host): self
    {
        $this->host = $host;
        return $this;
    }

    public function withPort(int $port): self
    {
        $this->port = $port;
        return $this;
    }

    public function withWorkers(int $workers): self
    {
        $this->workers = $workers;
        return $this;
    }

    public function withRequestId(bool $enable): self
    {
        $this->enableRequestId = $enable;
        return $this;
    }

    public function withMaxBodySize(?int $size): self
    {
        $this->maxBodySize = $size;
        return $this;
    }

    public function withRequestTimeout(?int $timeout): self
    {
        $this->requestTimeout = $timeout;
        return $this;
    }

    public function withGracefulShutdown(bool $enable): self
    {
        $this->gracefulShutdown = $enable;
        return $this;
    }

    public function withShutdownTimeout(int $timeout): self
    {
        $this->shutdownTimeout = $timeout;
        return $this;
    }

    public function withCompression(CompressionConfig $config): self
    {
        $this->compression = $config;
        return $this;
    }

    public function withRateLimit(RateLimitConfig $config): self
    {
        $this->rateLimit = $config;
        return $this;
    }

    public function withCors(CorsConfig $config): self
    {
        $this->cors = $config;
        return $this;
    }

    public function withStaticFiles(StaticFilesConfig $config): self
    {
        $this->staticFiles = $config;
        return $this;
    }

    public function withJwtAuth(JwtConfig $config): self
    {
        $this->jwtAuth = $config;
        return $this;
    }

    public function withApiKeyAuth(ApiKeyConfig $config): self
    {
        $this->apiKeyAuth = $config;
        return $this;
    }

    public function withOpenApi(OpenApiConfig $config): self
    {
        $this->openapi = $config;
        return $this;
    }

    public function withLifecycleHooks(LifecycleHooks $hooks): self
    {
        $this->hooks = $hooks;
        return $this;
    }

    public function build(): ServerConfig
    {
        return new ServerConfig(
            host: $this->host,
            port: $this->port,
            workers: $this->workers,
            enableRequestId: $this->enableRequestId,
            maxBodySize: $this->maxBodySize,
            requestTimeout: $this->requestTimeout,
            gracefulShutdown: $this->gracefulShutdown,
            shutdownTimeout: $this->shutdownTimeout,
            compression: $this->compression,
            rateLimit: $this->rateLimit,
            cors: $this->cors,
            staticFiles: $this->staticFiles,
            jwtAuth: $this->jwtAuth,
            apiKeyAuth: $this->apiKeyAuth,
            openapi: $this->openapi,
            hooks: $this->hooks,
        );
    }
}
