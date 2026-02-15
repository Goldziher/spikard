<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcBidirectionalStreamingEmptyRequestStreamTest extends TestCase
{
    public function testGrpcBidirectionalStreamingEmptyRequestStream(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests bidirectional streaming RPC with empty request stream but server sends response.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.EmptyService',
            methodName: 'HandleEmpty',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcBidirectionalStreamingEmptyRequestStream($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
