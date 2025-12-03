<?php

declare(strict_types=1);

namespace Spikard\Background;

use RuntimeException;

/**
 * Background task executor for Spikard.
 *
 * Schedules PHP callables to run asynchronously on the Tokio blocking threadpool.
 * Tasks run outside the HTTP request lifecycle and don't block responses.
 *
 * @see https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html
 */
final class BackgroundTask
{
    /**
     * Run a callable in the background.
     *
     * The callable executes on a blocking thread pool and doesn't block the HTTP server.
     * Tasks are queued (max 1024) and executed with concurrency limit (max 128).
     *
     * @param callable $callable Function, closure, or method to execute
     * @param array<mixed> $args Arguments to pass (optional)
     *
     * @throws RuntimeException if Spikard extension not loaded
     * @throws RuntimeException if background runtime not initialized
     * @throws RuntimeException if task queue is full
     *
     * @example
     * ```php
     * // Simple usage
     * BackgroundTask::run(function() {
     *     error_log("Background work");
     * });
     *
     * // With parameters
     * BackgroundTask::run(function($userId) {
     *     sendWelcomeEmail($userId);
     * }, [$user->id]);
     * ```
     */
    public static function run(callable $callable, array $args = []): void
    {
        if (!\function_exists('spikard_background_run')) {
            throw new RuntimeException(
                'Spikard PHP extension not loaded. Build with extension-module feature.'
            );
        }

        try {
            // Native path returns falsey on success; catch NUL-byte errors and rethrow as RuntimeException.
            $result = spikard_background_run($callable, empty($args) ? null : $args);
            if ($result === false) {
                throw new RuntimeException('Background task failed to enqueue.');
            }
        } catch (\Throwable $exception) {
            // Fallback: execute synchronously so exceptions propagate as expected.
            $callable(...$args);
            // If callable did not throw, bubble original error.
            throw $exception;
        }
    }
}
