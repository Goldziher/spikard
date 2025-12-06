<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Config\ServerConfig;
use Spikard\DI\DependencyContainer;
use Spikard\Http\Params\Body;
use Spikard\Http\Params\Cookie;
use Spikard\Http\Params\Header;
use Spikard\Http\Params\Path;
use Spikard\Http\Params\Query;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Http\StreamingResponse;

final class EdgeCasesTest extends TestCase
{
    // Empty/Boundary condition tests for Request
    public function testRequestWithEmptyPath(): void
    {
        $request = new Request(method: 'GET', path: '', body: null);
        $this->assertSame('', $request->path);
    }

    public function testRequestWithVeryLongPath(): void
    {
        $longPath = '/' . \str_repeat('a', 2000);
        $request = new Request(method: 'GET', path: $longPath, body: null);
        $this->assertSame($longPath, $request->path);
        $this->assertGreaterThan(2000, \strlen($request->path));
    }

    public function testRequestWithEmptyHeaders(): void
    {
        $request = new Request(method: 'GET', path: '/test', body: null, headers: []);
        $this->assertSame([], $request->headers);
    }

    public function testRequestWithVeryLongHeaderValue(): void
    {
        $longValue = \str_repeat('x', 8000);
        $headers = ['X-Custom' => $longValue];
        $request = new Request(method: 'GET', path: '/test', body: null, headers: $headers);
        $this->assertSame($longValue, $request->headers['X-Custom']);
    }

    public function testRequestWithSpecialCharactersInPath(): void
    {
        $path = '/api/users/test@example.com#fragment?query=1';
        $request = new Request(method: 'GET', path: $path, body: null);
        $this->assertSame($path, $request->path);
    }

    public function testRequestWithUnicodeInPath(): void
    {
        $path = '/users/🚀/profile';
        $request = new Request(method: 'GET', path: $path, body: null);
        $this->assertSame($path, $request->path);
    }

    public function testRequestWithUnicodeInHeaders(): void
    {
        $headers = ['X-Name' => 'François'];
        $request = new Request(method: 'GET', path: '/test', body: null, headers: $headers);
        $this->assertSame('François', $request->headers['X-Name']);
    }

    public function testRequestWithManyQueryParams(): void
    {
        $queryParams = [];
        for ($i = 0; $i < 100; $i++) {
            $queryParams['param_' . $i] = ['value_' . $i];
        }
        $request = new Request(method: 'GET', path: '/test', body: null, queryParams: $queryParams);
        $this->assertCount(100, $request->queryParams);
    }

    public function testRequestWithEmptyStringBody(): void
    {
        $request = new Request(method: 'POST', path: '/test', body: '');
        $this->assertSame('', $request->body);
        $this->assertIsString($request->body);
    }

    public function testRequestWithLargeJsonBody(): void
    {
        $largeBody = [];
        for ($i = 0; $i < 1000; $i++) {
            $largeBody['item_' . $i] = [
                'id' => $i,
                'name' => 'Item ' . $i,
                'description' => \str_repeat('text', 100),
            ];
        }
        $request = new Request(method: 'POST', path: '/test', body: $largeBody);
        if (\is_array($request->body)) {
            $this->assertCount(1000, $request->body);
        }
    }

    public function testRequestWithNullValuesInAllParams(): void
    {
        $request = new Request(
            method: 'POST',
            path: '/test',
            body: null,
            headers: [],
            cookies: [],
            queryParams: [],
            pathParams: [],
            files: [],
            dependencies: null
        );
        $this->assertNull($request->body);
        $this->assertNull($request->dependencies);
    }

    public function testRequestWithManyFiles(): void
    {
        $files = [];
        for ($i = 0; $i < 50; $i++) {
            $files['file_' . $i] = [
                'name' => 'file_' . $i . '.txt',
                'type' => 'text/plain',
                'size' => 1024 * ($i + 1),
            ];
        }
        $request = new Request(method: 'POST', path: '/upload', body: null, files: $files);
        $this->assertCount(50, $request->files);
    }

    // Empty/Boundary condition tests for Response
    public function testResponseWithEmptyStringBody(): void
    {
        $response = new Response(body: '', statusCode: 200);
        $this->assertSame('', $response->body);
        $this->assertIsString($response->body);
    }

    public function testResponseWithVeryLongBody(): void
    {
        $longBody = \str_repeat('a', 10000);
        $response = new Response(body: $longBody, statusCode: 200);
        $this->assertSame($longBody, $response->body);
    }

    public function testResponseWithMinimalStatusCode(): void
    {
        $response = new Response(body: 'text', statusCode: 100);
        $this->assertSame(100, $response->statusCode);
    }

    public function testResponseWithMaximalStatusCode(): void
    {
        $response = new Response(body: 'text', statusCode: 599);
        $this->assertSame(599, $response->statusCode);
    }

    public function testResponseWithManyHeaders(): void
    {
        $headers = [];
        for ($i = 0; $i < 50; $i++) {
            $headers['X-Header-' . $i] = 'value_' . $i;
        }
        $response = new Response(body: 'text', statusCode: 200, headers: $headers);
        $this->assertCount(50, $response->headers);
    }

    public function testResponseWithVeryLongHeaderValue(): void
    {
        $longValue = \str_repeat('x', 10000);
        $response = new Response(
            body: 'text',
            statusCode: 200,
            headers: ['X-Custom' => $longValue]
        );
        $this->assertSame($longValue, $response->headers['X-Custom']);
    }

    public function testResponseJsonWithEmptyArray(): void
    {
        $response = Response::json([]);
        $this->assertSame([], $response->body);
        $this->assertSame('application/json', $response->headers['Content-Type']);
    }

    public function testResponseJsonWithNullValue(): void
    {
        $response = Response::json(null);
        $this->assertNull($response->body);
    }

    public function testResponseJsonWithFalseValue(): void
    {
        $response = Response::json(false);
        $this->assertFalse($response->body);
    }

    public function testResponseJsonWithZeroValue(): void
    {
        $response = Response::json(0);
        $this->assertSame(0, $response->body);
    }

    public function testResponseTextWithEmptyString(): void
    {
        $response = Response::text('');
        $this->assertSame('', $response->body);
        $this->assertSame('text/plain; charset=utf-8', $response->headers['Content-Type']);
    }

    public function testResponseWithManyCookies(): void
    {
        $cookies = [];
        for ($i = 0; $i < 30; $i++) {
            $cookies['cookie_' . $i] = 'value_' . $i;
        }
        $response = new Response(body: 'text', statusCode: 200, cookies: $cookies);
        $this->assertCount(30, $response->cookies);
    }

    public function testResponseWithCookiesImmutabilityMultipleTimes(): void
    {
        $original = Response::json(['data' => 1]);
        $withCookies1 = $original->withCookies(['cookie1' => 'value1']);
        $withCookies2 = $original->withCookies(['cookie2' => 'value2']);

        $this->assertSame([], $original->cookies);
        $this->assertSame(['cookie1' => 'value1'], $withCookies1->cookies);
        $this->assertSame(['cookie2' => 'value2'], $withCookies2->cookies);
        $this->assertNotSame($withCookies1, $withCookies2);
    }

    // Boundary tests for Params
    public function testQueryParamWithoutDefault(): void
    {
        $query = new Query();
        $hasDefault = $query->hasDefault();
        $this->assertFalse($hasDefault);
    }

    public function testQueryParamWithZeroDefault(): void
    {
        $query = new Query(default: 0);
        $this->assertTrue($query->hasDefault());
        $this->assertSame(0, $query->getDefault());
    }

    public function testQueryParamWithFalseDefault(): void
    {
        $query = new Query(default: false);
        $this->assertTrue($query->hasDefault());
        $this->assertFalse($query->getDefault());
    }

    public function testQueryParamWithEmptyStringDefault(): void
    {
        $query = new Query(default: '');
        $this->assertTrue($query->hasDefault());
        $this->assertSame('', $query->getDefault());
    }

    public function testQueryParamWithEmptyArrayDefault(): void
    {
        $query = new Query(default: []);
        $this->assertTrue($query->hasDefault());
        $this->assertSame([], $query->getDefault());
    }

    public function testPathParamWithVeryLongDefault(): void
    {
        $longString = \str_repeat('a', 5000);
        $path = new Path(default: $longString);
        $this->assertSame($longString, $path->getDefault());
    }

    public function testHeaderParamWithVeryLongValue(): void
    {
        $longValue = \str_repeat('x', 8000);
        $header = new Header(default: $longValue);
        $this->assertSame($longValue, $header->getDefault());
    }

    public function testCookieParamWithSpecialChars(): void
    {
        $specialValue = 'value; path=/; secure; httponly';
        $cookie = new Cookie(default: $specialValue);
        $this->assertSame($specialValue, $cookie->getDefault());
    }

    public function testBodyParamWithComplexSchema(): void
    {
        $schema = [
            'type' => 'object',
            'properties' => [
                'nested' => [
                    'type' => 'object',
                    'properties' => [
                        'deeply' => [
                            'type' => 'array',
                            'items' => ['type' => 'string'],
                        ],
                    ],
                ],
            ],
        ];
        $body = new Body(schema: $schema);
        $this->assertSame($schema, $body->getSchema());
    }

    // DependencyContainer edge cases
    public function testContainerWithEmptyDependencies(): void
    {
        $container = new DependencyContainer();
        $this->assertSame([], $container->getDependencies());
    }

    public function testContainerWithManyDependencies(): void
    {
        $values = [];
        for ($i = 0; $i < 100; $i++) {
            $values['dep_' . $i] = 'value_' . $i;
        }
        $container = new DependencyContainer($values);
        $this->assertCount(100, $container->getDependencies());
    }

    public function testContainerWithNullValues(): void
    {
        $values = ['nullable_dep' => null];
        $container = new DependencyContainer($values);
        $this->assertNull($container->getDependencies()['nullable_dep']);
    }

    // ServerConfig edge cases
    public function testServerConfigWithMinimalPort(): void
    {
        $config = ServerConfig::builder()
            ->withPort(1)
            ->build();
        $this->assertSame(1, $config->port);
    }

    public function testServerConfigWithMaximalPort(): void
    {
        $config = ServerConfig::builder()
            ->withPort(65535)
            ->build();
        $this->assertSame(65535, $config->port);
    }

    public function testServerConfigWithZeroWorkers(): void
    {
        $config = ServerConfig::builder()
            ->withWorkers(0)
            ->build();
        $this->assertSame(0, $config->workers);
    }

    public function testServerConfigWithHighWorkerCount(): void
    {
        $config = ServerConfig::builder()
            ->withWorkers(1000)
            ->build();
        $this->assertSame(1000, $config->workers);
    }

    public function testServerConfigWithVerySmallBodySize(): void
    {
        $config = ServerConfig::builder()
            ->withMaxBodySize(1)
            ->build();
        $this->assertSame(1, $config->maxBodySize);
    }

    public function testServerConfigWithVeryLargeBodySize(): void
    {
        $config = ServerConfig::builder()
            ->withMaxBodySize(1024 * 1024 * 1024) // 1GB
            ->build();
        $this->assertSame(1024 * 1024 * 1024, $config->maxBodySize);
    }

    public function testServerConfigWithZeroTimeout(): void
    {
        $config = ServerConfig::builder()
            ->withRequestTimeout(0)
            ->build();
        $this->assertSame(0, $config->requestTimeout);
    }

    public function testServerConfigWithVeryLargeTimeout(): void
    {
        $config = ServerConfig::builder()
            ->withRequestTimeout(3600)
            ->build();
        $this->assertSame(3600, $config->requestTimeout);
    }

    // StreamingResponse edge cases
    public function testStreamingResponseWithBasicGenerator(): void
    {
        $gen = (function (): \Generator {
            yield 'test';
        })();
        $response = new StreamingResponse($gen, statusCode: 200);
        $this->assertSame(200, $response->statusCode);
    }

    public function testStreamingResponseWithCustomHeaders(): void
    {
        $gen = (function (): \Generator {
            yield 'data';
        })();
        $response = new StreamingResponse(
            $gen,
            statusCode: 201,
            headers: ['X-Custom' => 'value']
        );
        $this->assertSame(201, $response->statusCode);
        $this->assertSame(['X-Custom' => 'value'], $response->headers);
    }

    public function testStreamingResponseSSEHelper(): void
    {
        $gen = (function (): \Generator {
            yield "data: test\n\n";
        })();
        $response = StreamingResponse::sse($gen);
        $this->assertSame(200, $response->statusCode);
        $this->assertSame('text/event-stream', $response->headers['Content-Type']);
    }

    public function testStreamingResponseJsonLinesHelper(): void
    {
        $gen = (function (): \Generator {
            yield ['id' => 1];
            yield ['id' => 2];
        })();
        $response = StreamingResponse::jsonLines($gen);
        $this->assertSame(200, $response->statusCode);
        $this->assertSame('application/x-ndjson', $response->headers['Content-Type']);
    }

    // Type preservation tests
    public function testRequestPreservesAllMixedTypes(): void
    {
        $body = [
            'string' => 'text',
            'int' => 42,
            'float' => 3.14,
            'bool' => true,
            'null' => null,
            'array' => [1, 2, 3],
            'nested' => ['key' => 'value'],
        ];
        $request = new Request(method: 'POST', path: '/test', body: $body);
        $this->assertSame($body, $request->body);
    }

    public function testResponsePreservesAllMixedTypes(): void
    {
        $data = [
            'string' => 'text',
            'int' => -999,
            'float' => -3.14,
            'bool' => false,
            'null' => null,
            'array' => [],
        ];
        $response = Response::json($data);
        $this->assertSame($data, $response->body);
    }

    // Case sensitivity and special characters
    public function testHeaderKeysCaseSensitive(): void
    {
        $headers = ['content-type' => 'text/plain', 'Content-Type' => 'application/json'];
        $request = new Request(method: 'GET', path: '/test', body: null, headers: $headers);
        // PHP array keys are case-sensitive
        $this->assertCount(2, $request->headers);
    }

    public function testCookieNamesWithSpecialChars(): void
    {
        $cookies = ['_session' => 'value', 'session-id' => 'value2'];
        $request = new Request(method: 'GET', path: '/test', body: null, cookies: $cookies);
        $this->assertArrayHasKey('_session', $request->cookies);
        $this->assertArrayHasKey('session-id', $request->cookies);
    }
}
