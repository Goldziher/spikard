<?php

declare(strict_types=1);

namespace Spikard\Tests;

use PHPUnit\Framework\TestCase;
use Spikard\Grpc\Request;

final class GrpcRequestTest extends TestCase
{
    public function testCreateRequest(): void
    {
        $request = new Request(
            'test.Service',
            'TestMethod',
            'test_payload'
        );

        self::assertSame('test.Service', $request->serviceName);
        self::assertSame('TestMethod', $request->methodName);
        self::assertSame('test_payload', $request->payload);
        self::assertSame([], $request->metadata);
    }

    public function testCreateRequestWithMetadata(): void
    {
        $metadata = [
            'authorization' => 'Bearer token123',
            'custom-header' => 'custom-value',
        ];

        $request = new Request(
            'test.Service',
            'TestMethod',
            'payload',
            $metadata
        );

        self::assertSame($metadata, $request->metadata);
    }

    public function testGetMetadata(): void
    {
        $request = new Request(
            'test.Service',
            'TestMethod',
            'payload',
            ['authorization' => 'Bearer token']
        );

        self::assertSame('Bearer token', $request->getMetadata('authorization'));
        self::assertNull($request->getMetadata('nonexistent'));
    }

    public function testHasMetadata(): void
    {
        $request = new Request(
            'test.Service',
            'TestMethod',
            'payload',
            ['authorization' => 'Bearer token']
        );

        self::assertTrue($request->hasMetadata('authorization'));
        self::assertFalse($request->hasMetadata('nonexistent'));
    }

    public function testGetPayloadSize(): void
    {
        $payload = 'test_payload_data';
        $request = new Request(
            'test.Service',
            'TestMethod',
            $payload
        );

        self::assertSame(strlen($payload), $request->getPayloadSize());
    }

    public function testGetPayloadSizeEmpty(): void
    {
        $request = new Request('test.Service', 'TestMethod', '');

        self::assertSame(0, $request->getPayloadSize());
    }

    public function testGetPayloadSizeBinary(): void
    {
        $payload = "\x00\x01\x02\x03\x04";
        $request = new Request(
            'test.Service',
            'TestMethod',
            $payload
        );

        self::assertSame(5, $request->getPayloadSize());
    }

    public function testGetAllMetadata(): void
    {
        $metadata = [
            'auth' => 'token',
            'user-id' => '123',
        ];

        $request = new Request(
            'test.Service',
            'TestMethod',
            'payload',
            $metadata
        );

        self::assertSame($metadata, $request->getAllMetadata());
    }

    public function testToString(): void
    {
        $request = new Request(
            'mypackage.MyService',
            'MyMethod',
            '12345'
        );

        $str = (string) $request;

        self::assertStringContainsString('mypackage.MyService', $str);
        self::assertStringContainsString('MyMethod', $str);
        self::assertStringContainsString('payloadSize=5', $str);
    }

    public function testRequestImmutable(): void
    {
        $request = new Request('test.Service', 'TestMethod', 'payload');

        $property = new \ReflectionProperty(Request::class, 'serviceName');
        self::assertTrue($property->isReadOnly());
    }

    public function testEmptyPayload(): void
    {
        $request = new Request('test.Service', 'TestMethod', '');

        self::assertSame('', $request->payload);
        self::assertSame(0, $request->getPayloadSize());
    }

    public function testLargePayload(): void
    {
        $largePayload = str_repeat('x', 10000);
        $request = new Request('test.Service', 'TestMethod', $largePayload);

        self::assertSame(10000, $request->getPayloadSize());
    }

    public function testUnicodeMetadata(): void
    {
        $metadata = [
            'user-name' => 'José García',
            'location' => '日本',
        ];

        $request = new Request(
            'test.Service',
            'TestMethod',
            'payload',
            $metadata
        );

        self::assertSame('José García', $request->getMetadata('user-name'));
        self::assertSame('日本', $request->getMetadata('location'));
    }

    public function testMetadataCaseInsensitivity(): void
    {
        // Metadata keys should be case-sensitive (standard gRPC behavior)
        $metadata = ['Authorization' => 'token'];
        $request = new Request('test.Service', 'TestMethod', 'payload', $metadata);

        self::assertNull($request->getMetadata('authorization'));
        self::assertSame('token', $request->getMetadata('Authorization'));
    }
}
