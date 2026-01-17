<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcGoogleProtobufAnyTypeUsageTest extends TestCase
{
    public function testGrpcGoogleProtobufAnyTypeUsage(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests usage of google.protobuf.Any for storing arbitrary message types. Validates type URL encoding and message packing.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "any-test-001", "any_content" => "{\"type_url\": \"example.v1.Container\", \"value\": \"base64encodedvalue\"}"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AnyService',
            methodName: 'ProcessAny',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcGoogleProtobufAnyTypeUsage($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "any-test-001", "type_name" => "example.v1.Container", "success" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
