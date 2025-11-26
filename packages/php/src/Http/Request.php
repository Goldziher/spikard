<?php

declare(strict_types=1);

namespace Spikard\Http;

use Spikard\DI\ResolvedDependencies;

final class Request
{
    /**
     * @param array<string, string> $headers
     * @param array<string, string> $cookies
     * @param array<string, array<int, string>> $queryParams
     * @param array<string, string> $pathParams
     * @param array<string, mixed> $files
     */
    public function __construct(
        public readonly string $method,
        public readonly string $path,
        public readonly mixed $body,
        public readonly array $headers = [],
        public readonly array $cookies = [],
        public readonly array $queryParams = [],
        public readonly array $pathParams = [],
        public readonly array $files = [],
        public readonly ?ResolvedDependencies $dependencies = null,
    ) {
    }
}
