<?php

declare(strict_types=1);

require_once __DIR__ . '/../vendor/autoload.php';
require_once __DIR__ . '/helpers.php';

if (!\function_exists('spikard_background_run')) {
    /**
     * @param array<mixed>|null $args
     */
    function spikard_background_run(callable $callable, ?array $args = null): void
    {
        if ($args === null || $args === []) {
            $callable();
            return;
        }

        $callable(...$args);
    }
}
