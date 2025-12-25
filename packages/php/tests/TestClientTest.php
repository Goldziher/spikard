<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\Attributes\DataProvider;
use PHPUnit\Framework\Attributes\Test;
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

    #[Test]
    public function testCreateReturnsTestClient(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertInstanceOf(TestClient::class, $client);
    }

    #[Test]
    public function testAppMethodReturnsCorrectInstance(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        $this->assertSame($app, $client->app());
    }

    #[Test]
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

    #[Test]
    #[DataProvider('httpMethodsProvider')]
    public function testRequestWithAllHttpMethods(string $method, string $expectedMethod): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute($method, '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request($method, '/test');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    #[DataProvider('caseInsensitiveMethodsProvider')]
    public function testRequestWithCaseInsensitiveMethods(string $method, string $expectedMethod): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute($expectedMethod, '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request($method, '/test');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testRequestWithAllMethodsCombined(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $app = $this->addRoute($app, 'POST', '/test', $handler);
        $app = $this->addRoute($app, 'PUT', '/test', $handler);
        $app = $this->addRoute($app, 'DELETE', '/test', $handler);
        $app = $this->addRoute($app, 'PATCH', '/test', $handler);

        $client = TestClient::create($app);

        $this->assertSame(200, $client->request('GET', '/test')->statusCode);
        $this->assertSame(200, $client->request('POST', '/test')->statusCode);
        $this->assertSame(200, $client->request('PUT', '/test')->statusCode);
        $this->assertSame(200, $client->request('DELETE', '/test')->statusCode);
        $this->assertSame(200, $client->request('PATCH', '/test')->statusCode);
    }

    // ======================== Convenience Methods Tests ========================

    #[Test]
    public function testGetMethodCallsRequest(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->get('/items');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testPostWithoutBody(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testPostWithBody(): void
    {
        $bodyData = ['name' => 'test'];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use ($bodyData): void {
                TestClientTest::assertEquals($bodyData, $request->body);
            }
        );
        $app = $this->appWithRoute('POST', '/items', $handler);
        $client = TestClient::create($app);

        $response = $client->post('/items', $bodyData);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Headers and Cookies Tests ========================

    #[Test]
    public function testRequestPassesHeaders(): void
    {
        $capturedHeaders = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedHeaders): void {
                $capturedHeaders = $request->headers;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $headers = ['X-Custom' => 'value', 'Authorization' => 'Bearer token'];
        $client->request('GET', '/test', ['headers' => $headers]);

        $this->assertSame('value', $capturedHeaders['X-Custom']);
        $this->assertSame('Bearer token', $capturedHeaders['Authorization']);
    }

    #[Test]
    public function testRequestPassesCookies(): void
    {
        $capturedCookies = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedCookies): void {
                $capturedCookies = $request->cookies;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $cookies = ['session' => 'abc123', 'user' => '42'];
        $client->request('GET', '/test', ['cookies' => $cookies]);

        $this->assertSame('abc123', $capturedCookies['session']);
        $this->assertSame('42', $capturedCookies['user']);
    }

    #[Test]
    public function testRequestPassesBody(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = $this->appWithRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $body = ['key' => 'value'];
        $client->request('POST', '/test', ['body' => $body]);

        $this->assertSame($body, $capturedBody);
    }

    // ======================== Query Parameter Tests ========================

    #[Test]
    public function testRequestParsesQueryParams(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?foo=bar&baz=qux');

        $this->assertSame(['bar'], $capturedParams['foo']);
        $this->assertSame(['qux'], $capturedParams['baz']);
    }

    #[Test]
    public function testRequestWithUrlEncodedParams(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?search=hello%20world&email=test%40example.com');

        $this->assertSame(['hello world'], $capturedParams['search']);
        $this->assertSame(['test@example.com'], $capturedParams['email']);
    }

    #[Test]
    public function testRequestWithMultipleQueryValues(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?tags=php&tags=rust&tags=python');

        $this->assertSame(['php', 'rust', 'python'], $capturedParams['tags']);
    }

    #[Test]
    public function testRequestWithEmptyQueryString(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?');

        $this->assertSame([], $capturedParams);
    }

    #[Test]
    public function testQueryParamsWithEmptyValues(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?key1=&key2=value&key3=');

        $this->assertSame([''], $capturedParams['key1']);
        $this->assertSame(['value'], $capturedParams['key2']);
        $this->assertSame([''], $capturedParams['key3']);
    }

    #[Test]
    public function testQueryParamsWithEncodedSpecialChars(): void
    {
        $capturedParams = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedParams): void {
                $capturedParams = $request->queryParams;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test?q=hello%2Bworld&special=%3C%3E');

        $this->assertSame(['hello+world'], $capturedParams['q']);
        $this->assertSame(['<>'], $capturedParams['special']);
    }

    #[Test]
    #[DataProvider('queryParamEdgeCasesProvider')]
    public function testQueryParamEdgeCases(string $url): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', $url);
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testParseQueryParamsWithMultipleAmpersand(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/filter', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/filter?a=1&&b=2&&&c=3');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testParseQueryParamsEmptyKey(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?=value&valid=yes');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testParseQueryParamsWithPlus(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?q=hello+world');
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Path Handling Tests ========================

    #[Test]
    public function testRequestPathOnlyExtraction(): void
    {
        $capturedPath = '';
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedPath): void {
                $capturedPath = $request->path;
            }
        );

        $app = $this->appWithRoute('GET', '/users/123', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/users/123?extra=param');

        $this->assertSame('/users/123', $capturedPath);
    }

    #[Test]
    public function testRequestWithSpecialCharactersInPath(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/api/v1/users', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/api/v1/users?filter[name]=test&sort=-created_at');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testPathWithSpecialCharacters(): void
    {
        $handler = $this->createBasicHandler();
        $path = '/api/v1/resource-123';
        $app = $this->appWithRoute('GET', $path, $handler);

        $client = TestClient::create($app);
        $response = $client->request('GET', $path);

        $this->assertInstanceOf(Response::class, $response);
    }

    // ======================== File Upload Tests ========================

    #[Test]
    public function testRequestWithFiles(): void
    {
        $capturedFiles = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedFiles): void {
                $capturedFiles = $request->files;
            }
        );

        $app = $this->appWithRoute('POST', '/upload', $handler);
        $client = TestClient::create($app);

        $files = ['profile' => 'file_data'];
        $client->request('POST', '/upload', ['files' => $files]);

        $this->assertSame($files, $capturedFiles);
    }

    #[Test]
    public function testRequestPreferBodyOverFiles(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = $this->appWithRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $body = ['explicit' => 'body'];
        $files = ['file' => 'data'];
        $client->request('POST', '/test', ['body' => $body, 'files' => $files]);

        $this->assertSame($body, $capturedBody);
    }

    #[Test]
    public function testRequestUsesFilesAsBodyWhenNoBody(): void
    {
        $capturedBody = null;
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedBody): void {
                $capturedBody = $request->body;
            }
        );

        $app = $this->appWithRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $files = ['file' => 'data'];
        $client->request('POST', '/test', ['files' => $files]);

        $this->assertSame($files, $capturedBody);
    }

    #[Test]
    public function testRequestWithFilesAsBody(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('POST', '/file-body', $handler);
        $client = TestClient::create($app);

        $files = ['file.txt' => 'content'];
        $response = $client->request('POST', '/file-body', ['files' => $files]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testRequestPreferExplicitBodyOverFiles(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('POST', '/priority', $handler);
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

    #[Test]
    public function testRequestWithEmptyOptions(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', []);
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testRequestWithNoOptions(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    #[Test]
    public function testRequestWithInvalidHeadersOptionIsIgnored(): void
    {
        $capturedHeaders = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedHeaders): void {
                $capturedHeaders = $request->headers;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test', ['headers' => 'invalid']);

        $this->assertSame([], $capturedHeaders);
    }

    #[Test]
    public function testRequestWithInvalidCookiesOptionIsIgnored(): void
    {
        $capturedCookies = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedCookies): void {
                $capturedCookies = $request->cookies;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('GET', '/test', ['cookies' => 'invalid']);

        $this->assertSame([], $capturedCookies);
    }

    #[Test]
    public function testRequestWithInvalidFilesOptionIsIgnored(): void
    {
        $capturedFiles = [];
        $handler = $this->createRequestTrackingHandler(
            static function (Request $request) use (&$capturedFiles): void {
                $capturedFiles = $request->files;
            }
        );

        $app = $this->appWithRoute('POST', '/test', $handler);
        $client = TestClient::create($app);

        $client->request('POST', '/test', ['files' => 'invalid']);

        $this->assertSame([], $capturedFiles);
    }

    #[Test]
    public function testRequestWithAllOptionsAtOnce(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('POST', '/combined', $handler);
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

    #[Test]
    public function testConnectWebSocketThrowsWithoutExtension(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$app = new \\Spikard\\App();'
            . '$app = $app->addWebSocket(\'/ws\', new class implements \\Spikard\\Handlers\\WebSocketHandlerInterface {'
            . '    public function onConnect(): void {}'
            . '    public function onMessage(string $message): void {}'
            . '    public function onClose(int $code, ?string $reason = null): void {}'
            . '});'
            . '$client = \\Spikard\\Testing\\TestClient::create($app);'
            . '$client->connectWebSocket(\'/ws\');'
        );

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString('WebSocket client requires the native extension', $output);
    }

    #[Test]
    public function testConnectSseThrowsWithoutExtension(): void
    {
        [$exitCode, $output] = run_without_extension(
            '$app = new \\Spikard\\App();'
            . '$app = $app->addSse(\'/events\', new class implements \\Spikard\\Handlers\\SseEventProducerInterface {'
            . '    public function __invoke(): \\Generator { if (false) { yield \'\'; } }'
            . '});'
            . '$client = \\Spikard\\Testing\\TestClient::create($app);'
            . '$client->connectSse(\'/events\');'
        );

        $this->assertNotSame(0, $exitCode);
        $this->assertStringContainsString('SSE client requires the native extension', $output);
    }

    // ======================== Error Handling Tests ========================

    #[Test]
    public function testRequestThrowsForUnregisteredRoute(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/existing', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('GET', '/nonexistent');
    }

    #[Test]
    public function testRequestThrowsForUnregisteredMethod(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('POST', '/test');
    }

    // ======================== Lifecycle Tests ========================

    #[Test]
    public function testCloseDoesNotThrow(): void
    {
        $app = new App();
        $client = TestClient::create($app);

        // Should not throw an exception when closing
        $client->close();
        $this->expectNotToPerformAssertions();
    }

    #[Test]
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

    #[Test]
    public function testMultipleSequentialRequests(): void
    {
        $handler1 = $this->createBasicHandler();
        $handler2 = $this->createBasicHandler();

        $app = $this->appWithRoute('GET', '/first', $handler1);
        $app = $this->addRoute($app, 'GET', '/second', $handler2);

        $client = TestClient::create($app);

        $response1 = $client->get('/first');
        $response2 = $client->get('/second');

        $this->assertSame(200, $response1->statusCode);
        $this->assertSame(200, $response2->statusCode);
    }

    #[Test]
    public function testMultipleRequests(): void
    {
        $handler = $this->createBasicHandler();
        $app = $this->appWithRoute('GET', '/first', $handler);
        $app = $this->addRoute($app, 'GET', '/second', $handler);

        $client = TestClient::create($app);
        $response1 = $client->get('/first');
        $response2 = $client->get('/second');

        $this->assertInstanceOf(Response::class, $response1);
        $this->assertInstanceOf(Response::class, $response2);
    }

    #[Test]
    public function testRequestCallsCorrectHandler(): void
    {
        $called = [];
        $handler = $this->createTrackingHandler(
            static function () use (&$called): void {
                $called[] = true;
            }
        );

        $app = $this->appWithRoute('GET', '/test', $handler);
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertCount(1, $called);
    }
}
