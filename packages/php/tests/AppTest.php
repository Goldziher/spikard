<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class AppTest extends TestCase
{
    public function testAppCreation(): void
    {
        $app = new App();

        $this->assertInstanceOf(App::class, $app);
        $this->assertNull($app->config());
        $this->assertNull($app->lifecycleHooks());
        $this->assertNull($app->dependencies());
        $this->assertSame([], $app->routes());
    }

    public function testAppWithConfig(): void
    {
        $config = ServerConfig::builder()->build();
        $app = new App($config);

        $this->assertSame($config, $app->config());
    }

    public function testAppWithConfigMethod(): void
    {
        $config = ServerConfig::builder()->build();
        $app = (new App())->withConfig($config);

        $this->assertSame($config, $app->config());
    }

    public function testAppWithConfigIsImmutable(): void
    {
        $config = ServerConfig::builder()->build();
        $original = new App();
        $modified = $original->withConfig($config);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->config());
        $this->assertSame($config, $modified->config());
    }

    public function testAppWithLifecycleHooks(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $app = (new App())->withLifecycleHooks($hooks);

        $this->assertSame($hooks, $app->lifecycleHooks());
    }

    public function testAppWithLifecycleHooksIsImmutable(): void
    {
        $hooks = LifecycleHooks::builder()->build();
        $original = new App();
        $modified = $original->withLifecycleHooks($hooks);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->lifecycleHooks());
        $this->assertSame($hooks, $modified->lifecycleHooks());
    }

    public function testAppWithDependencies(): void
    {
        $deps = DependencyContainer::builder()->build();
        $app = (new App())->withDependencies($deps);

        $this->assertSame($deps, $app->dependencies());
    }

    public function testAppWithDependenciesIsImmutable(): void
    {
        $deps = DependencyContainer::builder()->build();
        $original = new App();
        $modified = $original->withDependencies($deps);

        $this->assertNotSame($original, $modified);
        $this->assertNull($original->dependencies());
        $this->assertSame($deps, $modified->dependencies());
    }

    public function testAppAddRoute(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/test', $routes[0]['path']);
        $this->assertSame($handler, $routes[0]['handler']);
    }

    public function testAppAddRouteIsImmutable(): void
    {
        $handler = new TestHandler();
        $original = new App();
        $modified = $original->addRoute('GET', '/test', $handler);

        $this->assertNotSame($original, $modified);
        $this->assertSame([], $original->routes());
        $this->assertCount(1, $modified->routes());
    }

    public function testAppAddMultipleRoutes(): void
    {
        $handler1 = new TestHandler();
        $handler2 = new TestHandler();

        $app = (new App())
            ->addRoute('GET', '/users', $handler1)
            ->addRoute('POST', '/users', $handler2);

        $routes = $app->routes();
        $this->assertCount(2, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('POST', $routes[1]['method']);
    }

    public function testAppAddRouteWithSchemas(): void
    {
        $handler = new TestHandler();
        $requestSchema = ['type' => 'object'];
        $responseSchema = ['type' => 'object'];
        $paramSchema = ['type' => 'object'];

        $app = (new App())->addRouteWithSchemas(
            'POST',
            '/items',
            $handler,
            $requestSchema,
            $responseSchema,
            $paramSchema
        );

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $route = $routes[0];
        $this->assertTrue(isset($route['request_schema']));
        $this->assertTrue(isset($route['response_schema']));
        $this->assertTrue(isset($route['parameter_schema']));
        $this->assertSame($requestSchema, $route['request_schema']);
    }

    public function testAppAddWebSocket(): void
    {
        $wsHandler = new TestWebSocketHandler();
        $app = (new App())->addWebSocket('/ws', $wsHandler);

        $handlers = $app->websocketHandlers();
        $this->assertCount(1, $handlers);
        $this->assertArrayHasKey('/ws', $handlers);
        $this->assertSame($wsHandler, $handlers['/ws']);
    }

    public function testAppAddSse(): void
    {
        $sseProducer = new TestSseProducer();
        $app = (new App())->addSse('/events', $sseProducer);

        $producers = $app->sseProducers();
        $this->assertCount(1, $producers);
        $this->assertArrayHasKey('/events', $producers);
        $this->assertSame($sseProducer, $producers['/events']);
    }

    public function testAppFindHandlerMatching(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('GET', '/test', null);
        $found = $app->findHandler($request);

        $this->assertSame($handler, $found);
    }

    public function testAppFindHandlerNotFound(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('GET', '/other', null);
        $found = $app->findHandler($request);

        $this->assertNull($found);
    }

    public function testAppFindHandlerDifferentMethod(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $request = new Request('POST', '/test', null);
        $found = $app->findHandler($request);

        $this->assertNull($found);
    }

    public function testAppFindHandlerStripQueryString(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test?page=1', $handler);

        // Should match without query string
        $request = new Request('GET', '/test', null);
        $found = $app->findHandler($request);

        $this->assertSame($handler, $found);
    }

    public function testAppNativeRoutes(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/test', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['handler'])) {
            $this->assertSame($handler, $nativeRoutes[0]['handler']);
        }
    }

    public function testAppNativeRoutesIncludesWebSocket(): void
    {
        $wsHandler = new TestWebSocketHandler();
        $app = (new App())->addWebSocket('/ws', $wsHandler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/ws', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['websocket'])) {
            $this->assertTrue($nativeRoutes[0]['websocket']);
        }
    }

    public function testAppNativeRoutesIncludesSse(): void
    {
        $sseProducer = new TestSseProducer();
        $app = (new App())->addSse('/events', $sseProducer);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/events', $nativeRoutes[0]['path']);
        if (isset($nativeRoutes[0]['sse'])) {
            $this->assertTrue($nativeRoutes[0]['sse']);
        }
    }

    public function testAppSingleRoute(): void
    {
        $handler = new TestHandler();
        $app = App::singleRoute('GET', '/hello', $handler);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/hello', $routes[0]['path']);
    }

    public function testAppMethodsCaseInsensitive(): void
    {
        $handler = new TestHandler();
        $app = (new App())
            ->addRoute('get', '/test1', $handler)
            ->addRoute('POST', '/test2', $handler);

        $request1 = new Request('GET', '/test1', null);
        $request2 = new Request('post', '/test2', null);

        $this->assertSame($handler, $app->findHandler($request1));
        $this->assertSame($handler, $app->findHandler($request2));
    }

    public function testAppChaining(): void
    {
        $config = ServerConfig::builder()->build();
        $hooks = LifecycleHooks::builder()->build();
        $deps = DependencyContainer::builder()->build();
        $handler = new TestHandler();

        $app = (new App())
            ->withConfig($config)
            ->withLifecycleHooks($hooks)
            ->withDependencies($deps)
            ->addRoute('GET', '/test', $handler);

        $this->assertSame($config, $app->config());
        $this->assertSame($hooks, $app->lifecycleHooks());
        $this->assertSame($deps, $app->dependencies());
        $this->assertCount(1, $app->routes());
    }

    public function testAppImmutabilityThroughChain(): void
    {
        $original = new App();
        $step1 = $original->withConfig(ServerConfig::builder()->build());
        $step2 = $step1->addRoute('GET', '/test', new TestHandler());

        $this->assertNotSame($original, $step1);
        $this->assertNotSame($step1, $step2);
        $this->assertNull($original->config());
        $this->assertSame([], $original->routes());
        $this->assertCount(1, $step2->routes());
    }
}

// Test helpers
final class TestHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['ok' => true], 200);
    }
}

final class TestWebSocketHandler implements WebSocketHandlerInterface
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

final class TestSseProducer implements SseEventProducerInterface
{
    public function __invoke(): \Generator
    {
        yield "data: test\n\n";
    }
}
