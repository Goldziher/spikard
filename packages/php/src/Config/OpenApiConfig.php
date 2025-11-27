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
}
