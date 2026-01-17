<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcRequestIdForDistributedTracingTest extends TestCase
{
    public function testGrpcRequestIdForDistributedTracing(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests request ID header propagation for distributed tracing. Validates X-Request-ID generation and propagation.

        // Build gRPC request from fixture
        $metadata = ["x-request-id" => "req-12345-67890", "content-type" => "application/grpc"];
        $requestPayload = json_encode(["operation" => "trace_test"]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.TracingService',
            methodName: 'Trace',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcRequestIdForDistributedTracing($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('OK', $statusCode);

        /** @var string $payload */
        $payload = $response->payload;
        $this->assertEquals(json_encode(["request_id" => "req-12345-67890"]), $payload);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
