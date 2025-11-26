<?php

declare(strict_types=1);

namespace Spikard\Http;

use Spikard\DI\ResolvedDependencies;

// When the native extension is loaded, alias the internal request to this class name.
if (\class_exists(\Spikard\Internal\Request::class)) {
    \class_alias(\Spikard\Internal\Request::class, __NAMESPACE__ . '\Request');
    return;
}

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
