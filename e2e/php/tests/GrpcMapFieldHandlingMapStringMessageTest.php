<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcMapFieldHandlingMapStringMessageTest extends TestCase
{
    public function testGrpcMapFieldHandlingMapStringMessage(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests protobuf map fields with string keys and message values. Validates proper key-value pair serialization and access patterns.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => "map-test-001", "data_map" => [["key" => "key1", "value" => "value1"], ["key" => "key2", "value" => "value2"], ["key" => "key3", "value" => "value3"]]]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MapService',
            methodName: 'ProcessMap',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcMapFieldHandlingMapStringMessage($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => "map-test-001", "map_count" => 3, "keys" => ["key1", "key2", "key3"]]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
