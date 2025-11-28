<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Query parameter with optional default or default factory
 *
 * Use this to specify defaults for query string parameters.
 *
 * Examples:
 * ```php
 * use Spikard\Http\Params\Query;
 *
 * $app->get('/items', function(
 *     array $tags = new Query(defaultFactory: fn() => [])
 * ) {
 *     return ['tags' => $tags];
 * });
 *
 * $app->get('/items', function(
 *     int $limit = new Query(default: 10)
 * ) {
 *     return ['limit' => $limit];
 * });
 *
 * // With custom JSON schema for validation
 * $app->get('/items', function(
 *     int $limit = new Query(
 *         default: 10,
 *         schema: ['minimum' => 1, 'maximum' => 100]
 *     )
 * ) {
 *     return ['limit' => $limit];
 * });
 * ```
 *
 * @template T
 * @extends ParamBase<T>
 */
final class Query extends ParamBase
{
    /**
     * @param T|null $default Static default value (if no defaultFactory provided)
     * @param Closure|null $defaultFactory Callable that generates default value when invoked
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
