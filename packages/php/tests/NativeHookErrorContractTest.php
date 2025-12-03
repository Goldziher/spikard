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

final class NativeHookErrorContractTest extends TestCase
{
    public function test_invalid_hook_return_surfaces_structured_error(): void
    {
        try {
            $client = TestClient::create($this->appWithBadHook());
        } catch (\RuntimeException $exception) {
            $this->markTestSkipped($exception->getMessage());
            return;
        }

        $response = $client->get('/oops');
        $json = $response->jsonBody();
        $this->assertIsArray($json);
        $this->assertSame('hook_pre_validation_failed', $json['code'] ?? null);
        $this->assertArrayHasKey('details', $json);
    }

    private function appWithBadHook(): App
    {
        $hooks = LifecycleHooks::builder()
            ->onRequest(static fn (Request $req): HookResult => 'not-a-hook')
            ->build();

        return (new App())
            ->withLifecycleHooks($hooks)
            ->addRoute('GET', '/oops', new class implements HandlerInterface {
                public function matches(Request $request): bool
                {
                    return true;
                }

                public function handle(Request $request): Response
                {
                    return Response::json(['ok' => true]);
                }
            });
    }
}
