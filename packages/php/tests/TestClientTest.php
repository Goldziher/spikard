<?php

declare(strict_types=1);

namespace Spikard\Tests;

use RuntimeException;
use Spikard\App;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;
use Throwable;

/**
 * Comprehensive test suite for TestClient.
 *
 * This test suite consolidates:
 * - TestClientBehavioralTest (34 tests)
 * - TestClientBehavioralExtensionTest (32 tests)
 * - TestClientExtendedTest (39 tests)
 *
 * Tests are organized into logical groups:
 * - HTTP Methods Tests
 * - WebSocket/SSE Tests
 * - Query Parameter Tests
 * - Options/Assertions Tests
 * - Convenience Methods Tests
 * - Error Handling Tests
 * - Lifecycle Tests
 */
final class TestClientTest extends TestClientTestCase
{
    // ======================== Factory and Initialization Tests ========================

    /**
     * @test
     */
    public function testCreateReturnsTestClient(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertInstanceOf(TestClient::class, $client);
    }

    /**
     * @test
     */
    public function testAppMethodReturnsCorrectInstance(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertSame($app, $client->app());
    }

    /**
     * @test
     */
    public function testCreateMultipleClients(): void
    {
        $app = new App();
        $client1 = TestClient::create($app);
        $client2 = TestClient::create($app);

        $this->assertNotSame($client1, $client2);
        $this->assertSame($app, $client1->app());
        $this->assertSame($app, $client2->app());
    }

    // ======================== HTTP Methods Tests ========================

    /**
     * @test
     * @dataProvider httpMethodsProvider
     */
    public function testRequestWithAllHttpMethods(string $method, string $expectedMethod): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute($method, '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request($method, '/test');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     * @dataProvider caseInsensitiveMethodsProvider
     */
    public function testRequestWithCaseInsensitiveMethods(string $method, string $expectedMethod): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute($expectedMethod, '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request($method, '/test');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testRequestWithAllMethodsCombined(): void
    {
        $handler = $this->createBasicHandler();
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

    // ======================== Convenience Methods Tests ========================

    /**
     * @test
     */
    public function testGetMethodCallsRequest(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->get('/items');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testPostWithoutBody(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testPostWithBody(): void
    {
        $bodyData = ['name' => 'test'];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use ($bodyData): void {
                TestClientTest::assertEquals($bodyData, $request->body);
            }
        );
        $app = (new App())->addRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items', $bodyData);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Headers and Cookies Tests ========================

    /**
     * @test
     */
    public function testRequestPassesHeaders(): void
    {
        $capturedHeaders = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestPassesCookies(): void
    {
        $capturedCookies = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestPassesBody(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
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

    // ======================== Query Parameter Tests ========================

    /**
     * @test
     */
    public function testRequestParsesQueryParams(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestWithUrlEncodedParams(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestWithMultipleQueryValues(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestWithEmptyQueryString(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testQueryParamsWithEmptyValues(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testQueryParamsWithEncodedSpecialChars(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     * @dataProvider queryParamEdgeCasesProvider
     */
    public function testQueryParamEdgeCases(string $url): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', $url);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testParseQueryParamsWithMultipleAmpersand(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/filter', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/filter?a=1&&b=2&&&c=3');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testParseQueryParamsEmptyKey(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?=value&valid=yes');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testParseQueryParamsWithPlus(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?q=hello+world');
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Path Handling Tests ========================

    /**
     * @test
     */
    public function testRequestPathOnlyExtraction(): void
    {
        $capturedPath = '';
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestWithSpecialCharactersInPath(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/api/v1/users', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/api/v1/users?filter[name]=test&sort=-created_at');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testPathWithSpecialCharacters(): void
    {
        $handler = $this->createBasicHandler();
        $path = '/api/v1/resource-123';
        $app = (new App())->addRoute('GET', $path, $handler);

        $client = TestClient::create($app);
        $response = $client->request('GET', $path);

        $this->assertInstanceOf(Response::class, $response);
    }

    // ======================== File Upload Tests ========================

    /**
     * @test
     */
    public function testRequestWithFiles(): void
    {
        $capturedFiles = [];
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestPreferBodyOverFiles(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestUsesFilesAsBodyWhenNoBody(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
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
     * @test
     */
    public function testRequestWithFilesAsBody(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('POST', '/file-body', $handler);
        $client = TestClient::create($app);

        $files = ['file.txt' => 'content'];
        $response = $client->request('POST', '/file-body', ['files' => $files]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testRequestPreferExplicitBodyOverFiles(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('POST', '/priority', $handler);
        $client = TestClient::create($app);

        $body = ['explicit' => 'body'];
        $files = ['file.txt' => 'content'];
        $response = $client->request('POST', '/priority', [
            'body' => $body,
            'files' => $files,
        ]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Options/Assertions Tests ========================

    /**
     * @test
     */
    public function testRequestWithEmptyOptions(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', []);
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testRequestWithNoOptions(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    /**
     * @test
     */
    public function testRequestWithInvalidHeadersOptionIsIgnored(): void
    {
        $capturedHeaders = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedHeaders): void {
                $capturedHeaders = $request->headers;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test', ['headers' => 'invalid']);

        $this->assertSame([], $capturedHeaders);
    }

    /**
     * @test
     */
    public function testRequestWithInvalidCookiesOptionIsIgnored(): void
    {
        $capturedCookies = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedCookies): void {
                $capturedCookies = $request->cookies;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test', ['cookies' => 'invalid']);

        $this->assertSame([], $capturedCookies);
    }

    /**
     * @test
     */
    public function testRequestWithInvalidFilesOptionIsIgnored(): void
    {
        $capturedFiles = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedFiles): void {
                $capturedFiles = $request->files;
            }
        );

        $app = (new App())->addRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('POST', '/test', ['files' => 'invalid']);

        $this->assertSame([], $capturedFiles);
    }

    /**
     * @test
     */
    public function testRequestWithAllOptionsAtOnce(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('POST', '/combined', $handler);
        $client = TestClient::create($app);

        $response = $client->request('POST', '/combined?page=1', [
            'headers' => ['Authorization' => 'Bearer token'],
            'cookies' => ['session' => '123'],
            'body' => ['key' => 'value'],
            'files' => [],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== WebSocket/SSE Tests ========================

    /**
     * @test
     */
    public function testConnectWebSocketThrowsWithoutExtension(): void
    {
        $app = new App();
        $wsHandler = $this->createDummyWebSocketHandler();
        $app = $app->addWebSocket('/ws', $wsHandler);

        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('WebSocket');
        $client->connectWebSocket('/ws');
    }

    /**
     * @test
     */
    public function testConnectSseThrowsWithoutExtension(): void
    {
        $app = new App();
        $sseProducer = $this->createDummySseEventProducer();
        $app = $app->addSse('/events', $sseProducer);

        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('SSE');
        $client->connectSse('/events');
    }

    // ======================== Error Handling Tests ========================

    /**
     * @test
     */
    public function testRequestThrowsForUnregisteredRoute(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/existing', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('GET', '/nonexistent');
    }

    /**
     * @test
     */
    public function testRequestThrowsForUnregisteredMethod(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('POST', '/test');
    }

    // ======================== Lifecycle Tests ========================

    /**
     * @test
     */
    public function testCloseDoesNotThrow(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        // Should not throw an exception when closing
        $client->close();
        $this->expectNotToPerformAssertions();
    }

    /**
     * @test
     */
    public function testCloseMethodIsIdempotent(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        // Should not throw when called multiple times
        $client->close();
        $client->close();
        $client->close();
        $this->expectNotToPerformAssertions();
    }

    /**
     * @test
     */
    public function testMultipleSequentialRequests(): void
    {
        $handler1 = $this->createBasicHandler();
        $handler2 = $this->createBasicHandler();

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
     * @test
     */
    public function testMultipleRequests(): void
    {
        $handler = $this->createBasicHandler();
        $app = (new App())
            ->addRoute('GET', '/first', $handler)
            ->addRoute('GET', '/second', $handler);

        $client = TestClient::create($app);
        $response1 = $client->get('/first');
        $response2 = $client->get('/second');

        $this->assertInstanceOf(Response::class, $response1);
        $this->assertInstanceOf(Response::class, $response2);
    }

    /**
     * @test
     */
    public function testRequestCallsCorrectHandler(): void
    {
        $called = [];
        $handler = $this->createTrackingHandler(
            static function () use (&$called): void {
                $called[] = true;
            }
        );

        $app = (new App())->addRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertCount(1, $called);
    }
}
