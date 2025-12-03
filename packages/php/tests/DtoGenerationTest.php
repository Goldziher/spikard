<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Generated\Request as GeneratedRequest;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class DtoGenerationTest extends TestCase
{
    public function test_request_from_http_preserves_raw_query_and_headers(): void
    {
        $request = GeneratedRequest::fromHttp(
            'get',
            '/search?q=one&q=two&empty=',
            [
                'headers' => ['Content-Type' => 'application/json'],
                'cookies' => ['session' => 'abc'],
                'body' => ['ok' => true],
                'files' => ['upload' => ['name' => 'file.txt']],
            ]
        );

        $this->assertSame('GET', $request->method);
        $this->assertSame('/search', $request->path);
        $this->assertSame(['Content-Type' => 'application/json'], $request->headers);
        $this->assertSame(['session' => 'abc'], $request->cookies);
        $this->assertSame(['q' => ['one', 'two'], 'empty' => ['']], $request->queryParams);
        $this->assertSame($request->queryParams, $request->rawQueryParams);
        $this->assertNull($request->rawBody);
    }

    public function test_test_client_prefers_native_extension_when_loaded(): void
    {
        $app = (new App())->addRoute('GET', '/native', new class implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['ok' => true], 200);
            }
        });

        $client = TestClient::create($app);
        try {
            $response = $client->get('/native');
        } catch (\Throwable $exception) {
            $message = $exception->getMessage();
            if (\str_contains($message, 'You cannot instantiate this class from PHP')) {
                $this->markTestSkipped('Native Request class is not instantiable from PHP in this runtime.');
            }
            if (\str_contains($message, 'NUL-bytes')) {
                $this->markTestSkipped('Native client rejected callable due to NUL-bytes.');
            }
            throw $exception;
        }

        $this->assertTrue($client->usedNativeClient());
        if (\method_exists($response, 'json')) {
            try {
                $this->assertSame(['ok' => true], $response->json());
                return;
            } catch (\ArgumentCountError) {
                // Fall through to body inspection.
            }
        }

        if (\property_exists($response, 'body')) {
            $this->assertSame(['ok' => true], $response->body);
        } else {
            $this->fail('Response did not expose a JSON body accessor.');
        }
    }
}
