<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * PUT route attribute for HTTP endpoints.
 *
 * This attribute can be applied to controller methods to define PUT routes.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\Put;
 * use Spikard\Http\Params\Body;
 *
 * class UserController {
 *     #[Put('/users/:id')]
 *     public function update(string $id, #[Body] array $data): array {
 *         return ['user' => array_merge($data, ['id' => $id])];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Put extends Route
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
        parent::__construct('PUT', $path, $middleware, $name, $requestSchema, $responseSchema, $parameterSchema);
    }
}
