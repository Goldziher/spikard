<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcCircuitBreakerTriggeredUnavailableWithMetadataTest extends TestCase
{
    public function testGrpcCircuitBreakerTriggeredUnavailableWithMetadata(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests UNAVAILABLE status code with circuit breaker metadata. Indicates service degradation and when to retry.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["request_id" => "circuit-001"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.DownstreamService',
            methodName: 'Query',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcCircuitBreakerTriggeredUnavailableWithMetadata($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('UNAVAILABLE', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
