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

final class NativeHookRequestPayloadTest extends TestCase
{
    public function test_hooks_receive_request_context(): void
    {
        try {
            $client = TestClient::create($this->appWithHook());
        } catch (\RuntimeException $exception) {
            $this->markTestSkipped($exception->getMessage());
            return;
        }

        $response = $client->request('POST', '/users/42?role=admin&role=owner', [
            'body' => ['name' => 'Ada'],
            'headers' => ['Cookie' => 'session=abc; theme=dark'],
        ]);

        $payload = $response->jsonBody();
        $this->assertSame(['id' => '42'], $payload['path'] ?? null);
        $this->assertSame(['role' => ['admin', 'owner']], $payload['query'] ?? null);
        $this->assertSame(['session' => 'abc', 'theme' => 'dark'], $payload['cookies'] ?? null);
        $this->assertSame('{"name":"Ada"}', $payload['rawBody'] ?? null);
    }

    private function appWithHook(): App
    {
        $hooks = LifecycleHooks::builder()
            ->onRequest(static fn (Request $request): HookResult => HookResult::shortCircuit(
                Response::json([
                    'path' => $request->pathParams,
                    'query' => $request->queryParams,
                    'cookies' => $request->cookies,
                    'rawBody' => $request->rawBody,
                ])
            ))
            ->build();

        return (new App())
            ->withLifecycleHooks($hooks)
            ->addRoute('POST', '/users/{id}', new class implements HandlerInterface {
                public function matches(Request $request): bool
                {
                    return $request->path === '/users/' . ($request->pathParams['id'] ?? '');
                }

                public function handle(Request $request): Response
                {
                    return Response::json(['ok' => true]);
                }
            });
    }
}
