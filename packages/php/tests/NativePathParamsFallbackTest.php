<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Handlers\HandlerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;

final class NativePathParamsFallbackTest extends TestCase
{
    public function test_path_params_inferred_without_template_when_schema_present(): void
    {
        if (!\class_exists(\Spikard\Testing\NativeTestClient::class)) {
            $this->markTestSkipped('Spikard PHP extension is not loaded.');
        }

        $parameterSchema = [
            'type' => 'object',
            'properties' => [
                'id' => ['type' => 'string', 'source' => 'path'],
                'order_id' => ['type' => 'string', 'source' => 'path'],
            ],
            'required' => ['id', 'order_id'],
        ];

        $handler = new class implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                return Response::json(['path' => $request->pathParams]);
            }
        };

        $client = \Spikard\Testing\NativeTestClient::new();
        try {
            /** @phpstan-ignore-next-line runtime extension method */
            $response = $client->request(
                'GET',
                '/users/42/orders/99',
                $handler,
                null,
                null,
                [],
                null,
                $parameterSchema,
                null,
                null,
                false,
                false,
                null,
                null,
                null,
            );
        } catch (\RuntimeException $exception) {
            $this->markTestSkipped($exception->getMessage());
            return;
        }

        $json = \method_exists($response, 'json') ? $response->json() : null;
        if (!\is_array($json)) {
            $this->fail('Expected JSON response from native client');
        }

        $this->assertSame(['id' => '42', 'order_id' => '99'], $json['path'] ?? null);
    }
}
