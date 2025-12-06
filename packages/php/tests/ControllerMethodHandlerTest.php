<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use ReflectionMethod;
use RuntimeException;
use Spikard\Attributes\Get;
use Spikard\Attributes\Post;
use Spikard\Handlers\ControllerMethodHandler;
use Spikard\Http\Request;
use Spikard\Http\Response;

/**
 * Tests for ControllerMethodHandler.
 *
 * Tests the parameter resolution and response conversion logic that handles
 * routing controller methods to HTTP requests. This covers parameter resolution
 * across multiple source types (query, path, header, cookie, body) and response
 * conversion for various scalar and complex types.
 *
 * @internal
 */
final class ControllerMethodHandlerTest extends TestCase
{
    /**
     * Handler Basic Behavior Tests.
     */

    public function testHandlerAlwaysMatches(): void
    {
        $controller = new SimpleTestController();
        $reflectionMethod = new ReflectionMethod($controller, 'handle');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/test', null);
        $this->assertTrue($handler->matches($request));

        $request2 = new Request('POST', '/other', null);
        $this->assertTrue($handler->matches($request2));
    }

    public function testHandlerInvokesMethodAndConvertsResponse(): void
    {
        $controller = new SimpleTestController();
        $reflectionMethod = new ReflectionMethod($controller, 'handle');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/test', null);
        $response = $handler->handle($request);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Query Parameter Resolution Tests.
     */

    public function testResolveParameterFromQueryParam(): void
    {
        $controller = new QueryParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'search');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/search',
            null,
            queryParams: ['q' => ['test']]
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromQueryParamMultipleValues(): void
    {
        $controller = new QueryParamMultiController();
        $reflectionMethod = new ReflectionMethod($controller, 'filter');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/filter',
            null,
            queryParams: ['tags' => ['php', 'testing', 'spikard']]
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromQueryParamWithDefault(): void
    {
        $controller = new QueryParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'list');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/list',
            null,
            queryParams: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Path Parameter Resolution Tests.
     */

    public function testResolveParameterFromPathParam(): void
    {
        $controller = new PathParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'getById');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/items/123',
            null,
            pathParams: ['id' => '123']
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromPathParamWithDefault(): void
    {
        $controller = new PathParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'getByIdDefault');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/items',
            null,
            pathParams: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Header Parameter Resolution Tests.
     */

    public function testResolveParameterFromHeaderParam(): void
    {
        $controller = new HeaderParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'checkAuth');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/auth',
            null,
            headers: ['Authorization' => 'Bearer token123']
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromHeaderParamWithDefault(): void
    {
        $controller = new HeaderParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'checkAuthDefault');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/auth',
            null,
            headers: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Cookie Parameter Resolution Tests.
     */

    public function testResolveParameterFromCookieParam(): void
    {
        $controller = new CookieParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'getSession');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/session',
            null,
            cookies: ['sessionId' => 'sess_abc123']
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromCookieParamWithDefault(): void
    {
        $controller = new CookieParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'getSessionDefault');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/session',
            null,
            cookies: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Body Parameter Resolution Tests.
     */

    public function testResolveParameterFromBodyParam(): void
    {
        $controller = new BodyParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'create');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $data = ['name' => 'Test Item', 'price' => 99.99];
        $request = new Request('POST', '/items', $data);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveParameterFromBodyParamWithDefault(): void
    {
        $controller = new BodyParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'createDefault');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'POST',
            '/items',
            null
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Multiple Parameter Type Resolution Tests.
     */

    public function testResolveMultipleParameterTypes(): void
    {
        $controller = new MultiParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'complexRoute');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'POST',
            '/users/42/profile',
            ['bio' => 'Test'],
            headers: ['x_custom' => 'value'],
            queryParams: ['expand' => ['details']],
            pathParams: ['id' => '42']
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResolveNullableParameter(): void
    {
        $controller = new NullableParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'maybeFilter');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/items',
            null,
            queryParams: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Response Conversion Tests.
     */

    public function testConvertResponseObject(): void
    {
        $controller = new ResponseObjectController();
        $reflectionMethod = new ReflectionMethod($controller, 'customResponse');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/custom', null);
        $response = $handler->handle($request);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(201, $response->statusCode);
    }

    public function testConvertArrayToJsonResponse(): void
    {
        $controller = new ArrayResponseController();
        $reflectionMethod = new ReflectionMethod($controller, 'getArray');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/array', null);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame(['items' => []], $response->body);
        $this->assertSame('application/json', $response->headers['Content-Type']);
    }

    public function testConvertStringToTextResponse(): void
    {
        $controller = new StringResponseController();
        $reflectionMethod = new ReflectionMethod($controller, 'getString');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/string', null);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame('Hello World', $response->body);
        $this->assertSame('text/plain', $response->headers['Content-Type']);
    }

    public function testConvertNullToNoContentResponse(): void
    {
        $controller = new NullResponseController();
        $reflectionMethod = new ReflectionMethod($controller, 'getNull');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/null', null);
        $response = $handler->handle($request);

        $this->assertSame(204, $response->statusCode);
    }

    public function testConvertScalarToJsonResponse(): void
    {
        $controller = new ScalarResponseController();
        $reflectionMethod = new ReflectionMethod($controller, 'getNumber');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/number', null);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $this->assertSame(['result' => 42], $response->body);
        $this->assertSame('application/json', $response->headers['Content-Type']);
    }

    public function testConvertObjectToJsonResponse(): void
    {
        $controller = new ObjectResponseController();
        $reflectionMethod = new ReflectionMethod($controller, 'getObject');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/object', null);
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
        $this->assertIsObject($response->body);
        $this->assertSame('application/json', $response->headers['Content-Type']);
    }

    /**
     * Error Handling Tests.
     */

    public function testMissingRequiredParameterThrowsException(): void
    {
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('Cannot resolve required parameter');

        $controller = new RequiredParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'needsParam');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/test',
            null,
            queryParams: []
        );
        $handler->handle($request);
    }

    public function testComplexTypeWithoutResolverThrowsException(): void
    {
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('Cannot resolve parameter');

        $controller = new ComplexTypeController();
        $reflectionMethod = new ReflectionMethod($controller, 'needsService');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request('GET', '/test', null);
        $handler->handle($request);
    }

    /**
     * Implicit Parameter Resolution Tests.
     */

    public function testImplicitPathParameterResolution(): void
    {
        $controller = new ImplicitPathParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'getById');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/items/99',
            null,
            pathParams: ['id' => '99']
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testImplicitQueryParameterResolution(): void
    {
        $controller = new ImplicitQueryParamController();
        $reflectionMethod = new ReflectionMethod($controller, 'search');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/search',
            null,
            queryParams: ['query' => ['spikard']]
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }

    public function testEmptyQueryParamsHandling(): void
    {
        $controller = new QueryParamDefaultController();
        $reflectionMethod = new ReflectionMethod($controller, 'list');
        $handler = new ControllerMethodHandler($controller, $reflectionMethod);

        $request = new Request(
            'GET',
            '/list',
            null,
            queryParams: []
        );
        $response = $handler->handle($request);

        $this->assertSame(200, $response->statusCode);
    }
}

/**
 * Test Controller Fixtures.
 */

/**
 * Simple test controller for basic handler behavior.
 *
 * @internal
 */
final class SimpleTestController
{
    /**
     * Handle basic request.
     *
     * @return array<string, bool>
     */
    #[Get('/test')]
    public function handle(): array
    {
        return ['ok' => true];
    }
}

/**
 * Controller for testing single query parameter resolution.
 *
 * @internal
 */
final class QueryParamController
{
    /**
     * Search with query parameter.
     *
     * @return array<string, string>
     */
    #[Get('/search')]
    public function search(string $q = 'default'): array
    {
        return ['q' => $q];
    }
}

/**
 * Controller for testing multiple query parameter values.
 *
 * @internal
 */
final class QueryParamMultiController
{
    /**
     * Filter with multiple tag values.
     *
     * @param list<string> $tags
     * @return array<string, list<string>>
     */
    #[Get('/filter')]
    public function filter(array $tags = []): array
    {
        return ['tags' => $tags];
    }
}

/**
 * Controller for testing query parameter with default value.
 *
 * @internal
 */
final class QueryParamDefaultController
{
    /**
     * List items with optional sort parameter.
     *
     * @return array<string, string>
     */
    #[Get('/list')]
    public function list(string $sort = 'name'): array
    {
        return ['sort' => $sort];
    }
}

/**
 * Controller for testing path parameter resolution.
 *
 * @internal
 */
final class PathParamController
{
    /**
     * Get item by ID.
     *
     * @return array<string, string>
     */
    #[Get('/items/:id')]
    public function getById(string $id): array
    {
        return ['id' => $id];
    }
}

/**
 * Controller for testing path parameter with default value.
 *
 * @internal
 */
final class PathParamDefaultController
{
    /**
     * Get item by ID with default.
     *
     * @return array<string, string>
     */
    #[Get('/items/:id')]
    public function getByIdDefault(string $id = 'unknown'): array
    {
        return ['id' => $id];
    }
}

/**
 * Controller for testing header parameter resolution.
 *
 * @internal
 */
final class HeaderParamController
{
    /**
     * Check authorization header.
     *
     * @return array<string, string>
     */
    #[Get('/auth')]
    public function checkAuth(string $authorization = ''): array
    {
        return ['auth' => $authorization];
    }
}

/**
 * Controller for testing header parameter with default value.
 *
 * @internal
 */
final class HeaderParamDefaultController
{
    /**
     * Check authorization with default.
     *
     * @return array<string, string>
     */
    #[Get('/auth')]
    public function checkAuthDefault(string $authorization = 'none'): array
    {
        return ['auth' => $authorization];
    }
}

/**
 * Controller for testing cookie parameter resolution.
 *
 * @internal
 */
final class CookieParamController
{
    /**
     * Get session from cookie.
     *
     * @return array<string, string>
     */
    #[Get('/session')]
    public function getSession(string $sessionId = ''): array
    {
        return ['session' => $sessionId];
    }
}

/**
 * Controller for testing cookie parameter with default value.
 *
 * @internal
 */
final class CookieParamDefaultController
{
    /**
     * Get session with default.
     *
     * @return array<string, string>
     */
    #[Get('/session')]
    public function getSessionDefault(string $sessionId = 'none'): array
    {
        return ['session' => $sessionId];
    }
}

/**
 * Controller for testing body parameter resolution.
 *
 * @internal
 */
final class BodyParamController
{
    /**
     * Create item from body.
     *
     * @param array<string, mixed> $payload
     * @return array<string, mixed>
     */
    #[Post('/items')]
    public function create(array $payload = []): array
    {
        return ['item' => $payload];
    }
}

/**
 * Controller for testing body parameter with default value.
 *
 * @internal
 */
final class BodyParamDefaultController
{
    /**
     * Create item with default body.
     *
     * @param array<string, mixed> $payload
     * @return array<string, mixed>
     */
    #[Post('/items')]
    public function createDefault(array $payload = []): array
    {
        return ['item' => $payload];
    }
}

/**
 * Controller for testing multiple parameter types together.
 *
 * @internal
 */
final class MultiParamController
{
    /**
     * Complex route with path, header, query, and body parameters.
     *
     * @param array<string, mixed> $data
     * @return array<string, mixed>
     */
    #[Post('/users/:id/profile')]
    public function complexRoute(
        string $id,
        string $expand = 'none',
        string $x_custom = 'default',
        array $data = []
    ): array {
        return [
            'id' => $id,
            'expand' => $expand,
            'x_custom' => $x_custom,
            'data' => $data,
        ];
    }
}

/**
 * Controller for testing nullable parameter resolution.
 *
 * @internal
 */
final class NullableParamController
{
    /**
     * Get items with optional filter.
     *
     * @return array<string, ?string>
     */
    #[Get('/items')]
    public function maybeFilter(?string $filter = null): array
    {
        return ['filter' => $filter];
    }
}

/**
 * Controller for testing custom Response object conversion.
 *
 * @internal
 */
final class ResponseObjectController
{
    /**
     * Return custom Response object.
     */
    #[Get('/custom')]
    public function customResponse(): Response
    {
        return new Response(statusCode: 201, body: ['created' => true]);
    }
}

/**
 * Controller for testing array response conversion.
 *
 * @internal
 */
final class ArrayResponseController
{
    /**
     * Return array response.
     *
     * @return array<string, list<mixed>>
     */
    #[Get('/array')]
    public function getArray(): array
    {
        return ['items' => []];
    }
}

/**
 * Controller for testing string response conversion.
 *
 * @internal
 */
final class StringResponseController
{
    /**
     * Return string response.
     */
    #[Get('/string')]
    public function getString(): string
    {
        return 'Hello World';
    }
}

/**
 * Controller for testing null response conversion.
 *
 * @internal
 */
final class NullResponseController
{
    /**
     * Return null response.
     */
    #[Get('/null')]
    public function getNull(): null
    {
        return null;
    }
}

/**
 * Controller for testing scalar response conversion.
 *
 * @internal
 */
final class ScalarResponseController
{
    /**
     * Return integer response.
     */
    #[Get('/number')]
    public function getNumber(): int
    {
        return 42;
    }
}

/**
 * Controller for testing object response conversion.
 *
 * @internal
 */
final class ObjectResponseController
{
    /**
     * Return object response.
     */
    #[Get('/object')]
    public function getObject(): object
    {
        return (object)['id' => 1, 'name' => 'Test'];
    }
}

/**
 * Controller for testing required parameter validation.
 *
 * @internal
 */
final class RequiredParamController
{
    /**
     * Method requiring a parameter.
     */
    #[Get('/test')]
    public function needsParam(string $required): string
    {
        return $required;
    }
}

/**
 * Controller for testing complex type parameter resolution.
 *
 * @internal
 */
final class ComplexTypeController
{
    /**
     * Method requiring complex type.
     */
    #[Get('/test')]
    public function needsService(SomeService $service): string
    {
        return 'ok';
    }
}

/**
 * Controller for testing implicit path parameter resolution.
 *
 * @internal
 */
final class ImplicitPathParamController
{
    /**
     * Get item by ID implicitly.
     *
     * @return array<string, string>
     */
    #[Get('/items/:id')]
    public function getById(string $id): array
    {
        return ['id' => $id];
    }
}

/**
 * Controller for testing implicit query parameter resolution.
 *
 * @internal
 */
final class ImplicitQueryParamController
{
    /**
     * Search with implicit query parameter.
     *
     * @return array<string, string>
     */
    #[Get('/search')]
    public function search(string $query = 'default'): array
    {
        return ['query' => $query];
    }
}

/**
 * Dummy service for complex type testing.
 *
 * @internal
 */
final class SomeService
{
}
