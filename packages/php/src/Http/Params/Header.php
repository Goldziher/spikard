<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Extract a value from request headers
 *
 * Use this as a default parameter value to inject header values into route handlers.
 *
 * Examples:
 * ```php
 * use Spikard\Http\Params\Header;
 *
 * $app->get('/items', function(
 *     string $userAgent = new Header(default: 'unknown')
 * ) {
 *     return ['user_agent' => $userAgent];
 * });
 *
 * $app->get('/users/me', function(
 *     ?string $authorization = new Header(default: null)
 * ) {
 *     if ($authorization) {
 *         return ['authenticated' => true];
 *     }
 *     return ['authenticated' => false];
 * });
 *
 * // With alias for different header name
 * $app->get('/items', function(
 *     string $apiKey = new Header(alias: 'X-API-Key')
 * ) {
 *     return ['authenticated' => true];
 * });
 *
 * // With custom JSON schema
 * $app->get('/items', function(
 *     string $apiKey = new Header(
 *         alias: 'X-API-Key',
 *         schema: ['minLength' => 32]
 *     )
 * ) {
 *     return ['authenticated' => true];
 * });
 * ```
 *
 * @extends ParamBase<string>
 */
final class Header extends ParamBase
{
    /**
     * @param string|null $default Default value if header is not present
     * @param Closure|null $defaultFactory Callable that generates default value when invoked
     * @param string|null $alias Alternative header name (e.g., "X-API-Key")
     * @param bool $convertUnderscores Convert underscores to hyphens in header name
     * @param array<string, mixed>|null $schema Optional JSON schema dict for custom validation
     */
    public function __construct(
        ?string $default = null,
        ?Closure $defaultFactory = null,
        private ?string $alias = null,
        private bool $convertUnderscores = true,
        ?array $schema = null,
    ) {
        parent::__construct($default, $defaultFactory, $schema);
    }

    /**
     * Get the header alias
     */
    public function getAlias(): ?string
    {
        return $this->alias;
    }

    /**
     * Check if underscores should be converted to hyphens
     */
    public function shouldConvertUnderscores(): bool
    {
        return $this->convertUnderscores;
    }
}
