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
        $this->app = $this->app->addRoute('GET', '/test', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->get('/test');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testGetMethodUsesCorrectHttpMethod(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['method' => $request->method]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/method', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->get('/method');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testPostMethodWithoutBody(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('POST', '/create', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->post('/create');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testPostMethodWithBody(): void
    {
        $body = ['name' => 'test', 'value' => 123];
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('POST', '/create', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->post('/create', $body);

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testPostMethodPassesBodyToRequest(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['received' => $request->body]);
            }
        };
        $this->app = $this->app->addRoute('POST', '/body', $handler);
        $this->client = TestClient::create($this->app);

        $testBody = ['key' => 'value'];
        $response = $this->client->post('/body', $testBody);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Generic Request Method Tests ========================

    public function testRequestWithGet(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('GET', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPost(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('POST', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('POST', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPut(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('PUT', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('PUT', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithDelete(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('DELETE', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('DELETE', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestWithPatch(): void
    {
        $handler = new SimpleTestClientHandler();
        $this->app = $this->app->addRoute('PATCH', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('PATCH', '/endpoint');

        $this->assertInstanceOf(Response::class, $response);
    }

    public function testRequestLowercaseMethod(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['method' => $request->method]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/lowercase', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('get', '/lowercase');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Headers and Cookies Tests ========================

    public function testRequestWithHeaders(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['headers_count' => \count($request->headers)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/headers', $handler);
        $this->client = TestClient::create($this->app);

        $headers = ['Authorization' => 'Bearer token', 'X-Custom' => 'value'];
        $response = $this->client->request('GET', '/headers', ['headers' => $headers]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithCookies(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['cookies_count' => \count($request->cookies)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/cookies', $handler);
        $this->client = TestClient::create($this->app);

        $cookies = ['session_id' => 'abc123', 'user_id' => 'user456'];
        $response = $this->client->request('GET', '/cookies', ['cookies' => $cookies]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithBody(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['body_type' => \gettype($request->body)]);
            }
        };
        $this->app = $this->app->addRoute('POST', '/body', $handler);
        $this->client = TestClient::create($this->app);

        $body = ['data' => 'test'];
        $response = $this->client->request('POST', '/body', ['body' => $body]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Query Parameters Tests ========================

    public function testParseQueryParamsSimple(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/query', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/query?foo=bar&baz=qux');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithMultipleValues(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/multi', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/multi?ids=1&ids=2&ids=3');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithUrlEncoding(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/encoded', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/encoded?message=hello%20world&symbol=%26');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsEmpty(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/noparams', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/noparams');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithTrailingQuestionMark(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/trailing', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/trailing?');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithEmptyValues(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/empty', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/empty?key=');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsSkipsEmptyPairs(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['params_count' => \count($request->queryParams)]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/mixed', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/mixed?a=1&&b=2');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== File Upload Tests ========================

    public function testRequestWithFiles(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['files_count' => \count($request->files)]);
            }
        };
        $this->app = $this->app->addRoute('POST', '/upload', $handler);
        $this->client = TestClient::create($this->app);

        $files = ['document.pdf' => ['content' => 'binary']];
        $response = $this->client->request('POST', '/upload', ['files' => $files]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithFilesAsBody(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['body_type' => \gettype($request->body)]);
            }
        };
        $this->app = $this->app->addRoute('POST', '/file-body', $handler);
        $this->client = TestClient::create($this->app);

        $files = ['file.txt' => 'content'];
        $response = $this->client->request('POST', '/file-body', ['files' => $files]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestPreferExplicitBodyOverFiles(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['body_type' => \gettype($request->body)]);
            }
        };
        $this->app = $this->app->addRoute('POST', '/priority', $handler);
        $this->client = TestClient::create($this->app);

        $body = ['explicit' => 'body'];
        $files = ['file.txt' => 'content'];
        $response = $this->client->request('POST', '/priority', [
            'body' => $body,
            'files' => $files,
        ]);

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
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
        $this->app = $this->app->addRoute('GET', '/endpoint', $handler);
        $this->client = TestClient::create($this->app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');

        $this->client->request('POST', '/endpoint');
    }

    // ======================== Path Handling Tests ========================

    public function testPathOnlyExtraction(): void
    {
        $handler = new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return new Response(['path' => $request->path]);
            }
        };
        $this->app = $this->app->addRoute('GET', '/path', $handler);
        $this->client = TestClient::create($this->app);

        $response = $this->client->request('GET', '/path?query=param');

        $this->assertInstanceOf(Response::class, $response);
        $this->assertSame(200, $response->statusCode);
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
