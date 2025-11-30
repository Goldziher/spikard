<?php

declare(strict_types=1);

namespace Spikard\DI;

/**
 * Wrapper for dependency factories.
 *
 * Similar to Python's Provide class (packages/python/spikard/di.py:56-172),
 * this class wraps a factory callable that creates a dependency when needed.
 *
 * The factory can depend on other dependencies (resolved first and passed as arguments).
 */
final class Provide
{
    /**
     * @param callable $factory The factory callable to create the dependency
     * @param list<string> $dependsOn List of dependency keys this factory depends on
     * @param bool $singleton Whether to cache globally across requests
     */
    public function __construct(
        public readonly mixed $factory,
        public readonly array $dependsOn = [],
        public readonly bool $singleton = false,
    ) {
    }

    /**
     * Create a Provide instance from a callable.
     *
     * @param list<string> $dependsOn
     */
    public static function factory(
        callable $factory,
        array $dependsOn = [],
        bool $singleton = false,
    ): self {
        return new self($factory, $dependsOn, $singleton);
    }
}
