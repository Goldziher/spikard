<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionClass;
use ReflectionMethod;
use RuntimeException;
use Spikard\App;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Handlers\ControllerMethodHandler;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Behavioral tests for ControllerMethodHandler to increase coverage from 1.52% to 80%+.
 *
 * Tests focus on:
 * 1. Parameter resolution from various sources (query, path, header, cookie, body)
 * 2. Response conversion from various return types
 * 3. Error handling for unresolvable parameters
 * 4. Complex parameter binding scenarios
 */
final class ControllerMethodHandlerBehavioralTest extends TestCase
{
    // ======================== Parameter Resolution - Query Params ========================

    public function testResolveParameterFromQueryParam(): void
    {
        $app = (new App())->registerController(QueryParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/search',
            queryParams: ['q' => ['test']],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('test', $body['query'] ?? null);
    }

    public function testResolveParameterFromMultipleQueryValues(): void
    {
        $app = (new App())->registerController(QueryParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/search',
            queryParams: ['q' => ['tag1', 'tag2']],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        // Multiple values should be returned as array
        $this->assertIsArray($body['query'] ?? null);
    }

    public function testResolveQueryParameterWithDefault(): void
    {
        $app = (new App())->registerController(QueryParamWithDefaultController::class);
        $request = new Request(
            method: 'GET',
            path: '/items',
            queryParams: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('default-limit', $body['limit'] ?? null);
    }

    // ======================== Parameter Resolution - Path Params ========================

    public function testResolveParameterFromPathParam(): void
    {
        $app = (new App())->registerController(PathParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/users/123',
            pathParams: ['id' => '123'],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('123', $body['id'] ?? null);
    }

    public function testResolveParameterFromPathParamWithDefault(): void
    {
        $app = (new App())->registerController(PathParamWithDefaultController::class);
        $request = new Request(
            method: 'GET',
            path: '/posts',
            pathParams: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('0', $body['postId'] ?? null);
    }

    // ======================== Parameter Resolution - Headers ========================

    public function testResolveParameterFromHeader(): void
    {
        $app = (new App())->registerController(HeaderParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/auth',
            headers: ['Authorization' => 'Bearer token123'],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('Bearer token123', $body['authHeader'] ?? null);
    }

    public function testResolveParameterFromHeaderWithDefault(): void
    {
        $app = (new App())->registerController(HeaderParamWithDefaultController::class);
        $request = new Request(
            method: 'GET',
            path: '/api',
            headers: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('application/json', $body['contentType'] ?? null);
    }

    // ======================== Parameter Resolution - Cookies ========================

    public function testResolveParameterFromCookie(): void
    {
        $app = (new App())->registerController(CookieParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/session',
            cookies: ['session_id' => 'sess123'],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('sess123', $body['sessionId'] ?? null);
    }

    public function testResolveParameterFromCookieWithDefault(): void
    {
        $app = (new App())->registerController(CookieParamWithDefaultController::class);
        $request = new Request(
            method: 'GET',
            path: '/user',
            cookies: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('guest', $body['userId'] ?? null);
    }

    // ======================== Parameter Resolution - Body ========================

    public function testResolveParameterFromBody(): void
    {
        $app = (new App())->registerController(BodyParamController::class);
        $request = new Request(
            method: 'POST',
            path: '/create',
            body: ['name' => 'Alice', 'age' => 30],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('Alice', $body['name'] ?? null);
    }

    public function testResolveParameterFromBodyWithDefault(): void
    {
        $app = (new App())->registerController(BodyParamWithDefaultController::class);
        $request = new Request(
            method: 'POST',
            path: '/process',
            body: null,
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertArrayHasKey('received', $body);
    }

    // ======================== Parameter Resolution - Complex Scenarios ========================

    public function testResolveMultipleParameterTypes(): void
    {
        $app = (new App())->registerController(MultiParamTypeController::class);
        $request = new Request(
            method: 'POST',
            path: '/combined/42',
            body: ['data' => 'payload'],
            headers: ['X-Custom' => 'header-val'],
            cookies: ['track' => 'cookie-val'],
            queryParams: ['page' => ['1']],
            pathParams: ['id' => '42'],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
    }

    public function testResolveNullableParameter(): void
    {
        $app = (new App())->registerController(NullableParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/check',
            queryParams: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertNull($body['optional'] ?? 'NOTFOUND');
    }

    // ======================== Response Conversion ========================

    public function testConvertResponseObjectReturnValue(): void
    {
        $app = (new App())->registerController(ResponseObjectController::class);
        $request = new Request('GET', '/response', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(201, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('created', $body['status'] ?? null);
    }

    public function testConvertArrayReturnValue(): void
    {
        $app = (new App())->registerController(ArrayReturnController::class);
        $request = new Request('GET', '/array', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('application/json', $response->headers['Content-Type'] ?? null);
    }

    public function testConvertStringReturnValue(): void
    {
        $app = (new App())->registerController(StringReturnController::class);
        $request = new Request('GET', '/string', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('Hello World', $response->body);
        $this->assertSame('text/plain', $response->headers['Content-Type'] ?? null);
    }

    public function testConvertNullReturnValue(): void
    {
        $app = (new App())->registerController(NullReturnController::class);
        $request = new Request('GET', '/null', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(204, $response->statusCode);
    }

    public function testConvertScalarReturnValue(): void
    {
        $app = (new App())->registerController(ScalarReturnController::class);
        $request = new Request('GET', '/scalar', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('result', \array_key_first($body) ?: null);
    }

    public function testConvertObjectReturnValue(): void
    {
        $app = (new App())->registerController(ObjectReturnController::class);
        $request = new Request('GET', '/object', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsObject($body) || $this->assertIsArray($body);
    }

    // ======================== Handler Matching ========================

    public function testHandlerAlwaysMatches(): void
    {
        $controller = new SimpleResponseController();
        $method = (new ReflectionClass($controller))->getMethod('getData');
        $handler = new ControllerMethodHandler($controller, $method);

        $request = new Request('GET', '/any', null);
        $this->assertTrue($handler->matches($request));
    }

    public function testHandlerInvokesCorrectMethod(): void
    {
        $app = (new App())->registerController(MethodCallVerificationController::class);
        $request = new Request('GET', '/verify', null);

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame('verified', $body['status'] ?? null);
    }

    // ======================== Edge Cases ========================

    public function testResolveParameterSkipsNonPublicMethods(): void
    {
        $app = (new App())->registerController(PrivateMethodController::class);

        $routes = $app->routes();
        // Should only register public method
        $this->assertCount(1, $routes);
        $this->assertSame('public', $routes[0]['path']);
    }

    public function testResolveParameterWithEmptyQueryParams(): void
    {
        $app = (new App())->registerController(QueryParamController::class);
        $request = new Request(
            method: 'GET',
            path: '/search',
            queryParams: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        // Should not throw; will use default or null
        $response = $handler->handle($request);
        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveRequiredParameterMissing(): void
    {
        $controller = new MissingRequiredParamController();
        $method = (new ReflectionClass($controller))->getMethod('getData');
        $handler = new ControllerMethodHandler($controller, $method);

        $request = new Request(
            method: 'GET',
            path: '/test',
            queryParams: [],
        );

        // Should throw for missing required parameter
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('Cannot resolve required parameter');
        $handler->handle($request);
    }

    public function testResolveComplexTypeWithoutDependency(): void
    {
        $controller = new ComplexTypeParamController();
        $method = (new ReflectionClass($controller))->getMethod('getData');
        $handler = new ControllerMethodHandler($controller, $method);

        $request = new Request(
            method: 'GET',
            path: '/test',
            queryParams: [],
        );

        // Should throw for non-builtin type without resolver
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('Cannot resolve parameter');
        $handler->handle($request);
    }

    public function testDefaultValueWithoutParam(): void
    {
        $app = (new App())->registerController(DefaultValueController::class);
        $request = new Request(
            method: 'GET',
            path: '/items',
            queryParams: [],
        );

        $handler = $app->findHandler($request);
        $this->assertNotNull($handler);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $body = $response->body;
        $this->assertIsArray($body);
        $this->assertSame(10, $body['count'] ?? null);
    }
}

// ======================== Test Controllers ========================

final class QueryParamController
{
    #[Get('/search')]
    public function search(string $q = new Query('default-query')): array
    {
        return ['query' => $q];
    }
}

final class QueryParamWithDefaultController
{
    #[Get('/items')]
    public function list(string $limit = new Query('default-limit')): array
    {
        return ['limit' => $limit];
    }
}

final class PathParamController
{
    #[Get('/users/{id}')]
    public function show(string $id = new Path('0')): array
    {
        return ['id' => $id];
    }
}

final class PathParamWithDefaultController
{
    #[Get('/posts')]
    public function list(string $postId = new Path('0')): array
    {
        return ['postId' => $postId];
    }
}

final class HeaderParamController
{
    #[Get('/auth')]
    public function check(string $Authorization = new Header('none')): array
    {
        return ['authHeader' => $Authorization];
    }
}

final class HeaderParamWithDefaultController
{
    #[Get('/api')]
    public function list(string $ContentType = new Header('application/json')): array
    {
        return ['contentType' => $ContentType];
    }
}

final class CookieParamController
{
    #[Get('/session')]
    public function get(string $session_id = new Cookie('default')): array
    {
        return ['sessionId' => $session_id];
    }
}

final class CookieParamWithDefaultController
{
    #[Get('/user')]
    public function get(string $userId = new Cookie('guest')): array
    {
        return ['userId' => $userId];
    }
}

final class BodyParamController
{
    /** @param array<string, mixed> $body */
    #[Post('/create')]
    public function create(array $body = new Body()): array
    {
        return $body;
    }
}

final class BodyParamWithDefaultController
{
    /** @param array<string, mixed> $body */
    #[Post('/process')]
    public function process(array $body = new Body(default: [])): array
    {
        return ['received' => $body];
    }
}

final class MultiParamTypeController
{
    /** @param array<string, mixed> $body */
    #[Post('/combined/{id}')]
    public function handle(
        string $id = new Path('0'),
        array $body = new Body(),
        string $page = new Query('1'),
        string $X_Custom = new Header('default'),
        string $track = new Cookie('default'),
    ): array {
        return [
            'id' => $id,
            'body' => $body,
            'page' => $page,
            'header' => $X_Custom,
            'cookie' => $track,
        ];
    }
}

final class NullableParamController
{
    #[Get('/check')]
    public function check(?string $optional = null): array
    {
        return ['optional' => $optional];
    }
}

final class ResponseObjectController
{
    #[Get('/response')]
    public function get(): Response
    {
        return new Response(
            statusCode: 201,
            body: ['status' => 'created'],
            headers: ['X-Custom' => 'value'],
        );
    }
}

final class ArrayReturnController
{
    /** @return array<string, string> */
    #[Get('/array')]
    public function get(): array
    {
        return ['data' => 'array'];
    }
}

final class StringReturnController
{
    #[Get('/string')]
    public function get(): string
    {
        return 'Hello World';
    }
}

final class NullReturnController
{
    #[Get('/null')]
    public function get(): void
    {
        // Intentionally returns null
    }
}

final class ScalarReturnController
{
    #[Get('/scalar')]
    public function get(): int
    {
        return 42;
    }
}

final class ObjectReturnController
{
    #[Get('/object')]
    public function get(): object
    {
        return (object)['key' => 'value'];
    }
}

final class MethodCallVerificationController
{
    #[Get('/verify')]
    public function verify(): array
    {
        return ['status' => 'verified'];
    }
}

final class PrivateMethodController
{
    #[Get('/public')]
    public function publicEndpoint(): array
    {
        return ['public' => true];
    }

    // Private method should not be registered
    #[Get('/private')]
    private function privateEndpoint(): array
    {
        return ['private' => true];
    }
}

final class MissingRequiredParamController
{
    #[Get('/test')]
    public function getData(string $required): array
    {
        return ['data' => $required];
    }
}

final class ComplexTypeParamController
{
    #[Get('/test')]
    public function getData(\DateTime $date): array
    {
        return ['date' => $date->format('Y-m-d')];
    }
}

final class DefaultValueController
{
    #[Get('/items')]
    public function list(int $count = 10): array
    {
        return ['count' => $count];
    }
}

final class SimpleResponseController
{
    #[Get('/data')]
    public function getData(): array
    {
        return ['data' => 'value'];
    }
}
