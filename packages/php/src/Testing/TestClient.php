<?php

declare(strict_types=1);

namespace Spikard\Testing;

use RuntimeException;
use Spikard\App;
use Spikard\Http\Response;

final class TestClient
{
    private function __construct(private readonly App $app)
    {
    }

    public static function create(App $app): self
    {
        return new self($app);
    }

    public function app(): App
    {
        return $this->app;
    }

    public function get(string $path): Response
    {
        throw new RuntimeException('TestClient not implemented for PHP bindings yet.');
    }

    public function post(string $path, mixed $body = null): Response
    {
        throw new RuntimeException('TestClient not implemented for PHP bindings yet.');
    }

    public function close(): void
    {
        // placeholder for resource cleanup once HTTP runtime is wired
    }
}
