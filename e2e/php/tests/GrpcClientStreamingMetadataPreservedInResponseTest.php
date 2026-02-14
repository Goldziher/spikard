<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingMetadataPreservedInResponseTest extends TestCase
{
    public function testGrpcClientStreamingMetadataPreservedInResponse(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC where request metadata is forwarded to and preserved in the response. Validates metadata propagation through streaming pipeline.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-user-id" => "user-789", "custom-header" => "custom-value", "authorization" => "Bearer token-xyz123", "x-trace-id" => "trace-abc456"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MetadataService',
            methodName: 'ProcessWithMetadata',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingMetadataPreservedInResponse($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "req-meta-001", "processed_by" => "grpc-handler-1", "received_user_id" => "user-789", "message_count" => 3, "trace_id" => "trace-abc456", "status" => "COMPLETE_WITH_METADATA"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
