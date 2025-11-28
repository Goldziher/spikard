<?php

declare(strict_types=1);

namespace Spikard\DI;

/**
 * Runtime container for dependency injection.
 *
 * Resolution is handled in Rust (crates/spikard-php/src/php/di.rs).
 * This class provides the PHP interface for registering value and factory dependencies.
 */
final class DependencyContainer
{
    /**
     * Combined dependencies array (values and Provide instances).
     *
     * @var array<string, mixed>
     */
    public readonly array $dependencies;

    /**
     * @param array<string, mixed> $values Value dependencies (singletons)
     * @param array<string, Provide> $factories Factory dependencies (callables)
     */
    public function __construct(
        array $values = [],
        array $factories = [],
    ) {
        // Combine values and factories into single dependencies array
        // Rust will distinguish them by checking if instanceof Provide
        $this->dependencies = \array_merge($values, $factories);
    }

    public static function builder(): DependencyContainerBuilder
    {
        return new DependencyContainerBuilder();
    }

    /**
     * Get all dependencies (for Rust FFI extraction).
     *
     * @return array<string, mixed>
     */
    public function getDependencies(): array
    {
        return $this->dependencies;
    }
}
