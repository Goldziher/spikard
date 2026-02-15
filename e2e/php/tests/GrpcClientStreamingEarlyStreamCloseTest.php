<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingEarlyStreamCloseTest extends TestCase
{
    public function testGrpcClientStreamingEarlyStreamClose(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC where client closes stream after sending 3 messages instead of the expected 5. Server should gracefully handle partial stream.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'SendChunks',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingEarlyStreamClose($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["session_id" => "sess-early-001", "received_chunks" => 3, "expected_chunks" => 5, "status" => "INCOMPLETE"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
