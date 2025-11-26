<?php

declare(strict_types=1);

namespace Spikard\DI;

/** Runtime container placeholder; resolution is handled in Rust. */
final class DependencyContainer
{
    /**
     * @param array<string, mixed> $values
     * @param array<string, DependencyFactory> $factories
     */
    public function __construct(
        public readonly array $values = [],
        public readonly array $factories = [],
    ) {
    }

    public static function builder(): DependencyContainerBuilder
    {
        return new DependencyContainerBuilder();
    }
}
