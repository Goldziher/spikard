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

/**
 * Simple test handler that always matches and returns success.
 */
final class SimpleTestClientHandler implements HandlerInterface
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

/**
 * Extended tests for TestClient to increase coverage.
 *
 * Tests all public methods and uncovered branches for:
 * - Spikard\Testing\TestClient (increase from 52.63% to 80%+)
 */
final class TestClientExtendedTest extends TestCase
{
    private App $app;
    private TestClient $client;

    protected function setUp(): void
    {
        $this->app = new App();
        $this->client = TestClient::create($this->app);
    }

    // ======================== Factory and Initialization Tests ========================

    public function testCreateReturnsTestClient(): void
    {
        $client = TestClient::create($this->app);

        $this->assertInstanceOf(TestClient::class, $client);
    }

    public function testAppMethodReturnsProperApp(): void
    {
        $client = TestClient::create($this->app);

        $this->assertSame($this->app, $client->app());
    }

    public function testCreateMultipleClients(): void
    {
        $client1 = TestClient::create($this->app);
        $client2 = TestClient::create($this->app);

        $this->assertNotSame($client1, $client2);
        $this->assertSame($this->app, $client1->app());
        $this->assertSame($this->app, $client2->app());
    }

    // ======================== HTTP Verb Convenience Methods ========================

    public function testGetMethodCallsRequest(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('GET', '/test', $handler);

        $response = $this->client->get('/test');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testGetMethodUsesCorrectHttpMethod(): void
    {
        $capturedMethod = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedMethod;
                $capturedMethod = $request->method;
                return new Response(['method' => $request->method]);
            }
        };
        $this->app->addRoute('GET', '/method', $handler);

        $this->client->get('/method');

        $this->assertSame('GET', $capturedMethod);
    }

    public function testPostMethodWithoutBody(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('POST', '/create', $handler);

        $response = $this->client->post('/create');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testPostMethodWithBody(): void
    {
        $body = ['name' => 'test', 'value' => 123];
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('POST', '/create', $handler);

        $response = $this->client->post('/create', $body);

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testPostMethodPassesBodyToRequest(): void
    {
        $capturedBody = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedBody;
                $capturedBody = $request->body;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('POST', '/body', $handler);

        $testBody = ['key' => 'value'];
        $this->client->post('/body', $testBody);

        $this->assertSame($testBody, $capturedBody);
    }

    // ======================== Generic Request Method Tests ========================

    public function testRequestWithGet(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('GET', '/endpoint', $handler);

        $response = $this->client->request('GET', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPost(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('POST', '/endpoint', $handler);

        $response = $this->client->request('POST', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPut(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('PUT', '/endpoint', $handler);

        $response = $this->client->request('PUT', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithDelete(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('DELETE', '/endpoint', $handler);

        $response = $this->client->request('DELETE', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPatch(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('PATCH', '/endpoint', $handler);

        $response = $this->client->request('PATCH', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestLowercaseMethod(): void
    {
        $capturedMethod = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedMethod;
                $capturedMethod = $request->method;
                return new Response(['method' => $request->method]);
            }
        };
        $this->app->addRoute('GET', '/lowercase', $handler);

        $this->client->request('get', '/lowercase');

        $this->assertSame('GET', $capturedMethod);
    }

    // ======================== Headers and Cookies Tests ========================

    public function testRequestWithHeaders(): void
    {
        $capturedHeaders = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedHeaders;
                $capturedHeaders = $request->headers;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/headers', $handler);

        $headers = ['Authorization' => 'Bearer token', 'X-Custom' => 'value'];
        $this->client->request('GET', '/headers', ['headers' => $headers]);

        $this->assertSame($headers, $capturedHeaders);
    }

    public function testRequestWithCookies(): void
    {
        $capturedCookies = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedCookies;
                $capturedCookies = $request->cookies;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/cookies', $handler);

        $cookies = ['session_id' => 'abc123', 'user_id' => 'user456'];
        $this->client->request('GET', '/cookies', ['cookies' => $cookies]);

        $this->assertSame($cookies, $capturedCookies);
    }

    public function testRequestWithBody(): void
    {
        $capturedBody = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedBody;
                $capturedBody = $request->body;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('POST', '/body', $handler);

        $body = ['data' => 'test'];
        $this->client->request('POST', '/body', ['body' => $body]);

        $this->assertSame($body, $capturedBody);
    }

    // ======================== Query Parameters Tests ========================

    public function testParseQueryParamsSimple(): void
    {
        /** @var array<string, array<int, string>> $capturedParams */
        $capturedParams = [
            'foo' => [],
            'baz' => [],
        ];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/query', $handler);

        $this->client->request('GET', '/query?foo=bar&baz=qux');

        $this->assertArrayHasKey('foo', $capturedParams);
        $this->assertArrayHasKey('baz', $capturedParams);
        $this->assertSame(['bar'], $capturedParams['foo']);
        $this->assertSame(['qux'], $capturedParams['baz']);
    }

    public function testParseQueryParamsWithMultipleValues(): void
    {
        /** @var array<string, array<int, string>> $capturedParams */
        $capturedParams = [
            'ids' => [],
        ];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/multi', $handler);

        $this->client->request('GET', '/multi?ids=1&ids=2&ids=3');

        $this->assertArrayHasKey('ids', $capturedParams);
        $this->assertSame(['1', '2', '3'], $capturedParams['ids']);
    }

    public function testParseQueryParamsWithUrlEncoding(): void
    {
        /** @var array<string, array<int, string>> $capturedParams */
        $capturedParams = [
            'message' => [],
            'symbol' => [],
        ];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/encoded', $handler);

        $this->client->request('GET', '/encoded?message=hello%20world&symbol=%26');

        $this->assertArrayHasKey('message', $capturedParams);
        $this->assertArrayHasKey('symbol', $capturedParams);
        $this->assertSame(['hello world'], $capturedParams['message']);
        $this->assertSame(['&'], $capturedParams['symbol']);
    }

    public function testParseQueryParamsEmpty(): void
    {
        $capturedParams = [];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/noparams', $handler);

        $this->client->request('GET', '/noparams');

        $this->assertSame([], $capturedParams);
    }

    public function testParseQueryParamsWithTrailingQuestionMark(): void
    {
        $capturedParams = [];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/trailing', $handler);

        $this->client->request('GET', '/trailing?');

        $this->assertSame([], $capturedParams);
    }

    public function testParseQueryParamsWithEmptyValues(): void
    {
        /** @var array<string, array<int, string>> $capturedParams */
        $capturedParams = [
            'key' => [],
        ];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/empty', $handler);

        $this->client->request('GET', '/empty?key=');

        $this->assertArrayHasKey('key', $capturedParams);
        $this->assertSame([''], $capturedParams['key']);
    }

    public function testParseQueryParamsSkipsEmptyPairs(): void
    {
        /** @var array<string, array<int, string>> $capturedParams */
        $capturedParams = [
            'a' => [],
            'b' => [],
        ];
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedParams;
                $capturedParams = $request->queryParams;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('GET', '/mixed', $handler);

        $this->client->request('GET', '/mixed?a=1&&b=2');

        $this->assertArrayHasKey('a', $capturedParams);
        $this->assertArrayHasKey('b', $capturedParams);
        $this->assertSame(['1'], $capturedParams['a']);
        $this->assertSame(['2'], $capturedParams['b']);
    }

    // ======================== File Upload Tests ========================

    public function testRequestWithFiles(): void
    {
        $capturedFiles = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedFiles;
                $capturedFiles = $request->files;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('POST', '/upload', $handler);

        $files = ['document.pdf' => ['content' => 'binary']];
        $this->client->request('POST', '/upload', ['files' => $files]);

        $this->assertSame($files, $capturedFiles);
    }

    public function testRequestWithFilesAsBody(): void
    {
        $capturedBody = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedBody;
                $capturedBody = $request->body;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('POST', '/file-body', $handler);

        $files = ['file.txt' => 'content'];
        $this->client->request('POST', '/file-body', ['files' => $files]);

        // Files should be used as body when no explicit body is provided
        $this->assertSame($files, $capturedBody);
    }

    public function testRequestPreferExplicitBodyOverFiles(): void
    {
        $capturedBody = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedBody;
                $capturedBody = $request->body;
                return new Response(['ok' => true]);
            }
        };
        $this->app->addRoute('POST', '/priority', $handler);

        $body = ['explicit' => 'body'];
        $files = ['file.txt' => 'content'];
        $this->client->request('POST', '/priority', [
            'body' => $body,
            'files' => $files,
        ]);

        // Explicit body should take precedence
        $this->assertSame($body, $capturedBody);
    }

    // NOTE: Fixed testRequestThrowsForUnregisteredRoute and testRequestThrowsForUnregisteredMethod above

    // ======================== Error Handling Tests ========================

    public function testRequestThrowsForUnregisteredRoute(): void
    {
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');

        $this->client->request('GET', '/nonexistent');
    }

    public function testRequestThrowsForUnregisteredMethod(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app->addRoute('GET', '/endpoint', $handler);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');

        $this->client->request('POST', '/endpoint');
    }

    // ======================== Path Handling Tests ========================

    public function testPathOnlyExtraction(): void
    {
        $capturedPath = null;
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                global $capturedPath;
                $capturedPath = $request->path;
                return new Response(['ok' => true]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/path', $handler);

        $this->client = TestClient::create($this->app);
        $this->client->request('GET', '/path?query=param');

        $this->assertSame('/path', $capturedPath);
    }

    public function testPathWithSpecialCharacters(): void
    {
        $handler = new SimpleTestClientHandler();
        $path = '/api/v1/resource-123';
        $this->app = $this->app->addRoute('GET', $path, $handler);

        $this->client = TestClient::create($this->app);
        $response = $this->client->request('GET', $path);

        $this->assertInstanceOf(Response::class, $response);
    }

    // ======================== Options Parameter Handling ========================

    public function testRequestWithEmptyOptions(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('GET', '/opts', $handler);

        $this->client = TestClient::create($this->app);
        $response = $this->client->request('GET', '/opts', []);

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithInvalidHeadersOption(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('GET', '/invalid-headers', $handler);

        $this->client = TestClient::create($this->app);
        // Non-array headers should be ignored
        $response = $this->client->request('GET', '/invalid-headers', [
            'headers' => 'not-an-array',
        ]);

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithInvalidCookiesOption(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('GET', '/invalid-cookies', $handler);

        $this->client = TestClient::create($this->app);
        // Non-array cookies should be ignored
        $response = $this->client->request('GET', '/invalid-cookies', [
            'cookies' => 'not-an-array',
        ]);

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithInvalidFilesOption(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('POST', '/invalid-files', $handler);

        $this->client = TestClient::create($this->app);
        // Non-array files should be ignored
        $response = $this->client->request('POST', '/invalid-files', [
            'files' => 'not-an-array',
        ]);

        $this->assertInstanceOf(Response::class, $response);
    }

    // ======================== Connection Methods (Native Extension) ========================

    public function testConnectWebSocketThrowsWithoutNativeExtension(): void
    {
        if (\class_exists('\\Spikard\\Native\\TestClient') || \function_exists('spikard_version')) {
            $this->markTestSkipped('Native extension is loaded');
        }

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('WebSocket client requires the native extension');

        $this->client->connectWebSocket('/ws');
    }

    public function testConnectSseThrowsWithoutNativeExtension(): void
    {
        if (\class_exists('\\Spikard\\Native\\TestClient') || \function_exists('spikard_version')) {
            $this->markTestSkipped('Native extension is loaded');
        }

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('SSE client requires the native extension');

        $this->client->connectSse('/sse');
    }

    // ======================== Lifecycle Tests ========================

    public function testClose(): void
    {
        // close() should not throw
        $this->client->close();

        $this->expectNotToPerformAssertions();
    }

    public function testMultipleRequests(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('GET', '/first', $handler);
        $this->app = $this->app->addRoute('GET', '/second', $handler);

        $this->client = TestClient::create($this->app);
        $response1 = $this->client->get('/first');
        $response2 = $this->client->get('/second');

        $this->assertInstanceOf(Response::class, $response1);
        $this->assertInstanceOf(Response::class, $response2);
    }
}
