<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcClientStreaming10MessagesSumTest extends TestCase
{
    public function testGrpcClientStreaming10MessagesSum(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests client streaming RPC where client sends 10 integer values. Server sums all values and returns result.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode([]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.MathService',
            methodName: 'SumNumbers',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcClientStreaming10MessagesSum($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["sequence_id" => "seq-001", "count" => 10, "sum" => 550, "status" => "COMPLETE"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
