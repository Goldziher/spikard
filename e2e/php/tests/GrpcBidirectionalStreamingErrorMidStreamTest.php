<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcBidirectionalStreamingErrorMidStreamTest extends TestCase
{
    public function testGrpcBidirectionalStreamingErrorMidStream(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests bidirectional streaming RPC where server returns error after processing some messages.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorService',
            methodName: 'ProcessWithError',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcBidirectionalStreamingErrorMidStream($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INTERNAL', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("Error after processing 2 messages"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
