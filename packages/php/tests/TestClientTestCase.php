<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

/**
 * Abstract base class for TestClient tests providing common helpers and fixtures.
 *
 * Provides:
 * - Handler factories for various test scenarios
 * - Request builder utilities
 * - Response assertion helpers
 * - Data providers for parametrized tests
 */
abstract class TestClientTestCase extends TestCase
{
    /**
     * Create a basic handler that always returns 200 OK.
     */
    protected function createBasicHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['ok' => true], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that tracks when it's called.
     *
     * @param callable(): void $callback
     */
    protected function createTrackingHandler(callable $callback): HandlerInterface
    {
        return new class ($callback) implements HandlerInterface {
            /** @var callable(): void */
            private mixed $callback;

            public function __construct(callable $callback)
            {
                $this->callback = $callback;
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

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that tracks the request and calls a callback with it.
     *
     * @param callable(Request): void $callback
     */
    protected function createRequestTrackingHandler(callable $callback): HandlerInterface
    {
        return new class ($callback) implements HandlerInterface {
            /** @var callable(Request): void */
            private mixed $callback;

            public function __construct(callable $callback)
            {
                $this->callback = $callback;
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

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns query parameters.
     */
    protected function createQueryCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['params' => $request->queryParams], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns headers.
     */
    protected function createHeaderCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['headers' => $request->headers], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns cookies.
     */
    protected function createCookieCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['cookies' => $request->cookies], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns body.
     */
    protected function createBodyCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['body' => $request->body], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns files.
     */
    protected function createFilesCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['files' => $request->files], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a handler that captures and returns the request method.
     */
    protected function createMethodCapturingHandler(): HandlerInterface
    {
        return new class () implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['method' => $request->method], 200);
            }

            public function __invoke(Request $request): Response
            {
                return $this->handle($request);
            }
        };
    }

    /**
     * Create a dummy WebSocket handler for testing.
     */
    protected function createDummyWebSocketHandler(): WebSocketHandlerInterface
    {
        return new class () implements WebSocketHandlerInterface {
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
        };
    }

    /**
     * Create a dummy SSE event producer for testing.
     */
    protected function createDummySseEventProducer(): SseEventProducerInterface
    {
        return new class () implements SseEventProducerInterface {
            public function __invoke(): \Generator
            {
                yield "data: test\n\n";
            }
        };
    }

    /**
     * Data provider for HTTP methods.
     *
     * @return array<string, array{0: string, 1: string}>
     */
    public static function httpMethodsProvider(): array
    {
        return [
            'GET' => ['GET', 'GET'],
            'POST' => ['POST', 'POST'],
            'PUT' => ['PUT', 'PUT'],
            'DELETE' => ['DELETE', 'DELETE'],
            'PATCH' => ['PATCH', 'PATCH'],
            'HEAD' => ['HEAD', 'HEAD'],
            'OPTIONS' => ['OPTIONS', 'OPTIONS'],
        ];
    }

    /**
     * Data provider for case-insensitive HTTP methods.
     *
     * @return array<string, array{0: string, 1: string}>
     */
    public static function caseInsensitiveMethodsProvider(): array
    {
        return [
            'lowercase get' => ['get', 'GET'],
            'lowercase post' => ['post', 'POST'],
            'mixed case Get' => ['Get', 'GET'],
            'mixed case Post' => ['Post', 'POST'],
        ];
    }

    /**
     * Data provider for query parameter edge cases.
     *
     * @return array<string, array{0: string}>
     */
    public static function queryParamEdgeCasesProvider(): array
    {
        return [
            'empty query string' => ['/test?'],
            'no query string' => ['/test'],
            'single param' => ['/test?foo=bar'],
            'multiple params' => ['/test?foo=bar&baz=qux'],
            'multiple values' => ['/test?tags=php&tags=rust&tags=python'],
            'url encoded' => ['/test?search=hello%20world&email=test%40example.com'],
            'empty values' => ['/test?key1=&key2=value&key3='],
            'special chars' => ['/test?q=hello%2Bworld&special=%3C%3E'],
        ];
    }

    /**
     * Assert that a response is successful (2xx status code).
     */
    protected function assertResponseSuccess(Response $response): void
    {
        $this->assertGreaterThanOrEqual(200, $response->statusCode);
        $this->assertLessThan(300, $response->statusCode);
    }

    /**
     * Assert that a response has a specific status code.
     */
    protected function assertResponseStatus(int $expected, Response $response): void
    {
        $this->assertSame($expected, $response->statusCode);
    }
}
