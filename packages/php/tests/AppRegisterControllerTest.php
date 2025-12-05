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
        $app = (new App())->registerController(MultiRouteController::class);

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
    #[Get('/items')]
    /** @return array<string, list<mixed>> */
    public function list(): array
    {
        return ['items' => []];
    }
}

final class SimplePostController
{
    #[Post('/items')]
    /** @return array<string, int> */
    public function create(): array
    {
        return ['id' => 1];
    }
}

final class SimplePutController
{
    #[Put('/items/123')]
    /** @return array<string, bool> */
    public function update(): array
    {
        return ['updated' => true];
    }
}

final class SimpleDeleteController
{
    #[Delete('/items/123')]
    /** @return array<string, bool> */
    public function delete(): array
    {
        return ['deleted' => true];
    }
}

final class SimplePatchController
{
    #[Patch('/items/123')]
    /** @return array<string, bool> */
    public function patch(): array
    {
        return ['patched' => true];
    }
}

final class MultiRouteController
{
    #[Get('/items')]
    /** @return array<string, list<mixed>> */
    public function list(): array
    {
        return ['items' => []];
    }

    #[Post('/items')]
    /** @return array<string, int> */
    public function create(): array
    {
        return ['id' => 1];
    }

    #[Delete('/items/123')]
    /** @return array<string, bool> */
    public function delete(): array
    {
        return ['deleted' => true];
    }
}

final class ControllerWithPrivateMethod
{
    #[Get('/public')]
    /** @return array<string, bool> */
    public function publicMethod(): array
    {
        return ['public' => true];
    }

    #[Get('/private')]
    /** @return array<string, bool> */
    private function privateMethod(): array
    {
        return ['private' => true];
    }
}

final class ControllerWithPlainMethods
{
    #[Get('/routed')]
    /** @return array<string, bool> */
    public function routed(): array
    {
        return ['routed' => true];
    }

    /** @return array<string, bool> */
    public function notRouted(): array
    {
        return ['not_routed' => true];
    }
}

final class ControllerWithPathParams
{
    #[Get('/users/:id')]
    /** @return array<string, string> */
    public function getUserById(string $id): array
    {
        return ['user_id' => $id];
    }

    #[Get('/posts/:postId/comments/:commentId')]
    /** @return array<string, string> */
    public function getComment(string $postId, string $commentId): array
    {
        return ['post_id' => $postId, 'comment_id' => $commentId];
    }
}

final class ControllerWithQueryParams
{
    #[Get('/search')]
    /** @return array<string, string> */
    public function search(
        string $query = 'default'
    ): array {
        return ['query' => $query];
    }
}

final class ControllerWithBodyParam
{
    #[Post('/data')]
    /** @param array<string, mixed> $payload */
    /** @return array<string, mixed> */
    public function create(
        array $payload = []
    ): array {
        return ['received' => $payload];
    }
}

final class ControllerWithHeaderParam
{
    #[Get('/auth')]
    /** @return array<string, string> */
    public function checkAuth(
        string $authorization = 'none'
    ): array {
        return ['auth' => $authorization];
    }
}

final class ControllerWithCookieParam
{
    #[Get('/session')]
    /** @return array<string, string> */
    public function checkSession(
        string $sessionId = 'none'
    ): array {
        return ['session' => $sessionId];
    }
}

final class ControllerWithSchemas
{
    #[Get(
        '/schema',
        requestSchema: ['type' => 'object'],
        responseSchema: ['type' => 'array'],
        parameterSchema: ['type' => 'object']
    )]
    /** @return list<mixed> */
    public function withSchemas(): array
    {
        return [];
    }
}

#[Middleware(['auth'])]
final class ControllerWithMiddleware
{
    #[Get('/protected')]
    /** @return array<string, bool> */
    public function protected(): array
    {
        return ['protected' => true];
    }
}

final class ControllerWithMixedParams
{
    #[Get('/items/:id')]
    /** @return array<string, string> */
    public function getItem(
        string $id,
        string $expand = 'none'
    ): array {
        return ['id' => $id, 'expand' => $expand];
    }

    #[Post('/items')]
    /** @param array<string, mixed> $data */
    /** @return array<string, mixed> */
    public function createItem(
        array $data = []
    ): array {
        return ['item' => $data];
    }
}
