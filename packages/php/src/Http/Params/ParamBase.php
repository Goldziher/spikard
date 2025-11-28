<?php

declare(strict_types=1);

namespace Spikard\Http\Params;

use Closure;

/**
 * Base class for all parameter extraction helpers
 *
 * Provides common functionality for default values and default factories.
 * When used as a callable default parameter value, PHP will invoke __invoke
 * to lazily generate defaults.
 *
 * @template T
 */
abstract class ParamBase
{
    /**
     * @param T|null $default Static default value
     * @param Closure|null $defaultFactory Callable that generates default value
     * @param array<string, mixed>|null $schema Optional JSON schema for validation
     */
    public function __construct(
        protected mixed $default = null,
        protected ?Closure $defaultFactory = null,
        protected ?array $schema = null,
    ) {
        if ($this->default !== null && $this->defaultFactory !== null) {
            throw new \InvalidArgumentException(
                'Cannot specify both "default" and "defaultFactory"'
            );
        }
    }

    /**
     * Make the wrapper callable so PHP can invoke it as a default
     *
     * When a parameter with this wrapper is not provided, PHP will
     * call this method to get the actual default value.
     *
     * @return T|null
     */
    public function __invoke(): mixed
    {
        return $this->getDefault();
    }

    /**
     * Get the default value, invoking factory if needed
     *
     * @return T|null
     */
    public function getDefault(): mixed
    {
        if ($this->defaultFactory !== null) {
            return ($this->defaultFactory)();
        }

        return $this->default;
    }

    /**
     * Check if this parameter has a default value
     */
    public function hasDefault(): bool
    {
        return $this->default !== null || $this->defaultFactory !== null;
    }

    /**
     * Get the JSON schema for validation
     *
     * @return array<string, mixed>|null
     */
    public function getSchema(): ?array
    {
        return $this->schema;
    }
}
