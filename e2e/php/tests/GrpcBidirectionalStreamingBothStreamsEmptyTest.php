<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcBidirectionalStreamingBothStreamsEmptyTest extends TestCase
{
    public function testGrpcBidirectionalStreamingBothStreamsEmpty(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests bidirectional streaming RPC where both request and response streams are empty.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.EmptyBothService',
            methodName: 'Empty',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcBidirectionalStreamingBothStreamsEmpty($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
