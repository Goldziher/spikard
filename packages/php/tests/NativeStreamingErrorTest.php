<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use RuntimeException;
use Spikard\App;
use Spikard\Handlers\HandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;
use Spikard\Http\Request;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;

final class NativeStreamingErrorTest extends TestCase
{
    public function test_streaming_invalid_chunk_returns_structured_error(): void
    {
        $app = (new App())->addSse('/stream', new class implements SseEventProducerInterface {
            public function __invoke(): \Generator
            {
                yield "data: keep-alive\n\n";
                throw new RuntimeException('invalid chunk');
            }
        });

        $client = TestClient::create($app);
        $response = $client->connectSse('/stream');
        $payload = $this->extractPayload($response);
        $this->assertSame('invalid chunk', $payload['error'] ?? null);
        $this->assertSame('panic', $payload['code'] ?? null);
    }

    public function test_handler_panic_is_shielded(): void
    {
        $app = (new App())->addRoute('GET', '/panic', new class implements HandlerInterface {
            public function matches(Request $request): bool
            {
                return true;
            }

            public function handle(Request $request): Response
            {
                throw new RuntimeException('boom');
            }
        });

        $client = TestClient::create($app);
        $response = $client->get('/panic');
        $payload = $this->extractPayload($response);
        $this->assertArrayHasKey('error', $payload);
        $this->assertArrayHasKey('code', $payload);
        $this->assertArrayHasKey('details', $payload);
    }

    /**
     * @return array<string, mixed>
     */
    private function extractPayload(object $response): array
    {
        if (\method_exists($response, 'json')) {
            try {
                $json = $response->json();
                if (\is_array($json)) {
                    return $json;
                }
            } catch (\ArgumentCountError) {
                // Fall through to body parsing.
            }
        }

        if (\property_exists($response, 'body')) {
            $body = $response->body;
            if (\is_array($body)) {
                return $body;
            }
            if (\is_string($body)) {
                $decoded = \json_decode($body, true);
                if (\is_array($decoded)) {
                    return $decoded;
                }
            }
        }

        return [];
    }
}
