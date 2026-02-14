<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingMidStreamErrorTest extends TestCase
{
    public function testGrpcServerStreamingMidStreamError(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that sends 5 messages successfully, then encounters an error before completing the stream. Validates partial stream delivery and error handling.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["stream_id" => "stream-001", "fail_after" => 5]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'StreamData',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingMidStreamError($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INTERNAL', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
