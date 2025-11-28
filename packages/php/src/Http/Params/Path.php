<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Path parameter metadata
 *
 * Note: Path parameters are typically required and don't use defaults,
 * but this class is provided for API consistency.
 *
 * Examples:
 * ```php
 * use Spikard\Http\Params\Path;
 *
 * $app->get('/users/{id}', function(
 *     int $id = new Path()
 * ) {
 *     return User::find($id);
 * });
 *
 * // With custom validation schema
 * $app->get('/users/{id}', function(
 *     int $id = new Path(schema: ['minimum' => 1])
 * ) {
 *     return User::find($id);
 * });
 * ```
 *
 * @template T
 * @extends ParamBase<T>
 */
final class Path extends ParamBase
{
    /**
     * @param T|null $default Static default value (rarely used for path params)
     * @param Closure|null $defaultFactory Callable that generates default value (rarely used)
     * @param array<string, mixed>|null $schema Optional JSON schema dict for custom validation
     */
    public function __construct(
        mixed $default = null,
        ?Closure $defaultFactory = null,
        ?array $schema = null,
    ) {
        parent::__construct($default, $defaultFactory, $schema);
    }
}
