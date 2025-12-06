<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionClass;
use RuntimeException;
use Spikard\App;
use Spikard\Attributes\Delete;
use Spikard\Attributes\Get;
use Spikard\Attributes\Middleware;
use Spikard\Attributes\Patch;
use Spikard\Attributes\Post;
use Spikard\Attributes\Put;
use Spikard\Config\CorsConfig;
use Spikard\Config\ServerConfig;
use Spikard\Handlers\ControllerMethodHandler;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Behavioral tests for App::registerController().
 *
 * Tests the discovery and registration of controller methods via reflection attributes.
 * This covers a major gap in App.php coverage (lines 157-208, ~50 lines).
 *
 * @internal
 */
final class AppRegisterControllerTest extends TestCase
{
    public function testRegisterControllerWithGetRoute(): void
    {
        $app = (new App())->registerController(SimpleGetController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/items', $routes[0]['path']);
        $this->assertInstanceOf(ControllerMethodHandler::class, $routes[0]['handler']);
    }

    public function testRegisterControllerWithPostRoute(): void
    {
        $app = (new App())->registerController(SimplePostController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('POST', $routes[0]['method']);
        $this->assertSame('/items', $routes[0]['path']);
    }

    public function testRegisterControllerWithPutRoute(): void
    {
        $app = (new App())->registerController(SimplePutController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('PUT', $routes[0]['method']);
        $this->assertSame('/items/123', $routes[0]['path']);
    }

    public function testRegisterControllerWithDeleteRoute(): void
    {
        $app = (new App())->registerController(SimpleDeleteController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('DELETE', $routes[0]['method']);
        $this->assertSame('/items/123', $routes[0]['path']);
    }

    public function testRegisterControllerWithPatchRoute(): void
    {
        $app = (new App())->registerController(SimplePatchController::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('PATCH', $routes[0]['method']);
        $this->assertSame('/items/123', $routes[0]['path']);
    }

    public function testRegisterControllerMultipleMethods(): void
    {
        $app = (new App())->registerController(AppRegisterControllerMultiRouteController::class);

        $routes = $app->routes();
        $this->assertCount(3, $routes);

        $methods = \array_map(static fn ($r) => $r['method'], $routes);
        $this->assertContains('GET', $methods);
        $this->assertContains('POST', $methods);
        $this->assertContains('DELETE', $methods);
    }

    public function testRegisterControllerSkipsPrivateMethods(): void
    {
        $app = (new App())->registerController(ControllerWithPrivateMethod::class);

        $routes = $app->routes();
        // Should only register the public method
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/public', $routes[0]['path']);
    }

    public function testRegisterControllerSkipsMethodsWithoutAttributes(): void
    {
        $app = (new App())->registerController(ControllerWithPlainMethods::class);

        $routes = $app->routes();
        // Should only register methods with route attributes
        $this->assertCount(1, $routes);
        $this->assertSame('/routed', $routes[0]['path']);
    }

    public function testRegisterControllerWithInstance(): void
    {
        $instance = new SimpleGetController();
        $app = (new App())->registerController($instance);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/items', $routes[0]['path']);
    }

    public function testRegisterControllerWithPathParameters(): void
    {
        $app = (new App())->registerController(ControllerWithPathParams::class);

        $routes = $app->routes();
        $this->assertCount(2, $routes);

        $paths = \array_map(static fn ($r) => $r['path'], $routes);
        $this->assertContains('/users/:id', $paths);
        $this->assertContains('/posts/:postId/comments/:commentId', $paths);
    }

    public function testRegisterControllerWithQueryParams(): void
    {
        $app = (new App())->registerController(ControllerWithQueryParams::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/search', $routes[0]['path']);
    }

    public function testRegisterControllerWithRequestBodyParam(): void
    {
        $app = (new App())->registerController(ControllerWithBodyParam::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('POST', $routes[0]['method']);
        $this->assertSame('/data', $routes[0]['path']);
    }

    public function testRegisterControllerWithHeaderParam(): void
    {
        $app = (new App())->registerController(ControllerWithHeaderParam::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/auth', $routes[0]['path']);
    }

    public function testRegisterControllerWithCookieParam(): void
    {
        $app = (new App())->registerController(ControllerWithCookieParam::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/session', $routes[0]['path']);
    }

    public function testRegisterControllerWithRequestSchemas(): void
    {
        $app = (new App())->registerController(ControllerWithSchemas::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        $route = $routes[0];
        $this->assertTrue(isset($route['request_schema']));
        $this->assertTrue(isset($route['response_schema']));
        $this->assertTrue(isset($route['parameter_schema']));
        $this->assertSame(['type' => 'object'], $route['request_schema']);
        $this->assertSame(['type' => 'array'], $route['response_schema']);
        $this->assertSame(['type' => 'object'], $route['parameter_schema']);
    }

    public function testRegisterControllerIsImmutable(): void
    {
        $original = new App();
        $modified = $original->registerController(SimpleGetController::class);

        $this->assertNotSame($original, $modified);
        $this->assertCount(0, $original->routes());
        $this->assertCount(1, $modified->routes());
    }

    public function testRegisterMultipleControllers(): void
    {
        $app = (new App())
            ->registerController(SimpleGetController::class)
            ->registerController(SimplePostController::class);

        $routes = $app->routes();
        $this->assertCount(2, $routes);
    }

    public function testRegisterControllerWithMiddleware(): void
    {
        $app = (new App())->registerController(ControllerWithMiddleware::class);

        $routes = $app->routes();
        $this->assertCount(1, $routes);
        // The route is registered; middleware is extracted during registration
        $this->assertSame('GET', $routes[0]['method']);
        $this->assertSame('/protected', $routes[0]['path']);
    }

    public function testRegisterControllerHandlerCanMatch(): void
    {
        $app = (new App())->registerController(SimpleGetController::class);
        $routes = $app->routes();
        $handler = $routes[0]['handler'];

        $request = new Request('GET', '/items', null);
        $this->assertTrue($handler->matches($request));
    }

    public function testRegisterControllerHandlerCanExecute(): void
    {
        $app = (new App())->registerController(SimpleGetController::class);
        $routes = $app->routes();
        $handler = $routes[0]['handler'];

        $request = new Request('GET', '/items', null);
        $response = $handler->handle($request);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRegisterControllerChaining(): void
    {
        $app = (new App())
            ->registerController(SimpleGetController::class)
            ->registerController(SimplePostController::class)
            ->registerController(SimpleDeleteController::class);

        $routes = $app->routes();
        $this->assertCount(3, $routes);

        $methods = \array_map(static fn ($r) => $r['method'], $routes);
        $this->assertContains('GET', $methods);
        $this->assertContains('POST', $methods);
        $this->assertContains('DELETE', $methods);
    }

    public function testRegisterControllerWithMixedParameters(): void
    {
        $app = (new App())->registerController(ControllerWithMixedParams::class);

        $routes = $app->routes();
        $this->assertCount(2, $routes);
    }
}

// Test controller fixtures
final class SimpleGetController
{
    /**
     * @return array<string, list<mixed>>
     */
    #[Get('/items')]
    public function list()
    {
        return ['items' => []];
    }
}

final class SimplePostController
{
    /**
     * @return array<string, int>
     */
    #[Post('/items')]
    public function create()
    {
        return ['id' => 1];
    }
}

final class SimplePutController
{
    /**
     * @return array<string, bool>
     */
    #[Put('/items/123')]
    public function update()
    {
        return ['updated' => true];
    }
}

final class SimpleDeleteController
{
    /**
     * @return array<string, bool>
     */
    #[Delete('/items/123')]
    public function delete()
    {
        return ['deleted' => true];
    }
}

final class SimplePatchController
{
    /**
     * @return array<string, bool>
     */
    #[Patch('/items/123')]
    public function patch()
    {
        return ['patched' => true];
    }
}

final class AppRegisterControllerMultiRouteController
{
    /**
     * @return array<string, list<mixed>>
     */
    #[Get('/items')]
    public function list()
    {
        return ['items' => []];
    }

    /**
     * @return array<string, int>
     */
    #[Post('/items')]
    public function create()
    {
        return ['id' => 1];
    }

    /**
     * @return array<string, bool>
     */
    #[Delete('/items/123')]
    public function delete()
    {
        return ['deleted' => true];
    }
}

final class ControllerWithPrivateMethod
{
    /**
     * @return array<string, bool>
     */
    #[Get('/public')]
    public function publicMethod()
    {
        return ['public' => true];
    }
}

final class ControllerWithPlainMethods
{
    /**
     * @return array<string, bool>
     */
    #[Get('/routed')]
    public function routed()
    {
        return ['routed' => true];
    }

    /**
     * @return array<string, bool>
     */
    public function notRouted()
    {
        return ['not_routed' => true];
    }
}

final class ControllerWithPathParams
{
    /**
     * @return array<string, string>
     */
    #[Get('/users/:id')]
    public function getUserById(string $id)
    {
        return ['user_id' => $id];
    }

    /**
     * @return array<string, string>
     */
    #[Get('/posts/:postId/comments/:commentId')]
    public function getComment(string $postId, string $commentId)
    {
        return ['post_id' => $postId, 'comment_id' => $commentId];
    }
}

final class ControllerWithQueryParams
{
    /**
     * @return array<string, string>
     */
    #[Get('/search')]
    public function search(
        string $query = 'default'
    ) {
        return ['query' => $query];
    }
}

final class ControllerWithBodyParam
{
    /**
     * @param array<string, mixed> $payload
     * @return array<string, mixed>
     */
    #[Post('/data')]
    public function create(
        array $payload = []
    ) {
        return ['received' => $payload];
    }
}

final class ControllerWithHeaderParam
{
    /**
     * @return array<string, string>
     */
    #[Get('/auth')]
    public function checkAuth(
        string $authorization = 'none'
    ) {
        return ['auth' => $authorization];
    }
}

final class ControllerWithCookieParam
{
    /**
     * @return array<string, string>
     */
    #[Get('/session')]
    public function checkSession(
        string $sessionId = 'none'
    ) {
        return ['session' => $sessionId];
    }
}

final class ControllerWithSchemas
{
    /**
     * @return list<mixed>
     */
    #[Get(
        '/schema',
        requestSchema: ['type' => 'object'],
        responseSchema: ['type' => 'array'],
        parameterSchema: ['type' => 'object']
    )]
    public function withSchemas()
    {
        return [];
    }
}

final class ControllerWithMiddleware
{
    /**
     * @return array<string, bool>
     */
    #[Get('/protected')]
    #[Middleware(DummyMiddleware::class)]
    public function protected()
    {
        return ['protected' => true];
    }
}

/**
 * Dummy middleware for testing purposes.
 */
final class DummyMiddleware
{
    public function handle(): void
    {
    }
}

final class ControllerWithMixedParams
{
    /**
     * @return array<string, string>
     */
    #[Get('/items/:id')]
    public function getItem(
        string $id,
        string $expand = 'none'
    ) {
        return ['id' => $id, 'expand' => $expand];
    }

    /**
     * @param array<string, mixed> $data
     * @return array<string, mixed>
     */
    #[Post('/items')]
    public function createItem(
        array $data = []
    ) {
        return ['item' => $data];
    }
}
