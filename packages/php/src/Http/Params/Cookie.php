<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Extract a value from request cookies
 *
 * Use this as a default parameter value to inject cookie values into route handlers.
 *
 * Examples:
 * ```php
 * use Spikard\Http\Params\Cookie;
 *
 * $app->get('/items', function(
 *     ?string $sessionId = new Cookie(default: null)
 * ) {
 *     return ['session_id' => $sessionId];
 * });
 *
 * $app->get('/users/me', function(
 *     string $key = new Cookie(schema: ['minLength' => 10])
 * ) {
 *     if ($key === 'secret') {
 *         return ['username' => 'secret'];
 *     }
 *     return ['error' => 'Invalid key'];
 * });
 *
 * // With default factory
 * $app->get('/items', function(
 *     array $sessionData = new Cookie(defaultFactory: fn() => [])
 * ) {
 *     return $sessionData;
 * });
 *
 * // With pattern validation (regex)
 * $app->get('/items', function(
 *     string $token = new Cookie(
 *         pattern: '/^[a-zA-Z0-9]{32}$/'
 *     )
 * ) {
 *     return ['token' => $token];
 * });
 * ```
 *
 * @template T
 * @extends ParamBase<T>
 */
final class Cookie extends ParamBase
{
    /**
     * @param T|null $default Default value if cookie is not present
     * @param Closure|null $defaultFactory Callable that generates default value when invoked
     * @param int|null $minLength Minimum string length for validation
     * @param int|null $maxLength Maximum string length for validation
     * @param string|null $pattern Regex pattern for validation
     * @param array<string, mixed>|null $schema Optional JSON schema dict for custom validation
     */
    public function __construct(
        mixed $default = null,
        ?Closure $defaultFactory = null,
        private ?int $minLength = null,
        private ?int $maxLength = null,
        private ?string $pattern = null,
        ?array $schema = null,
    ) {
        parent::__construct($default, $defaultFactory, $schema);
    }

    /**
     * Get minimum length constraint
     */
    public function getMinLength(): ?int
    {
        return $this->minLength;
    }

    /**
     * Get maximum length constraint
     */
    public function getMaxLength(): ?int
    {
        return $this->maxLength;
    }

    /**
     * Get pattern constraint
     */
    public function getPattern(): ?string
    {
        return $this->pattern;
    }
}
