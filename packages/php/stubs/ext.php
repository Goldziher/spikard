<?php

declare(strict_types=1);

// Stub definitions for static analysis when the native extension is absent.
if (false) {
    function spikard_version(): string
    {
        return '0.0.0';
    }

    /**
     * @param array<int, array{method: string, path: string, handler: object}> $routes
     * @param array<string, mixed> $config
     * @param array<string, callable> $lifecycle
     */
    function spikard_start_server(array $routes, array $config, array $lifecycle): int
    {
        return 1;
    }

    function spikard_stop_server(int $handle): bool
    {
        return true;
    }
}
