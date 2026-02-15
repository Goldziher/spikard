<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingMessageSizeLimitExceededTest extends TestCase
{
    public function testGrpcClientStreamingMessageSizeLimitExceeded(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC where one message exceeds the max_message_size limit. Server rejects the oversized message and terminates the stream.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "grpc-max-message-size" => "4096"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.PayloadService',
            methodName: 'ProcessPayloads',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingMessageSizeLimitExceeded($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('RESOURCE_EXHAUSTED', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["message_id" => "payload-002", "processed_count" => 1, "status" => "FAILED", "error_detail" => "Message payload size 10240 exceeds maximum allowed size 4096"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
