<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionException;
use RuntimeException;
use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Config\LifecycleHooks;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class AppBehavioralTest extends TestCase
{
    /**
     * Test route matching with multiple routes (find correct handler).
     */
    public function testFindHandlerWithMultipleRoutesReturnsCorrectOne(): void
    {
        $handler1 = new AppBehavioralTestHandler();
        $handler2 = new AppBehavioralTestHandler();
        $handler3 = new AppBehavioralTestHandler();

        $app = RouteTestHelper::withRoute(new App(), 'GET', '/users', $handler1);
        $app = RouteTestHelper::withRoute($app, 'POST', '/users', $handler2);
        $app = RouteTestHelper::withRoute($app, 'GET', '/posts', $handler3);

        $requestGetUsers = make_request('GET', '/users', null);
        $requestPostUsers = make_request('POST', '/users', null);
        $requestGetPosts = make_request('GET', '/posts', null);

        $this->assertSame($handler1, $app->findHandler($requestGetUsers));
        $this->assertSame($handler2, $app->findHandler($requestPostUsers));
        $this->assertSame($handler3, $app->findHandler($requestGetPosts));
    }

    /**
     * Test case-insensitive method matching.
     */
    public function testFindHandlerMethodCaseInsensitive(): void
    {
        $handler = new AppBehavioralTestHandler();
        $app = RouteTestHelper::withRoute(new App(), 'get', '/test', $handler);

        $requestUppercase = make_request('GET', '/test', null);
        $requestMixed = make_request('Get', '/test', null);
        $requestLowercase = make_request('get', '/test', null);

        $this->assertSame($handler, $app->findHandler($requestUppercase));
        $this->assertSame($handler, $app->findHandler($requestMixed));
        $this->assertSame($handler, $app->findHandler($requestLowercase));
    }

    /**
     * Test route path matching with query string in registered route (should strip it).
     */
    public function testFindHandlerStripsQueryStringFromRegisteredPath(): void
    {
        $handler = new AppBehavioralTestHandler();
        // Register with query string
        $app = RouteTestHelper::withRoute(new App(), 'GET', '/test?param=value', $handler);

        // Should match request without query string
        $request = make_request('GET', '/test', null);
        $this->assertSame($handler, $app->findHandler($request));
    }

    /**
     * Test that handler.matches() is called (respects handler's own matching logic).
     */
    public function testFindHandlerCallsHandlerMatches(): void
    {
        $handlerThatMatches = new SelectiveHandler(true);
        $handlerThatDoesNotMatch = new SelectiveHandler(false);

        $app = RouteTestHelper::withRoute(new App(), 'GET', '/test', $handlerThatDoesNotMatch);
        $app = RouteTestHelper::withRoute($app, 'GET', '/test', $handlerThatMatches);

        $request = make_request('GET', '/test', null);
        // Should return the one that matches
        $found = $app->findHandler($request);
        $this->assertSame($handlerThatMatches, $found);
    }

    /**
     * Test registerController with public methods only.
     */
    public function testRegisterControllerScansPublicMethodsOnly(): void
    {
        $app = (new App())->registerController(ControllerWithMixedVisibility::class);

        $routes = $app->routes();
        // Should only find public method
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/public', $routes[0]['path']);
    }

    /**
     * Test registerController with instance.
     */
    public function testRegisterControllerWithInstance(): void
    {
        $instance = new SimpleController();
        $app = (new App())->registerController($instance);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
    }

    /**
     * Test registerController with class name.
     */
    public function testRegisterControllerWithClassName(): void
    {
        $app = (new App())->registerController(SimpleController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
    }

    /**
     * Test registerController finds multiple routes in same controller.
     */
    public function testRegisterControllerWithMultipleMethods(): void
    {
        $app = (new App())->registerController(AppBehavioralMultiRouteController::class);

        $routes = $app->routes();
        $this->assertCount(2, $routes);

        // Verify both routes are registered
        $paths = \array_map(static fn (array $route) => $route['path'], $routes);
        $this->assertContains('/items', $paths);
        $this->assertContains('/items', $paths);
    }

    /**
     * Test registerController creates ControllerMethodHandler.
     */
    public function testRegisterControllerCreatesControllerMethodHandler(): void
    {
        $app = (new App())->registerController(SimpleController::class);

        $routes = $app->routes();
        $handler = $routes[0]['handler'];
        // ControllerMethodHandler is the wrapper used
        $this->assertInstanceOf(\Spikard\Handlers\ControllerMethodHandler::class, $handler);
    }

    /**
     * Test registerController is immutable.
     */
    public function testRegisterControllerIsImmutable(): void
    {
        $original = new App();
        $modified = $original->registerController(SimpleController::class);

        $this->assertNotSame($original, $modified);
        $this->assertSame([], $original->routes());
        $this->assertCount(1, $modified->routes());
    }

    /**
     * Test chaining registerController with other methods.
     */
    public function testRegisterControllerChains(): void
    {
        $manualHandler = new AppBehavioralTestHandler();
        $app = (new App())->registerController(SimpleController::class);
        $app = RouteTestHelper::withRoute($app, 'POST', '/manual', $manualHandler);

        $routes = $app->routes();
        $this->assertCount(2, $routes);
    }

    /**
     * Test run() throws without ServerConfig.
     */
    public function testRunThrowsWithoutConfig(): void
    {
        $app = new App();
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('ServerConfig is required');
        $app->run();
    }

    /**
     * Test run() throws without extension.
     */
    public function testRunThrowsWithoutExtension(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$app = new \\Spikard\\App();'
            . '$config = \\Spikard\\Config\\ServerConfig::builder()->build();'
            . '$app->run($config);'
        );

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString('extension is not loaded', $output);
    }

    /**
     * Test run() accepts config as parameter (overrides instance config).
     */
    public function testRunAcceptsConfigParameter(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$config = \\Spikard\\Config\\ServerConfig::builder()->withPort(9000)->build();'
            . '$app = new \\Spikard\\App();'
            . '$app->run($config);'
        );

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString('extension is not loaded', $output);
    }

    /**
     * Test close() is idempotent (can call multiple times).
     */
    public function testCloseIsIdempotent(): void
    {
        $app = RouteTestHelper::withRoute(new App(), 'GET', '/test', new AppBehavioralTestHandler());
        $client = TestClient::create($app);

        // Should not throw when called multiple times
        $client->close();
        $client->close();

        $this->expectNotToPerformAssertions();
    }

    /**
     * Test nativeRoutes() includes HTTP routes.
     */
    public function testNativeRoutesIncludesHttpRoutes(): void
    {
        $handler = new AppBehavioralTestHandler();
        $app = RouteTestHelper::withRoute(new App(), 'GET', '/users', $handler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('/users', $nativeRoutes[0]['path']);
    }

    /**
     * Test nativeRoutes() includes WebSocket handlers.
     */
    public function testNativeRoutesIncludesWebSocketHandlers(): void
    {
        $wsHandler = new AppBehavioralTestWebSocketHandler();
        $app = (new App())->addWebSocket('/ws', $wsHandler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertTrue($nativeRoutes[0]['websocket'] ?? false);
    }

    /**
     * Test nativeRoutes() includes SSE producers.
     */
    public function testNativeRoutesIncludesSseProducers(): void
    {
        $sseProducer = new AppBehavioralTestSseProducer();
        $app = (new App())->addSse('/events', $sseProducer);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(1, $nativeRoutes);
        $this->assertTrue($nativeRoutes[0]['sse'] ?? false);
    }

    /**
     * Test nativeRoutes() combines all route types.
     */
    public function testNativeRoutesCombinesAllTypes(): void
    {
        $httpHandler = new AppBehavioralTestHandler();
        $wsHandler = new AppBehavioralTestWebSocketHandler();
        $sseProducer = new AppBehavioralTestSseProducer();

        $app = RouteTestHelper::withRoute(new App(), 'GET', '/api', $httpHandler)
            ->addWebSocket('/ws', $wsHandler)
            ->addSse('/events', $sseProducer);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertCount(3, $nativeRoutes);
    }

    /**
     * Test nativeRoutes() uppercases HTTP methods.
     */
    public function testNativeRoutesUppercasesHttpMethods(): void
    {
        $handler = new AppBehavioralTestHandler();
        $app = RouteTestHelper::withRoute(new App(), 'get', '/test1', $handler);
        $app = RouteTestHelper::withRoute($app, 'Post', '/test2', $handler);

        $nativeRoutes = $app->nativeRoutes();
        $this->assertSame('GET', $nativeRoutes[0]['method']);
        $this->assertSame('POST', $nativeRoutes[1]['method']);
    }

    /**
     * Test websocketHandlers() returns registered handlers.
     */
    public function testWebsocketHandlersReturnsRegistered(): void
    {
        $ws1 = new AppBehavioralTestWebSocketHandler();
        $ws2 = new AppBehavioralTestWebSocketHandler();

        $app = (new App())
            ->addWebSocket('/ws1', $ws1)
            ->addWebSocket('/ws2', $ws2);

        $handlers = $app->websocketHandlers();
        $this->assertCount(2, $handlers);
        $this->assertSame($ws1, $handlers['/ws1']);
        $this->assertSame($ws2, $handlers['/ws2']);
    }

    /**
     * Test sseProducers() returns registered producers.
     */
    public function testSseProducersReturnsRegistered(): void
    {
        $sse1 = new AppBehavioralTestSseProducer();
        $sse2 = new AppBehavioralTestSseProducer();

        $app = (new App())
            ->addSse('/events1', $sse1)
            ->addSse('/events2', $sse2);

        $producers = $app->sseProducers();
        $this->assertCount(2, $producers);
        $this->assertSame($sse1, $producers['/events1']);
        $this->assertSame($sse2, $producers['/events2']);
    }

    /**
     * Test addWebSocket is immutable.
     */
    public function testAddWebSocketIsImmutable(): void
    {
        $ws = new AppBehavioralTestWebSocketHandler();
        $original = new App();
        $modified = $original->addWebSocket('/ws', $ws);

        $this->assertNotSame($original, $modified);
        $this->assertCount(0, $original->websocketHandlers());
        $this->assertCount(1, $modified->websocketHandlers());
    }

    /**
     * Test addSse is immutable.
     */
    public function testAddSseIsImmutable(): void
    {
        $sse = new AppBehavioralTestSseProducer();
        $original = new App();
        $modified = $original->addSse('/events', $sse);

        $this->assertNotSame($original, $modified);
        $this->assertCount(0, $original->sseProducers());
        $this->assertCount(1, $modified->sseProducers());
    }

    /**
     * Test schema-backed routes preserve all schema fields.
     */
    public function testAddRouteWithSchemasPreservesAllSchemas(): void
    {
        $handler = new AppBehavioralTestHandler();
        $requestSchema = ['type' => 'object', 'properties' => ['name' => ['type' => 'string']]];
        $responseSchema = ['type' => 'object', 'properties' => ['id' => ['type' => 'number']]];
        $paramSchema = ['type' => 'object'];

        $app = RouteTestHelper::withRouteWithSchemas(
            new App(),
            'POST',
            '/items',
            $handler,
            $requestSchema,
            $responseSchema,
            $paramSchema
        );

        $routes = $app->routes();
        $route = $routes[0] + [
            'request_schema' => null,
            'response_schema' => null,
            'parameter_schema' => null,
        ];

        $this->assertSame($requestSchema, $route['request_schema']);
        $this->assertSame($responseSchema, $route['response_schema']);
        $this->assertSame($paramSchema, $route['parameter_schema']);
    }

    /**
     * Test schema-backed routes with null schemas.
     */
    public function testAddRouteWithSchemasAcceptsNullSchemas(): void
    {
        $handler = new AppBehavioralTestHandler();
        $app = RouteTestHelper::withRouteWithSchemas(
            new App(),
            'GET',
            '/items',
            $handler,
            null,
            null,
            null
        );

        $routes = $app->routes();
        $route = $routes[0] + [
            'request_schema' => null,
            'response_schema' => null,
            'parameter_schema' => null,
        ];

        $this->assertNull($route['request_schema']);
        $this->assertNull($route['response_schema']);
        $this->assertNull($route['parameter_schema']);
    }

    /**
     * Test cloning behavior (immutability mechanism).
     */
    public function testCloningPreservesIndependence(): void
    {
        $handler1 = new AppBehavioralTestHandler();
        $handler2 = new AppBehavioralTestHandler();

        $app1 = RouteTestHelper::withRoute(new App(), 'GET', '/test1', $handler1);
        $app2 = RouteTestHelper::withRoute($app1, 'GET', '/test2', $handler2);

        // app1 should only have one route
        $this->assertCount(1, $app1->routes());
        // app2 should have both
        $this->assertCount(2, $app2->routes());
    }

    /**
     * Test single-route helper registration.
     */
    public function testSingleRouteConvenience(): void
    {
        $handler = new AppBehavioralTestHandler();
        $app = RouteTestHelper::withRoute(new App(), 'DELETE', '/items/42', $handler);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('DELETE', $routes[0]['method']);
        $this->assertSame('/items/42', $routes[0]['path']);
        $this->assertSame($handler, $routes[0]['handler']);
    }
}

// Test helpers

final class SimpleController
{
    /**
     * @return array<int, string>
     */
    #[Get('/items')]
    public function list()
    {
        return ['item'];
    }
}

final class AppBehavioralMultiRouteController
{
    /**
     * @return array<int, string>
     */
    #[Get('/items')]
    public function list()
    {
        return ['list'];
    }

    /**
     * @return array<string, string>
     */
    #[Post('/items')]
    public function create()
    {
        return ['status' => 'created'];
    }
}

final class ControllerWithMixedVisibility
{
    /**
     * @return array<string, string>
     */
    #[Get('/public')]
    public function publicMethod()
    {
        return ['visible' => 'public'];
    }
}

final class SelectiveHandler implements HandlerInterface
{
    public function __construct(private readonly bool $shouldMatch)
    {
    }

    public function matches(Request $request): bool
    {
        return $this->shouldMatch;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['ok' => true], 200);
    }

    public function __invoke(Request $request): Response
    {
        return $this->handle($request);
    }
}

final class AppBehavioralTestHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['ok' => true], 200);
    }

    public function __invoke(Request $request): Response
    {
        return $this->handle($request);
    }
}

final class AppBehavioralTestWebSocketHandler implements WebSocketHandlerInterface
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

final class AppBehavioralTestSseProducer implements SseEventProducerInterface
{
    public function __invoke(): \Generator
    {
        yield "data: test\n\n";
    }
}
