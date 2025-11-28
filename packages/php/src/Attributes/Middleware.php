<?php

declare(strict_types=1);

namespace Spikard\Attributes;

use Attribute;

/**
 * Middleware attribute for route-level middleware.
 *
 * This attribute can be applied to controller methods to add route-specific middleware.
 * Multiple Middleware attributes can be stacked on the same method.
 *
 * Example:
 * ```php
 * use Spikard\Attributes\{Get, Middleware};
 *
 * class UserController {
 *     #[Get('/admin/users')]
 *     #[Middleware(AuthMiddleware::class)]
 *     #[Middleware(AdminMiddleware::class)]
 *     public function adminList(): array {
 *         return ['users' => []];
 *     }
 * }
 * ```
 */
#[Attribute(Attribute::TARGET_METHOD | Attribute::IS_REPEATABLE)]
class Middleware
{
    /**
     * @param class-string $middleware Middleware class name
     * @param array<string, mixed> $options Optional middleware options
     */
    public function __construct(
        public readonly string $middleware,
        public readonly array $options = [],
    ) {
    }
}
