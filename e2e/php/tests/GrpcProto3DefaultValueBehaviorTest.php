<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcProto3DefaultValueBehaviorTest extends TestCase
{
    public function testGrpcProto3DefaultValueBehavior(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests how proto3 handles implicit default values. When fields are omitted from the request, response should reflect appropriate defaults.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["id" => 1]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.DefaultService',
            methodName: 'CheckDefaults',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcProto3DefaultValueBehavior($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["id" => 1, "name" => "", "active" => false, "has_id" => true]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
