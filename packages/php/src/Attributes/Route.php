<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * Base route attribute for HTTP endpoints.
 *
 * This attribute can be applied to controller methods to define HTTP routes.
 * It serves as the foundation for HTTP method-specific attributes (Get, Post, etc.).
 *
 * Example:
 * ```php
 * use Spikard\Attributes\Route;
 *
 * class UserController {
 *     #[Route('GET', '/users')]
 *     public function list(): array {
 *         return ['users' => []];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD)]
class Route
{
    /**
     * @param string $method HTTP method (GET, POST, PUT, DELETE, PATCH, etc.)
     * @param string $path Route path (e.g., '/users', '/users/:id')
     * @param array<int, class-string|callable> $middleware Route-level middleware
     * @param string|null $name Optional route name for reverse routing
     * @param array<string, mixed>|null $requestSchema Optional JSON schema for request validation
     * @param array<string, mixed>|null $responseSchema Optional JSON schema for response validation
     * @param array<string, mixed>|null $parameterSchema Optional JSON schema for parameter validation
     */
    public function __construct(
        public readonly string $method,
        public readonly string $path,
        public readonly array $middleware = [],
        public readonly ?string $name = null,
        public readonly ?array $requestSchema = null,
        public readonly ?array $responseSchema = null,
        public readonly ?array $parameterSchema = null,
    ) {
    }
}
