<?php

declare(strict_types=1);

namespace Spikard\Config;

/**
 * OpenAPI documentation configuration.
 *
 * Configures OpenAPI (formerly Swagger) documentation generation and UI serving.
 */
final class OpenApiConfig
{
    /**
     * @param bool $enabled Enable OpenAPI generation (default: false for zero overhead)
     * @param string $title API title
     * @param string $version API version
     * @param string|null $description API description (supports markdown)
     * @param string $swaggerUiPath Path to serve Swagger UI
     * @param string $redocPath Path to serve Redoc
     * @param string $openapiJsonPath Path to serve OpenAPI JSON spec
     */
    public function __construct(
        public readonly bool $enabled = false,
        public readonly string $title = 'API Documentation',
        public readonly string $version = '1.0.0',
        public readonly ?string $description = null,
        public readonly string $swaggerUiPath = '/docs',
        public readonly string $redocPath = '/redoc',
        public readonly string $openapiJsonPath = '/openapi.json',
    ) {
    }

    public static function builder(): OpenApiConfigBuilder
    {
        return new OpenApiConfigBuilder();
    }
}

/**
 * Builder for OpenApiConfig.
 *
 * Provides a fluent interface for constructing OpenApiConfig instances.
 */
final class OpenApiConfigBuilder
{
    private bool $enabled = false;
    private string $title = 'API Documentation';
    private string $version = '1.0.0';
    private ?string $description = null;
    private string $swaggerUiPath = '/docs';
    private string $redocPath = '/redoc';
    private string $openapiJsonPath = '/openapi.json';

    public function withEnabled(bool $enabled): self
    {
        $this->enabled = $enabled;
        return $this;
    }

    public function withTitle(string $title): self
    {
        $this->title = $title;
        return $this;
    }

    public function withVersion(string $version): self
    {
        $this->version = $version;
        return $this;
    }

    public function withDescription(?string $description): self
    {
        $this->description = $description;
        return $this;
    }

    public function withSwaggerUiPath(string $path): self
    {
        $this->swaggerUiPath = $path;
        return $this;
    }

    public function withRedocPath(string $path): self
    {
        $this->redocPath = $path;
        return $this;
    }

    public function withOpenApiJsonPath(string $path): self
    {
        $this->openapiJsonPath = $path;
        return $this;
    }

    public function build(): OpenApiConfig
    {
        return new OpenApiConfig(
            enabled: $this->enabled,
            title: $this->title,
            version: $this->version,
            description: $this->description,
            swaggerUiPath: $this->swaggerUiPath,
            redocPath: $this->redocPath,
            openapiJsonPath: $this->openapiJsonPath,
        );
    }
}
