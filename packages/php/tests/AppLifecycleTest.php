<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class AppLifecycleTest extends TestCase
{
    public function test_run_without_extension_throws(): void
    {
        $app = (new App())->addRoute('GET', '/hello', new DummyHandler());
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('Spikard PHP extension is not loaded');
        $app->run(ServerConfig::builder()->build());
    }

    public function test_close_is_noop_without_extension(): void
    {
        $app = (new App())->addRoute('GET', '/hello', new DummyHandler());
        $client = TestClient::create($app);
        $response = $client->get('/hello');

        $this->assertSame(200, $response->statusCode);
        $client->close(); // should not throw
    }
}

final class DummyHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return $request->path === '/hello';
    }

    public function handle(Request $request): Response
    {
        return Response::json(['ok' => true], 200);
    }
}
