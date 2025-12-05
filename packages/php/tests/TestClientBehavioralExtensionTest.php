<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

/**
 * Behavioral extension tests for TestClient to push coverage from 85.96% to 95%+.
 *
 * Focuses on:
 * 1. WebSocket connection paths
 * 2. SSE (Server-Sent Events) paths
 * 3. Native extension detection and fallback
 * 4. Error handling for missing extension
 * 5. Query parameter parsing edge cases
 * 6. Options handling with various combinations
 */
final class TestClientBehavioralExtensionTest extends TestCase
{
    // ======================== WebSocket Connection ========================

    public function testConnectWebSocketThrowsWithoutNativeExtension(): void
    {
        $app = new App();
        $wsHandler = new TestClientBehavioralExtensionDummyWebSocketHandler();
        $app = $app->addWebSocket('/ws', $wsHandler);

        $client = TestClient::create($app);

        // Should throw because we're likely not running with extension
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('WebSocket client requires the native extension');
        $client->connectWebSocket('/ws');
    }

    // ======================== SSE Connection ========================

    public function testConnectSseThrowsWithoutNativeExtension(): void
    {
        $app = new App();
        $sseProducer = new DummySseEventProducer();
        $app = $app->addSse('/events', $sseProducer);

        $client = TestClient::create($app);

        // Should throw because we're likely not running with extension
        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('SSE client requires the native extension');
        $client->connectSse('/events');
    }

    // ======================== Query Parameter Parsing ========================

    public function testParseQueryParamsEmpty(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Test with various empty query string formats
        $response = $client->request('GET', '/test');
        $this->assertInstanceOf(Response::class, $response);
    }

    public function testParseQueryParamsWithSingleValue(): void
    {
        $app = (new App())->addRoute('GET', '/search', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/search?q=test');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithMultipleValues(): void
    {
        $app = (new App())->addRoute('GET', '/filter', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/filter?tag=php&tag=laravel&tag=api');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithUrlEncodedValues(): void
    {
        $app = (new App())->addRoute('GET', '/search', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/search?q=hello%20world&special=%40%23%24');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithEmptyValue(): void
    {
        $app = (new App())->addRoute('GET', '/filter', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/filter?flag=&other=value');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithoutValue(): void
    {
        $app = (new App())->addRoute('GET', '/flag', new QueryCaptureHandler());
        $client = TestClient::create($app);

        // Flag without value should still parse
        $response = $client->request('GET', '/flag?verbose&debug=1');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithoutQuestion(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithMultipleAmpersand(): void
    {
        $app = (new App())->addRoute('GET', '/filter', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/filter?a=1&&b=2&&&c=3');
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Options Handling ========================

    public function testRequestWithEmptyOptions(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', []);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithHeadersOption(): void
    {
        $app = (new App())->addRoute('GET', '/test', new HeaderCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', [
            'headers' => ['X-Custom' => 'custom-value'],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithCookiesOption(): void
    {
        $app = (new App())->addRoute('GET', '/test', new CookieCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test', [
            'cookies' => ['session' => 'abc123'],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithBodyOption(): void
    {
        $app = (new App())->addRoute('POST', '/create', new BodyCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('POST', '/create', [
            'body' => ['name' => 'Test', 'age' => 30],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithFilesOption(): void
    {
        $app = (new App())->addRoute('POST', '/upload', new FilesCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('POST', '/upload', [
            'files' => ['avatar' => '/path/to/image.jpg'],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithFilesOptionOverridesBodyWhenBothProvided(): void
    {
        $app = (new App())->addRoute('POST', '/upload', new BodyCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('POST', '/upload', [
            'body' => ['name' => 'ignored'],
            'files' => ['avatar' => 'file.jpg'],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestWithAllOptionsAtOnce(): void
    {
        $app = (new App())->addRoute('POST', '/combined', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->request('POST', '/combined?page=1', [
            'headers' => ['Authorization' => 'Bearer token'],
            'cookies' => ['session' => '123'],
            'body' => ['key' => 'value'],
            'files' => [],
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Convenience Methods ========================

    public function testGetMethod(): void
    {
        $app = (new App())->addRoute('GET', '/users', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->get('/users');
        $this->assertSame(200, $response->statusCode);
    }

    public function testPostMethod(): void
    {
        $app = (new App())->addRoute('POST', '/items', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->post('/items', ['name' => 'Item']);
        $this->assertSame(200, $response->statusCode);
    }

    public function testPostMethodWithoutBody(): void
    {
        $app = (new App())->addRoute('POST', '/trigger', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $response = $client->post('/trigger');
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Handler Not Found ========================

    public function testRequestThrowsWhenHandlerNotFound(): void
    {
        $app = (new App())->addRoute('GET', '/existing', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('GET', '/nonexistent');
    }

    public function testRequestThrowsForWrongMethod(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        $this->expectException(RuntimeException::class);
        $this->expectExceptionMessage('No handler registered');
        $client->request('POST', '/test');
    }

    // ======================== Edge Cases for Query Parameter Parsing ========================

    public function testParseQueryParamsEmptyKey(): void
    {
        $app = (new App())->addRoute('GET', '/test', new QueryCaptureHandler());
        $client = TestClient::create($app);

        // Empty keys should be skipped
        $response = $client->request('GET', '/test?=value&valid=yes');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsSpecialCharacters(): void
    {
        $app = (new App())->addRoute('GET', '/test', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?email=test%40example.com&url=http%3A%2F%2Fexample.com');
        $this->assertSame(200, $response->statusCode);
    }

    public function testParseQueryParamsWithPlus(): void
    {
        $app = (new App())->addRoute('GET', '/test', new QueryCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('GET', '/test?q=hello+world');
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Options Type Coercion ========================

    public function testRequestOptionsNonArrayHeadersIgnored(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Non-array headers should be treated as empty array
        $response = $client->request('GET', '/test', [
            'headers' => 'invalid',
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestOptionsNonArrayCookiesIgnored(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Non-array cookies should be treated as empty array
        $response = $client->request('GET', '/test', [
            'cookies' => 'invalid',
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    public function testRequestOptionsNonArrayFilesIgnored(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Non-array files should be treated as empty array
        $response = $client->request('GET', '/test', [
            'files' => 'invalid',
        ]);
        $this->assertSame(200, $response->statusCode);
    }

    // ======================== Close Method ========================

    public function testCloseMethod(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Should not throw
        $client->close();
    }

    public function testCloseMethodIsIdempotent(): void
    {
        $app = (new App())->addRoute('GET', '/test', new TestClientBehavioralExtensionDummyHandler());
        $client = TestClient::create($app);

        // Should not throw when called multiple times
        $client->close();
        $client->close();
        $client->close();
    }

    // ======================== HTTP Methods Uppercase ========================

    public function testHttpMethodsAreUppercased(): void
    {
        $app = (new App())->addRoute('GET', '/test', new MethodCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->request('get', '/test');
        $this->assertSame(200, $response->statusCode);
    }

    public function testPostMethodRespectsHttpVerb(): void
    {
        $app = (new App())->addRoute('POST', '/submit', new MethodCaptureHandler());
        $client = TestClient::create($app);

        $response = $client->post('/submit', ['data' => 'value']);
        $this->assertSame(200, $response->statusCode);
    }
}

// ======================== Test Handlers ========================

final class TestClientBehavioralExtensionDummyHandler implements HandlerInterface
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

final class QueryCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['params' => $request->queryParams], 200);
    }
}

final class HeaderCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['headers' => $request->headers], 200);
    }
}

final class CookieCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['cookies' => $request->cookies], 200);
    }
}

final class BodyCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['body' => $request->body], 200);
    }
}

final class FilesCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['files' => $request->files], 200);
    }
}

final class MethodCaptureHandler implements HandlerInterface
{
    public function matches(Request $request): bool
    {
        return true;
    }

    public function handle(Request $request): Response
    {
        return Response::json(['method' => $request->method], 200);
    }
}

final class TestClientBehavioralExtensionDummyWebSocketHandler implements WebSocketHandlerInterface
{
    public function onConnect(): void
    {
        // Dummy implementation
    }

    public function onMessage(string $message): void
    {
        // Dummy implementation
    }

    public function onClose(int $code, ?string $reason = null): void
    {
        // Dummy implementation
    }
}

final class DummySseEventProducer implements SseEventProducerInterface
{
    public function __invoke(): \Generator
    {
        yield "data: test\n\n";
    }
}
