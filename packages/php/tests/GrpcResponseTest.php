<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Grpc\Response;

final class GrpcResponseTest extends TestCase
{
    public function testCreateResponse(): void
    {
        $response = new Response('response_payload');

        self::assertSame('response_payload', $response->payload);
        self::assertSame([], $response->metadata);
    }

    public function testCreateResponseWithMetadata(): void
    {
        $metadata = [
            'content-type' => 'application/grpc',
            'custom-header' => 'value',
        ];

        $response = new Response('payload', $metadata);

        self::assertSame('payload', $response->payload);
        self::assertSame($metadata, $response->metadata);
    }

    public function testGetPayloadSize(): void
    {
        $payload = 'test_response_data';
        $response = new Response($payload);

        self::assertSame(\strlen($payload), $response->getPayloadSize());
    }

    public function testGetPayloadSizeEmpty(): void
    {
        $response = new Response('');

        self::assertSame(0, $response->getPayloadSize());
    }

    public function testGetMetadata(): void
    {
        $response = new Response('payload', [
            'content-type' => 'application/grpc',
        ]);

        self::assertSame('application/grpc', $response->getMetadata('content-type'));
        self::assertNull($response->getMetadata('nonexistent'));
    }

    public function testHasMetadata(): void
    {
        $response = new Response('payload', [
            'content-type' => 'application/grpc',
        ]);

        self::assertTrue($response->hasMetadata('content-type'));
        self::assertFalse($response->hasMetadata('nonexistent'));
    }

    public function testGetAllMetadata(): void
    {
        $metadata = [
            'content-type' => 'application/grpc',
            'custom' => 'value',
        ];

        $response = new Response('payload', $metadata);

        self::assertSame($metadata, $response->getAllMetadata());
    }

    public function testErrorResponse(): void
    {
        $response = Response::error('Something went wrong');

        self::assertSame('', $response->payload);
        self::assertSame('INTERNAL', $response->getMetadata('grpc-status'));
        self::assertSame('Something went wrong', $response->getMetadata('grpc-message'));
    }

    public function testErrorResponseWithMetadata(): void
    {
        $customMetadata = ['request-id' => '123'];
        $response = Response::error('Error occurred', $customMetadata);

        self::assertSame('', $response->payload);
        self::assertSame('INTERNAL', $response->getMetadata('grpc-status'));
        self::assertSame('Error occurred', $response->getMetadata('grpc-message'));
        self::assertSame('123', $response->getMetadata('request-id'));
    }

    public function testToString(): void
    {
        $response = new Response('12345');

        $str = (string) $response;

        self::assertStringContainsString('payloadSize=5', $str);
    }

    public function testResponseImmutable(): void
    {
        $response = new Response('payload');

        $property = new \ReflectionProperty(Response::class, 'payload');
        self::assertTrue($property->isReadOnly());
    }

    public function testBinaryPayload(): void
    {
        $payload = "\x00\x01\x02\x03\x04\x05";
        $response = new Response($payload);

        self::assertSame($payload, $response->payload);
        self::assertSame(6, $response->getPayloadSize());
    }

    public function testLargePayload(): void
    {
        $largePayload = \str_repeat('x', 100000);
        $response = new Response($largePayload);

        self::assertSame(100000, $response->getPayloadSize());
    }

    public function testUnicodeMetadata(): void
    {
        $metadata = [
            'user-name' => 'José García',
            'location' => '日本',
        ];

        $response = new Response('payload', $metadata);

        self::assertSame('José García', $response->getMetadata('user-name'));
        self::assertSame('日本', $response->getMetadata('location'));
    }

    public function testEmptyPayload(): void
    {
        $response = new Response('');

        self::assertSame('', $response->payload);
        self::assertSame(0, $response->getPayloadSize());
    }

    public function testMultipleErrorCalls(): void
    {
        $response1 = Response::error('Error 1');
        $response2 = Response::error('Error 2');

        self::assertSame('Error 1', $response1->getMetadata('grpc-message'));
        self::assertSame('Error 2', $response2->getMetadata('grpc-message'));
    }
}
