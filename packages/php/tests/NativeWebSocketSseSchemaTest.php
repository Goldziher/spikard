<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\App;
use Spikard\Http\Response;
use Spikard\Testing\TestClient;
use Spikard\Handlers\WebSocketHandlerInterface;
use Spikard\Handlers\SseEventProducerInterface;

final class NativeWebSocketSseSchemaTest extends TestCase
{
    public function test_websocket_schema_rejects_invalid_message(): void
    {
        $handler = new class implements WebSocketHandlerInterface {
            public array $received = [];
            public function onConnect(): void {}
            public function onMessage(string $message): void
            {
                $this->received[] = $message;
            }
            public function onClose(int $code, ?string $reason = null): void {}
        };

        if (!\class_exists(\Spikard\Testing\HttpTestClient::class)) {
            $this->markTestSkipped('Spikard PHP extension is not loaded.');
        }

        $native = \Spikard\Testing\HttpTestClient::new();
        /** @phpstan-ignore-next-line runtime extension method */
        $connection = $native->websocket(
            '/ws',
            $handler,
            [
                'type' => 'object',
                'properties' => ['text' => ['type' => 'string']],
                'required' => ['text'],
            ],
            null
        );

        $connection->sendText('{"text":"ok"}');
        $connection->sendText('{"invalid":true}');
        $error = $connection->receiveText();
        $this->assertStringContainsString('validation failed', \strtolower($error));
    }

    public function test_sse_schema_rejects_invalid_event(): void
    {
        $producer = new class implements SseEventProducerInterface {
            public function __invoke(): \Generator
            {
                yield ['event' => 'good', 'data' => ['text' => 'ok']];
                yield ['event' => 'bad', 'data' => ['invalid' => true]];
            }
        };

        if (!\class_exists(\Spikard\Testing\HttpTestClient::class)) {
            $this->markTestSkipped('Spikard PHP extension is not loaded.');
        }

        $native = \Spikard\Testing\HttpTestClient::new();
        /** @phpstan-ignore-next-line runtime extension method */
        $stream = $native->sse(
            '/events',
            $producer,
            [
                'type' => 'object',
                'properties' => [
                    'event' => ['type' => 'string'],
                    'data' => [
                        'type' => 'object',
                        'properties' => ['text' => ['type' => 'string']],
                        'required' => ['text'],
                    ],
                ],
                'required' => ['event', 'data'],
            ]
        );

        $events = $stream->events();
        $this->assertCount(1, $events);
        $first = $events[0];
        $this->assertSame('good', $first->getEventType());
        $this->assertSame(['text' => 'ok'], \json_decode($first->getData(), true));
    }
}
