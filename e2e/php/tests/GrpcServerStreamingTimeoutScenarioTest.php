<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingTimeoutScenarioTest extends TestCase
{
    public function testGrpcServerStreamingTimeoutScenario(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that exceeds the deadline/timeout. The server starts streaming but doesn't complete before the client-imposed timeout expires. Validates proper timeout handling and stream cancellation.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc", "grpc-timeout" => "1000m"];
        $requestPayload = json_encode(["delay_ms" => 500, "message_count" => 10]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'StreamWithDelay',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingTimeoutScenario($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('DEADLINE_EXCEEDED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
