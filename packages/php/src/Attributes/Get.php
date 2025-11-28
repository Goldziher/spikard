<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * GET route attribute for HTTP endpoints.
 *
 * This attribute can be applied to controller methods to define GET routes.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\Get;
 *
 * class UserController {
 *     #[Get('/users')]
 *     public function list(): array {
 *         return ['users' => []];
 *     }
 *
 *     #[Get('/users/:id')]
 *     public function get(string $id): array {
 *         return ['user' => ['id' => $id]];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Get extends Route
{
    /**
     * @param string $path Route path (e.g., '/users', '/users/:id')
     * @param array<int, class-string|callable> $middleware Route-level middleware
     * @param string|null $name Optional route name for reverse routing
     * @param array<string, mixed>|null $requestSchema Optional JSON schema for request validation
     * @param array<string, mixed>|null $responseSchema Optional JSON schema for response validation
     * @param array<string, mixed>|null $parameterSchema Optional JSON schema for parameter validation
     */
    public function __construct(
        string $path,
        array $middleware = [],
        ?string $name = null,
        ?array $requestSchema = null,
        ?array $responseSchema = null,
        ?array $parameterSchema = null,
    ) {
        parent::__construct('GET', $path, $middleware, $name, $requestSchema, $responseSchema, $parameterSchema);
    }
}
