<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcErrorHandlingDeadlineExceededTest extends TestCase
{
    public function testGrpcErrorHandlingDeadlineExceeded(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that exceeds deadline. Expects DEADLINE_EXCEEDED status when RPC time exceeds configured timeout.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["delay_ms" => 500]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorTestService',
            methodName: 'SlowStream',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingDeadlineExceeded($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('DEADLINE_EXCEEDED', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
