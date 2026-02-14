<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingWithMetadataAndTrailersTest extends TestCase
{
    public function testGrpcServerStreamingWithMetadataAndTrailers(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC with gRPC metadata headers and trailers. Validates that metadata is accessible before streaming begins and trailers are delivered after stream completion.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "x-client-version" => "1.0.0", "x-request-id" => "metadata-stream-001"];
        $requestPayload = json_encode(["request_id" => "metadata-stream-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'StreamWithMetadata',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingWithMetadataAndTrailers($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("Stream completed with metadata and trailers"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
