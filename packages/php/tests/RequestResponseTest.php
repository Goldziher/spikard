<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class RequestResponseTest extends TestCase
{
    // Request tests
    public function testRequestConstruction(): void
    {
        $request = new Request(
            method: 'GET',
            path: '/test',
            body: null
        );

        $this->assertSame('GET', $request->method);
        $this->assertSame('/test', $request->path);
        $this->assertNull($request->body);
        $this->assertSame([], $request->headers);
        $this->assertSame([], $request->cookies);
        $this->assertSame([], $request->queryParams);
        $this->assertSame([], $request->pathParams);
        $this->assertSame([], $request->files);
        $this->assertNull($request->dependencies);
    }

    public function testRequestWithHeaders(): void
    {
        $headers = ['Content-Type' => 'application/json', 'Authorization' => 'Bearer token'];
        $request = new Request(
            method: 'POST',
            path: '/api',
            body: ['data' => 'value'],
            headers: $headers
        );

        $this->assertSame($headers, $request->headers);
    }

    public function testRequestWithCookies(): void
    {
        $cookies = ['session' => 'abc123', 'user_id' => '42'];
        $request = new Request(
            method: 'GET',
            path: '/profile',
            body: null,
            cookies: $cookies
        );

        $this->assertSame($cookies, $request->cookies);
    }

    public function testRequestWithQueryParams(): void
    {
        $queryParams = ['page' => ['1'], 'limit' => ['10'], 'tags' => ['php', 'rust']];
        $request = new Request(
            method: 'GET',
            path: '/items',
            body: null,
            queryParams: $queryParams
        );

        $this->assertSame($queryParams, $request->queryParams);
    }

    public function testRequestWithPathParams(): void
    {
        $pathParams = ['id' => '123', 'category' => 'books'];
        $request = new Request(
            method: 'GET',
            path: '/items/123/books',
            body: null,
            pathParams: $pathParams
        );

        $this->assertSame($pathParams, $request->pathParams);
    }

    public function testRequestWithFiles(): void
    {
        $files = ['upload' => ['name' => 'file.txt', 'type' => 'text/plain']];
        $request = new Request(
            method: 'POST',
            path: '/upload',
            body: null,
            files: $files
        );

        $this->assertSame($files, $request->files);
    }

    public function testRequestWithJsonBody(): void
    {
        $body = ['user' => ['name' => 'Alice', 'age' => 30]];
        $request = new Request(
            method: 'POST',
            path: '/users',
            body: $body
        );

        $this->assertSame($body, $request->body);
    }

    public function testRequestWithStringBody(): void
    {
        $body = 'plain text content';
        $request = new Request(
            method: 'POST',
            path: '/text',
            body: $body
        );

        $this->assertSame($body, $request->body);
    }

    public function testRequestWithAllParameters(): void
    {
        $request = new Request(
            method: 'PUT',
            path: '/api/users/123',
            body: ['name' => 'Updated'],
            headers: ['Content-Type' => 'application/json'],
            cookies: ['session' => 'xyz'],
            queryParams: ['notify' => ['true']],
            pathParams: ['id' => '123'],
            files: [],
            dependencies: null
        );

        $this->assertSame('PUT', $request->method);
        $this->assertSame('/api/users/123', $request->path);
        $this->assertIsArray($request->body);
        $this->assertArrayHasKey('Content-Type', $request->headers);
        $this->assertArrayHasKey('session', $request->cookies);
        $this->assertArrayHasKey('notify', $request->queryParams);
        $this->assertArrayHasKey('id', $request->pathParams);
    }

    // Response tests
    public function testResponseConstruction(): void
    {
        $response = new Response(
            body: 'Hello',
            statusCode: 200
        );

        $this->assertSame('Hello', $response->body);
        $this->assertSame(200, $response->statusCode);
        $this->assertSame([], $response->headers);
        $this->assertSame([], $response->cookies);
    }

    public function testResponseJsonHelper(): void
    {
        $data = ['status' => 'ok', 'count' => 42];
        $response = Response::json($data, 201);

        $this->assertSame($data, $response->body);
        $this->assertSame(201, $response->statusCode);
        $this->assertSame('application/json', $response->headers['Content-Type']);
    }

    public function testResponseJsonWithDefaultStatus(): void
    {
        $response = Response::json(['ok' => true]);

        $this->assertSame(200, $response->statusCode);
    }

    public function testResponseJsonWithCustomHeaders(): void
    {
        $response = Response::json(
            ['data' => 'value'],
            200,
            ['X-Custom' => 'header']
        );

        $this->assertSame('application/json', $response->headers['Content-Type']);
        $this->assertSame('header', $response->headers['X-Custom']);
    }

    public function testResponseTextHelper(): void
    {
        $response = Response::text('Hello World', 200);

        $this->assertSame('Hello World', $response->body);
        $this->assertSame(200, $response->statusCode);
        $this->assertSame('text/plain; charset=utf-8', $response->headers['Content-Type']);
    }

    public function testResponseTextWithDefaultStatus(): void
    {
        $response = Response::text('content');

        $this->assertSame(200, $response->statusCode);
    }

    public function testResponseTextWithCustomHeaders(): void
    {
        $response = Response::text(
            'content',
            200,
            ['X-Message' => 'test']
        );

        $this->assertSame('text/plain; charset=utf-8', $response->headers['Content-Type']);
        $this->assertSame('test', $response->headers['X-Message']);
    }

    public function testResponseWithCookies(): void
    {
        $cookies = ['session' => 'new_session', 'theme' => 'dark'];
        $response = new Response(
            body: 'content',
            statusCode: 200,
            cookies: $cookies
        );

        $this->assertSame($cookies, $response->cookies);
    }

    public function testResponseWithCookiesMethod(): void
    {
        $response = Response::json(['ok' => true], 200);
        $cookies = ['user_id' => '123', 'token' => 'abc'];
        $responseWithCookies = $response->withCookies($cookies);

        // Original unchanged
        $this->assertSame([], $response->cookies);

        // New instance has cookies
        $this->assertSame($cookies, $responseWithCookies->cookies);
        $this->assertSame(['ok' => true], $responseWithCookies->body);
        $this->assertSame(200, $responseWithCookies->statusCode);
        $this->assertSame($response->headers, $responseWithCookies->headers);
    }

    public function testResponseWithCookiesIsImmutable(): void
    {
        $original = Response::json(['data' => 1], 200);
        $modified = $original->withCookies(['key' => 'value']);

        $this->assertNotSame($original, $modified);
        $this->assertSame([], $original->cookies);
        $this->assertSame(['key' => 'value'], $modified->cookies);
    }

    public function testResponseWithNullBody(): void
    {
        $response = new Response(null, 204);

        $this->assertNull($response->body);
        $this->assertSame(204, $response->statusCode);
    }

    public function testResponseWithArrayBody(): void
    {
        $body = ['nested' => ['array' => [1, 2, 3]]];
        $response = new Response($body, 200);

        $this->assertSame($body, $response->body);
    }

    public function testResponseWithMultipleHeaders(): void
    {
        $headers = [
            'Content-Type' => 'application/json',
            'Cache-Control' => 'no-cache',
            'X-Request-ID' => 'abc-123',
            'X-Rate-Limit' => '100',
        ];
        $response = new Response('body', 200, $headers);

        $this->assertSame($headers, $response->headers);
        $this->assertCount(4, $response->headers);
    }

    public function testResponseStatusCodeVariations(): void
    {
        $response200 = new Response('ok', 200);
        $response201 = new Response('created', 201);
        $response400 = new Response('bad request', 400);
        $response404 = new Response('not found', 404);
        $response500 = new Response('error', 500);

        $this->assertSame(200, $response200->statusCode);
        $this->assertSame(201, $response201->statusCode);
        $this->assertSame(400, $response400->statusCode);
        $this->assertSame(404, $response404->statusCode);
        $this->assertSame(500, $response500->statusCode);
    }

    public function testJsonResponsePreservesDataTypes(): void
    {
        $data = [
            'string' => 'text',
            'int' => 42,
            'float' => 3.14,
            'bool' => true,
            'null' => null,
            'array' => [1, 2, 3],
        ];
        $response = Response::json($data);

        $this->assertSame($data, $response->body);
        $this->assertIsString($response->body['string']);
        $this->assertIsInt($response->body['int']);
        $this->assertIsFloat($response->body['float']);
        $this->assertTrue($response->body['bool']);
        $this->assertNull($response->body['null']);
        $this->assertIsArray($response->body['array']);
    }
}
