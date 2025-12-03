<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;

final class NativeErrorSurfaceTest extends TestCase
{
    public function test_handler_registration_rejects_non_callable(): void
    {
        $routes = [[
            'method' => 'GET',
            'path' => '/bad',
            'handler_name' => 'bad_handler',
            'handler' => new \stdClass(), // not callable; should surface structured error
        ]];
        $deps = [];

        try {
            spikard_start_server($routes, $this->minimalConfig(), [], $deps);
            $this->fail('Expected handler registration to fail.');
        } catch (\Throwable $exception) {
            $message = $exception->getMessage();
            if (
                !\str_contains($message, 'handler_not_callable')
                && !\str_contains($message, 'Missing handler callable')
            ) {
                throw $exception;
            }
        }
        $this->assertTrue(true);
    }

    public function test_handler_registration_requires_handler(): void
    {
        $routes = [[
            'method' => 'GET',
            'path' => '/missing',
            'handler_name' => 'missing_handler',
            // handler intentionally omitted
        ]];
        $deps = [];

        try {
            spikard_start_server($routes, $this->minimalConfig(), [], $deps);
            $this->fail('Expected handler registration to fail.');
        } catch (\Throwable $exception) {
            $message = $exception->getMessage();
            if (
                !\str_contains($message, 'handler_not_callable')
                && !\str_contains($message, 'Missing handler callable')
            ) {
                throw $exception;
            }
        }
        $this->assertTrue(true);
    }

    /** @return array<string, mixed> */
    private function minimalConfig(): array
    {
        return [
            'host' => '127.0.0.1',
            'port' => 0, // unused because server will not start on error path
            'workers' => 1,
            'enable_request_id' => true,
            'graceful_shutdown' => true,
            'shutdown_timeout' => 1,
        ];
    }
}
