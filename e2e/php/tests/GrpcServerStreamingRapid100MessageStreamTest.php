<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcServerStreamingRapid100MessageStreamTest extends TestCase
{
    public function testGrpcServerStreamingRapid100MessageStream(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC with 100 messages sent in rapid succession. Validates backpressure handling, buffering, and delivery of high-volume message streams without loss or corruption.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["count" => 100]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.StreamService',
            methodName: 'StreamRapidMessages',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcServerStreamingRapid100MessageStream($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode("100 messages streamed successfully in sequence"), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
