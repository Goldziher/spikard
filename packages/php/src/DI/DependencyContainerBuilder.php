<?php

declare(strict_types=1);

namespace Spikard\DI;

final class DependencyContainerBuilder
{
    /** @var array<string, mixed> */
    private array $values = [];

    /** @var array<string, Provide> */
    private array $factories = [];

    public function provideValue(string $key, mixed $value): self
    {
        $this->values[$key] = $value;
        return $this;
    }

    public function provideFactory(string $key, Provide $factory): self
    {
        $this->factories[$key] = $factory;
        return $this;
    }

    public function build(): DependencyContainer
    {
        return new DependencyContainer(values: $this->values, factories: $this->factories);
    }
}
