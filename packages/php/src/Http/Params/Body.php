<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Request body parameter with optional default or default factory
 *
 * Use this to specify defaults for request body parameters.
 *
 * Examples:
 * ```php
 * use Spikard\Http\Params\Body;
 *
 * $app->post('/items', function(
 *     array $data = new Body(defaultFactory: fn() => [])
 * ) {
 *     return $data;
 * });
 *
 * // With custom JSON schema for validation
 * $app->post('/items', function(
 *     array $data = new Body(
 *         schema: [
 *             'type' => 'object',
 *             'required' => ['name', 'price'],
 *             'properties' => [
 *                 'name' => ['type' => 'string'],
 *                 'price' => ['type' => 'number', 'minimum' => 0],
 *             ],
 *         ]
 *     )
 * ) {
 *     return $data;
 * });
 *
 * // Typed body parameter
 * $app->post('/users', function(
 *     CreateUserRequest $data = new Body()
 * ) {
 *     return User::create($data);
 * });
 * ```
 *
 * @template T
 * @extends ParamBase<T>
 */
final class Body extends ParamBase
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
