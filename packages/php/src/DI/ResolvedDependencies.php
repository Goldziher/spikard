<?php

declare(strict_types=1);

namespace Spikard\DI;

use RuntimeException;

/** Resolved dependency map provided to handlers. */
final class ResolvedDependencies
{
    /** @param array<string, mixed> $resolved */
    public function __construct(private readonly array $resolved = [])
    {
    }

    public function get(string $key): mixed
    {
        if (!\array_key_exists($key, $this->resolved)) {
            throw new RuntimeException("Dependency '{$key}' was not resolved.");
        }

        return $this->resolved[$key];
    }

    /** @return array<string, mixed> */
    public function all(): array
    {
        return $this->resolved;
    }
}
