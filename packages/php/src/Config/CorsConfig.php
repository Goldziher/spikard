<?php

declare(strict_types=1);

namespace Spikard\Config;

final class CorsConfig
{
    /**
     * @param list<string> $allowedOrigins
     * @param list<string> $allowedMethods
     * @param list<string> $allowedHeaders
     * @param list<string> $exposedHeaders
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
}
