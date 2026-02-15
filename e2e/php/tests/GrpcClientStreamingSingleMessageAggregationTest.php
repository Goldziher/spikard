<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreamingSingleMessageAggregationTest extends TestCase
{
    public function testGrpcClientStreamingSingleMessageAggregation(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC where client sends a single message. Server acknowledges and returns aggregated result.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.AggregateService',
            methodName: 'AggregateData',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreamingSingleMessageAggregation($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["count" => 1, "total" => 42, "average" => 42.0, "status" => "AGGREGATED"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
