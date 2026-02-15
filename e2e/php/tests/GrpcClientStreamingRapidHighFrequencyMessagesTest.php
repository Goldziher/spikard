<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingRapidHighFrequencyMessagesTest extends TestCase
{
    public function testGrpcClientStreamingRapidHighFrequencyMessages(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC with rapid-fire message delivery. Server handles 50 messages in quick succession and returns aggregated metrics.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MetricsService',
            methodName: 'ProcessEvents',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingRapidHighFrequencyMessages($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["event_id" => "rapid-batch-001", "event_count" => 50, "min_value" => 0.1, "max_value" => 5.0, "avg_value" => 2.55, "throughput_mps" => 500.0, "status" => "PROCESSED"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
