<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * DELETE route attribute for HTTP endpoints.
 *
 * This attribute can be applied to controller methods to define DELETE routes.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\Delete;
 *
 * class UserController {
 *     #[Delete('/users/:id')]
 *     public function delete(string $id): array {
 *         return ['deleted' => true, 'id' => $id];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Delete extends Route
{
    /**
     * @param string $path Route path (e.g., '/users/:id')
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
        parent::__construct('DELETE', $path, $middleware, $name, $requestSchema, $responseSchema, $parameterSchema);
    }
}
