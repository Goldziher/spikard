<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class TestClientBehavioralTest extends TestCase
{
    /**
     * Test that request() finds and calls the correct handler.
     */
    public function testRequestCallsCorrectHandler(): void
    {
        $called = [];
        $handler = new TrackingHandler(
            static function () use (&$called): void {
                $called[] = true;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertCount(1, $called);
    }

    /**
     * Test GET convenience method.
     */
    public function testGetMethodCallsRequest(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->get('/items');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test POST convenience method without body.
     */
    public function testPostWithoutBody(): void
    {
        $handler = new TrackingRequestHandler(
            static function (Request $request): void {
                // Body should be null
            }
        );
        $app = (new App())->addRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test POST convenience method with body.
     */
    public function testPostWithBody(): void
    {
        $bodyData = ['name' => 'test'];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use ($bodyData): void {
                TestCase::assertSame($bodyData, $request->body);
            }
        );
        $app = (new App())->addRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items', $bodyData);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test request() with all HTTP methods.
     */
    public function testRequestWithAllMethods(): void
    {
        $handler = new TestHandler();
        $app = (new App())
            ->addRoute('GET', '/test', $handler)
            ->addRoute('POST', '/test', $handler)
            ->addRoute('PUT', '/test', $handler)
            ->addRoute('DELETE', '/test', $handler)
            ->addRoute('PATCH', '/test', $handler);

        $client = TestClient::create($app);

        $this->assertSame(200, $client->request('GET', '/test')->statusCode);
        $this->assertSame(200, $client->request('POST', '/test')->statusCode);
        $this->assertSame(200, $client->request('PUT', '/test')->statusCode);
        $this->assertSame(200, $client->request('DELETE', '/test')->statusCode);
        $this->assertSame(200, $client->request('PATCH', '/test')->statusCode);
    }

    /**
     * Test request() with lowercase method names.
     */
    public function testRequestWithLowercaseMethod(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('post', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test request() with mixed-case method names.
     */
    public function testRequestWithMixedCaseMethod(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('Get', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test request() passes headers correctly.
     */
    public function testRequestPassesHeaders(): void
    {
        $capturedHeaders = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedHeaders): void {
                $capturedHeaders = $request->headers;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $headers = ['X-Custom' => 'value', 'Authorization' => 'Bearer token'];
        $client->request('GET', '/test', ['headers' => $headers]);

        $this->assertSame('value', $capturedHeaders['X-Custom']);
        $this->assertSame('Bearer token', $capturedHeaders['Authorization']);
    }

    /**
     * Test request() passes cookies correctly.
     */
    public function testRequestPassesCookies(): void
    {
        $capturedCookies = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedCookies): void {
                $capturedCookies = $request->cookies;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $cookies = ['session' => 'abc123', 'user' => '42'];
        $client->request('GET', '/test', ['cookies' => $cookies]);

        $this->assertSame('abc123', $capturedCookies['session']);
        $this->assertSame('42', $capturedCookies['user']);
    }

    /**
     * Test request() passes body correctly.
     */
    public function testRequestPassesBody(): void
    {
        $capturedBody = null;
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $body = ['key' => 'value'];
        $client->request('POST', '/test', ['body' => $body]);

        $this->assertSame($body, $capturedBody);
    }

    /**
     * Test request() parses query parameters from URL.
     */
    public function testRequestParsesQueryParams(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?foo=bar&baz=qux');

        $this->assertSame(['bar'], $capturedParams['foo']);
        $this->assertSame(['qux'], $capturedParams['baz']);
    }

    /**
     * Test request() with URL-encoded query parameters.
     */
    public function testRequestWithUrlEncodedParams(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?search=hello%20world&email=test%40example.com');

        $this->assertSame(['hello world'], $capturedParams['search']);
        $this->assertSame(['test@example.com'], $capturedParams['email']);
    }

    /**
     * Test request() with multiple values for same query parameter.
     */
    public function testRequestWithMultipleQueryValues(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?tags=php&tags=rust&tags=python');

        $this->assertSame(['php', 'rust', 'python'], $capturedParams['tags']);
    }

    /**
     * Test request() with empty query string.
     */
    public function testRequestWithEmptyQueryString(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?');

        $this->assertSame([], $capturedParams);
    }

    /**
     * Test request() with path only (no query string).
     */
    public function testRequestPathOnlyExtraction(): void
    {
        $capturedPath = '';
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedPath): void {
                $capturedPath = $request->path;
            }
        );

        $app = (new App())->addRoute('GET', '/users/123', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/users/123?extra=param');

        $this->assertSame('/users/123', $capturedPath);
    }

    /**
     * Test request() with special characters in path.
     */
    public function testRequestWithSpecialCharactersInPath(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/api/v1/users', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/api/v1/users?filter[name]=test&sort=-created_at');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test request() throws for unregistered route.
     */
    public function testRequestThrowsForUnregisteredRoute(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/existing', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('GET', '/nonexistent');
    }

    /**
     * Test request() throws for unregistered method.
     */
    public function testRequestThrowsForUnregisteredMethod(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('POST', '/test');
    }

    /**
     * Test request() with files option.
     */
    public function testRequestWithFiles(): void
    {
        $capturedFiles = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedFiles): void {
                $capturedFiles = $request->files;
            }
        );

        $app = (new App())->addRoute('POST', '/upload', $handler);
        $client = TestClient::create($app);

        $files = ['profile' => 'file_data'];
        $client->request('POST', '/upload', ['files' => $files]);

        $this->assertSame($files, $capturedFiles);
    }

    /**
     * Test request() uses explicit body over files.
     */
    public function testRequestPreferBodyOverFiles(): void
    {
        $capturedBody = null;
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $body = ['explicit' => 'body'];
        $files = ['file' => 'data'];
        $client->request('POST', '/test', ['body' => $body, 'files' => $files]);

        $this->assertSame($body, $capturedBody);
    }

    /**
     * Test request() uses files as body when no explicit body.
     */
    public function testRequestUsesFilesAsBodyWhenNoBody(): void
    {
        $capturedBody = null;
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $files = ['file' => 'data'];
        $client->request('POST', '/test', ['files' => $files]);

        $this->assertSame($files, $capturedBody);
    }

    /**
     * Test request() with invalid headers option (not array).
     */
    public function testRequestWithInvalidHeadersOptionIsIgnored(): void
    {
        $capturedHeaders = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedHeaders): void {
                $capturedHeaders = $request->headers;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        // Non-array headers should be ignored (converted to empty array)
        $client->request('GET', '/test', ['headers' => 'invalid']);

        $this->assertSame([], $capturedHeaders);
    }

    /**
     * Test request() with invalid cookies option (not array).
     */
    public function testRequestWithInvalidCookiesOptionIsIgnored(): void
    {
        $capturedCookies = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedCookies): void {
                $capturedCookies = $request->cookies;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        // Non-array cookies should be ignored
        $client->request('GET', '/test', ['cookies' => 'invalid']);

        $this->assertSame([], $capturedCookies);
    }

    /**
     * Test request() with invalid files option (not array).
     */
    public function testRequestWithInvalidFilesOptionIsIgnored(): void
    {
        $capturedFiles = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedFiles): void {
                $capturedFiles = $request->files;
            }
        );

        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        // Non-array files should be ignored
        $client->request('POST', '/test', ['files' => 'invalid']);

        $this->assertSame([], $capturedFiles);
    }

    /**
     * Test connectWebSocket throws without extension.
     */
    public function testConnectWebSocketThrowsWithoutExtension(): void
    {
        $app = (new App());
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('WebSocket');
        $client->connectWebSocket('/ws');
    }

    /**
     * Test connectSse throws without extension.
     */
    public function testConnectSseThrowsWithoutExtension(): void
    {
        $app = (new App());
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('SSE');
        $client->connectSse('/events');
    }

    /**
     * Test close() method does not throw.
     */
    public function testCloseDoesNotThrow(): void
    {
        $app = (new App());
        $client = TestClient::create($app);

        $client->close(); // Should not throw
    }

    /**
     * Test app() returns the same app instance.
     */
    public function testAppReturnsCorrectInstance(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertSame($app, $client->app());
    }

    /**
     * Test create() factory method.
     */
    public function testCreateFactoryMethod(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertInstanceOf(TestClient::class, $client);
    }

    /**
     * Test multiple requests in sequence.
     */
    public function testMultipleSequentialRequests(): void
    {
        $handler1 = new TestHandler();
        $handler2 = new TestHandler();

        $app = (new App())
            ->addRoute('GET', '/first', $handler1)
            ->addRoute('GET', '/second', $handler2);

        $client = TestClient::create($app);

        $response1 = $client->get('/first');
        $response2 = $client->get('/second');

        $this->assertSame(200, $response1->statusCode);
        $this->assertSame(200, $response2->statusCode);
    }

    /**
     * Test query parameters with empty values.
     */
    public function testQueryParamsWithEmptyValues(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?key1=&key2=value&key3=');

        $this->assertSame([''], $capturedParams['key1']);
        $this->assertSame(['value'], $capturedParams['key2']);
        $this->assertSame([''], $capturedParams['key3']);
    }

    /**
     * Test query params with special encoded characters.
     */
    public function testQueryParamsWithEncodedSpecialChars(): void
    {
        $capturedParams = [];
        $handler = new TrackingRequestHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?q=hello%2Bworld&special=%3C%3E');

        $this->assertSame(['hello+world'], $capturedParams['q']);
        $this->assertSame(['<>'], $capturedParams['special']);
    }

    /**
     * Test empty options array.
     */
    public function testRequestWithEmptyOptions(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', []);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * Test request with no options (defaults to empty array).
     */
    public function testRequestWithNoOptions(): void
    {
        $handler = new TestHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertSame(200, $response->statusCode);
    }
}

// Test helpers

final class TrackingHandler implements HandlerInterface
{
    /**
     * @param callable(): void $callback
     */
    public function __construct(private readonly mixed $callback)
    {
    }

    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        ($this->callback)();
        return Response::json(['ok' => true], 200);
    }
}

final class TrackingRequestHandler implements HandlerInterface
{
    /**
     * @param callable(Request): void $callback
     */
    public function __construct(private readonly mixed $callback)
    {
    }

    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        ($this->callback)($request);
        return Response::json(['ok' => true], 200);
    }
}

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
