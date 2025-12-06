<?php

declare(strict_types=1);

require_once __DIR__ . '/../vendor/autoload.php';

// Mock the Spikard PHP extension if not loaded
// This allows us to test the PHP side of the code without requiring the extension
if (!\function_exists('spikard_background_run')) {
    /**
     * Mock implementation of spikard_background_run for testing.
     * In production, this is provided by the ext-php-rs binding.
     *
     * @param array<mixed>|null $args
     */
    function spikard_background_run(callable $callable, ?array $args = null): void
    {
        // In test environment, execute immediately (synchronously)
        // In production, this would queue to Tokio's blocking threadpool
        if ($args === null || empty($args)) {
            $callable();
        } else {
            $callable(...$args);
        }
    }

    // Mark that we're using the mock
    \define('SPIKARD_EXTENSION_MOCKED', true);
}
