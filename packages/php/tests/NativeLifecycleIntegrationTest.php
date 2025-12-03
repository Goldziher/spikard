<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Config\HookResult;
use Spikard\Config\LifecycleHooks;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class NativeLifecycleIntegrationTest extends TestCase
{
    public function test_on_request_short_circuit_returns_custom_response(): void
    {
        $hooks = LifecycleHooks::builder()
            ->onRequest(static fn (Request $req): HookResult => HookResult::shortCircuit(
                Response::json(['early' => true], 418)
            ))
            ->build();

        $app = (new App())
            ->withLifecycleHooks($hooks)
            ->addRoute('GET', '/hello', new class implements HandlerInterface {
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
        $response = $client->get('/hello');
        $this->assertSame(418, $response->statusCode);
        $json = \method_exists($response, 'jsonBody') ? $response->jsonBody() : null;
        if ($json === null && \method_exists($response, 'json')) {
            $json = $response->json();
        }
        $this->assertSame(['early' => true], $json);
    }

    public function test_on_response_hook_can_mutate_response(): void
    {
        $hooks = LifecycleHooks::builder()
            ->onResponse(static function (Request $req, HookResult $result): HookResult {
                $resp = $result->response();
                if ($resp instanceof Response) {
                    $headers = $resp->headers;
                    $headers['X-Hook'] = 'mutated';
                    return HookResult::shortCircuit(new Response(
                        body: $resp->body,
                        statusCode: $resp->statusCode,
                        headers: $headers,
                        cookies: $resp->cookies,
                    ));
                }
                return HookResult::shortCircuit(Response::json(['ok' => true], 200));
            })
            ->build();

        $app = (new App())
            ->withLifecycleHooks($hooks)
            ->addRoute('GET', '/hello', new class implements HandlerInterface {
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
        $response = $client->get('/hello');
        $this->assertSame(200, $response->statusCode);
        $json = \method_exists($response, 'jsonBody') ? $response->jsonBody() : null;
        if ($json === null && \method_exists($response, 'json')) {
            $json = $response->json();
        }
        $this->assertSame(['ok' => true], $json);
        $this->assertSame('mutated', $response->headers['X-Hook'] ?? null);
    }
}
