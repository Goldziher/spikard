<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Http\Response;

final class NativeValidationCoverageTest extends TestCase
{
    public function test_request_schema_validation_failure_is_structured(): void
    {
        if (!\class_exists(\Spikard\Testing\NativeTestClient::class)) {
            $this->markTestSkipped('Spikard PHP extension is not loaded.');
        }

        $handler = new class {
            public function handle(object $req): Response
            {
                return Response::json(['ok' => true]);
            }
        };

        $schema = [
            'type' => 'object',
            'properties' => [
                'name' => ['type' => 'string'],
            ],
            'required' => ['name'],
        ];

        /** @phpstan-ignore-next-line runtime extension method */
        $client = \Spikard\Testing\NativeTestClient::new();
        /** @phpstan-ignore-next-line runtime extension method */
        $response = $client->request(
            'POST',
            '/validate',
            $handler,
            ['name' => 123],
            null,
            [],
            $schema,
        );

        $this->assertGreaterThanOrEqual(400, $response->getStatus());
        $json = $response->json();
        $this->assertIsArray($json);
        $this->assertArrayHasKey('error', $json);
        $this->assertArrayHasKey('code', $json);
        $this->assertArrayHasKey('details', $json);
    }
}
