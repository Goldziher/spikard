<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * POST route attribute for HTTP endpoints.
 *
 * This attribute can be applied to controller methods to define POST routes.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\Post;
 * use Spikard\Http\Params\Body;
 *
 * class UserController {
 *     #[Post('/users')]
 *     public function create(#[Body] array $data): array {
 *         return ['user' => $data];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Post extends Route
{
    /**
     * @param string $path Route path (e.g., '/users')
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
        parent::__construct('POST', $path, $middleware, $name, $requestSchema, $responseSchema, $parameterSchema);
    }
}
