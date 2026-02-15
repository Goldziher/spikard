<?php
declare(strict_types=1);

use PHPUnit\Framework\TestCase;

final class GrpcErrorHandlingStreamErrorMidTransmissionTest extends TestCase
{
    public function testGrpcErrorHandlingStreamErrorMidTransmission(): void
    {
        if (!\class_exists('\\Spikard\\Grpc\\GrpcRequest')) {
            $this->markTestSkipped('gRPC support not available');
        }

        // Tests server streaming RPC that errors after yielding 3 messages. The stream opens successfully and delivers 3 messages before encountering an INTERNAL error. Validates that partial stream data is delivered before the error is signaled.

        // Build gRPC request from fixture
        $metadata = ["content-type" => "application/grpc"];
        $requestPayload = json_encode(["count" => 5]);

        $request = new \Spikard\Grpc\GrpcRequest(
            serviceName: 'example.v1.ErrorTestService',
            methodName: 'StreamWithError',
            payload: $requestPayload,
            metadata: $metadata,
        );

        // Call handler
        /** @var \Spikard\Grpc\GrpcResponse $response */
        $response = handleGrpcErrorHandlingStreamErrorMidTransmission($request);

        // Verify response
        /** @var string $statusCode */
        $statusCode = $response->statusCode;
        $this->assertSame('INTERNAL', $statusCode);

        /** @var mixed $metadata */
        $metadata = $response->metadata;
        $this->assertNotNull($metadata);
    }

}
