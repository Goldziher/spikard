<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Testing\TestClient;

final class WebSocketSseTest extends TestCase
{
    public function test_sse_route_is_exposed_to_native_routes(): void
    {
        $app = (new App())->addSse('/events', new DummySseProducer());
        $routes = $app->nativeRoutes();

        $sseRoute = \array_filter($routes, fn ($r) => ($r['path'] === '/events'));
        $this->assertNotEmpty($sseRoute);
        $route = \array_values($sseRoute)[0];
        $this->assertTrue($route['sse'] ?? false);
    }

    public function test_websocket_route_is_exposed_to_native_routes(): void
    {
        $app = (new App())->addWebSocket('/ws', new DummyWebSocketHandler());
        $routes = $app->nativeRoutes();

        $wsRoute = \array_filter($routes, fn ($r) => ($r['path'] === '/ws'));
        $this->assertNotEmpty($wsRoute);
        $route = \array_values($wsRoute)[0];
        $this->assertTrue($route['websocket'] ?? false);
    }
}

final class DummySseProducer implements SseEventProducerInterface
{
    public function __invoke(): \Generator
    {
        yield 'data: {"hello":"world"}' . "\n\n";
    }
}

final class DummyWebSocketHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
    }

    public function onMessage(string $message): void
    {
    }

    public function onClose(int $code, ?string $reason = null): void
    {
    }
}
